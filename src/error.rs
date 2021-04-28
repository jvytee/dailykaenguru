#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    HttpError(reqwest::Error),
    TelegramError(telegram_bot::Error),
    VarError(std::env::VarError),
    ParseError(chrono::ParseError)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(error) => write!(f, "IO error: {}", error),
            Self::HttpError(error) => write!(f, "HTTP error: {}", error),
            Self::TelegramError(error) => write!(f, "Telegram error: {}", error),
            Self::VarError(error) => write!(f, "Variable error: {}", error),
            Self::ParseError(error) => write!(f, "Parse error: {}", error)
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

impl From<std::env::VarError> for Error {
    fn from(error: std::env::VarError) -> Self {
        Error::VarError(error)
    }
}

impl From<chrono::ParseError> for Error {
    fn from(error: chrono::ParseError) -> Self {
        Error::ParseError(error)
    }
}
