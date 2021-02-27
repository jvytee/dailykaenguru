use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use telegram_bot::{prelude::*, Api, Error, Message, MessageChat, MessageKind, UpdateKind};
use tokio_stream::StreamExt;

type ChatCache = Arc<Mutex<HashSet<MessageChat>>>;

pub async fn handle_updates(token: String) -> Result<(), Error> {
    let api = Api::new(token);
    let mut stream = api.stream();
    let chat_cache: ChatCache = Arc::new(Mutex::new(HashSet::new()));

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
            if chats.insert(chat.clone()) {
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
        chats.remove(&chat);
    }

    match chat {
        MessageChat::Private(_) => log::debug!("Cannot leave private chat"),
        _ => api.send(chat.leave()).await?,
    }

    api.send(chat.text("Ciao!")).await?;
    Ok(())
}
