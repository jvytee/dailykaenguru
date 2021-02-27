use chrono::prelude::*;
use std::fs;
use std::path::Path;


pub async fn get_comic(datetime: DateTime<Local>, data_path: &str, base_url: &str, filename: &str) -> Result<Vec<u8>, Error> {
    return if let Some(comic) = load_comic(datetime, data_path)? {
	Ok(comic)
    } else {
	let comic = download_comic(datetime, base_url, filename).await?;
	save_comic(comic.clone(), datetime, data_path)?;
	Ok(comic)
    }
}


async fn download_comic(datetime: DateTime<Local>, base_url: &str, filename: &str) -> Result<Vec<u8>, reqwest::Error> {
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


fn save_comic(comic: Vec<u8>, datetime: DateTime<Local>, data_path: &str) -> Result<(), std::io::Error> {
    let filename = format!("kaenguru_{}.webp", datetime.format("%Y-%m-%d"));
    let filepath = Path::new(data_path).join(filename);
    fs::write(filepath.as_path(), comic)
}


fn load_comic(datetime: DateTime<Local>, data_path: &str) -> Result<Option<Vec<u8>>, std::io::Error> {
    let filename = format!("kaenguru_{}.webp", datetime.format("%Y-%m-%d"));
    let filepath = Path::new(data_path).join(filename);

    return if filepath.exists() {
	fs::read(filepath.as_path()).map(|bytes| Some(bytes))
    } else {
	Ok(None)
    }
}


#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    HttpError(reqwest::Error)
}


impl std::error::Error for Error {}


impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	match self {
	    Self::IoError(io_error) => write!(f, "IO error: {}", io_error.to_string()),
	    Self::HttpError(http_error) => write!(f, "HTTP error: {}", http_error.to_string())
	}
    }
}


impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
	Error::IoError(error)
    }
}


impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
	Error::HttpError(error)
    }
}
