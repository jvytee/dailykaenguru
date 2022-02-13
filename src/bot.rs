use crate::download::Download;
use crate::error::Error;

use chrono::{Duration, Local, NaiveTime};
use std::collections::HashSet;
use std::fs::File;
use std::iter::FromIterator;
use std::sync::{Arc, Mutex};
use teloxide::{
    prelude::*,
    types::{ChatId, InputFile},
    utils::command::BotCommand,
    RequestError,
};
use tokio::time;

type ChatCache = Arc<Mutex<HashSet<ChatId>>>;

#[derive(BotCommand)]
#[command(rename = "lowercase")]
enum Command {
    #[command(description = "Startet den Bot")]
    Start,
    #[command(description = "Stoppt den Bot")]
    Stop,
}

#[derive(Clone)]
pub struct TelegramBot {
    pub token: String,
    pub cache_path: String,
    pub delivery_time: NaiveTime,
    pub download: Download,
}

impl TelegramBot {
    pub async fn run_forever(self) -> Result<(), Error> {
        let chat_ids = match self.load_chat_cache() {
            Ok(chat_ids) => chat_ids,
            Err(_) => HashSet::new(),
        };
        let chat_cache: ChatCache = Arc::new(Mutex::new(chat_ids));
        let bot = Bot::new(self.token.clone()).auto_send();

        {
            let wrapper = self.clone();
            let bot = bot.clone();
            let chat_cache = chat_cache.clone();
            tokio::spawn(async move {
                wrapper.deliver_comic(&bot, chat_cache).await;
            });
        }

        teloxide::commands_repl(bot, "DailyKaenguruBot", move |cx, command: Command| {
            let wrapper = self.clone();
            let chat_cache = chat_cache.clone();

            async move {
                match command {
                    Command::Start => wrapper.start_cmd(&cx, chat_cache).await,
                    Command::Stop => wrapper.stop_cmd(&cx, chat_cache).await,
                }?;
                respond(())
            }
        })
        .await;

        Ok(())
    }

    async fn deliver_comic(self, bot: &AutoSend<Bot>, chat_cache: ChatCache) {
        loop {
            time::sleep(self.time_remaining()).await;

            let comic = match self.download.get_comic(Local::now()).await {
                Ok(content) => InputFile::memory("känguru.jpg", content),
                Err(error) => {
                    log::warn!("Could not get comic: {}", error);
                    continue;
                }
            };

            let chats = match chat_cache.lock() {
                Ok(chats) => chats.clone(),
                Err(error) => {
                    log::warn!("Could not lock chat cache: {}", error);
                    continue;
                }
            };

            for chat in chats.iter() {
                let send_photo = bot.send_photo(chat.to_owned(), comic.to_owned());
                if let Err(error) = send_photo.await {
                    log::warn!("Could not deliver comic to {}: {}", chat, error);
                }
            }
        }
    }

    fn time_remaining(&self) -> time::Duration {
        let delivery_datetime = Local::today().and_time(self.delivery_time).unwrap();
        let duration = delivery_datetime.signed_duration_since(Local::now());

        match duration.to_std() {
            Ok(duration) => duration,
            Err(_) => (duration + Duration::hours(24)).to_std().unwrap(),
        }
    }

    async fn start_cmd(&self, cx: &UpdateWithCx<AutoSend<Bot>, Message>, chat_cache: ChatCache) -> Result<(), RequestError> {
        let chat_id = cx.chat_id();

        let answer = match chat_cache.lock() {
            Ok(mut chats) => {
                if chats.insert(chat_id.into()) {
                    if let Err(error) = self.dump_chat_cache(chats.clone()) {
                        log::error!("Could not dump chat cache: {}", error);
                    }
                    log::info!("Starting delivery to chat {}", chat_id);
                    cx.answer("Hallo!")
                } else {
                    log::info!("Already delivering to chat {}", chat_id);
                    cx.answer("Schon unterwegs!")
                }
            }
            Err(error) => {
                log::warn!("Could not lock chat cache: {}", error);
                cx.answer("Razupaltuff.")
            }
        };

        answer.await?;
        match self.download.get_comic(Local::now()).await {
            Ok(content) => {
                let comic = InputFile::memory("känguru.jpg", content);
                cx.answer_photo(comic).await?;
            }
            Err(error) => log::warn!("Could not get comic: {}", error),
        }

        Ok(())
    }

    async fn stop_cmd(&self, cx: &UpdateWithCx<AutoSend<Bot>, Message>, chat_cache: ChatCache) -> Result<(), RequestError> {
        let chat = &cx.update.chat;

        log::info!("Stopping delivery to chat {}", chat.id);
        if let Ok(mut chats) = chat_cache.lock() {
            chats.remove(&chat.id.into());
            if let Err(error) = self.dump_chat_cache(chats.clone()) {
                log::error!("Could not dump chat cache: {}", error);
            }
        }

        if chat.is_private() {
            log::debug!("Cannot leave private chat");
        } else {
            cx.requester.leave_chat(chat.id).await?;
        }

        cx.answer("Ciao!").await?;
        Ok(())
    }

    fn load_chat_cache(&self) -> Result<HashSet<ChatId>, std::io::Error> {
        let file = File::open(&self.cache_path)?;
        serde_json::from_reader(file)
            .map(|chat_ids: Vec<ChatId>| HashSet::from_iter(chat_ids))
            .map_err(|err| std::io::Error::from(err))
    }

    fn dump_chat_cache(&self, chat_cache: HashSet<ChatId>) -> Result<(), std::io::Error> {
        let chat_ids: Vec<ChatId> = Vec::from_iter(chat_cache);
        let file = File::create(&self.cache_path)?;
        serde_json::to_writer(file, &chat_ids).map_err(|err| std::io::Error::from(err))
    }
}
