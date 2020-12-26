use thiserror::Error;

#[cfg(unix)]
static MPV_BIN: &str = "mpv";
#[cfg(windows)]
static MPV_BIN: &str = "mpv.com";

#[derive(Error, Debug)]
enum HandlerError {
    #[error("Not found mpv executable binary")]
    NotFoundMpv,
    #[error("No argument is given")]
    NotFoundArgs,
    #[error("Too many arguments are given")]
    TooManyArgs,
    #[error("Wrong protocol: {0}")]
    WrongProtocol(String),
    #[error("Base64 Decode Error: {0}")]
    DecodeError(#[from] base64::DecodeError),
    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match play_video(args) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    }
}

fn play_video(args: Vec<String>) -> Result<std::process::ExitStatus, HandlerError> {
    let video_url = decode_url(args)?;

    std::process::Command::new(MPV_BIN)
        .arg(video_url)
        .status()
        .or(Err(HandlerError::NotFoundMpv))
}

fn decode_url(args: Vec<String>) -> Result<String, HandlerError> {
    if args.len() < 2 {
        return Err(HandlerError::NotFoundArgs);
    }
    if args.len() > 2 {
        return Err(HandlerError::TooManyArgs);
    }

    let mut arg = args[1].clone();

    if !arg.starts_with("mpv://") {
        return Err(HandlerError::WrongProtocol(arg));
    }

    arg.replace_range(0.."mpv://".len(), "");
    let video_url = String::from_utf8(base64::decode(arg)?)?;

    Ok(video_url)
}
