use thiserror::Error;

use crate::config::Config;
use crate::protocol::Protocol;

const PLAYER: &str = "mpv";
const CONFIG_FILE_NAME: &str = "mpv-handler.toml";

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error(transparent)]
    Config(#[from] crate::config::ConfigError),
    #[error(transparent)]
    Protocol(#[from] crate::protocol::ProtocolError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Error: No argument is given")]
    NoArg,
    #[error("Error: Too many arguments are given")]
    TooManyArgs,
    #[cfg(unix)]
    #[error("Error: Get user home directory failed")]
    GetHomeDirFailed,
    #[error("Error: The player \"{0}\" value is empty")]
    ConfigPlayerEmptyValue(String),
    #[error("Error: The downloader \"{0}\" settings was not found")]
    ConfigDownloaderNotFound(String),
    #[error("Error: The downloader \"{0}\" bin value is empty")]
    ConfigDownloaderBinEmptyValue(String),
    #[error("Error: The downloader \"{0}\" cookies value is empty, but you passed cookies")]
    ConfigDownloaderCookiesEmptyValue(String),
    #[error("Error: The downloader \"{0}\" requires a quality LEVEL")]
    ConfigDownloaderRequireQuality(String),
    #[error("Error: The downloader \"{0}\" quality \"{1}\" was not found")]
    ConfigDownloaderQualityNotFound(String, String),
    #[error("Error: The downloader \"{0}\" quailty \"{1}\" value is empty")]
    ConfigDownloaderQualityEmptyValue(String, String),
    #[error("Error: Downloader or player executable binary not found, check your configuration")]
    DownloaderNotFound,
    #[error("Error: Downloader or player exited with error or termination signal")]
    DownloaderExited,
}

#[derive(Debug)]
pub struct Handler {
    config: Config,
    protocol: Protocol,
}

impl Handler {
    /// Generate Handler struct
    ///
    /// Read configuration file and parse protocol URL
    ///
    /// ## Errors
    ///
    /// - `ConfigError`
    /// - `ProtocolError`
    /// - `NoArg`
    /// - `TooManyArgs`
    /// - `GetHomeDirFailed` (unix only)
    pub fn new() -> Result<Handler, HandlerError> {
        let mut args: Vec<String> = std::env::args().collect();
        let arg: &mut String = match args.len() {
            2 => &mut args[1],
            1 => return Err(HandlerError::NoArg),
            _ => return Err(HandlerError::TooManyArgs),
        };

        match arg.as_str() {
            "version" | "-v" | "-V" => version(),
            _ => {}
        };

        let config: Config;
        let protocol: Protocol;
        let mut path: std::path::PathBuf;

        #[cfg(unix)]
        {
            path = match dirs::home_dir() {
                Some(path) => path,
                None => return Err(HandlerError::GetHomeDirFailed),
            };
            path.push(".config");
            path.push("mpv");
            path.push(CONFIG_FILE_NAME);
        }

        #[cfg(windows)]
        {
            path = std::env::current_exe()?;
            path.pop();
            path.push(CONFIG_FILE_NAME);
        }

        config = Config::read(path)?;
        protocol = Protocol::parse(arg)?;

        Ok(Handler {
            config: config,
            protocol: protocol,
        })
    }

    /// Generate arguments for downloader or player
    ///
    /// ## Errors
    ///
    /// - `IoError`
    /// - `ConfigPlayerNotFound`
    /// - `ConfigPlayerEmptyValue`
    /// - `ConfigDownloaderNotFound`
    /// - `ConfigDownloaderBinEmptyValue`
    /// - `ConfigDownloaderCookiesEmptyValue`
    /// - `ConfigDownloaderQualityNotFound`
    /// - `ConfigDownloaderQualityEmptyValue`
    pub fn run(&self) -> Result<(), HandlerError> {
        let mut args: Vec<&String> = Vec::new();
        let mut cookies: String;

        if !self
            .config
            .downloader
            .contains_key(&self.protocol.downloader)
        {
            return Err(HandlerError::ConfigDownloaderNotFound(
                self.protocol.downloader.clone(),
            ));
        }

        if self.config.downloader[&self.protocol.downloader].bin.len() == 0 {
            return Err(HandlerError::ConfigDownloaderBinEmptyValue(
                self.protocol.downloader.clone(),
            ));
        }

        if self.config.player.len() == 0 {
            return Err(HandlerError::ConfigPlayerEmptyValue(PLAYER.to_string()));
        }

        // Append video URL to arguments
        {
            args.push(&self.protocol.url);
        }

        // Append cookies option and cookies file path to arguments
        if self.protocol.cookies.len() != 0 {
            if self.config.downloader[&self.protocol.downloader]
                .cookies
                .len()
                == 0
            {
                return Err(HandlerError::ConfigDownloaderCookiesEmptyValue(
                    self.protocol.downloader.clone(),
                ));
            }

            let mut path: std::path::PathBuf;

            #[cfg(unix)]
            {
                path = std::path::PathBuf::from("~");
                path.push(".config");
                path.push("mpv");
                path.push("cookies");
                path.push(&self.protocol.cookies);
            }

            #[cfg(windows)]
            {
                path = std::env::current_exe()?;
                path.pop();
                path.push("cookies");
                path.push(&self.protocol.cookies);
            }

            cookies = path.as_path().display().to_string();

            if self.config.downloader[&self.protocol.downloader].cookies_prefix == false {
                args.push(&self.config.downloader[&self.protocol.downloader].cookies);
                args.push(&cookies);
            } else {
                cookies.insert_str(
                    0,
                    &self.config.downloader[&self.protocol.downloader].cookies,
                );
                args.push(&cookies);
            }
        }

        // Append quality option
        if self.config.downloader[&self.protocol.downloader].require_quality == true {
            if self.protocol.quality.len() == 0 {
                return Err(HandlerError::ConfigDownloaderRequireQuality(
                    self.protocol.downloader.clone(),
                ));
            }
        }

        if self.protocol.quality.len() != 0 {
            if !self.config.downloader[&self.protocol.downloader]
                .quality
                .contains_key(&self.protocol.quality)
            {
                return Err(HandlerError::ConfigDownloaderQualityNotFound(
                    self.protocol.downloader.clone(),
                    self.protocol.quality.clone(),
                ));
            }

            if self.config.downloader[&self.protocol.downloader].quality[&self.protocol.quality]
                .len()
                == 0
            {
                return Err(HandlerError::ConfigDownloaderQualityEmptyValue(
                    self.protocol.downloader.clone(),
                    self.protocol.quality.clone(),
                ));
            }

            args.push(
                &self.config.downloader[&self.protocol.downloader].quality[&self.protocol.quality],
            )
        }

        // Append output or player options
        if self.config.downloader[&self.protocol.downloader]
            .options
            .len()
            != 0
        {
            for option in &self.config.downloader[&self.protocol.downloader].options {
                args.push(option);
            }
        }

        if self.config.downloader[&self.protocol.downloader].direct == true {
            return self.play_direct(args, &self.config.downloader[&self.protocol.downloader].bin);
        }

        if self.config.downloader[&self.protocol.downloader].pipeline == false {
            return self.play(
                args,
                &self.config.downloader[&self.protocol.downloader].bin,
                &self.config.player,
            );
        } else {
            return self.play_pipeline(
                args,
                &self.config.downloader[&self.protocol.downloader].bin,
                &self.config.player,
            );
        }
    }

    /// Run downloader and set player
    ///
    /// ## Errors
    ///
    /// - `DownloaderExited`
    /// - `DownloaderNotFound`
    fn play(
        &self,
        args: Vec<&String>,
        downloader_bin: &String,
        player_bin: &String,
    ) -> Result<(), HandlerError> {
        println!("Playing: {}", self.protocol.url);

        let downloader = std::process::Command::new(downloader_bin)
            .args(args)
            .arg(player_bin)
            .status();

        match downloader {
            Ok(status) => match status.success() {
                true => Ok(()),
                false => Err(HandlerError::DownloaderExited),
            },
            Err(_) => Err(HandlerError::DownloaderNotFound),
        }
    }

    /// Run downloader directly (mpv has ytdl-hooks)
    ///
    /// ## Errors
    ///
    /// - `DownloaderExited`
    /// - `DownloaderNotFound`
    fn play_direct(&self, args: Vec<&String>, downloader_bin: &String) -> Result<(), HandlerError> {
        println!("Playing: {}", self.protocol.url);

        let downloader = std::process::Command::new(downloader_bin)
            .args(args)
            .status();

        match downloader {
            Ok(status) => match status.success() {
                true => Ok(()),
                false => Err(HandlerError::DownloaderExited),
            },
            Err(_) => Err(HandlerError::DownloaderNotFound),
        }
    }

    /// Run downloader transfer video data through pipeline
    ///
    /// ## Errors
    ///
    /// - `IoError`
    /// - `DownloaderExited`
    /// - `DownloaderNotFound`
    fn play_pipeline(
        &self,
        args: Vec<&String>,
        downloader_bin: &String,
        player_bin: &String,
    ) -> Result<(), HandlerError> {
        println!("Playing: {}", self.protocol.url);

        let downloader = std::process::Command::new(downloader_bin)
            .args(args)
            .stdout(std::process::Stdio::piped())
            .spawn()?;

        let player = std::process::Command::new(player_bin)
            .arg("-")
            .stdin(downloader.stdout.unwrap())
            .status();

        match player {
            Ok(status) => match status.success() {
                true => Ok(()),
                false => Err(HandlerError::DownloaderExited),
            },
            Err(_) => Err(HandlerError::DownloaderNotFound),
        }
    }
}

/// Print `mpv-handler` version infomation
fn version() {
    let version: &str = option_env!("MPV_HANDLER_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"));

    println!("mpv-handler {}", version);

    std::process::exit(0)
}
