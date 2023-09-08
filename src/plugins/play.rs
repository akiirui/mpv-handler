use crate::config::Config;
use crate::error::Error;
use crate::protocol::Protocol;

const PREFIX_COOKIES: &str = "--ytdl-raw-options-append=cookies=";
const PREFIX_PROFILE: &str = "--profile=";
const PREFIX_QUALITY: &str = "--ytdl-format=";
const PREFIX_V_CODEC: &str = "--ytdl-raw-options-append=format-sort=";
const PREFIX_SUBFILE: &str = "--sub-file=";

/// Execute player with given options
pub fn exec(proto: &Protocol, config: &Config) -> Result<(), Error> {
    let mut options: Vec<&str> = Vec::new();
    let option_cookies: String;
    let option_profile: String;
    let option_quality: String;
    let option_v_codec: String;
    let option_subfile: String;

    // Append cookies option
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
            option_cookies = cookies(p.display());

            options.push(&option_cookies);
        } else {
            eprintln!("Cookies file {v} doesn't exist");
        }
    }

    // Append profile option
    if let Some(v) = proto.profile {
        option_profile = profile(v);

        options.push(&option_profile);
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
            options.push(&option_quality);
        }
    };

    // Append v_codec option
    if let Some(v) = proto.v_codec {
        option_v_codec = v_codec(v);

        options.push(&option_v_codec);
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
    // Print video URL
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

/// Return cookies option
fn cookies(cookies: std::path::Display) -> String {
    format!("{PREFIX_COOKIES}{cookies}").to_string()
}

/// Return profile option
fn profile(profile: &str) -> String {
    format!("{PREFIX_PROFILE}{profile}").to_string()
}

/// Return quality option
fn quality(quality: i32) -> String {
    format!("{PREFIX_QUALITY}bv*[height<={quality}]+ba/b[height<={quality}]/b").to_string()
}

/// Return v_codec option
fn v_codec(v_codec: &str) -> String {
    format!("{PREFIX_V_CODEC}+vcodec:{v_codec}").to_string()
}

/// Return subfile option
fn subfile(subfile: &str) -> String {
    format!("{PREFIX_SUBFILE}{subfile}").to_string()
}

#[test]
fn test_cookies_option() {
    let option_cookies =
        cookies(std::path::PathBuf::from("/some/cookies/domain.com.txt").display());

    assert_eq!(
        option_cookies,
        "--ytdl-raw-options-append=cookies=/some/cookies/domain.com.txt".to_string()
    )
}

#[test]
fn test_profile_option() {
    let option_profile = profile("low-latency");

    assert_eq!(option_profile, "--profile=low-latency".to_string());
}

#[test]
fn test_quality_option() {
    let option_quality_1080 = quality(1080);
    let option_quality_2160 = quality(2160);

    assert_eq!(
        option_quality_1080,
        "--ytdl-format=bv*[height<=1080]+ba/b[height<=1080]/b".to_string()
    );
    assert_eq!(
        option_quality_2160,
        "--ytdl-format=bv*[height<=2160]+ba/b[height<=2160]/b".to_string()
    );
}

#[test]
fn test_v_codec_option() {
    let option_v_codec_av01 = v_codec("av01");
    let option_v_codec_h265 = v_codec("h265");
    let option_v_codec_vp92 = v_codec("vp9.2");
    let option_v_codec_vp9 = v_codec("vp9");

    assert_eq!(
        option_v_codec_av01,
        "--ytdl-raw-options-append=format-sort=+vcodec:av01".to_string()
    );
    assert_eq!(
        option_v_codec_h265,
        "--ytdl-raw-options-append=format-sort=+vcodec:h265".to_string()
    );
    assert_eq!(
        option_v_codec_vp92,
        "--ytdl-raw-options-append=format-sort=+vcodec:vp9.2".to_string()
    );
    assert_eq!(
        option_v_codec_vp9,
        "--ytdl-raw-options-append=format-sort=+vcodec:vp9".to_string()
    );
}

#[test]
fn test_subfile_option() {
    let option_subfile = subfile("http://example.com/en.ass");

    assert_eq!(
        option_subfile,
        "--sub-file=http://example.com/en.ass".to_string()
    );
}
