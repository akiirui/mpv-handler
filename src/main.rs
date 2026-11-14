#![cfg_attr(
    all(target_os = "windows", not(feature = "console"), not(debug_assertions)),
    windows_subsystem = "windows"
)]

mod config;
mod error;
mod plugins;
mod protocol;

#[cfg(target_os = "macos")]
mod macos;

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
        1 => {
            // On macOS the handler is an application bundle, and
            // LaunchServices passes the URL with an Apple Event instead of
            // the command line. `run_url_service` never returns.
            #[cfg(target_os = "macos")]
            if macos::launched_by_launch_services() {
                macos::run_url_service();
            }

            return Ok(print_usage());
        }
        _ => return Err(Error::TooManyArgs),
    };

    dispatch(arg, false)
}

/// Parse `arg` as a handler URL and call the plugin it selects
///
/// When `detached` is true the player is started without waiting for it to
/// exit, which is what the macOS URL service needs.
pub(crate) fn dispatch(arg: &str, detached: bool) -> Result<(), Error> {
    let proto = Protocol::parse(arg)?;
    let config = Config::load()?;

    // Call plugin by scheme
    match proto.plugin {
        Plugins::Play => {
            if detached {
                crate::plugins::play::exec_detached(&proto, &config)
            } else {
                crate::plugins::play::exec(&proto, &config)
            }
        }
    }
}

/// Print usage
fn print_usage() {
    let version: &str = option_env!("MPV_HANDLER_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"));

    println!("mpv-handler {}\n", version);
    println!("Usage:\n  {}\n", "mpv-handler <url>",);

    #[cfg(target_os = "macos")]
    println!(
        "On macOS this binary is meant to live inside mpv-handler.app, which\n\
         receives mpv-handler:// URLs from LaunchServices. Run\n\
         share/macos/install.sh to install it.\n"
    );
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
