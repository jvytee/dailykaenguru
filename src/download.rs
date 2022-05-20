use chrono::prelude::*;

#[derive(Clone, Debug)]
pub struct Download {
    pub base_url: String,
    pub filename: String,
}

impl Download {
    pub async fn download_comic(&self, datetime: &DateTime<Local>) -> Result<Vec<u8>, reqwest::Error> {
        log::info!("Downloading comic for {}", datetime);

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
}
