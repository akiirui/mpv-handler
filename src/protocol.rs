use crate::error::Error;
use crate::plugins::Plugins;

#[derive(Debug, PartialEq)]
pub enum Schemes {
    MpvHandler,
    MpvHandlerDebug,
}

const SAFE_PROTOS: [&str; 11] = [
    "http", "https", "ftp", "ftps", "rtmp", "rtmps", "rtmpe", "rtmpt", "rtmpts", "rtmpte", "data",
];

/// Protocol of mpv-handler
///
/// ```
/// mpv-handler://PLUGINS/ENCODED_URL/?PARAMETERS=VALUES
/// mpv-handler-debug://PLUGINS/ENCODED_URL/?PARAMETERS=VALUES
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
/// - v_title
/// - subfile
/// - startat
/// - referrer
#[derive(Debug, PartialEq)]
pub struct Protocol<'a> {
    pub scheme: Schemes,
    pub plugin: Plugins,
    pub url: String,
    pub cookies: Option<&'a str>,
    pub profile: Option<&'a str>,
    pub quality: Option<&'a str>,
    pub v_codec: Option<&'a str>,
    pub v_title: Option<String>,
    pub subfile: Option<String>,
    pub startat: Option<&'a str>,
    pub referrer: Option<String>,
}

impl Protocol<'_> {
    /// Parse the given argument and returns `Protocol`
    pub fn parse(arg: &str) -> Result<Protocol<'_>, Error> {
        let scheme;
        let plugin;
        let url;
        let mut cookies: Option<&str> = None;
        let mut profile: Option<&str> = None;
        let mut quality: Option<&str> = None;
        let mut v_codec: Option<&str> = None;
        let mut v_title: Option<String> = None;
        let mut subfile: Option<String> = None;
        let mut startat: Option<&str> = None;
        let mut referrer: Option<String> = None;

        let mut i: usize;

        // Check scheme `mpv-handler://` and `mpv-handler-debug://`
        (i, scheme) = if let Some(s) = arg.find("://") {
            match &arg[..s] {
                "mpv-handler" => (s + "://".len(), Schemes::MpvHandler),
                "mpv-handler-debug" => (s + "://".len(), Schemes::MpvHandlerDebug),
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
            (i + s + 1, decode_url(&arg[i..i + s])?)
        } else {
            (arg.len(), decode_url(&arg[i..])?)
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
                    "v_title" => v_title = Some(decode_txt(v)?),
                    "subfile" => subfile = Some(decode_url(v)?),
                    "startat" => startat = Some(v),
                    "referrer" => referrer = Some(decode_txt(v)?),
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
            v_title,
            subfile,
            startat,
            referrer,
        })
    }
}

/// Decode base64 data (URL-safe) and return `String`
fn decode_txt(data: &str) -> Result<String, Error> {
    Ok(String::from_utf8(base64::Engine::decode(
        &base64::prelude::BASE64_URL_SAFE_NO_PAD,
        data,
    )?)?)
}

/// Decode base64 data (URL-safe) and check URL protocol
///
/// Allowed protocols:
///
/// ```
/// "http", "https", "ftp", "ftps", "rtmp", "rtmps",
/// "rtmpe", "rtmpt", "rtmpts", "rtmpte", "data"
/// ```
fn decode_url(data: &str) -> Result<String, Error> {
    let url = decode_txt(data)?;

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
        Protocol::parse("mpv-handler://play/aHR0cHM6Ly93d3cueW91dHViZS5jb20vd2F0Y2g_dj1HZ2tuMmY1ZS1JVQ/?cookies=www.youtube.com.txt&profile=low-latency&quality=1080p&v_codec=av01&v_title=VGl0bGU&subfile=aHR0cDovL2V4YW1wbGUuY29tL2VuLmFzcw&startat=233&referrer=aHR0cHM6Ly93d3cueW91dHViZS5jb20v").unwrap();

    assert_eq!(proto.scheme, Schemes::MpvHandler);
    assert_eq!(proto.plugin, Plugins::Play);
    assert_eq!(proto.url, "https://www.youtube.com/watch?v=Ggkn2f5e-IU");
    assert_eq!(proto.cookies, Some("www.youtube.com.txt"));
    assert_eq!(proto.profile, Some("low-latency"));
    assert_eq!(proto.quality, Some("1080p"));
    assert_eq!(proto.v_codec, Some("av01"));
    assert_eq!(proto.v_title, Some("Title".to_string()));
    assert_eq!(proto.subfile, Some("http://example.com/en.ass".to_string()));
    assert_eq!(proto.startat, Some("233"));
    assert_eq!(proto.referrer, Some("https://www.youtube.com/".to_string()));

    // No parameter and last slash
    let proto = Protocol::parse(
        "mpv-handler://play/aHR0cHM6Ly93d3cueW91dHViZS5jb20vd2F0Y2g_dj1HZ2tuMmY1ZS1JVQ",
    )
    .unwrap();

    assert_eq!(proto.scheme, Schemes::MpvHandler);
    assert_eq!(proto.plugin, Plugins::Play);
    assert_eq!(proto.url, "https://www.youtube.com/watch?v=Ggkn2f5e-IU");

    // No parameter and protocol `mpv`
    let proto = Protocol::parse(
        "mpv-handler://play/aHR0cHM6Ly93d3cueW91dHViZS5jb20vd2F0Y2g_dj1HZ2tuMmY1ZS1JVQ/",
    )
    .unwrap();

    assert_eq!(proto.scheme, Schemes::MpvHandler);
    assert_eq!(proto.plugin, Plugins::Play);
    assert_eq!(proto.url, "https://www.youtube.com/watch?v=Ggkn2f5e-IU");

    // No parameter and protocol `mpv-handler-debug`
    let proto = Protocol::parse(
        "mpv-handler-debug://play/aHR0cHM6Ly93d3cueW91dHViZS5jb20vd2F0Y2g_dj1HZ2tuMmY1ZS1JVQ",
    )
    .unwrap();

    assert_eq!(proto.scheme, Schemes::MpvHandlerDebug);
    assert_eq!(proto.plugin, Plugins::Play);
    assert_eq!(proto.url, "https://www.youtube.com/watch?v=Ggkn2f5e-IU");
}
