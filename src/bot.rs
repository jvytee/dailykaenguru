use crate::download::{self, DownloadConfig};
use crate::error::Error;

use chrono::{Duration, Local, NaiveTime};
use std::collections::HashSet;
use std::fs::File;
use std::iter::FromIterator;
use std::sync::{Arc, Mutex};
use teloxide::{RequestError, prelude::*, types::{
        ChatId,
        InputFile,
    }, utils::command::BotCommand};
use tokio::time;

type ChatCache = Arc<Mutex<HashSet<ChatId>>>;

pub async fn start_bot(
    token: String,
    config: DownloadConfig,
    delivery_time: NaiveTime,
    cache_path: String
) -> Result<(), Error> {
    let chat_ids = match load_chat_cache(&cache_path) {
        Ok(chat_ids) => chat_ids,
        Err(_) => HashSet::new(),
    };
    let chat_cache: ChatCache = Arc::new(Mutex::new(chat_ids));
    let bot = Bot::new(token).auto_send();

    {
        let bot= bot.clone();
        let chat_cache= chat_cache.clone();
        tokio::spawn(async move {
            deliver_comic(&bot, chat_cache, delivery_time, &config).await;
        });
    }

    teloxide::commands_repl(bot, "DailyKaenguruBot", move |cx, command: Command| {
        let chat_cache = chat_cache.clone();
        let cache_path = cache_path.clone();

        async move {
            match command {
                Command::Start => start_cmd(&cx, &chat_cache, &cache_path).await,
                Command::Stop => stop_cmd(&cx, &chat_cache, &cache_path).await
            }?;
            respond(())
        }
    }).await;

    Ok(())
}

async fn deliver_comic(
    bot: &AutoSend<Bot>,
    chat_cache: ChatCache,
    delivery_time: NaiveTime,
    config: &DownloadConfig,
) {
    loop {
        time::sleep(time_remaining(delivery_time)).await;

        let comic = match download::get_comic(Local::now(), &config).await {
            Ok(content) => InputFile::memory("känguru.jpg", content),
            Err(error) => {
                log::warn!("Could not get comic: {}", error);
                continue;
            }
        };

        let chats = match chat_cache.lock() {
            Ok(chats) => chats.clone(),
            Err(error) => {
                log::warn!("Cannot lock chat cache: {}", error);
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

fn time_remaining(delivery_time: NaiveTime) -> time::Duration {
    let delivery_datetime = Local::today().and_time(delivery_time).unwrap();
    let duration = delivery_datetime.signed_duration_since(Local::now());

    match duration.to_std() {
        Ok(duration) => duration,
        Err(_) => (duration + Duration::hours(24)).to_std().unwrap()
    }
}

#[derive(BotCommand)]
#[command(rename = "lowercase")]
enum Command {
    #[command(description = "Startet den Bot")]
    Start,
    #[command(description = "Stoppt den Bot")]
    Stop
}

async fn start_cmd(cx: &UpdateWithCx<AutoSend<Bot>, Message>, chat_cache: &ChatCache, cache_path: &str) -> Result<(), RequestError> {
    let chat_id = cx.chat_id();

    let answer = match chat_cache.lock() {
        Ok(mut chats) => {
            if chats.insert(chat_id.into()) {
                if let Err(error) = dump_chat_cache(cache_path, chats.clone()) {
                    log::error!("Could not dump chat cache: {}", error);
                }
                log::info!("Starting delivery to chat {}", chat_id);
                "Hallo!"
            } else {
                log::info!("Already delivering to chat {}", chat_id);
                "Schon unterwegs!"
            }
        }
        Err(error) => {
            log::warn!("Could not lock chat cache: {}", error);
            "Razupaltuff."
        }
    };

    cx.answer(answer).await?;
    Ok(())
}

async fn stop_cmd(cx: &UpdateWithCx<AutoSend<Bot>, Message>, chat_cache: &ChatCache, cache_path: &str) -> Result<(), RequestError> {
    let chat = &cx.update.chat;

    log::info!("Stopping delivery to chat {}", chat.id);
    if let Ok(mut chats) = chat_cache.lock() {
        chats.remove(&chat.id.into());
        if let Err(error) = dump_chat_cache(cache_path, chats.clone()) {
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

fn load_chat_cache(file_path: &str) -> Result<HashSet<ChatId>, std::io::Error> {
    let file = File::open(file_path)?;
    serde_json::from_reader(file)
        .map(|chat_ids: Vec<ChatId>| HashSet::from_iter(chat_ids))
        .map_err(|err| std::io::Error::from(err))
}

fn dump_chat_cache(file_path: &str, chat_cache: HashSet<ChatId>) -> Result<(), std::io::Error> {
    let chat_ids: Vec<ChatId> = Vec::from_iter(chat_cache);
    let file = File::create(file_path)?;
    serde_json::to_writer(file, &chat_ids).map_err(|err| std::io::Error::from(err))
}
