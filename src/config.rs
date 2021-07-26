use serde::Deserialize;
use thiserror::Error;

use std::{collections::HashMap, path::PathBuf};

const DEFAULT_CONFIG_FILE: &str = "config.toml";
const CUSTOM_CONFIG_FILE: &str = "custom.toml";

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Error: Failed to decode config file:\n{0}: {1}")]
    FailedDeserializeToml(PathBuf, toml::de::Error),
    #[error("Error: Failed to read config file:\n{0}: {1}")]
    FailedReadConfig(PathBuf, std::io::Error),
    #[cfg(unix)]
    #[error("Error: Failed to get config directory")]
    FailedGetConfigDir,
    #[error("Error: The player value is empty")]
    PlayerEmptyValue,
    #[error("Error: The downloader \"{0}\" settings is not found")]
    DownloaderNotFound(String),
    #[error("Error: The downloader \"{0}\" bin value is empty")]
    DownloaderBinEmptyValue(String),
    #[error("Error: The downloader \"{0}\" cookies value is empty, but you passed cookies")]
    DownloaderCookiesEmptyValue(String),
    #[error("Error: The downloader \"{0}\" quality \"{1}\" is not found")]
    DownloaderQualityNotFound(String, String),
    #[error("Error: The downloader \"{0}\" quality \"{1}\" value is empty")]
    DownloaderQualityEmptyValue(String, String),
    #[error("Error: The downloader \"{0}\" play mode is wrong")]
    DownloaderWrongPlayMode(String),
}
#[derive(Debug, Deserialize)]
pub struct Config {
    player: String,
    #[serde(flatten)]
    downloader: HashMap<String, Downloader>,
}

#[derive(Debug, Default, Deserialize)]
pub struct Downloader {
    bin: String,
    #[serde(default)]
    cookies: String,
    #[serde(default)]
    pub cookies_prefix: bool,
    #[serde(default)]
    pub require_quality: bool,
    #[serde(default = "Config::default_play_mode")]
    play_mode: String,
    #[serde(default)]
    pub options: Vec<String>,
    #[serde(default)]
    quality: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct CustomConfig {
    player: Option<String>,
    #[serde(flatten)]
    downloader: HashMap<String, CustomDownloader>,
}

#[derive(Debug, Deserialize)]
struct CustomDownloader {
    bin: Option<String>,
    cookies: Option<String>,
    cookies_prefix: Option<bool>,
    require_quality: Option<bool>,
    play_mode: Option<String>,
    options: Option<Vec<String>>,
    quality: Option<HashMap<String, String>>,
}

#[derive(Debug)]
pub enum PlayMode {
    Normal,
    Direct,
    Pipe,
}

impl Config {
    /// Load configs and merge default and custom (if it exist)
    pub fn load() -> Result<Config, ConfigError> {
        let default: Config = load_default()?;

        match load_custom()? {
            Some(custom) => merge(default, custom),
            None => Ok(default),
        }
    }

    /// Return the player binary path
    pub fn player(&self) -> Result<&String, ConfigError> {
        match self.player.len() != 0 {
            true => Ok(&self.player),
            false => Err(ConfigError::PlayerEmptyValue),
        }
    }

    /// Return the downloader from the given name
    pub fn downloader(&self, downloader: &String) -> Result<&Downloader, ConfigError> {
        match self.downloader.get(downloader) {
            Some(v) => Ok(v),
            None => Err(ConfigError::DownloaderNotFound(downloader.clone())),
        }
    }

    /// The default value of play_mode
    fn default_play_mode() -> String {
        "normal".to_string()
    }
}

impl Downloader {
    /// Return the downloader binary path
    pub fn bin(&self, downloader: &String) -> Result<&String, ConfigError> {
        match self.bin.len() != 0 {
            true => Ok(&self.bin),
            false => Err(ConfigError::DownloaderBinEmptyValue(downloader.clone())),
        }
    }

    /// Return the downloader cookies option
    pub fn cookies(&self, downloader: &String) -> Result<&String, ConfigError> {
        match self.cookies.len() != 0 {
            true => Ok(&self.cookies),
            false => Err(ConfigError::DownloaderCookiesEmptyValue(downloader.clone())),
        }
    }

