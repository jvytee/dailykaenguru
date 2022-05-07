use chrono::{DateTime, Local};
use std::{
    collections::HashSet,
    fs::{self, File},
    path::PathBuf,
};
use teloxide::types::ChatId;

#[derive(Clone, Debug)]
pub struct Persistence {
    pub path: PathBuf,
    pub chat_ids_file: String
}

impl Persistence {
    pub fn load_chat_ids(&self) -> std::io::Result<HashSet<ChatId>> {
        let file = File::open(self.chat_ids_path())?;
        serde_json::from_reader::<File, HashSet<ChatId>>(file).map_err(std::io::Error::from)
    }

    pub fn save_chat_ids(&self, chat_ids: &HashSet<ChatId>) -> std::io::Result<()> {
        let file = File::create(self.chat_ids_path())?;
        serde_json::to_writer(file, chat_ids).map_err(std::io::Error::from)
    }

    pub fn load_comic(&self, dt: &DateTime<Local>) -> std::io::Result<Vec<u8>> {
        fs::read(self.comic_path(dt))
    }

    pub fn save_comic(&self, dt: &DateTime<Local>, comic: &Vec<u8>) -> std::io::Result<()> {
        fs::write(self.comic_path(dt), comic)
    }

    fn chat_ids_path(&self) -> PathBuf {
        self.path.join(&self.chat_ids_file)
    }

    fn comic_path(&self, dt: &DateTime<Local>) -> PathBuf {
        self.path.join(format!("kaenguru_{}.webp", dt.format("kaenguru_%Y-%m-%d.webp")))
    }
}
