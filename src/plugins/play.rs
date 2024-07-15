use crate::config::Config;
use crate::error::Error;
use crate::protocol::Protocol;

const PREFIX_COOKIES: &str = "--ytdl-raw-options-append=cookies=";
const PREFIX_PROFILE: &str = "--profile=";
const PREFIX_FORMATS: &str = "--ytdl-raw-options-append=format-sort=";
const PREFIX_SUBFILE: &str = "--sub-file=";
const PREFIX_YT_PATH: &str = "--script-opts=ytdl_hook-ytdl_path=";

/// Execute player with given options
pub fn exec(proto: &Protocol, config: &Config) -> Result<(), Error> {
    let mut options: Vec<&str> = Vec::new();
    let option_cookies: String;
    let option_profile: String;
    let option_formats: String;
    let option_subfile: String;
    let option_yt_path: String;

    // Append cookies option
    if let Some(v) = proto.cookies {
        if let Some(v) = cookies(v) {
            option_cookies = v;
            options.push(&option_cookies);
        }
    }

    // Append profile option
    if let Some(v) = proto.profile {
        option_profile = profile(v);
        options.push(&option_profile);
    }

    // Append formats option
    if proto.quality.is_some() || proto.v_codec.is_some() {
        if let Some(v) = formats(proto.quality, proto.v_codec) {
            option_formats = v;
            options.push(&option_formats);
        }
    }

    // Append subfile option
    if let Some(v) = &proto.subfile {
        option_subfile = subfile(v);
        options.push(&option_subfile);
    }

    // Set custom ytdl execute file path
    if let Some(v) = &config.ytdl {
        option_yt_path = yt_path(v);
        options.push(&option_yt_path);
    }

    // Set HTTP(S) proxy environment variables
    if let Some(proxy) = &config.proxy {
        std::env::set_var("http_proxy", proxy);
        std::env::set_var("HTTP_PROXY", proxy);
        std::env::set_var("https_proxy", proxy);
        std::env::set_var("HTTPS_PROXY", proxy);
    }

    // Fix some browsers to overwrite "LD_LIBRARY_PATH" on Linux
    // It will be broken mpv player
    // mpv: symbol lookup error: mpv: undefined symbol: vkCreateWaylandSurfaceKHR
    #[cfg(unix)]
    std::env::remove_var("LD_LIBRARY_PATH");

    // Print options list (in debug build)
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
fn cookies(cookies: &str) -> Option<String> {
    match crate::config::get_config_dir() {
        Some(mut p) => {
            p.push("cookies");
            p.push(cookies);

            if p.exists() {
                let cookies = p.display();
                return Some(format!("{PREFIX_COOKIES}{cookies}"));
            } else {
                eprintln!("Cookies file not found \"{}\"", p.display());
                return None;
            }
        }
        None => None,
    }
}

/// Return profile option
fn profile(profile: &str) -> String {
    format!("{PREFIX_PROFILE}{profile}")
}

/// Return formats option
fn formats(quality: Option<&str>, v_codec: Option<&str>) -> Option<String> {
    let mut f: Vec<String> = Vec::new();
    let formats: String;

    if let Some(v) = quality {
        let i: String = v.matches(char::is_numeric).collect();
        f.push(format!("res:{i}"));
    }

    if let Some(v) = v_codec {
        f.push(format!("+vcodec:{v}"))
    }

    formats = f.join(",");

    Some(format!("{PREFIX_FORMATS}{formats}"))
}

/// Return subfile option
fn subfile(subfile: &str) -> String {
    format!("{PREFIX_SUBFILE}{subfile}")
}

/// Return yt_path option
fn yt_path(yt_path: &str) -> String {
    format!("{PREFIX_YT_PATH}{yt_path}")
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

#[test]
fn test_yt_path_option() {
    let option_yt_path = yt_path("/usr/bin/yt-dlp");

    assert_eq!(
        option_yt_path,
        "--script-opts=ytdl_hook-ytdl_path=/usr/bin/yt-dlp"
    );
}
