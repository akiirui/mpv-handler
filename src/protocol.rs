use thiserror::Error;

const DEFAULT_DOWNLOADER: &str = "mpv";

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("Error: Wrong protocol URL")]
    WrongProtocol,
    #[error("Error: Failed to decode video URL data, {0}")]
    WrongProtocolBase64(#[from] base64::DecodeError),
    #[error("Error: Failed to convert video URL string, {0}")]
    WrongProtocolFromUtf8(#[from] std::string::FromUtf8Error),
    #[error("Error: Not found video URL")]
    MissingVideoUrl,
}

#[derive(Debug)]
pub struct Protocol {
    pub cookies: String,
    pub downloader: String,
    pub quality: String,
    pub url: String,
}

impl Protocol {
    /// Parse the protocol URL
    ///
    /// ## Errors
    ///
    /// - `WrongProtocol`
    /// - `MissingVideoUrl`
    /// - `MissingDownloader`
    pub fn parse(arg: &mut String) -> Result<Protocol, ProtocolError> {
        if arg.starts_with("mpv://") {
            arg.replace_range(0.."mpv.//".len(), "");
        } else {
            return Err(ProtocolError::WrongProtocol);
        }

        if arg.ends_with('/') {
            arg.pop();
        }

        let args: Vec<&str> = arg.split("/?").collect();
        let mut protocol = Protocol {
            cookies: String::new(),
            downloader: String::from(DEFAULT_DOWNLOADER),
            quality: String::new(),
            url: String::new(),
        };

        match args.get(0) {
            Some(data) => protocol.url = decode_url(data)?,
            None => return Err(ProtocolError::MissingVideoUrl),
        };

        match args.get(1) {
            Some(data) => {
                let options: Vec<&str> = data.split('&').collect();

                for option in options {
                    let option_data: Vec<&str> = option.split('=').collect();

                    let option_name: &str = match option_data.get(0) {
                        Some(name) => *name,
                        None => return Err(ProtocolError::WrongProtocol),
                    };

                    let option_value: &str = match option_data.get(1) {
                        Some(value) => *value,
                        None => return Err(ProtocolError::WrongProtocol),
                    };

                    match option_name {
                        "cookies" => protocol.cookies = option_value.to_string(),
                        "downloader" => protocol.downloader = option_value.to_string(),
                        "quality" => protocol.quality = option_value.to_string(),
                        _ => return Err(ProtocolError::WrongProtocol),
                    }
                }
            }
            None => {}
        };

        // Check required options are already exists
        if protocol.url.len() == 0 {
            return Err(ProtocolError::MissingVideoUrl);
        }

        Ok(protocol)
    }
}

/// Get the video url from base64 encoded data
///
/// ## Errors
///
/// - `MissingVideoUrl`
/// - `WrongProtocolBase64`
/// - `WrongProtocolFromUtf8`
fn decode_url(data: &&str) -> Result<String, ProtocolError> {
    match data.len() {
        0 => Err(ProtocolError::MissingVideoUrl),
        _ => Ok(String::from_utf8(base64::decode(data)?)?),
    }
}
