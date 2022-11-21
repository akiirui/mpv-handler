use crate::config::Config;
use crate::error::Error;
use crate::protocol::Protocol;

const C_PREFIX: &str = "--ytdl-raw-options-append=cookies=";
const P_PREFIX: &str = "--profile=";
const Q_2160: &str = "--ytdl-format=bestvideo[height<=2160]+bestaudio/best[height<=2160]/best";
const Q_1440: &str = "--ytdl-format=bestvideo[height<=1440]+bestaudio/best[height<=1440]/best";
const Q_1080: &str = "--ytdl-format=bestvideo[height<=1080]+bestaudio/best[height<=1080]/best";
const Q_720: &str = "--ytdl-format=bestvideo[height<=720]+bestaudio/best[height<=720]/best";
const Q_480: &str = "--ytdl-format=bestvideo[height<=480]+bestaudio/best[height<=480]/best";
const Q_360: &str = "--ytdl-format=bestvideo[height<=360]+bestaudio/best[height<=360]/best";

/// Execute player with given options
pub fn exec(proto: &Protocol, config: &Config) -> Result<(), Error> {
    let mut options: Vec<&str> = Vec::new();

    // Append cookies option
    let mut cookies_option: String;

    if let Some(v) = proto.cookies {
        let mut p: std::path::PathBuf;

        #[cfg(unix)]
        {
            p = match dirs::config_dir() {
                Some(path) => path,
                None => return Err(Error::FailedGetConfigDir),
            };
            p.push("mpv-handler");
            p.push("cookies");
            p.push(v);
        }

        #[cfg(windows)]
        {
            p = std::env::current_exe()?;
            p.pop();
            p.push("cookies");
            p.push(v);
        }

        if p.exists() {
            cookies_option = String::from(C_PREFIX);
            cookies_option.push_str(&p.display().to_string());
            options.push(&cookies_option);
        } else {
            eprintln!("Cookies file {} doesn't exist", v);
        }
    }

    // Append profile option
    let mut profile_option: String;

    if let Some(v) = proto.profile {
        profile_option = String::from(P_PREFIX);
        profile_option.push_str(v);
        options.push(&profile_option);
    }

    // Append quality option
    if let Some(v) = proto.quality {
        match v {
            "2160p" => options.push(Q_2160),
            "1440p" => options.push(Q_1440),
            "1080p" => options.push(Q_1080),
            "720p" => options.push(Q_720),
            "480p" => options.push(Q_480),
            "360p" => options.push(Q_360),
            _ => (),
        };
    };

    // Fix some browsers to overwrite "LD_LIBRARY_PATH" on Linux
    // It will be broken mpv player
    // mpv: symbol lookup error: mpv: undefined symbol: vkCreateWaylandSurfaceKHR
    #[cfg(unix)]
    std::env::remove_var("LD_LIBRARY_PATH");

    println!("Playing: {}", proto.url);

    // Execute mpv player
    let player = std::process::Command::new(&config.mpv)
        .args(options)
        .arg("--")
        .arg(&proto.url)
        .status();

    match player {
        Ok(o) => match o.success() {
            true => Ok(()),
            false => Err(Error::PlayerExited),
        },
        Err(e) => Err(Error::PlayerRunFailed(e)),
    }
}
