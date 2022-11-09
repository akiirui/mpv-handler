use crate::error::Error;
use serde::Deserialize;
use std::path::PathBuf;

/// Config of mpv-handler
///
/// - `mpv`: MPV player binary path
/// - `ytdl`: Youtube-DL binary path
#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_mpv")]
    pub mpv: String,
    #[serde(default = "default_ytdl")]
    pub ytdl: String,
}

impl Config {
    /// Load config file and retruns `Config`
    ///
    /// If config file doesn't exists, returns default value
    pub fn load() -> Result<Config, Error> {
        let path = config_path()?;

        if path.exists() {
            let data: Vec<u8> = std::fs::read(&path)?;
            let config: Config = toml::from_slice(&data)?;

            return Ok(config);
        }

        Ok(default_config())
    }
}

#[cfg(unix)]
/// Returns a path of config on Unix
fn config_path() -> Result<PathBuf, Error> {
    let mut path: PathBuf;

    path = match dirs::config_dir() {
        Some(path) => path,
        None => return Err(Error::FailedGetConfigDir),
    };
    path.push("mpv-handler");
    path.push("config.toml");

    Ok(path)
}

#[cfg(windows)]
/// Returns a path of config on Windows
fn config_path() -> Result<PathBuf, Error> {
    let mut path: PathBuf;

    path = std::env::current_exe()?;
    path.pop();
    path.push("config.toml");

    Ok(path)
}

/// The defalut value of `Config`
fn default_config() -> Config {
    Config {
        mpv: default_mpv(),
        ytdl: default_ytdl(),
    }
}

/// The default value of `Config.mpv`
fn default_mpv() -> String {
    #[cfg(unix)]
    return "mpv".to_string();
    #[cfg(windows)]
    return "mpv.com".to_string();
}

/// The default value of `Config.ytdl`
fn default_ytdl() -> String {
    #[cfg(unix)]
    return "yt-dlp".to_string();
    #[cfg(windows)]
    return "yt-dlp.exe".to_string();
}

#[test]
fn test_config_parse() {
    let config: Config = toml::from_str(
        r#"
            mpv = "/usr/bin/mpv"
            ytdl = "/usr/bin/yt-dlp"
        "#,
    )
    .unwrap();

    assert_eq!(config.mpv, "/usr/bin/mpv");
    assert_eq!(config.ytdl, "/usr/bin/yt-dlp");

    let config: Config = toml::from_str(
        r#"
            key1 = "value1"
            key2 = "value2"
        "#,
    )
    .unwrap();

    #[cfg(unix)]
    {
        assert_eq!(config.mpv, "mpv");
        assert_eq!(config.ytdl, "yt-dlp");
    }
    #[cfg(windows)]
    {
        assert_eq!(config.mpv, "mpv.com");
        assert_eq!(config.ytdl, "yt-dlp.exe");
    }
}
