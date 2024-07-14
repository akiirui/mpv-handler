use crate::config::Config;
use crate::error::Error;
use crate::protocol::Protocol;

const PREFIX_COOKIES: &str = "--ytdl-raw-options-append=cookies=";
const PREFIX_PROFILE: &str = "--profile=";
const PREFIX_FORMATS: &str = "--ytdl-raw-options-append=format-sort=";
const PREFIX_SUBFILE: &str = "--sub-file=";

/// Execute player with given options
pub fn exec(proto: &Protocol, config: &Config) -> Result<(), Error> {
    let mut options: Vec<&str> = Vec::new();
    let option_cookies: String;
    let option_profile: String;
    let option_formats: String;
    let option_subfile: String;

    // Append cookies option
    if let Some(v) = proto.cookies {
        option_cookies = cookies(v)?;
        options.push(&option_cookies);
    }

    // Append profile option
    if let Some(v) = proto.profile {
        option_profile = profile(v);
        options.push(&option_profile);
    }

    // Append formats option
    if proto.quality.is_some() || proto.v_codec.is_some() {
        option_formats = formats(proto.quality, proto.v_codec)?;
        options.push(&option_formats);
    }

    // Append subfile option
    if let Some(v) = &proto.subfile {
        option_subfile = subfile(v);
        options.push(&option_subfile);
    }

    // Fix some browsers to overwrite "LD_LIBRARY_PATH" on Linux
    // It will be broken mpv player
    // mpv: symbol lookup error: mpv: undefined symbol: vkCreateWaylandSurfaceKHR
    #[cfg(unix)]
    std::env::remove_var("LD_LIBRARY_PATH");

    // Set HTTP(S) proxy environment variables
    if let Some(proxy) = &config.proxy {
        std::env::set_var("http_proxy", proxy);
        std::env::set_var("HTTP_PROXY", proxy);
        std::env::set_var("https_proxy", proxy);
        std::env::set_var("HTTPS_PROXY", proxy);
    }

    // Print options list
    if cfg!(debug_assertions) {
        println!("Options: {:?}", options);
    }

    // Print video URL
    println!("Playing: {}", proto.url);

    // Execute mpv player
    let mut command = std::process::Command::new(&config.mpv);
    command.args(&options).arg("--").arg(&proto.url);

    // Hide console window on Windows if not in debug mode
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        if &proto.scheme == &crate::protocol::Schemes::Mpv && !cfg!(debug_assertions) {
            command.creation_flags(0x08000000);
        }
    }

    match command.status() {
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

/// Return cookies option
fn cookies(cookies: &str) -> Result<String, Error> {
    let mut p: std::path::PathBuf;

    #[cfg(unix)]
    {
        p = match dirs::config_dir() {
            Some(path) => path,
            None => return Err(Error::FailedGetConfigDir),
        };
        p.push("mpv-handler");
        p.push("cookies");
        p.push(cookies);
    }

    #[cfg(windows)]
    {
        p = std::env::current_exe()?;
        p.pop();
        p.push("cookies");
        p.push(cookies);
    }

    let cookies = p.display();

    if p.exists() {
        Ok(format!("{PREFIX_COOKIES}{cookies}"))
    } else {
        Err(Error::CookiesFileNotFound(cookies.to_string()))
    }
}

/// Return profile option
fn profile(profile: &str) -> String {
    format!("{PREFIX_PROFILE}{profile}")
}

/// Return formats option
fn formats(quality: Option<&str>, v_codec: Option<&str>) -> Result<String, Error> {
    let mut f: Vec<String> = Vec::new();
    let formats: String;

    if let Some(v) = quality {
        let i = match v {
            "2160p" => 2160,
            "1440p" => 1440,
            "1080p" => 1080,
            "720p" => 720,
            "480p" => 480,
            "360p" => 360,
            _ => -1,
        };

        if i != -1 {
            f.push(format!("res:{i}"));
        }
    }

    if let Some(v) = v_codec {
        f.push(format!("+vcodec:{v}"))
    }

    formats = f.join(",");

    Ok(format!("{PREFIX_FORMATS}{formats}"))
}

/// Return subfile option
fn subfile(subfile: &str) -> String {
    format!("{PREFIX_SUBFILE}{subfile}")
}

#[test]
fn test_profile_option() {
    let option_profile = profile("low-latency");

    assert_eq!(option_profile, "--profile=low-latency");
}

#[test]
fn test_formats_option() {
    let option_formats_none = formats(None, None);
    let option_formats_quality = formats(Some("720p"), None);
    let option_formats_v_codec = formats(None, Some("vp9"));
    let option_formats_quality_vcodec = formats(Some("720p"), Some("vp9"));

    assert_eq!(
        option_formats_none.unwrap(),
        "--ytdl-raw-options-append=format-sort="
    );
    assert_eq!(
        option_formats_quality.unwrap(),
        "--ytdl-raw-options-append=format-sort=res:720"
    );
    assert_eq!(
        option_formats_v_codec.unwrap(),
        "--ytdl-raw-options-append=format-sort=+vcodec:vp9"
    );
    assert_eq!(
        option_formats_quality_vcodec.unwrap(),
        "--ytdl-raw-options-append=format-sort=res:720,+vcodec:vp9"
    );
}

#[test]
fn test_subfile_option() {
    let option_subfile = subfile("http://example.com/en.ass");

    assert_eq!(option_subfile, "--sub-file=http://example.com/en.ass");
}
