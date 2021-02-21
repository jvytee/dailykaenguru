mod bot;

use getopts::Options;
use log::{
    debug,
    info
};
use std::env;
use time::OffsetDateTime;


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
	
	let data_path: String = env::var("DAILYKAENGURU_DATA").expect("Could not fetch DAILYKAENGURU_DATA environment variable");
	if matches.opt_present("d") {
	    let datetime = OffsetDateTime::now_utc();
	    let comic = download_comic(datetime, "https://img.zeit.de/administratives/kaenguru-comics", "original").await;
	} else {
	    let token: String = env::var("DAILYKAENGURU_TOKEN").expect("Could not fetch DAILYKAENGURU_TOKEN environment variable");
	    bot::handle_updates(token).await;
	}
    }
}


async fn download_comic(datetime: OffsetDateTime, base_url: &str, filename: &str) -> Result<Vec<u8>, reqwest::Error> {
    let url = format!("{}/{}/{}/{}", base_url, datetime.format("%Y-%m"), datetime.format("%d"), filename);
    let response = reqwest::get(url.as_str()).await?;
    let bytes = response.bytes().await?;
    Ok(bytes.to_vec())
}
