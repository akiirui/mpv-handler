use thiserror::Error;

#[derive(Error, Debug)]
enum HandlerError {
    #[error("Not found mpv executable binary")]
    MpvNotFound,
    #[error("Player exited with error or termination signal")]
    MpvExitedError,
    #[error("No argument is given")]
    NoArgGiven,
    #[error("Wrong argument is given")]
    WrongArgGiven,
    #[error(transparent)]
    DecodeError(#[from] base64::DecodeError),
    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
}

fn main() {
    match args_parser() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            println!("Press ENTER to exit");
            std::io::Read::read(&mut std::io::stdin(), &mut [0]).unwrap();
            std::process::exit(1);
        }
    }
}

fn args_parser() -> Result<(), HandlerError> {
    let args: Vec<String> = std::env::args().collect();
    let arg = match args.get(1) {
        Some(data) => data.clone(),
        None => return Err(HandlerError::NoArgGiven),
    };
    match arg.as_str() {
        "version" | "-v" | "-V" => handler_version(),
        _ => handler_play(arg),
    }
}

fn handler_version() -> Result<(), HandlerError> {
    let version: &str = option_env!("MPV_HANDLER_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"));
    println!("mpv-handler {}", version);
    Ok(())
}

fn handler_play(mut arg: String) -> Result<(), HandlerError> {
    #[cfg(unix)]
    const MPV_BIN: &str = "mpv";
    #[cfg(windows)]
    const MPV_BIN: &str = "mpv.com";

    if arg.starts_with("mpv://") {
        arg.replace_range(0.."mpv://".len(), "");
    } else {
        return Err(HandlerError::WrongArgGiven);
    }

    #[cfg(windows)]
    if arg.ends_with("/") {
        arg.pop();
    }

    let video_url = String::from_utf8(base64::decode(arg)?)?;
    let mpv = std::process::Command::new(MPV_BIN).arg(video_url).status();
    match mpv {
        Ok(status) => match status.success() {
            true => Ok(()),
            false => Err(HandlerError::MpvExitedError),
        },
        Err(_) => Err(HandlerError::MpvNotFound),
    }
}
