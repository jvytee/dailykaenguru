#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    HttpError(reqwest::Error),
    TelegramError(telegram_bot::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(error) => write!(f, "IO error: {}", error.to_string()),
            Self::HttpError(error) => write!(f, "HTTP error: {}", error.to_string()),
            Self::TelegramError(error) => write!(f, "Telegram error: {}", error.to_string())
        }
    }
}

impl std::error::Error for Error {}

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

impl From<telegram_bot::Error> for Error {
    fn from(error: telegram_bot::Error) -> Self {
        Error::TelegramError(error)
    }
}
