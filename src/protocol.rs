use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("Wrong protocol url is given")]
    WrongProtocol,
    #[error("Wrong protocol url is given, base64 decoding failed")]
    WrongProtocolBase64(#[from] base64::DecodeError),
    #[error("Wrong protocol url is given, string converting failed")]
    WrongProtocolFromUtf8(#[from] std::string::FromUtf8Error),
    #[error("Downloader is not given in the protocol URL")]
    MissingDownloader,
    #[error("Video URL is not given in the protocol URL")]
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
    ///     - Protocol URL isn't start with `mpv://`
    ///     - Protocol URL `option=value` format is wrong
    ///     - Protocol URL is contains options are not listed in `option_name`
    /// - `MissingVideoUrl`
    ///     - Protocol URL isn't contains video URL data
    /// - `MissingDownloader`
    ///     - Protocol URL option `downloader` is not given
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
            downloader: String::new(),
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
        if protocol.downloader.len() == 0 {
            return Err(ProtocolError::MissingDownloader);
        }

        Ok(protocol)
    }
}

/// Get the video url from base64 encoded data
///
/// ## Errors
///
/// - `MissingVideoUrl`
///     - When the length of `data` equal zero
/// - `WrongProtocolBase64`
///     - When base64 decoding failed
/// - `WrongProtocolFromUtf8`
///     - When converting utf-8 bytes to string failed
fn decode_url(data: &&str) -> Result<String, ProtocolError> {
    match data.len() {
        0 => Err(ProtocolError::MissingVideoUrl),
        _ => Ok(String::from_utf8(base64::decode(data)?)?),
    }
}
