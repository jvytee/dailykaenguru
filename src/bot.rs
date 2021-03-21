use crate::download::{self, DownloadConfig};

use chrono::{Duration, Local, NaiveTime};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use telegram_bot::{
    prelude::*, Api, ChatId, Error, InputFileUpload, Message, MessageChat, MessageKind, SendPhoto,
    UpdateKind,
};
use tokio::time;
use tokio_stream::StreamExt;

type ChatCache = Arc<Mutex<HashSet<ChatId>>>;

async fn deliver_comic(
    api: &Api,
    chat_cache: ChatCache,
    delivery_time: NaiveTime,
    config: &DownloadConfig,
) {
    loop {
        let sleep_duration = time::Duration::from_secs(seconds_remaining(delivery_time));
        time::sleep(sleep_duration).await;

        let comic = match download::get_comic(Local::now(), &config).await {
            Ok(content) => InputFileUpload::with_data(content, "känguru.jpg"),
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
            let send_photo = SendPhoto::new(chat, &comic);
            let _ = api
                .send(send_photo)
                .await
                .map_err(|error| log::warn!("Could not deliver comic: {}", error));
        }
    }
}

fn seconds_remaining(delivery_time: NaiveTime) -> u64 {
    let delivery_datetime = Local::today().and_time(delivery_time).unwrap();
    let duration = delivery_datetime.signed_duration_since(Local::now());

    if duration.num_seconds() < 0 {
        (duration + Duration::hours(24)).num_seconds() as u64
    } else {
        duration.num_seconds() as u64
    }
}

pub async fn handle_updates(
    token: String,
    config: DownloadConfig,
    delivery_time: NaiveTime,
) -> Result<(), Error> {
    let chat_cache: ChatCache = Arc::new(Mutex::new(HashSet::new()));
    let api = Api::new(token);
    let mut stream = api.stream();

    let api_copy = api.clone();
    let chat_cache_copy = chat_cache.clone();
    tokio::spawn(async move {
        deliver_comic(&api_copy, chat_cache_copy, delivery_time, &config).await;
    });

    while let Some(update) = stream.next().await {
        let update = update?;

        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                match data.as_str() {
                    "/start" => start_cmd(&api, chat_cache.clone(), message).await?,
                    "/stop" => stop_cmd(&api, chat_cache.clone(), message).await?,
                    _ => (),
                }
            }
        }
    }

    Ok(())
}

async fn start_cmd(api: &Api, chat_cache: ChatCache, message: Message) -> Result<(), Error> {
    let username = message.from.username.unwrap_or("people".to_string());
    let chat = message.chat;

    match chat_cache.lock() {
        Ok(mut chats) => {
            if chats.insert(chat.id()) {
                log::info!("Starting delivery to {} in chat {}", username, chat.id());
                api.send(chat.text("Hallo!")).await?;
            } else {
                log::info!("Already delivering to {} in chat {}", username, chat.id());
                api.send(chat.text("Schon unterwegs!")).await?;
            }
        }
        Err(error) => {
            log::warn!("Could not lock chat cache: {}", error);
            api.send(chat.text("Razupaltuff")).await?;
        }
    }

    Ok(())
}

async fn stop_cmd(api: &Api, chat_cache: ChatCache, message: Message) -> Result<(), Error> {
    let username = message.from.username.unwrap_or("people".to_string());
    let chat = message.chat;

    log::info!("Stopping delivery to {} in chat {}", username, chat.id());
    if let Ok(mut chats) = chat_cache.lock() {
        chats.remove(&chat.id());
    }

    match chat {
        MessageChat::Private(_) => log::debug!("Cannot leave private chat"),
        _ => api.send(chat.leave()).await?,
    }

    api.send(chat.text("Ciao!")).await?;
    Ok(())
}
