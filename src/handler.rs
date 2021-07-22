use thiserror::Error;

use crate::config::Config;
use crate::protocol::Protocol;

const PLAYER: &str = "mpv";

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error(transparent)]
    Config(#[from] crate::config::ConfigError),
    #[error(transparent)]
    Protocol(#[from] crate::protocol::ProtocolError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("No argument is given")]
    NoArg,
    #[error("Too many arguments are given")]
    TooManyArgs,
    #[error("Get home directory failed")]
    GetHomeDirFailed,
    #[error("The downloader settings are broken in the configure file")]
    WrongDownloaderConfig,
    #[error("The downloader settings are broken in the configure file")]
    WrongPlayerConfig,
    #[error("Player or downloader executable binary not found")]
    DownloaderNotFound,
    #[error("Player or downloader exited with error or termination signal")]
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
    /// Read configure file and parse protocol URL
    ///
    /// ## Errors
    ///
    /// - `NoArg`
    ///    - No argument is given
    /// - `TooManyArgs`
    ///    - 2 or more arguments is given
    /// - `GetHomeDirFailed`
    ///    - (Unix only) Get user home directory failed
    /// - `ConfigError`
    ///    - Transparent from `Config::ConfigError`
    /// - `ProtocolError`
    ///    - Transparent from `Protocol::ProtocolError`
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
            path.push("mpv-handler.toml");

            dbg!(&path);

            config = Config::read(path)?;
        }

        #[cfg(windows)]
        {
            path = std::env::current_exe()?;
            path.pop();
            path.push("mpv-handler.toml");

            dbg!(&path);

            config = Config::read(path)?;
        }

        protocol = Protocol::parse(arg)?;

        dbg!(&config, &protocol);

        Ok(Handler {
            config: config,
            protocol: protocol,
        })
    }

    /// Generate arguments for downloader or player
    ///
    /// ## Errors
    ///
    /// - `WrongDownloaderConfig`
    ///     - Missing downloader table in configure file
    ///     - The downloader key value is empty
    /// - `WrongPlayerConfig`
    ///     - Missing player key in configure file
    ///     - The player key value is empty
    /// - `IoError`
    ///     - Transparent from `std::io::Error`

    pub fn run(&self) -> Result<(), HandlerError> {
        // Downloader Arguments
        let mut args: Vec<&String> = Vec::new();
        let mut cookies: String;

        if !self
            .config
            .downloader
            .contains_key(&self.protocol.downloader)
        {
            return Err(HandlerError::WrongDownloaderConfig);
        }

        // Append video URL
        args.push(&self.protocol.url);

        // Append cookies option and cookies file path
        if self.protocol.cookies.len() != 0
            && self.config.downloader[&self.protocol.downloader]
                .cookies
                .len()
                != 0
        {
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
        if self.protocol.quality.len() != 0
            && self.config.downloader[&self.protocol.downloader]
                .quality
                .contains_key(&self.protocol.quality)
        {
            if self.config.downloader[&self.protocol.downloader].quality[&self.protocol.quality]
                .len()
                == 0
            {
                return Err(HandlerError::WrongDownloaderConfig);
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

        if self.config.downloader[&self.protocol.downloader].bin.len() == 0 {
            return Err(HandlerError::WrongDownloaderConfig);
        }

        if !self.config.player.contains_key(PLAYER) || self.config.player[PLAYER].len() == 0 {
            return Err(HandlerError::WrongPlayerConfig);
        }

        dbg!(&args);

        if self.config.downloader[&self.protocol.downloader].direct == false {
            return self.play(
                args,
                &self.config.downloader[&self.protocol.downloader].bin,
                &self.config.player[PLAYER],
            );
        } else {
            return self.play_direct(args, &self.config.downloader[&self.protocol.downloader].bin);
        }
    }

    /// Run downloader and set player
    ///
    /// ## Errors
    ///
    /// - `DownloaderExited`
    ///     - The download or player exited with non-zero code or termination signal
    /// - `DownloaderNotFound`
    ///     - The downloader or player executable binary not found
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

    /// Run player directly (mpv include ytdl-hooks)
    ///
    /// ## Errors
    ///
    /// - `DownloaderExited`
    ///     - The download or player exited with non-zero code or termination signal
    /// - `DownloaderNotFound`
    ///     - The downloader or player executable binary not found
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
}

/// Print `mpv-handler` version infomation
fn version() {
    let version: &str = option_env!("MPV_HANDLER_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"));

    println!("mpv-handler {}", version);

    std::process::exit(0)
}
