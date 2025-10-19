use crate::config::Config;
use crate::error::Error;
use crate::protocol::Protocol;

const PREFIX_COOKIES: &str = "--ytdl-raw-options-append=cookies=";
const PREFIX_PROFILE: &str = "--profile=";
const PREFIX_FORMATS: &str = "--ytdl-raw-options-append=format-sort=";
const PREFIX_V_TITLE: &str = "--title=";
const PREFIX_SUBFILE: &str = "--sub-file=";
const PREFIX_STARTAT: &str = "--start=";
const PREFIX_REFERRER: &str = "--referrer=";
const PREFIX_YT_PATH: &str = "--script-opts=ytdl_hook-ytdl_path=";

/// Execute player with given options
pub fn exec(proto: &Protocol, config: &Config) -> Result<(), Error> {
    let mut options: Vec<&str> = Vec::new();
    let option_cookies: String;
    let option_profile: String;
    let option_formats: String;
    let option_v_title: String;
    let option_subfile: String;
    let option_startat: String;
    let option_yt_path: String;
    let option_referrer: String;

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

    // Append v_title option
    if let Some(v) = &proto.v_title {
        option_v_title = v_title(v);
        options.push(&option_v_title);
    }

    // Append subfile option
    if let Some(v) = &proto.subfile {
        option_subfile = subfile(v);
        options.push(&option_subfile);
    }

    // Append startat option
    if let Some(v) = &proto.startat {
        option_startat = startat(v);
        options.push(&option_startat);
    }

    // Append referrer option
    if let Some(v) = &proto.referrer {
        option_referrer = referrer(v);
        options.push(&option_referrer);
    }

    // Set custom ytdl execute file path
    if let Some(v) = &config.ytdl {
        option_yt_path = yt_path(v);
        options.push(&option_yt_path);
    }

    // Print binaries and options list (in debug build)
    if &proto.scheme == &crate::protocol::Schemes::MpvHandlerDebug || cfg!(debug_assertions) {
        // Print binaries
        println!("Binaries:");

        if let Some(v) = &config.mpv {
            println!("    {}", v);
        } else {
            println!("    {}", crate::config::default_mpv()?);
        }

        if let Some(v) = &config.ytdl {
            println!("    {}", v);
        }

        // Print options list
        if !options.is_empty() {
            println!("Options:");
            for option in &options {
                println!("    {}", option);
            }
        }
    }

    // Print video URL
    println!("Playing: {}", proto.url);

    // Execute mpv player
    let mut command;

    if let Some(v) = &config.mpv {
        command = std::process::Command::new(v);
    } else {
        command = std::process::Command::new(crate::config::default_mpv()?);
    }

    command.args(&options).arg("--").arg(&proto.url);

    // Hide console window on Windows if not in debug mode
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        if &proto.scheme == &crate::protocol::Schemes::MpvHandler && !cfg!(debug_assertions) {
            command.creation_flags(0x08000000);
        }
    }

    // Set HTTP(S) proxy environment variables
    if let Some(proxy) = &config.proxy {
        command.env("http_proxy", proxy);
        command.env("HTTP_PROXY", proxy);
        command.env("https_proxy", proxy);
        command.env("HTTPS_PROXY", proxy);
    }

    // Fix some browsers to overwrite "LD_LIBRARY_PATH" on Linux
    // It will be broken mpv player
    // mpv: symbol lookup error: mpv: undefined symbol: vkCreateWaylandSurfaceKHR
    #[cfg(unix)]
    command.env_remove("LD_LIBRARY_PATH");

    // Fix Vivaldi to overwrite "LD_PRELOAD" on Linux
    // https://github.com/akiirui/mpv-handler/issues/78
    #[cfg(unix)]
    command.env_remove("LD_PRELOAD");

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

/// Return v_title option
fn v_title(v_title: &str) -> String {
    format!("{PREFIX_V_TITLE}{v_title}")
}

/// Return subfile option
fn subfile(subfile: &str) -> String {
    format!("{PREFIX_SUBFILE}{subfile}")
}

/// Return startat option
fn startat(startat: &str) -> String {
    format!("{PREFIX_STARTAT}{startat}")
}

/// Return referrer option
fn referrer(referrer: &str) -> String {
    format!("{PREFIX_REFERRER}{referrer}")
}

/// Return yt_path option
fn yt_path(yt_path: &str) -> String {
    format!("{PREFIX_YT_PATH}{yt_path}")
}

#[test]
fn test_profile_option() {
    let p = profile("low-latency");
    assert_eq!(p, format!("{PREFIX_PROFILE}low-latency"));
}

#[test]
fn test_formats_option() {
    // Only quality
    let q = formats(Some("720p"), None);
    assert_eq!(q.unwrap(), format!("{PREFIX_FORMATS}res:720"));

    // Only v_codec
    let v = formats(None, Some("vp9"));
    assert_eq!(v.unwrap(), format!("{PREFIX_FORMATS}+vcodec:vp9"));

    // Both quality and v_codec
    let qv = formats(Some("720p"), Some("vp9"));
    assert_eq!(qv.unwrap(), format!("{PREFIX_FORMATS}res:720,+vcodec:vp9"));
}
#[test]
fn test_v_title_option() {
    let t = v_title("Hello World!");
    assert_eq!(t, format!("{PREFIX_V_TITLE}Hello World!"));
}

#[test]
fn test_subfile_option() {
    let s = subfile("http://example.com/en.ass");
    assert_eq!(s, format!("{PREFIX_SUBFILE}http://example.com/en.ass"));
}

#[test]
fn test_startat_option() {
    let s = startat("233");
    assert_eq!(s, format!("{PREFIX_STARTAT}233"));
}

#[test]
fn test_referrer_option() {
    let r = referrer("http://example.com/");
    assert_eq!(r, format!("{PREFIX_REFERRER}http://example.com/"));
}

#[test]
fn test_yt_path_option() {
    let y = yt_path("/usr/bin/yt-dlp");
    assert_eq!(y, format!("{PREFIX_YT_PATH}/usr/bin/yt-dlp"));
}
