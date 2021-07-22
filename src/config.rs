use serde::Deserialize;
use thiserror::Error;

use std::collections::HashMap;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Profile decode failed, {0}")]
    TomlDeError(#[from] toml::de::Error),
    #[error(transparent)]
    ReadConfigFailed(#[from] std::io::Error),
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub player: HashMap<String, String>,
    #[serde(flatten)]
    pub downloader: HashMap<String, Downloader>,
}

#[derive(Deserialize, Debug)]
pub struct Downloader {
    pub bin: String,
    #[serde(default)]
    pub cookies: String,
    #[serde(default)]
    pub cookies_prefix: bool,
    #[serde(default)]
    pub direct: bool,
    #[serde(default)]
    pub options: Vec<String>,
    #[serde(default)]
    pub quality: HashMap<String, String>,
}

impl Config {
    /// Read configure file from path
    ///
    /// ## Errors
    ///
    /// - `TomlDeError`
    ///     - Deserialize toml configure file failed
    /// - `ReadConfigFailed`
    ///     - Open configure file failed
    pub fn read(path: std::path::PathBuf) -> Result<Config, ConfigError> {
        let data: Vec<u8> = std::fs::read(path)?;
        let config: Config = toml::from_slice(&data)?;

        Ok(Config {
            player: config.player,
            downloader: config.downloader,
        })
    }
}
