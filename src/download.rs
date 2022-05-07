use crate::error::Error;

use chrono::prelude::*;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Download {
    pub data_path: String,
    pub base_url: String,
    pub filename: String,
}

impl Download {
    pub async fn get_comic(
        &self,
        datetime: DateTime<Local>,
    ) -> Result<Vec<u8>, Error> {
        return if let Some(comic) = self.load_comic(datetime)? {
            Ok(comic)
        } else {
            let comic = self.download_comic(datetime).await?;
            self.save_comic(&comic, datetime)?;
            Ok(comic)
        };
    }

    async fn download_comic(
        &self,
        datetime: DateTime<Local>,
    ) -> Result<Vec<u8>, reqwest::Error> {
        let url = format!(
            "{}/{}/{}/{}",
            &self.base_url,
            datetime.format("%Y-%m"),
            datetime.format("%d"),
            &self.filename
        );
        let response = reqwest::get(url.as_str()).await?;

        return match response.error_for_status() {
            Ok(res) => {
                let bytes = res.bytes().await?;
                Ok(bytes.to_vec())
            }
            Err(err) => Err(err),
        };
    }

    fn save_comic(
        &self,
        comic: &Vec<u8>,
        datetime: DateTime<Local>,
    ) -> Result<(), std::io::Error> {
        let filename = format!("kaenguru_{}.webp", datetime.format("%Y-%m-%d"));
        let filepath = Path::new(&self.data_path).join(filename);
        fs::write(filepath.as_path(), comic)
    }

    fn load_comic(
        &self,
        datetime: DateTime<Local>,
    ) -> Result<Option<Vec<u8>>, std::io::Error> {
        let filename = format!("kaenguru_{}.webp", datetime.format("%Y-%m-%d"));
        let filepath = Path::new(&self.data_path).join(filename);

        return if filepath.exists() {
            fs::read(filepath.as_path()).map(Some)
        } else {
            Ok(None)
        };
    }
}
