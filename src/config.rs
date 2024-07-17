use crate::error::Error;
use serde::Deserialize;
use std::path::PathBuf;

/// Config of mpv-handler
///
/// - `mpv`: mpv binary path
/// - `ytdl`: yt-dlp binary path
/// - `proxy: HTTP(S) proxy server address
#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_mpv")]
    pub mpv: String,
    pub ytdl: Option<String>,
    pub proxy: Option<String>,
}

impl Config {
    /// Load config file and retruns `Config`
    ///
    /// If config file doesn't exists, returns default value
    pub fn load() -> Result<Config, Error> {
        if let Some(mut path) = get_config_dir() {
            path.push("config.toml");

            if path.exists() {
                let data: String = std::fs::read_to_string(&path)?;
                let config: Config = toml::from_str(&data)?;

                return Ok(config);
            }
        }

        Ok(default_config())
    }
}

/// Returns config directory path of mpv-handler
pub fn get_config_dir() -> Option<PathBuf> {
    // Linux config directory location: $XDG_CONFIG_HOME/mpv-handler/
    #[cfg(unix)]
    {
        if let Some(mut v) = dirs::config_dir() {
            v.push("mpv-handler");
            return Some(v);
        }
    }

    // Windows config directory location: %WORKING_DIR%\
    #[cfg(windows)]
    {
        if let Ok(mut v) = std::env::current_exe() {
            v.pop();
            return Some(v);
        }
    }

    eprintln!("Failed to get config directory");
    None
}

/// The defalut value of `Config`
fn default_config() -> Config {
    Config {
        mpv: default_mpv(),
        ytdl: None,
        proxy: None,
    }
}

/// The default value of `Config.mpv`
fn default_mpv() -> String {
    #[cfg(unix)]
    return "mpv".to_string();
    #[cfg(windows)]
    return "mpv.com".to_string();
}

#[test]
fn test_config_parse() {
    // Custom values
    let config: Config = toml::from_str(
        r#"
            mpv = "/usr/bin/mpv"
            ytdl = "/usr/bin/yt-dlp"
            proxy = "http://example.com:8080"
        "#,
    )
    .unwrap();

    assert_eq!(config.mpv, "/usr/bin/mpv");
    assert_eq!(config.ytdl, Some("/usr/bin/yt-dlp".to_string()));
    assert_eq!(config.proxy, Some("http://example.com:8080".to_string()));

    // Unexpected values
    let config: Config = toml::from_str(
        r#"
            key1 = "value1"
            key2 = "value2"
            key3 = "value3"
        "#,
    )
    .unwrap();

    #[cfg(unix)]
    assert_eq!(config.mpv, "mpv");
    #[cfg(windows)]
    assert_eq!(config.mpv, "mpv.com");
    assert_eq!(config.ytdl, None);
    assert_eq!(config.proxy, None);
}
