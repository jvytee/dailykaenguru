mod bot;
mod download;
mod error;
mod persistence;

use anyhow::Result;
use chrono::prelude::*;
use download::Download;
use getopts::Options;
use std::env;
use std::path::PathBuf;
use teloxide::prelude::*;

use crate::persistence::Persistence;

#[tokio::main]
async fn main() {
    env_logger::init();

    if let Err(error) = run().await {
        log::error!("{}", error);
    };
}

async fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("h", "help", "Show help");
    opts.optflag("d", "deliver", "Deliver comic to recipients");
    opts.optflag("D", "download", "Download latest comic");

    let matches = opts.parse(&args[1..])?;

    if matches.opt_present("h") {
        let brief = format!("Usage: {} [OPTIONS]", &args[0]);
        println!("{}", opts.usage(&brief));
        return Ok(());
    }

    let data_path: PathBuf = env::var("DAILYKAENGURU_DATA").map(PathBuf::from)?;
    let download = Download {
        base_url: "https://img.zeit.de/administratives/kaenguru-comics".to_string(),
        filename: "original".to_string(),
    };

    if matches.opt_present("D") {
        log::info!("Downloading latest comic");
        download.download_comic(Local::now()).await?;
        return Ok(());
    }

    let token: String = env::var("DAILYKAENGURU_TOKEN")?;
    let teloxide_bot = Bot::new(token).auto_send();
    let persistence = Persistence { path: data_path, chat_ids_file: "chats.json".into() };

    if matches.opt_present("d") {
        bot::deliver_comic(&teloxide_bot, &persistence, &download).await?;
        return Ok(());
    }

    log::info!("Starting telegram bot");
    bot::run_bot(teloxide_bot, persistence, download).await?;

    Ok(())
}
