mod config;
mod error;
mod plugins;
mod protocol;

use crate::config::Config;
use crate::error::Error;
use crate::plugins::Plugins;
use crate::protocol::Protocol;

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => print_error(e),
    }
}

/// Run handler
fn run() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    let arg: &str = match args.len() {
        2 => &args[1],
        1 => return Ok(print_usage()),
        _ => return Err(Error::TooManyArgs),
    };

    match arg {
        "-v" | "--version" => Ok(print_usage()),
        _ => {
            let proto = Protocol::parse(arg)?;
            let config = Config::load()?;

            match proto.plugin {
                Plugins::Play => crate::plugins::play::exec(&proto, &config),
            }
        }
    }
}

/// Print usage
fn print_usage() {
    let version: &str = option_env!("MPV_HANDLER_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"));

    println!("mpv-handler {}\n", version);
    println!("Usage:\n  {}\n", "mpv-handler [options] <url>",);
    println!("OPTIONS:\n  {}    {}", "-v, --version", "show version");
}

/// Print error
fn print_error(e: Error) {
    eprint!("ERROR: {}", e);
    std::io::Read::read(&mut std::io::stdin(), &mut [0]).unwrap();
    std::process::exit(1);
}
