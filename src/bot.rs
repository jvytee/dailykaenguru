use crate::download::Download;
use crate::persistence::Persistence;

use anyhow::Result;
use chrono::prelude::*;
use teloxide::{
    prelude::*,
    types::{ChatId, InputFile},
    utils::command::BotCommands,
};
use tokio::sync::mpsc;

#[derive(Clone, BotCommands)]
#[command(rename = "lowercase")]
enum Command {
    #[command(description = "Startet den Bot")]
    Start,
    #[command(description = "Stoppt den Bot")]
    Stop,
}

#[derive(Clone, Debug)]
enum Action {
    Add(ChatId),
    Remove(ChatId)
}

#[derive(Clone, Debug)]
struct CommandsRepl {
    sender: mpsc::Sender<Action>,
    persistence: Persistence,
    download: Download
}

pub async fn run_bot(bot: AutoSend<Bot>, persistence: Persistence, download: Download) -> Result<()> {
    let (sender, receiver) = mpsc::channel::<Action>(32);

    {
        let persistence = persistence.clone();
        tokio::spawn(async move {
            manage_data(persistence, receiver).await
        });
    }

    let commands_repl = CommandsRepl { sender, persistence, download };
    teloxide::commands_repl(
        bot,
        move |bot, message, command| {
            let commands_repl = commands_repl.clone();
            async move {
                commands_repl.answer(bot, message, command).await
            }
        },
        Command::ty()
    ).await;

    Ok(())
}

async fn manage_data(persistence: Persistence, mut receiver: mpsc::Receiver<Action>) {
    let mut chat_ids = persistence.load_chat_ids().unwrap_or_default();
    while let Some(message) = receiver.recv().await {
        match message {
            Action::Add(chat_id) => chat_ids.insert(chat_id),
            Action::Remove(chat_id) => chat_ids.remove(&chat_id)
        };

        if let Err(error) = persistence.save_chat_ids(&chat_ids) {
            log::error!("Could not save chat IDs: {}", error);
        }
    };
}

pub async fn deliver_comic(bot: &AutoSend<Bot>, persistence: &Persistence, download: &Download) -> Result<()> {
    let comic = get_comic(persistence, download, Local::now()).await
        .map(InputFile::memory)?;
    let chat_ids = persistence.load_chat_ids()?;
    for chat in chat_ids {
        let send_photo = bot.send_photo(chat, comic.clone());
        if let Err(error) = send_photo.await {
            log::warn!("Could not deliver comic to {}: {}", chat, error);
        }
    }

    Ok(())
}

async fn get_comic(persistence: &Persistence, download: &Download, datetime: DateTime<Local>) -> Result<Vec<u8>> {
    return if let Ok(comic) = persistence.load_comic(&datetime) {
        Ok(comic)
    } else {
        let comic = download.download_comic(&datetime).await?;
        persistence.save_comic(&datetime, &comic)?;
        Ok(comic)
    };
}

impl CommandsRepl {
    pub async fn answer (&self, bot: AutoSend<Bot>, message: Message, command: Command) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match command {
            Command::Start => self.start_cmd(bot, message).await?,
            Command::Stop => self.stop_cmd(bot, message).await?,
        }

        Ok(())
    }

    async fn start_cmd(&self, bot: AutoSend<Bot>, message: Message) -> Result<()> {
        let chat_id = message.chat.id;
        let action = Action::Add(chat_id);

        log::info!("Starting delivery to chat {}", chat_id);
        self.sender.send(action).await?;
        bot.send_message(chat_id, "Hallo!").await?;

        match get_comic(&self.persistence, &self.download, Local::now()).await {
            Ok(content) => {
                let comic = InputFile::memory(content);
                bot.send_photo(chat_id, comic).await?;
            }
            Err(error) => log::warn!("Could not get comic: {}", error),
        }

        Ok(())
    }

    async fn stop_cmd(&self, bot: AutoSend<Bot>, message: Message) -> Result<()> {
        let chat = message.chat;
        let action = Action::Remove(chat.id);

        log::info!("Stopping delivery to chat {}", chat.id);
        self.sender.send(action).await?;

        if chat.is_private() {
            log::debug!("Cannot leave private chat");
        } else {
            bot.leave_chat(chat.id).await?;
        }

        bot.send_message(chat.id, "Ciao!").await?;
        Ok(())
    }
}
