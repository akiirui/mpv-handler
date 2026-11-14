use crate::error::Error;
use serde::Deserialize;
use std::path::PathBuf;

/// Name of the configuration directory
const CONFIG_DIR: &str = "mpv-handler";

/// Config of mpv-handler
///
/// - `mpv`: mpv binary path
/// - `ytdl`: yt-dlp binary path
/// - `proxy: HTTP(S) proxy server address
#[derive(Debug, Deserialize)]
pub struct Config {
    pub mpv: Option<String>,
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
                let mut config: Config = toml::from_str(&data)?;

                if let Some(mpv) = config.mpv {
                    config.mpv = Some(realpath(mpv)?);
                }
                if let Some(ytdl) = config.ytdl {
                    config.ytdl = Some(realpath(ytdl)?);
                }

                return Ok(config);
            }
        }

        Ok(default_config())
    }
}

/// Returns config directory path of mpv-handler
pub fn get_config_dir() -> Option<PathBuf> {
    // Linux config directory location: $XDG_CONFIG_HOME/mpv-handler/
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        if let Some(mut v) = dirs::config_dir() {
            v.push(CONFIG_DIR);
            return Some(v);
        }
    }

    // macOS config directory location: $HOME/.config/mpv-handler/
    #[cfg(target_os = "macos")]
    {
        if let Some(v) = macos_config_dir() {
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

/// Returns config directory path of mpv-handler on macOS
///
/// `dirs::config_dir` points at `$HOME/Library/Application Support`, which is
/// awkward to reach from a terminal. `$HOME/.config/mpv-handler/` is used
/// instead so the layout matches Linux, unless the native location already
/// exists, in which case it keeps being used.
#[cfg(target_os = "macos")]
fn macos_config_dir() -> Option<PathBuf> {
    // $XDG_CONFIG_HOME/mpv-handler/ wins when it is set explicitly
    if let Some(base) = std::env::var_os("XDG_CONFIG_HOME").filter(|v| !v.is_empty()) {
        let mut path = PathBuf::from(base);
        path.push(CONFIG_DIR);
        return Some(path);
    }

    let home = std::env::var_os("HOME").filter(|v| !v.is_empty())?;
    let mut path = PathBuf::from(home);
    path.push(".config");
    path.push(CONFIG_DIR);

    if path.exists() {
        return Some(path);
    }

    if let Some(mut native) = dirs::config_dir() {
        native.push(CONFIG_DIR);
        if native.exists() {
            return Some(native);
        }
    }

    Some(path)
}

/// The default value of `Config.mpv`
pub fn default_mpv() -> Result<String, Error> {
    #[cfg(unix)]
    return realpath("mpv");
    #[cfg(windows)]
    return realpath("mpv.com");
}

/// The default value of `Config.ytdl`
///
/// mpv's `ytdl_hook` looks yt-dlp up in its own `$PATH` unless it is told
/// otherwise, so the resolved path has to be handed to the player explicitly.
pub fn default_ytdl() -> Result<String, Error> {
    #[cfg(unix)]
    return realpath("yt-dlp");
    #[cfg(windows)]
    return realpath("yt-dlp.exe");
}

/// The defalut value of `Config`
fn default_config() -> Config {
    Config {
        mpv: None,
        ytdl: None,
        proxy: None,
    }
}

/// Returns directories searched in addition to `$PATH`
///
/// A process started by LaunchServices on macOS inherits only
/// `PATH=/usr/bin:/bin:/usr/sbin:/sbin`, which hides package manager
/// installations such as Homebrew or MacPorts.
#[cfg(target_os = "macos")]
pub(crate) fn extra_bin_dirs() -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = vec![
        // Homebrew on Apple Silicon
        PathBuf::from("/opt/homebrew/bin"),
        // Homebrew on Intel, and most manual installations
        PathBuf::from("/usr/local/bin"),
        // MacPorts
        PathBuf::from("/opt/local/bin"),
    ];

    if let Some(home) = std::env::var_os("HOME").filter(|v| !v.is_empty()) {
        let home = PathBuf::from(home);
        dirs.push(home.join(".local/bin"));
        dirs.push(home.join("bin"));
    }

    dirs
}

#[cfg(not(target_os = "macos"))]
fn extra_bin_dirs() -> Vec<PathBuf> {
    Vec::new()
}

/// Returns executables shipped inside a macOS application bundle
///
/// Catches an mpv.app installed from <https://mpv.io/> or by a cask, which
/// has no binary on `$PATH` at all.
#[cfg(target_os = "macos")]
fn bundle_candidates(name: &std::path::Path) -> Vec<PathBuf> {
    let app = format!("{}.app", name.to_string_lossy());
    let mut roots: Vec<PathBuf> = vec![PathBuf::from("/Applications")];

    if let Some(home) = std::env::var_os("HOME").filter(|v| !v.is_empty()) {
        roots.push(PathBuf::from(home).join("Applications"));
    }

    roots
        .into_iter()
        .map(|root| root.join(&app).join("Contents/MacOS").join(name))
        .collect()
}

#[cfg(not(target_os = "macos"))]
fn bundle_candidates(_name: &std::path::Path) -> Vec<PathBuf> {
    Vec::new()
}

fn realpath<T: AsRef<std::ffi::OsStr>>(path: T) -> Result<String, Error> {
    let path = std::path::PathBuf::from(&path);

    if path.is_relative() {
        #[cfg(windows)]
        {
            if let Some(mut p) = crate::config::get_config_dir() {
                p.push(&path);
                if let Ok(rp) = p.canonicalize() {
                    return Ok(rp.display().to_string());
                };
            }
        }

        if let Some(paths) = std::env::var_os("PATH") {
            for mut p in std::env::split_paths(&paths) {
                p.push(&path);
                if let Ok(rp) = p.canonicalize() {
                    return Ok(rp.display().to_string());
                };
            }
        }

        // `$PATH` is not enough on macOS, see `extra_bin_dirs`
        for mut p in extra_bin_dirs() {
            p.push(&path);
            if let Ok(rp) = p.canonicalize() {
                return Ok(rp.display().to_string());
            };
        }

        for p in bundle_candidates(&path) {
            if let Ok(rp) = p.canonicalize() {
                return Ok(rp.display().to_string());
            };
        }
    }

    Ok(path.display().to_string())
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

    assert_eq!(config.mpv, Some("/usr/bin/mpv".to_string()));
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
    assert_eq!(config.mpv, None);
    assert_eq!(config.ytdl, None);
    assert_eq!(config.proxy, None);
}

#[cfg(target_os = "macos")]
#[test]
fn test_macos_extra_bin_dirs() {
    // These are what makes a Homebrew or MacPorts mpv reachable from a
    // LaunchServices process, whose `$PATH` does not contain them.
    let dirs = extra_bin_dirs();

    assert!(dirs.contains(&PathBuf::from("/opt/homebrew/bin")));
    assert!(dirs.contains(&PathBuf::from("/usr/local/bin")));
    assert!(dirs.contains(&PathBuf::from("/opt/local/bin")));
}

#[cfg(target_os = "macos")]
#[test]
fn test_macos_bundle_candidates() {
    let candidates = bundle_candidates(std::path::Path::new("mpv"));

    assert!(candidates.contains(&PathBuf::from("/Applications/mpv.app/Contents/MacOS/mpv")));
}
