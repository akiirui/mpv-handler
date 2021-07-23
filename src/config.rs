use serde::Deserialize;
use thiserror::Error;

use std::collections::HashMap;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Configuration file decode failed, {0}")]
    TomlDeError(#[from] toml::de::Error),
    #[error(transparent)]
    ReadConfigFailed(#[from] std::io::Error),
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub player: String,
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
    pub pipeline: bool,
    #[serde(default)]
    pub options: Vec<String>,
    #[serde(default)]
    pub quality: HashMap<String, String>,
}

impl Config {
    /// Read configuration file from path
    ///
    /// ## Errors
    ///
    /// - `TomlDeError`
    /// - `ReadConfigFailed`
    pub fn read(path: std::path::PathBuf) -> Result<Config, ConfigError> {
        let data: Vec<u8> = std::fs::read(path)?;
        let config: Config = toml::from_slice(&data)?;

        Ok(config)
    }
}
