use crate::error::Error;
use serde::Deserialize;
use std::path::PathBuf;

/// Config of mpv-handler
///
/// - `mpv`: mpv binary path
/// - `ytdl`: yt-dlp binary path
/// - `streamlink`: streamlink binary path
/// - `stream_conf`: streamlink config file path
/// - `proxy: HTTP(S) proxy server address
/// - `hide_log`: Hide log in console (only for windows)
#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_mpv")]
    pub mpv: String,
    #[serde(default = "default_ytdl")]
    pub ytdl: String,
    #[serde(default = "default_streamlink")]
    pub streamlink: String,
    pub stream_conf: String,
    pub proxy: Option<String>,
    #[serde(default = "default_hide_log")]
    pub hide_log: bool
}

impl Config {
    /// Load config file and retruns `Config`
    ///
    /// If config file doesn't exists, returns default value
    pub fn load() -> Result<Config, Error> {
        let path = config_path()?;

        if path.exists() {
            let data: String = std::fs::read_to_string(&path)?;
            let config: Config = toml::from_str(&data)?;
            let config = Ok(config);
            return config;
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
        streamlink: default_streamlink(),
        stream_conf: String::new(),
        proxy: None,
        hide_log: default_hide_log()
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

/// The default value of `Config.streamlink`
fn default_streamlink() -> String {
    #[cfg(unix)]
    return "streamlink".to_string();
    #[cfg(windows)]
    return "streamlink.exe".to_string();
}

/// The default value of `Config.no_console`
fn default_hide_log() -> bool {
    false
}

#[test]
fn test_config_parse() {
    let config: Config = toml::from_str(
        r#"
            mpv = "/usr/bin/mpv"
            ytdl = "/usr/bin/yt-dlp"
            streamlink = "/usr/bin/streamlink"
            streamlink_conf = "/usr/bin/streamlink/stream_conf"
            proxy = "http://example.com:8080"
            hide_log = false
        "#,
    )
    .unwrap();

    assert_eq!(config.mpv, "/usr/bin/mpv");
    assert_eq!(config.ytdl, "/usr/bin/yt-dlp");
    assert_eq!(config.streamlink, "/usr/bin/streamlink");
    assert_eq!(config.stream_conf, "/usr/bin/streamlink/stream_conf");
    assert_eq!(config.proxy, Some("http://example.com:8080".to_string()));
    assert_eq!(config.hide_log, false);

    let config: Config = toml::from_str(
        r#"
            key1 = "value1"
            key2 = "value2"
            key3 = "value3"
            key4 = "value4"
            key5 = "value5"
            key6 = "value6"
        "#,
    )
    .unwrap();

    #[cfg(unix)]
    {
        assert_eq!(config.mpv, "mpv");
        assert_eq!(config.ytdl, "yt-dlp");
        assert_eq!(config.streamlink, "streamlink");
        assert_eq!(config.stream_conf, "");
        assert_eq!(config.proxy, None);
        assert_eq!(config.hide_log, false);
    }
    #[cfg(windows)]
    {
        assert_eq!(config.mpv, "mpv.com");
        assert_eq!(config.ytdl, "yt-dlp.exe");
        assert_eq!(config.streamlink, "streamlink.exe");
        assert_eq!(config.stream_conf, "");
        assert_eq!(config.proxy, None);
        assert_eq!(config.hide_log, false);
    }
}
