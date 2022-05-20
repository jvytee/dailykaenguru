mod bot;
mod download;
mod persistence;

use anyhow::Result;
use chrono::prelude::*;
use download::Download;
use getopts::Options;
use serde::Deserialize;
use std::env;
use std::path::PathBuf;
use teloxide::prelude::*;

use crate::persistence::Persistence;

#[derive(Clone, Debug, Deserialize)]
struct Config {
    data_path: PathBuf,
    chats_file: String,
    token: String,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    if let Err(error) = run().await {
        log::error!("{}", error);
    };
}

async fn run() -> Result<()> {
    let mut opts = Options::new();
    opts.optflag("h", "help", "Show help");
    opts.optflag("d", "deliver", "Deliver comic to recipients");
    opts.optflag("D", "download", "Download latest comic");

    let args: Vec<String> = env::args().collect();
    let matches = opts.parse(&args[1..])?;

    if matches.opt_present("h") {
        let brief = format!("Usage: {} [OPTIONS]", &args[0]);
        println!("{}", opts.usage(&brief));
        return Ok(());
    }

    let config: Config = envy::prefixed("KAENGURU_").from_env()?;
    let download = Download {
        base_url: "https://img.zeit.de/administratives/kaenguru-comics".to_string(),
        filename: "original".to_string(),
    };
    let persistence = Persistence { path: config.data_path, chat_ids_file: config.chats_file };

    if matches.opt_present("D") {
        log::info!("Downloading latest comic");
        let datetime = Local::now();
        let comic = download.download_comic(&datetime).await?;
        persistence.save_comic(&datetime, &comic)?;
        return Ok(());
    }

    let teloxide_bot = Bot::new(config.token).auto_send();

    if matches.opt_present("d") {
        bot::deliver_comic(&teloxide_bot, &persistence, &download).await?;
        return Ok(());
    }

    log::info!("Starting telegram bot");
    bot::run_bot(teloxide_bot, persistence, download).await?;

    Ok(())
}
