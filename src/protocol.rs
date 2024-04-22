use crate::error::Error;
use crate::plugins::Plugins;

#[derive(Debug, PartialEq)]
pub enum Schemes {
    Mpv,
    MpvDebug,
}

const SAFE_PROTOS: [&str; 11] = [
    "http", "https", "ftp", "ftps", "rtmp", "rtmps", "rtmpe", "rtmpt", "rtmpts", "rtmpte", "data",
];

/// Protocol of mpv-handler
///
/// ```
/// mpv://PLUGINS/ENCODED_URL/?PARAMETERS=VALUES
/// mpv-debug://PLUGINS/ENCODED_URL/?PARAMETERS=VALUES
/// ```
///
/// PLUGINS:
/// - play
///
/// ENCODED_URL:
/// - URL-safe base64 encoded URL
///
/// PARAMETERS:
/// - cookies
/// - profile
/// - quality
/// - v_codec
/// - subfile
#[derive(Debug, PartialEq)]
pub struct Protocol<'a> {
    pub scheme: Schemes,
    pub plugin: Plugins,
    pub url: String,
    pub cookies: Option<&'a str>,
    pub profile: Option<&'a str>,
    pub quality: Option<&'a str>,
    pub v_codec: Option<&'a str>,
    pub subfile: Option<String>,
}

impl Protocol<'_> {
    /// Parse the given argument and returns `Protocol`
    pub fn parse(arg: &str) -> Result<Protocol, Error> {
        let scheme;
        let plugin;
        let url;
        let mut cookies: Option<&str> = None;
        let mut profile: Option<&str> = None;
        let mut quality: Option<&str> = None;
        let mut v_codec: Option<&str> = None;
        let mut subfile: Option<String> = None;

        let mut i: usize;

        // Check scheme `mpv://` and `mpv-debug://`
        (i, scheme) = if let Some(s) = arg.find("://") {
            match &arg[..s] {
                "mpv" => (s + "://".len(), Schemes::Mpv),
                "mpv-debug" => (s + "://".len(), Schemes::MpvDebug),
                _ => return Err(Error::IncorrectProtocol(arg.to_string())),
            }
        } else {
            return Err(Error::IncorrectProtocol(arg.to_string()));
        };

        // Get plugin
        (i, plugin) = if let Some(s) = arg[i..].find('/') {
            match &arg[i..i + s] {
                "play" => (i + s + 1, Plugins::Play),
                _ => return Err(Error::IncorrectProtocol(arg.to_string())),
            }
        } else {
            return Err(Error::IncorrectProtocol(arg.to_string()));
        };

        // Get url and decode by base64
        (i, url) = if let Some(s) = arg[i..].find('/') {
            (i + s + 1, decode(&arg[i..i + s])?)
        } else {
            (arg.len(), decode(&arg[i..])?)
        };

        // Get parameters
        if let Some(s) = arg[i..].find('?') {
            let params: Vec<&str> = arg[i + s + 1..].split('&').collect();

            for param in params {
                let data: Vec<&str> = param.split_terminator('=').collect();

                if data.len() != 2 {
                    return Err(Error::IncorrectProtocol(arg.to_string()));
                }

                let k = data[0];
                let v = data[1];

                match k {
                    "cookies" => cookies = Some(v),
                    "profile" => profile = Some(v),
                    "quality" => quality = Some(v),
                    "v_codec" => v_codec = Some(v),
                    "subfile" => subfile = Some(decode(v)?),
                    _ => {}
                };
            }
        }

        Ok(Protocol {
            scheme,
            plugin,
            url,
            cookies,
            profile,
            quality,
            v_codec,
            subfile,
        })
    }
}

/// Decode base64 data (URL-safe) and check URL protocol
///
/// Allowed protocols:
///
/// ```
/// "http", "https", "ftp", "ftps", "rtmp", "rtmps",
/// "rtmpe", "rtmpt", "rtmpts", "rtmpte", "data"
/// ```
fn decode(data: &str) -> Result<String, Error> {
    let url = String::from_utf8(base64::Engine::decode(
        &base64::prelude::BASE64_URL_SAFE_NO_PAD,
        data,
    )?)?;

    match url.find("://") {
        Some(s) => {
            if !SAFE_PROTOS.contains(&&url[..s]) {
                return Err(Error::DangerousVideoProtocol(url[..s].to_string()));
            }
        }
        None => return Err(Error::IncorrectVideoURL(url)),
    };

    Ok(url)
}

#[test]
fn test_protocol_parse() {
    // All parameters
    let proto =
        Protocol::parse("mpv://play/aHR0cHM6Ly93d3cueW91dHViZS5jb20vd2F0Y2g_dj1HZ2tuMmY1ZS1JVQ/?cookies=www.youtube.com.txt&profile=low-latency&quality=1080p&v_codec=av01&subfile=aHR0cDovL2V4YW1wbGUuY29tL2VuLmFzcw").unwrap();

    assert_eq!(proto.plugin, Plugins::Play);
    assert_eq!(proto.url, "https://www.youtube.com/watch?v=Ggkn2f5e-IU");
    assert_eq!(proto.cookies, Some("www.youtube.com.txt"));
    assert_eq!(proto.profile, Some("low-latency"));
    assert_eq!(proto.quality, Some("1080p"));
    assert_eq!(proto.v_codec, Some("av01"));
    assert_eq!(proto.subfile, Some("http://example.com/en.ass".to_string()));

    // None parameter
    let proto =
        Protocol::parse("mpv://play/aHR0cHM6Ly93d3cueW91dHViZS5jb20vd2F0Y2g_dj1HZ2tuMmY1ZS1JVQ/")
            .unwrap();
    assert_eq!(proto.plugin, Plugins::Play);
    assert_eq!(proto.url, "https://www.youtube.com/watch?v=Ggkn2f5e-IU");
    assert_eq!(proto.cookies, None);
    assert_eq!(proto.profile, None);
    assert_eq!(proto.quality, None);
    assert_eq!(proto.v_codec, None);
    assert_eq!(proto.subfile, None);

    // None parameter and last slash
    let proto =
        Protocol::parse("mpv://play/aHR0cHM6Ly93d3cueW91dHViZS5jb20vd2F0Y2g_dj1HZ2tuMmY1ZS1JVQ")
            .unwrap();
    assert_eq!(proto.plugin, Plugins::Play);
    assert_eq!(proto.url, "https://www.youtube.com/watch?v=Ggkn2f5e-IU");
    assert_eq!(proto.cookies, None);
    assert_eq!(proto.profile, None);
    assert_eq!(proto.quality, None);
    assert_eq!(proto.v_codec, None);
    assert_eq!(proto.subfile, None);

    // None parameter and protocol `mpv-debug`
    let proto = Protocol::parse(
        "mpv-debug://play/aHR0cHM6Ly93d3cueW91dHViZS5jb20vd2F0Y2g_dj1HZ2tuMmY1ZS1JVQ",
    )
    .unwrap();
    assert_eq!(proto.plugin, Plugins::Play);
    assert_eq!(proto.url, "https://www.youtube.com/watch?v=Ggkn2f5e-IU");
    assert_eq!(proto.cookies, None);
    assert_eq!(proto.profile, None);
    assert_eq!(proto.quality, None);
    assert_eq!(proto.v_codec, None);
    assert_eq!(proto.subfile, None);
}
