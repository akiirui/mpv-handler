use crate::config::Config;
use crate::error::Error;
use crate::protocol::Protocol;

use std::env;
use std::process::Command;
#[cfg(windows)]
use std::os::windows::process::CommandExt;

const PREFIX_REFERER: &str = "--add-headers=Referer:";
const PREFIX_QUALITY: &str = "--format=";
const PREFIX_OUTPUT:  &str = "--output=-";
const PREFIX_PROFILE: &str = "--profile=";

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

/// Execute player with given options
pub fn exec(proto: &Protocol, config: &Config) -> Result<(), Error> {
    let command:        String;
    let cd_temp:        String;
    let ytdl_path:      String;
    let mpv_path:       String;
    let mut ytdl_options: Vec<&str> = Vec::new();
    let mut mpv_options:  Vec<&str> = Vec::new();
    let option_referer: String;
    let option_quality: String;
    let option_profile: String;

    // Set working path to temp folder
    cd_temp = temp();

    // Convert ytdl path
    ytdl_path = path(&config.ytdl);

    // Append referer option
    if let Some(v) = &proto.referer {
        option_referer = referer(v);

        ytdl_options.push(&option_referer);
    }

    // Append quality option
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
            ytdl_options.push(&option_quality);
        }
    };

    // Append ytdl output option
    ytdl_options.push(PREFIX_OUTPUT);

    // Convert mpv path
    mpv_path = path(&config.mpv);

    // Append profile option
    if let Some(v) = proto.profile {
        option_profile = profile(v);

        mpv_options.push(&option_profile);
    }

    // Create command
    command = format!(
        "{} {} {} {} | {} - {}", 
        cd_temp,
        ytdl_path, 
        ytdl_options.join(" "),
        &proto.url,
        mpv_path,
        mpv_options.join(" "),
    );

    // Print video URL
    println!("Playing: {}", proto.url);
    // Print command
    println!("Option: {}", command);

    // Execute sh/cmd
    #[cfg(unix)]
    let status = Command::new("sh")
        .arg("-c")
        .arg(command)
        .status();
    
    #[cfg(windows)]
    let status = if config.hide_log {
        Command::new("cmd")
            .creation_flags(CREATE_NO_WINDOW)
            .arg("/C")
            .arg(command)
            .status()
    } else {
        Command::new("cmd")
            .arg("/C")
            .arg(command)
            .status()
    };
    
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

/// Get executable path
fn path(path: &str) -> String {
    format!("{}", path.replace("\\\\", "\\")).to_string()
}

/// Return referer option
fn referer(referer: &str) -> String {
    format!("{PREFIX_REFERER}{referer}").to_string()
}

/// Return quality option
fn quality(quality: i32) -> String {
    format!("{PREFIX_QUALITY}bv*[height<={quality}]+ba/b[height<={quality}]/b").to_string()
}

/// Return temp folder as working path
fn temp() -> String {
    format!("cd {} &&", env::temp_dir().to_string_lossy().into_owned()).to_string()
}

/// Return profile option
fn profile(profile: &str) -> String {
    format!("{PREFIX_PROFILE}{profile}").to_string()
}