use thiserror::Error;

#[derive(Error, Debug)]
enum HandlerError {
    #[error("No argument is given")]
    NoArg,
    #[error("Too many arguments are given")]
    ManyArgs,
    #[error("Wrong argument is given")]
    WrongArg,
    #[error("Wrong protocol url is given")]
    WrongProtocol,
    #[error("Wrong protocol url is given, base64 decoding failed")]
    WrongProtocolBase64(#[from] base64::DecodeError),
    #[error("Wrong protocol url is given, string converting failed")]
    WrongProtocolFromUtf8(#[from] std::string::FromUtf8Error),
    #[error("Player executable binary not found")]
    PlayerNotFound,
    #[error("Player exited with error or termination signal")]
    PlayerExited,
    #[error("Handler get current execution path failed")]
    HandlerPath(#[from] std::io::Error),
}

#[derive(Debug)]
struct Handler {
    url: String,
    quality: String,
    cookies: String,
}

impl Handler {
    fn new() -> Handler {
        Handler {
            url: String::new(),
            quality: String::new(),
            cookies: String::new(),
        }
    }

    /// Parse the commandline arguments
    ///
    /// ## Errors
    ///
    /// - `WrongArg`
    ///   When the `arg` not handler protocol url or `version | -v | -V`
    fn init(&mut self) -> Result<(), HandlerError> {
        let mut args: Vec<String> = std::env::args().collect();
        let arg: &mut String = match args.len() {
            2 => &mut args[1],
            1 => return Err(HandlerError::NoArg),
            _ => return Err(HandlerError::ManyArgs),
        };

        if arg.starts_with("mpv://") {
            return self.parse(arg);
        }

        match arg.as_str() {
            "version" | "-v" | "-V" => self.version(),
            _ => Err(HandlerError::WrongArg),
        }
    }

    /// Print `mpv-handler` version infomation
    fn version(&self) -> Result<(), HandlerError> {
        let version: &str = option_env!("MPV_HANDLER_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"));

        println!("mpv-handler {}", version);

        Ok(())
    }

    /// Parse the protocol url
    ///
    /// ## Errors
    ///
    /// - `WrongProtocol`
    ///   When `options_name` not definition in match list or options value not found
    fn parse(&mut self, arg: &mut String) -> Result<(), HandlerError> {
        if arg.starts_with("mpv://") {
            arg.replace_range(0.."mpv.//".len(), "");
        }

        let protocol: Vec<&str> = arg.split("/?").collect();

        match protocol.get(0) {
            Some(data) => self.get_url(data)?,
            None => return Err(HandlerError::WrongProtocol),
        };

        match protocol.get(1) {
            Some(data) => {
                let options: Vec<&str> = data.split('&').collect();

                for option in options {
                    let option_data: Vec<&str> = option.split('=').collect();
                    let option_name: &str = match option_data.get(0) {
                        Some(name) => *name,
                        None => return Err(HandlerError::WrongProtocol),
                    };
                    let option_value: &str = match option_data.get(1) {
                        Some(value) => *value,
                        None => return Err(HandlerError::WrongProtocol),
                    };

                    match option_name {
                        "quality" => self.get_quality(option_value)?,
                        "cookies" => self.get_cookies(option_value)?,
                        _ => return Err(HandlerError::WrongProtocol),
                    }
                }
            }
            None => {}
        };

        self.play()
    }

    /// Play the videos with player
    ///
    /// For Unix, player executable binary is `mpv`
    ///
    /// For Windows, player executable binary is `mpv.com`
    ///
    /// ## Errors
    ///
    /// - `PlayerNotFound`
    ///   When player executable binary not found
    /// - `PlayerExited`
    ///   When player exited with non-zero code or termination signal
    fn play(&self) -> Result<(), HandlerError> {
        #[cfg(unix)]
        const MPV_BIN: &str = "mpv";

        #[cfg(windows)]
        const MPV_BIN: &str = "mpv.com";

        let mut args: Vec<&String> = Vec::new();

        if self.quality.len() != 0 {
            args.push(&self.quality);
        }
        if self.cookies.len() != 0 {
            args.push(&self.cookies);
        }
        if self.url.len() != 0 {
            args.push(&self.url);
        }

        println!("Playing: {}", self.url);

        let mpv = std::process::Command::new(MPV_BIN).args(args).status();

        match mpv {
            Ok(status) => match status.success() {
                true => Ok(()),
                false => Err(HandlerError::PlayerExited),
            },
            Err(_) => Err(HandlerError::PlayerNotFound),
        }
    }

    /// Get the video url from base64 encoded data
    ///
    /// ## Errors
    ///
    /// - `WrongProtocol`
    ///   When the length of `data` equal zero
    /// - `WrongProtocolBase64`
    ///   When base64 decoding failed
    /// - `WrongProtocolFromUtf8`
    ///   When converting utf-8 bytes to string failed
    fn get_url(&mut self, data: &&str) -> Result<(), HandlerError> {
        match data.len() {
            0 => Err(HandlerError::WrongProtocol),
            _ => Ok(self.url = String::from_utf8(base64::decode(data)?)?),
        }
    }

    /// Get the quality parameter from option value
    ///
    /// ## Errors
    ///
    /// - `WrongProtocol`
    ///   When `option_value` not definition in match list
    fn get_quality(&mut self, option_value: &str) -> Result<(), HandlerError> {
        let quality: &str = match option_value {
            "best" => "bestvideo+bestaudio/best",
            "4k" => "bestvideo[height<=2160]+bestaudio/best[height<=2160]/best",
            "2k" => "bestvideo[height<=1440]+bestaudio/best[height<=1440]/best",
            "1080p" => "bestvideo[height<=1080]+bestaudio/best[height<=1080]/best",
            "720p" => "bestvideo[height<=720]+bestaudio/best[height<=720]/best",
            _ => return Err(HandlerError::WrongProtocol),
        };

        self.quality.push_str("--ytdl-format=");
        self.quality.push_str(quality);

        Ok(())
    }

    /// Get cookies file path
    ///
    /// For Unix, cookies storage at:
    /// `~/.config/mpv/cookies/`
    ///
    /// For Windows, cookies storage at:
    /// `(mpv-handler.exe parent directory)\cookies\`
    ///
    /// ## Errors
    ///
    /// - `WrongProtocol`
    ///   When `option_value` not `yes` or `no`.
    fn get_cookies(&mut self, option_value: &str) -> Result<(), HandlerError> {
        match option_value {
            "yes" => {
                let mut url: String = self.url.clone();

                if let Some(index) = url.find("://") {
                    url.replace_range(0..index + 3, "");
                }
                if let Some(index) = url.find('/') {
                    url.replace_range(index.., "");
                }

                let mut path: std::path::PathBuf;

                #[cfg(unix)]
                {
                    path = std::path::PathBuf::from("~");
                    path.push(".config");
                    path.push("mpv");
                    path.push("cookies");
                    path.push(url.as_str());
                }

                #[cfg(windows)]
                {
                    path = std::env::current_exe()?;
                    path.pop();
                    path.push("cookies");
                    path.push(url.as_str());
                }

                if let Some(cookies) = path.to_str() {
                    self.cookies.push_str("--ytdl-raw-options-append=cookies=");
                    self.cookies.push_str(cookies);
                    self.cookies.push_str(".txt");
                };

                Ok(())
            }
            "no" => Ok(()),
            _ => Err(HandlerError::WrongProtocol),
        }
    }
}

fn main() {
    let mut handler = Handler::new();

    match handler.init() {
        Ok(_) => {}
        Err(error) => {
            eprintln!("{}", error);

            std::io::Read::read(&mut std::io::stdin(), &mut [0]).unwrap();
            std::process::exit(1);
        }
    }
}
