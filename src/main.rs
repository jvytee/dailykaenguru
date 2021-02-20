use std::env;
use telegram_bot::{
    prelude::*,
    Api,
    Error,
    Message,
    MessageKind,
    UpdateKind
};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Error> {
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
		    "/start" => start_cmd(&api, message).await,
		    "/stop" => stop_cmd(&api, message).await,
		    _ => ()
		}
	    }
	}
    }

    Ok(())
}

async fn start_cmd(api: &Api, message: Message) {
    let chat = message.chat;
    api.send(chat.text("Hallo!")).await;
}

async fn stop_cmd(api: &Api, message: Message) {
    let chat = message.chat;
    api.send(chat.text("Ciao!")).await;
    api.send(chat.leave()).await;
}
