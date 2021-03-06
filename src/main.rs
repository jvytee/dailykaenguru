mod bot;
mod download;

use chrono::prelude::*;
use download::DownloadConfig;
use getopts::Options;
use std::env;

#[tokio::main]
async fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("h", "help", "Show help");
    opts.optflag("d", "download", "Download latest comic");

    if let Ok(matches) = opts.parse(&args[1..]) {
        if matches.opt_present("h") {
            let brief = format!("Usage: {} [OPTIONS]", &args[0]);
            println!("{}", opts.usage(&brief));
            return;
        }

        let data_path: String = env::var("DAILYKAENGURU_DATA")
            .expect("Could not fetch DAILYKAENGURU_DATA environment variable");
	let download_config = DownloadConfig {
	    data_path: data_path,
	    base_url: "https://img.zeit.de/administratives/kaenguru-comic".to_string(),
	    filename: "original".to_string()
	};

        if matches.opt_present("d") {
            log::info!("Downloading latest comic");
            let datetime = Local::now();

            if let Err(err) = download::get_comic(
                datetime,
		&download_config
            )
            .await
            {
                log::error!("Could not get latest comic: {}", err);
            }
        } else {
            log::info!("Starting telegram bot");
            let token: String = env::var("DAILYKAENGURU_TOKEN")
                .expect("Could not fetch DAILYKAENGURU_TOKEN environment variable");

            if let Err(err) = bot::handle_updates(token, download_config).await {
                log::error!("Could not handle updates: {}", err);
            }
        }
    }
}
