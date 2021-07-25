use serde::Deserialize;
use thiserror::Error;

use std::{collections::HashMap, path::PathBuf};

const DEFAULT_CONFIG_NAME: &str = "config.toml";
const CUSTOMIZE_CONFIG_NAME: &str = "custom.toml";

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Error: Failed to decode configuration file:\n{0}: {1}")]
    FailedDeserializeToml(PathBuf, toml::de::Error),
    #[error("Error: Failed to open configuration file:\n{0}: {1}")]
    FailedReadConfig(PathBuf, std::io::Error),
    #[cfg(unix)]
    #[error("Error: Failed to get config directory")]
    FailedGetConfigDir,
}
#[derive(Debug, Deserialize)]
pub struct Config {
    pub player: String,
    #[serde(flatten)]
    pub downloader: HashMap<String, Downloader>,
}

#[derive(Debug, Default, Deserialize)]
pub struct Downloader {
    pub bin: String,
    #[serde(default)]
    pub cookies: String,
    #[serde(default)]
    pub cookies_prefix: bool,
    #[serde(default)]
    pub require_quality: bool,
    #[serde(default = "default_play_mode")]
    pub play_mode: String,
    #[serde(default)]
    pub options: Vec<String>,
    #[serde(default)]
    pub quality: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct CustomizeConfig {
    player: Option<String>,
    #[serde(flatten)]
    downloader: HashMap<String, CustomizeDownloader>,
}

#[derive(Debug, Deserialize)]
struct CustomizeDownloader {
    bin: Option<String>,
    cookies: Option<String>,
    cookies_prefix: Option<bool>,
    require_quality: Option<bool>,
    play_mode: Option<String>,
    options: Option<Vec<String>>,
    quality: Option<HashMap<String, String>>,
}

impl Config {
    /// Load configuration and merge default and customize (if it exist)
    ///
    /// ## Errors
    ///
    /// - `FailedDeserializeToml`
    /// - `FailedReadConfig`
    /// - `FailedGetConfigDir`
    pub fn load() -> Result<Config, ConfigError> {
        let default = load_default()?;

        if let Some(customize) = load_customize()? {
            Ok(merge(default, customize)?)
        } else {
            Ok(default)
        }
    }
}

fn default_play_mode() -> String {
    "normal".to_string()
}

fn load_default() -> Result<Config, ConfigError> {
    let mut path: PathBuf;

    #[cfg(unix)]
    {
        path = match dirs::config_dir() {
            Some(path) => path,
            None => return Err(ConfigError::FailedGetConfigDir),
        };
        path.push("mpv-handler");
        path.push(DEFAULT_CONFIG_NAME);

        if !path.exists() {
            path = PathBuf::from("/etc/mpv-handler/");
            path.push(DEFAULT_CONFIG_NAME);
        }
    }

    #[cfg(windows)]
    {
        path = std::env::current_exe()?;
        path.pop();
        path.push(DEFAULT_CONFIG_NAME);
    }

    let data: Vec<u8> = match std::fs::read(&path) {
        Ok(data) => data,
        Err(error) => return Err(ConfigError::FailedReadConfig(path, error)),
    };
    let config: Config = match toml::from_slice(&data) {
        Ok(config) => config,
        Err(error) => return Err(ConfigError::FailedDeserializeToml(path, error)),
    };

    Ok(config)
}

fn load_customize() -> Result<Option<CustomizeConfig>, ConfigError> {
    let mut path: PathBuf;

    #[cfg(unix)]
    {
        path = match dirs::config_dir() {
            Some(path) => path,
            None => return Err(ConfigError::FailedGetConfigDir),
        };
        path.push("mpv-handler");
        path.push(CUSTOMIZE_CONFIG_NAME);

        if !path.exists() {
            path = PathBuf::from("/etc/mpv-handler/");
            path.push(CUSTOMIZE_CONFIG_NAME);
        }
    }

    #[cfg(windows)]
    {
        path = std::env::current_exe()?;
        path.pop();
        path.push(CUSTOMIZE_CONFIG_NAME);
    }

    if !path.exists() {
        return Ok(None);
    }

    let data: Vec<u8> = match std::fs::read(&path) {
        Ok(path) => path,
        Err(error) => return Err(ConfigError::FailedReadConfig(path, error)),
    };
    let config: CustomizeConfig = match toml::from_slice(&data) {
        Ok(config) => config,
        Err(error) => return Err(ConfigError::FailedDeserializeToml(path, error)),
    };

    Ok(Some(config))
}

fn merge(default: Config, customize: CustomizeConfig) -> Result<Config, ConfigError> {
    let mut config = default;

    if let Some(player) = customize.player {
        config.player = player;
    }

    for (name, c) in customize.downloader {
        if let Some(d) = config.downloader.get_mut(&name) {
            merge_downloader(d, c);
        } else {
            let mut d = Downloader {
                ..Default::default()
            };

            merge_downloader(&mut d, c);
            config.downloader.insert(name, d);
        }
    }

    Ok(config)
}

fn merge_downloader(d: &mut Downloader, c: CustomizeDownloader) {
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
    }
    if let Some(v) = c.options {
        d.options = v;
    }
    if let Some(v) = c.quality {
        d.quality = v;
    }
}
