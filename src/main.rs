#![cfg_attr(
    all(target_os = "windows", not(feature = "console"), not(debug_assertions)),
    windows_subsystem = "windows"
)]

mod config;
mod error;
mod plugins;
mod protocol;

use std::process::ExitCode;

use crate::config::Config;
use crate::error::Error;
use crate::plugins::Plugins;
use crate::protocol::Protocol;

fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
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

    let proto = Protocol::parse(arg)?;
    let config = Config::load()?;

    // Call plugin by scheme
    match proto.plugin {
        Plugins::Play => crate::plugins::play::exec(&proto, &config),
    }
}

/// Print usage
fn print_usage() {
    let version: &str = option_env!("MPV_HANDLER_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"));

    println!("mpv-handler {}\n", version);
    println!("Usage:\n  {}\n", "mpv-handler <url>",);
}

/// Print error
fn print_error(e: Error) -> ExitCode {
    eprint!("{e}");
    std::io::Read::read(&mut std::io::stdin(), &mut []).unwrap();

    match e {
        Error::PlayerExited(code) => ExitCode::from(code),
        _ => ExitCode::FAILURE,
    }
}
