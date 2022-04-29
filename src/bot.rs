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
    utils::command::BotCommands,
    RequestError,
};
use tokio::time;

type ChatCache = Arc<Mutex<HashSet<ChatId>>>;

#[derive(Clone, BotCommands)]
#[command(rename = "lowercase")]
enum Command {
    #[command(description = "Startet den Bot")]
    Start,
    #[command(description = "Stoppt den Bot")]
    Stop,
}

pub async fn run_bot(bot: AutoSend<Bot>, cache_path: String, delivery_time: NaiveTime, download: Download) -> Result<(), Error> {
    let chat_ids = match load_chat_cache(&cache_path) {
        Ok(chat_ids) => chat_ids,
        Err(_) => HashSet::new(),
    };
    let chat_cache: ChatCache = Arc::new(Mutex::new(chat_ids));

    {
        let bot = bot.clone();
        let chat_cache = Arc::clone(&chat_cache);
        let download = download.clone();

        tokio::spawn(async move {
            deliver_comic(&bot, chat_cache, download, delivery_time).await;
        });
    }

    teloxide::commands_repl(
        bot,
        move |bot: AutoSend<Bot>, message: Message, command: Command| {
            let chat_cache = chat_cache.clone();
            let cache_path = cache_path.clone();
            let download = download.clone();

            async move {
                match command {
                    Command::Start => start_cmd(bot, message, chat_cache, cache_path, download).await,
                    Command::Stop => stop_cmd(bot, message, chat_cache, cache_path).await,
                }?;
                respond(())
            }
        },
        Command::ty(),
    )
    .await;

    Ok(())
}

async fn deliver_comic(bot: &AutoSend<Bot>, chat_cache: ChatCache, download: Download, delivery_time: NaiveTime) {
    loop {
        time::sleep(time_remaining(delivery_time)).await;

        let comic = match download.get_comic(Local::now()).await {
            Ok(content) => InputFile::memory(content),
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
            let send_photo = bot.send_photo(*chat, comic.clone());
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
        Err(_) => (duration + Duration::hours(24)).to_std().unwrap(),
    }
}

async fn start_cmd(
    bot: AutoSend<Bot>,
    message: Message,
    chat_cache: ChatCache,
    cache_path: String,
    download: Download
) -> Result<(), RequestError> {
    let chat_id = message.chat.id;

    let answer = match chat_cache.lock() {
        Ok(mut chats) => {
            if chats.insert(chat_id) {
                if let Err(error) = dump_chat_cache(&cache_path, chats.clone()) {
                    log::error!("Could not dump chat cache: {}", error);
                }
                log::info!("Starting delivery to chat {}", chat_id);
                bot.send_message(chat_id, "Hallo!")
            } else {
                log::info!("Already delivering to chat {}", chat_id);
                bot.send_message(chat_id, "Schon unterwegs!")
            }
        }
        Err(error) => {
            log::warn!("Could not lock chat cache: {}", error);
            bot.send_message(chat_id, "Razupaltuff.")
        }
    };

    answer.await?;
    match download.get_comic(Local::now()).await {
        Ok(content) => {
            let comic = InputFile::memory(content);
            bot.send_photo(chat_id, comic).await?;
        }
        Err(error) => log::warn!("Could not get comic: {}", error),
    }

    Ok(())
}

async fn stop_cmd(
    bot: AutoSend<Bot>,
    message: Message,
    chat_cache: ChatCache,
    cache_path: String
) -> Result<(), RequestError> {
    let chat = message.chat;

    log::info!("Stopping delivery to chat {}", chat.id);
    if let Ok(mut chats) = chat_cache.lock() {
        chats.remove(&chat.id);
        if let Err(error) = dump_chat_cache(&cache_path, chats.clone()) {
            log::error!("Could not dump chat cache: {}", error);
        }
    }

    if chat.is_private() {
        log::debug!("Cannot leave private chat");
    } else {
        bot.leave_chat(chat.id).await?;
    }

    bot.send_message(chat.id, "Ciao!").await?;
    Ok(())
}

fn load_chat_cache(cache_path: &str) -> Result<HashSet<ChatId>, std::io::Error> {
    let file = File::open(cache_path)?;
    serde_json::from_reader::<File, Vec<ChatId>>(file)
        .map(HashSet::from_iter)
        .map_err(std::io::Error::from)
}

fn dump_chat_cache(cache_path: &str, chat_cache: HashSet<ChatId>) -> Result<(), std::io::Error> {
    let chat_ids: Vec<ChatId> = Vec::from_iter(chat_cache);
    let file = File::create(cache_path)?;
    serde_json::to_writer(file, &chat_ids).map_err(std::io::Error::from)
}
