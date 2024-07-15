use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Too many arguments")]
    TooManyArgs,
    #[error("Incorrect protocol \"{0}\"")]
    IncorrectProtocol(String),
    #[error("Incorrect video URL \"{0}\"")]
    IncorrectVideoURL(String),
    #[error("Dangerous video protocol \"{0}\"")]
    DangerousVideoProtocol(String),
    #[error("Player exited by error")]
    PlayerExited(u8),
    #[error("Failed to run player ({0})")]
    PlayerRunFailed(std::io::Error),
    #[error("Failed to decode ({0})")]
    FromBase64Error(#[from] base64::DecodeError),
    #[error("Failed to decode ({0})")]
    FromStringError(#[from] std::string::FromUtf8Error),
    #[error("Failed to decode ({0})")]
    FromTomlError(#[from] toml::de::Error),
    #[error("Failed to decode ({0})")]
    FromIoError(#[from] std::io::Error),
}
