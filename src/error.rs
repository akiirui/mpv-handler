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
    // Only the macOS URL service opens Terminal, so the variant is limited to
    // that target to keep it from being reported as unused elsewhere.
    #[cfg(target_os = "macos")]
    #[error("Failed to open Terminal ({0})")]
    TerminalRunFailed(String),
    #[error("Failed to decode ({0})")]
    FromBase64Error(#[from] base64::DecodeError),
    #[error("Failed to decode ({0})")]
    FromStringError(#[from] std::string::FromUtf8Error),
    #[error("Failed to decode ({0})")]
    FromTomlError(#[from] toml::de::Error),
    #[error("Failed to decode ({0})")]
    FromIoError(#[from] std::io::Error),
}
