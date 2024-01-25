use crate::config::Config;
use crate::error::Error;
use crate::protocol::Protocol;

use std::process::Command;
#[cfg(windows)]
use std::os::windows::process::CommandExt;

const PREFIX_CONFIG:  &str = "--config=";
const PREFIX_REFERER: &str = "--http-header=Referer=";
const PREFIX_QUALITY: &str = "--default-stream=";

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

/// Execute player with given options
pub fn exec(proto: &Protocol, config: &Config) -> Result<(), Error> {
    let mut options: Vec<&str> = Vec::new();
    let option_config:  String;
    let option_referer: String;
    let option_quality: String;

    // Append config oprtion
    if config.stream_conf.is_empty() == false {
        option_config = stream_conf(&config.stream_conf);

        options.push(&option_config);
    }
    
    // Append referer option
    if let Some(v) = &proto.referer {
        option_referer = referer(v);

        options.push(&option_referer);
    }

    // Append quality option
    if let Some(v) = proto.quality {
        option_quality = match v {
            "best"  => quality("best"),
            "1080p" => quality("1080p60,1080p,best"),
            "720p"  => quality("720p60,720p,best"),
            "480p"  => quality("480p"),
            "360p"  => quality("360p"),
            "worst" => quality("worst"),
            _ => quality("best"),
        };

        options.push(&option_quality);
    } else {
        option_quality = quality("best");

        options.push(&option_quality);
    }

    // Print video URL
    println!("Playing: {}", proto.url);
    // Print command
    println!("Option: {:#?}", options);

    // Execute streamlink
    #[cfg(unix)]
    let status = Command::new(&config.streamlink)
        .args(options)
        .arg(&proto.url)
        .status();
    
    #[cfg(windows)]
    let status = if config.hide_log {
        Command::new(&config.streamlink)
            .creation_flags(CREATE_NO_WINDOW)
            .args(options)
            .arg(&proto.url)
            .status()
    } else {
        Command::new(&config.streamlink)
            .args(options)
            .arg(&proto.url)
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

/// Return streamlink config
fn stream_conf(stream_conf: &str) -> String {
    format!("{PREFIX_CONFIG}{}", stream_conf.replace("\\\\", "\\")).to_string()
}

/// Return referer option
fn referer(referer: &str) -> String {
    format!("{PREFIX_REFERER}{referer}").to_string()
}

/// Return streamlink config
fn quality(quality: &str) -> String {
    format!("{PREFIX_QUALITY}{quality}").to_string()
}