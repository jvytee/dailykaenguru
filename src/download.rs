use chrono::prelude::*;
use std::fs;
use std::io::Error;
use std::path::Path;


pub async fn download_comic(datetime: DateTime<Local>, base_url: &str, filename: &str) -> Result<Vec<u8>, reqwest::Error> {
    let url = format!("{}/{}/{}/{}", base_url, datetime.format("%Y-%m"), datetime.format("%d"), filename);
    let response = reqwest::get(url.as_str()).await?;

    return match response.error_for_status() {
	Ok(res) => {
	    let bytes = res.bytes().await?;
	    Ok(bytes.to_vec())
	},
	Err(err) => Err(err)
    }
}


pub fn save_comic(comic: Vec<u8>, datetime: DateTime<Local>, data_path: &str) -> Result<(), Error> {
    let filename = format!("kaenguru_{}.webp", datetime.format("%Y-%m-%d"));
    let filepath = Path::new(data_path).join(filename);
    fs::write(filepath.as_path(), comic)
}


pub fn load_comic(datetime: DateTime<Local>, data_path: &str) -> Result<Option<Vec<u8>>, Error> {
    let filename = format!("kaenguru_{}.webp", datetime.format("%Y-%m-%d"));
    let filepath = Path::new(data_path).join(filename);

    return if filepath.exists() {
	fs::read(filepath.as_path()).map(|bytes| Some(bytes))
    } else {
	Ok(None)
    }
}
