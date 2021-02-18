use std::env;
use telegram_bot::{
    Api,
    Error,
    MessageKind,
    UpdateKind,
};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token: String = env::var("DAILYKAENGURU_TOKEN").expect("Could not fetch DAILYKAENGURU_TOKEN environment variable");
    let api = Api::new(token);

    handle_updates(&api).await
}

async fn handle_updates(api: &Api) -> Result<(), Error> {
    let mut stream = api.stream();

    while let Some(update) = stream.next().await {
	let update = update.expect("Received broken update");

	if let UpdateKind::Message(message) = update.kind {
	    if let MessageKind::Text { ref data, .. } = message.kind {
		match data.as_str() {
		    "/start" => println!("{} started the bot", message.from.username.unwrap_or("Someone".to_string())),
		    "/stop" => println!("{} stopeed the bot", message.from.username.unwrap_or("Someone".to_string())),
		    _ => ()
		}
	    }
	}
    }

    Ok(())
}
