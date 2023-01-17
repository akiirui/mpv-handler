use crate::config::Config;
use crate::error::Error;
use crate::protocol::Protocol;

const PREFIX_COOKIES: &str = "--ytdl-raw-options-append=cookies=";
const PREFIX_PROFILE: &str = "--profile=";
const PREFIX_QUALITY: &str = "--ytdl-format=";

/// Execute player with given options
pub fn exec(proto: &Protocol, config: &Config) -> Result<(), Error> {
    let mut options: Vec<&str> = Vec::new();

    // Append cookies option
    let mut option_cookies: String;

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
            option_cookies = String::from(PREFIX_COOKIES);
            option_cookies.push_str(&p.display().to_string());
            options.push(&option_cookies);
        } else {
            eprintln!("Cookies file {v} doesn't exist");
        }
    }

    // Append profile option
    let mut option_profile: String;

    if let Some(v) = proto.profile {
        option_profile = String::from(PREFIX_PROFILE);
        option_profile.push_str(v);
        options.push(&option_profile);
    }

    // Append quality option
    let option_quality: String;

    if let Some(v) = proto.quality {
        option_quality = match v {
            "2160p" => quality(2160),
            "1440p" => quality(1440),
            "1080p" => quality(1080),
            "720p" => quality(720),
            "480p" => quality(480),
            "360p" => quality(360),
            _ => String::new(),
        };

        if option_quality.len() != 0 {
            options.push(&option_quality);
        }
    };

    // Fix some browsers to overwrite "LD_LIBRARY_PATH" on Linux
    // It will be broken mpv player
    // mpv: symbol lookup error: mpv: undefined symbol: vkCreateWaylandSurfaceKHR
    #[cfg(unix)]
    std::env::remove_var("LD_LIBRARY_PATH");

    println!("Playing: {}", proto.url);

    // Execute mpv player
    let status = std::process::Command::new(&config.mpv)
        .args(options)
        .arg("--")
        .arg(&proto.url)
        .status();

    match status {
        Ok(o) => match o.code() {
            Some(code) => match code {
                0 => Ok(()),
                _ => Err(Error::PlayerExited(code as u8)),
            },
            None => Ok(()),
        },
        Err(e) => Err(Error::PlayerRunFailed(e)),
    }
}

fn quality(height: i32) -> String {
    let mut option = String::from(PREFIX_QUALITY);

    option.push_str(&format!("bv*[height<={height}]+ba/b[height<={height}]/b"));

    option
}