    /// Return the downloader play mode
    pub fn play_mode(&self) -> Result<PlayMode, ConfigError> {
        match self.play_mode.as_str() {
            "normal" => Ok(PlayMode::Normal),
            "direct" => Ok(PlayMode::Direct),
            "pipe" => Ok(PlayMode::Pipe),
            _ => Err(ConfigError::DownloaderWrongPlayMode(self.play_mode.clone())),
        }
    }

    /// Return the downloader quality.LEVEL value from the given level
    pub fn quality(&self, downloader: &String, level: &String) -> Result<&String, ConfigError> {
        let v = match self.quality.get(level) {
            Some(v) => v,
            None => {
                return Err(ConfigError::DownloaderQualityNotFound(
                    downloader.clone(),
                    level.clone(),
                ))
            }
        };

        match v.len() != 0 {
            true => Ok(v),
            false => Err(ConfigError::DownloaderQualityEmptyValue(
                downloader.clone(),
                level.clone(),
            )),
        }
    }
}

/// Load default config
fn load_default() -> Result<Config, ConfigError> {
    let path = get_path(DEFAULT_CONFIG_FILE)?;

    let data: Vec<u8> = match std::fs::read(&path) {
        Ok(data) => data,
        Err(error) => return Err(ConfigError::FailedReadConfig(path, error)),
    };

    match toml::from_slice(&data) {
        Ok(config) => Ok(config),
        Err(error) => Err(ConfigError::FailedDeserializeToml(path, error)),
    }
}

/// Load custom config
fn load_custom() -> Result<Option<CustomConfig>, ConfigError> {
    let path = get_path(CUSTOM_CONFIG_FILE)?;

    if !path.exists() {
        return Ok(None);
    }

    let data: Vec<u8> = match std::fs::read(&path) {
        Ok(path) => path,
        Err(error) => return Err(ConfigError::FailedReadConfig(path, error)),
    };

    match toml::from_slice(&data) {
        Ok(config) => Ok(Some(config)),
        Err(error) => Err(ConfigError::FailedDeserializeToml(path, error)),
    }
}

/// Rerturn the config path per OS
fn get_path(file: &str) -> Result<PathBuf, ConfigError> {
    let mut path: PathBuf;

    #[cfg(unix)]
    {
        path = match dirs::config_dir() {
            Some(path) => path,
            None => return Err(ConfigError::FailedGetConfigDir),
        };
        path.push("mpv-handler");
        path.push(file);

        if !path.exists() {
            path = PathBuf::from("/etc/mpv-handler/");
            path.push(file);
        }
    }

    #[cfg(windows)]
    {
        path = std::env::current_exe()?;
        path.pop();
        path.push(file);
    }

    Ok(path)
}

/// Merge custom config to default
fn merge(default: Config, custom: CustomConfig) -> Result<Config, ConfigError> {
    let mut config: Config = default;

    if let Some(player) = custom.player {
        config.player = player;
    }

    for (n, c) in custom.downloader {
        match config.downloader.get_mut(&n) {
            Some(d) => merge_downloader(d, c),
            None => {
                let mut d: Downloader = Downloader {
                    ..Default::default()
                };

                merge_downloader(&mut d, c);
                config.downloader.insert(n, d);
            }
        }
    }

    Ok(config)
}

/// Merge custom downloader to default
fn merge_downloader(d: &mut Downloader, c: CustomDownloader) {
    if let Some(v) = c.bin {
        d.bin = v;
    }
    if let Some(v) = c.cookies {
        d.cookies = v;
    }
    if let Some(v) = c.cookies_prefix {
        d.cookies_prefix = v;
    }
    if let Some(v) = c.require_quality {
        d.require_quality = v;
    }
    if let Some(v) = c.play_mode {
        d.play_mode = v;
    } else {
        d.play_mode = Config::default_play_mode();
    }
    if let Some(v) = c.options {
        d.options = v;
    }
    if let Some(v) = c.quality {
        d.quality = v;
    }
}
