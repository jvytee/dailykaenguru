mod bot;
mod download;
mod error;

use crate::bot::TelegramBot;

use chrono::prelude::*;
use download::Download;
use error::Error;
use getopts::Options;
use std::env;
use std::path::Path;

#[tokio::main]
async fn main() {
    env_logger::init();

    if let Err(error) = run().await {
        log::error!("{}", error);
    };
}

async fn run() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("h", "help", "Show help");
    opts.optflag("d", "download", "Download latest comic");

    if let Ok(matches) = opts.parse(&args[1..]) {
        if matches.opt_present("h") {
            let brief = format!("Usage: {} [OPTIONS]", &args[0]);
            println!("{}", opts.usage(&brief));
            return Ok(());
        }

        let data_path: String = env::var("DAILYKAENGURU_DATA")?;
        let download = Download {
            data_path: data_path.clone(),
            base_url: "https://img.zeit.de/administratives/kaenguru-comics".to_string(),
            filename: "original".to_string(),
        };

        if matches.opt_present("d") {
            log::info!("Downloading latest comic");
            let datetime = Local::now();

            download.get_comic(datetime).await?;
        } else {
            log::info!("Starting telegram bot");
            let token: String = env::var("DAILYKAENGURU_TOKEN")?;

            let delivery_time = env::var("DAILYKAENGURU_DELIVERY")
                .map(|delivery_string| NaiveTime::parse_from_str(&delivery_string, "%H:%M"))
                .unwrap_or(Ok(NaiveTime::from_hms(9, 30, 0)))?;

            let cache_path = Path::new(&data_path)
                .join("chats.json")
                .to_str()
                .unwrap_or("chats.json")
                .to_string();

            let telegram_bot = TelegramBot {
                token,
                cache_path,
                delivery_time,
                download,
            };
            telegram_bot.run_forever().await?;
        }
    }

    Ok(())
}
