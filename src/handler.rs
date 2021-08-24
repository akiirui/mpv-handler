use thiserror::Error;

use crate::config::{Config, PlayMode};
use crate::protocol::Protocol;

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
    #[error("Error: Failed to get config directory")]
    FailedGetConfigDir,
    #[error("Error: The downloader \"{0}\" requires a quality LEVEL")]
    DownloaderRequireQuality(String),
    #[error("Error: Downloader or player exited with error or termination signal")]
    DownloaderExited,
    #[error("Error: Failed to run downloader \"{0}\": {1}")]
    FailedRunDownloader(String, std::io::Error),
    #[error("Error: Failed to run player: {0}")]
    FailedRunPlayer(std::io::Error),
}

#[derive(Debug)]
pub struct Handler {
    protocol: Protocol,
    config: Config,
}

impl Handler {
    /// Generate a Handler
    pub fn new() -> Result<Self, HandlerError> {
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

        let protocol: Protocol;
        let config: Config;

        protocol = Protocol::parse(arg)?;
        config = Config::load()?;

        Ok(Handler { protocol, config })
    }

    /// Prepare arguments and run downloader & player
    pub fn run(&self) -> Result<(), HandlerError> {
        let mut downloader_options: Vec<&String> = Vec::new();
        let downloader = self.config.downloader(&self.protocol.downloader)?;

        // Append cookies option and cookies file path to arguments
        let mut cookies_path: String;

        if self.protocol.cookies.len() != 0 {
            let mut path: std::path::PathBuf;
            let cookies = downloader.cookies(&self.protocol.downloader)?;

            #[cfg(unix)]
            {
                path = match dirs::config_dir() {
                    Some(path) => path,
                    None => return Err(HandlerError::FailedGetConfigDir),
                };
                path.push("mpv-handler");
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

            cookies_path = path.as_path().display().to_string();

            if downloader.cookies_prefix {
                cookies_path.insert_str(0, &cookies);
                downloader_options.push(&cookies_path);
            } else {
                downloader_options.push(&cookies);
                downloader_options.push(&cookies_path);
            }
        }

        // Append quality option
        if self.protocol.quality.len() != 0 {
            let quality = downloader.quality(&self.protocol.downloader, &self.protocol.quality)?;

            downloader_options.push(quality)
        } else if downloader.require_quality {
            return Err(HandlerError::DownloaderRequireQuality(
                self.protocol.downloader.clone(),
            ));
        }

        // Append output or player options
        for option in &downloader.options {
            downloader_options.push(option);
        }

        // Choose downloader play mode
        let play_mode = downloader.play_mode()?;
        let bin = downloader.bin(&self.protocol.downloader)?;
        let player = self.config.player()?;
        let player_options = &downloader.player_options;

        #[cfg(unix)]
        {
            self.set_environment()?;
        }

        println!("Playing: {}", self.protocol.url);

        match play_mode {
            PlayMode::Direct => self.play_direct(&bin, downloader_options),
            PlayMode::Normal => self.play(&bin, &player, downloader_options, player_options),
            PlayMode::Pipe => self.play_pipe(&bin, &player, downloader_options, player_options),
        }
    }

    /// Set environment variables for player and downloader if needed (Unix only)
    #[cfg(unix)]
    fn set_environment(&self) -> Result<(), HandlerError> {
        // Fix google-chrome overwrite "LD_LIBRARY_PATH" on Linux
        // It will be let mpv exit with error:
        // mpv: symbol lookup error: mpv: undefined symbol: vkCreateWaylandSurfaceKHR
        std::env::remove_var("LD_LIBRARY_PATH");

        // Set "LD_LIBRARY_PATH" if "ld_path" option is given in "custom.toml"
        if let Some(ld_path) = self.config.ld_path()? {
            std::env::set_var("LD_LIBRARY_PATH", ld_path);
        }

        Ok(())
    }

    /// Run downloader directly (mpv has ytdl-hooks)
    fn play_direct(
        &self,
        bin: &String,
        downloader_options: Vec<&String>,
    ) -> Result<(), HandlerError> {
        let downloader = std::process::Command::new(bin)
            .args(downloader_options)
            .arg(&self.protocol.url)
            .status();

        match downloader {
            Ok(status) => match status.success() {
                true => Ok(()),
                false => Err(HandlerError::DownloaderExited),
            },
            Err(error) => Err(HandlerError::FailedRunDownloader(
                self.protocol.downloader.clone(),
                error,
            )),
        }
    }

    /// Run downloader and set player
    fn play(
        &self,
        bin: &String,
        player: &String,
        downloader_options: Vec<&String>,
        player_options: &Vec<String>,
    ) -> Result<(), HandlerError> {
        let mut player = player.clone();

        for option in player_options {
            player.push(' ');
            player.push_str(option);
        }

        let downloader = std::process::Command::new(bin)
            .args(downloader_options)
            .arg(player)
            .arg(&self.protocol.url)
            .status();

        match downloader {
            Ok(status) => match status.success() {
                true => Ok(()),
                false => Err(HandlerError::DownloaderExited),
            },
            Err(error) => Err(HandlerError::FailedRunDownloader(
                self.protocol.downloader.clone(),
                error,
            )),
        }
    }

    /// Run downloader and transfer video data through pipeline
    fn play_pipe(
        &self,
        downloader_bin: &String,
        player_bin: &String,
        downloader_options: Vec<&String>,
        player_options: &Vec<String>,
    ) -> Result<(), HandlerError> {
        let downloader = match std::process::Command::new(downloader_bin)
            .args(downloader_options)
            .arg(&self.protocol.url)
            .stdout(std::process::Stdio::piped())
            .spawn()
        {
            Ok(child) => child,
            Err(error) => {
                return Err(HandlerError::FailedRunDownloader(
                    self.protocol.downloader.clone(),
                    error,
                ))
            }
        };

        let player = std::process::Command::new(player_bin)
            .args(player_options)
            .arg("-")
            .stdin(downloader.stdout.unwrap())
            .status();

        match player {
            Ok(status) => match status.success() {
                true => Ok(()),
                false => Err(HandlerError::DownloaderExited),
            },
            Err(error) => Err(HandlerError::FailedRunPlayer(error)),
        }
    }
}

/// Print `mpv-handler` version infomation
fn version() {
    let version: &str = option_env!("MPV_HANDLER_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"));

    println!("mpv-handler {}", version);

    std::process::exit(0)
}
