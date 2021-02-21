use log::{
    debug,
    info
};
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


pub async fn handle_updates(token: String) -> Result<(), Error> {
    let api = Api::new(token);
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

