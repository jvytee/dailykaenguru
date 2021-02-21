use log::{
    debug,
    info
};
use std::env;
use telegram_bot::{
    prelude::*,
    Api,
    Error,
    Message,
    MessageChat,
    MessageKind,
    UpdateKind
};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let token: String = env::var("DAILYKAENGURU_TOKEN").expect("Could not fetch DAILYKAENGURU_TOKEN environment variable");
    let api = Api::new(token);

    handle_updates(api).await
}

async fn handle_updates(api: Api) -> Result<(), Error> {
    let mut stream = api.stream();

    while let Some(update) = stream.next().await {
	let update = update?;

	if let UpdateKind::Message(message) = update.kind {
	    if let MessageKind::Text { ref data, .. } = message.kind {
		match data.as_str() {
		    "/start" => start_cmd(&api, message).await?,
		    "/stop" => stop_cmd(&api, message).await?,
		    _ => ()
		}
	    }
	}
    }

    Ok(())
}

async fn start_cmd(api: &Api, message: Message) -> Result<(), Error> {
    let username = message.from.username.unwrap_or("people".to_string());
    let chat = message.chat;

    info!("Starting delivery to {} in chat {}", username, chat.id());
    api.send(chat.text("Hallo!")).await?;

    Ok(())
}

async fn stop_cmd(api: &Api, message: Message) -> Result<(), Error> {
    let username = message.from.username.unwrap_or("people".to_string());
    let chat = message.chat;

    info!("Stopping delivery to {} in chat {}", username, chat.id());
    api.send(chat.text("Ciao!")).await?;

    match chat {
	MessageChat::Private(_) => debug!("Cannot leave private chat"),
	_ => api.send(chat.leave()).await?
    }

    Ok(())
}


async fn download_comic(datetime: OffsetDateTime, base_url: &str, filename: &str) -> Result<Vec<u8>, reqwest::Error> {
    let url = format!("{}/{}/{}/{}", base_url, datetime.format("%Y-%m"), datetime.format("%d"), filename);
    let response = reqwest::get(url.as_str()).await?;
    let bytes = response.bytes().await?;
    Ok(bytes.to_vec())
}
