//! macOS integration for mpv-handler.
//!
//! macOS has no equivalent of a Linux `.desktop` file or a Windows registry
//! entry: a URL scheme handler has to be an application bundle whose
//! `Info.plist` declares `CFBundleURLTypes`. LaunchServices then starts the
//! bundle and hands the URL over with the `GURL` Apple Event, which means the
//! URL never shows up in `argv`.
//!
//! This module runs a minimal AppKit event loop to receive that event,
//! dispatches the URL, and exits. Everything below the event is shared with
//! the command line path, so `mpv-handler <url>` keeps working in a terminal.

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{AnyThread, MainThreadMarker, define_class, msg_send};
use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy};
use objc2_foundation::{NSAppleEventDescriptor, NSAppleEventManager, NSObject, NSObjectProtocol};

use crate::error::Error;

/// `kInternetEventClass`, the event class that carries URLs.
const K_INTERNET_EVENT_CLASS: u32 = u32::from_be_bytes(*b"GURL");
/// `kAEGetURL`, the event LaunchServices sends for an opened URL.
const K_AE_GET_URL: u32 = u32::from_be_bytes(*b"GURL");
/// `keyDirectObject`, the parameter holding the URL itself.
const KEY_DIRECT_OBJECT: u32 = u32::from_be_bytes(*b"----");

/// How long to wait for a URL before giving up.
///
/// LaunchServices can also start the bundle without a URL, for example when
/// it is double clicked in the Finder. Without this limit the invisible
/// process would stay around until logout.
const URL_TIMEOUT: Duration = Duration::from_secs(30);

/// Set once the `GURL` handler ran.
static URL_RECEIVED: AtomicBool = AtomicBool::new(false);

unsafe extern "C" {
    fn getppid() -> i32;
}

/// Returns whether this process was started by LaunchServices.
///
/// A bundle started by LaunchServices is a direct child of `launchd`, while a
/// binary started from a terminal is a child of the shell. That difference is
/// what tells "run as a URL service" apart from "print the usage message".
pub fn launched_by_launch_services() -> bool {
    // SAFETY: `getppid` is a plain libc call without preconditions.
    let ppid = unsafe { getppid() };

    ppid == 1
}

define_class!(
    // SAFETY:
    // - The superclass `NSObject` has no subclassing requirements.
    // - `URLHandler` does not implement `Drop` and has no instance variables.
    #[unsafe(super(NSObject))]
    struct URLHandler;

    impl URLHandler {
        /// Handles the `GURL` Apple Event sent by LaunchServices.
        ///
        /// The Objective-C signature has to stay exactly
        /// `-handleGetURL:withReplyEvent:`, because `NSAppleEventManager`
        /// looks the method up by that name.
        #[unsafe(method(handleGetURL:withReplyEvent:))]
        fn handle_get_url(
            &self,
            event: &NSAppleEventDescriptor,
            _reply: &NSAppleEventDescriptor,
        ) {
            URL_RECEIVED.store(true, Ordering::SeqCst);

            let url = event
                .paramDescriptorForKeyword(KEY_DIRECT_OBJECT)
                .and_then(|desc| desc.stringValue())
                .map(|url| url.to_string());

            match url {
                Some(url) => handle_url(&url),
                None => report_error("The URL could not be read from the Apple Event."),
            }

            // `terminate:` exits the process, so each URL is handled by a
            // fresh instance and a long playback can never block the next one.
            let mtm =
                MainThreadMarker::new().expect("Apple Events arrive on the main thread");
            NSApplication::sharedApplication(mtm).terminate(None);
        }
    }

    unsafe impl NSObjectProtocol for URLHandler {}
);

impl URLHandler {
    fn new() -> Retained<Self> {
        // `URLHandler` has no instance variables to set up.
        let this = Self::alloc().set_ivars(());
        // SAFETY: `init` is the designated initializer of `NSObject`.
        unsafe { msg_send![super(this), init] }
    }
}

/// Runs the handler as a macOS URL service. This function never returns.
pub fn run_url_service() -> ! {
    let mtm = MainThreadMarker::new().expect("AppKit requires the main thread");

    let app = NSApplication::sharedApplication(mtm);
    // No Dock icon and no menu bar: this bundle only reacts to URLs.
    app.setActivationPolicy(NSApplicationActivationPolicy::Accessory);

    let handler = URLHandler::new();
    let manager = NSAppleEventManager::sharedAppleEventManager();

    // SAFETY: `handler` implements `handleGetURL:withReplyEvent:` with the
    // signature `NSAppleEventManager` requires, and the selector names it.
    unsafe {
        manager.setEventHandler_andSelector_forEventClass_andEventID(
            handler.as_ref() as &AnyObject,
            objc2::sel!(handleGetURL:withReplyEvent:),
            K_INTERNET_EVENT_CLASS,
            K_AE_GET_URL,
        );
    }

    // `NSAppleEventManager` does not retain the handler, so `handler` has to
    // outlive the event loop. `app.run()` below only returns through
    // `terminate:` or `exit`, both of which end the process.
    std::thread::spawn(|| {
        std::thread::sleep(URL_TIMEOUT);
        if !URL_RECEIVED.load(Ordering::SeqCst) {
            std::process::exit(0);
        }
    });

    app.run();
    std::process::exit(0);
}

/// Handles one URL that arrived from LaunchServices.
fn handle_url(url: &str) {
    // LaunchServices starts the bundle without a console, so the debug scheme
    // re-runs the command line interface inside Terminal to make its output
    // readable. That mirrors the console window used on Windows.
    if url.starts_with("mpv-handler-debug://") {
        if let Err(e) = run_in_terminal(url) {
            report_error(&format!("{e}"));
        }
        return;
    }

    if let Err(e) = crate::dispatch(url, true) {
        report_error(&format!("{e}"));
    }
}

/// Re-runs this executable in Terminal with `url` as its argument.
fn run_in_terminal(url: &str) -> Result<(), Error> {
    let exe = std::env::current_exe()?;
    let command = format!(
        "{} {}",
        shell_quote(&exe.to_string_lossy()),
        shell_quote(url)
    );
    let script = format!(
        "tell application \"Terminal\" to do script {}",
        apple_script_string(&command)
    );

    let output = std::process::Command::new("/usr/bin/osascript")
        .arg("-e")
        .arg(&script)
        .output()
        .map_err(|e| Error::TerminalRunFailed(e.to_string()))?;

    if output.status.success() {
        return Ok(());
    }

    let reason = String::from_utf8_lossy(&output.stderr);
    Err(Error::TerminalRunFailed(reason.trim().to_string()))
}

/// Shows an alert, the only way to report an error when LaunchServices
/// started the bundle without a console.
///
/// `giving up after` keeps a dismissed-by-nobody alert from holding the
/// process open forever.
fn report_error(message: &str) {
    eprintln!("{message}");

    let script = format!(
        "display alert \"mpv-handler\" message {} as warning giving up after 30",
        apple_script_string(message)
    );
    let _ = std::process::Command::new("/usr/bin/osascript")
        .arg("-e")
        .arg(&script)
        .status();
}

/// Quotes `value` as an AppleScript string literal.
fn apple_script_string(value: &str) -> String {
    let mut quoted = String::with_capacity(value.len() + 2);

    quoted.push('"');
    for c in value.chars() {
        match c {
            '"' => quoted.push_str("\\\""),
            '\\' => quoted.push_str("\\\\"),
            _ => quoted.push(c),
        }
    }
    quoted.push('"');

    quoted
}

/// Quotes `value` so that the shell passes it on as a single argument.
fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

#[test]
fn test_apple_script_string() {
    assert_eq!(apple_script_string("plain"), "\"plain\"");
    assert_eq!(apple_script_string("say \"hi\""), "\"say \\\"hi\\\"\"");
    assert_eq!(apple_script_string("back\\slash"), "\"back\\\\slash\"");
}

#[test]
fn test_shell_quote() {
    assert_eq!(
        shell_quote("mpv-handler://play/abc/?quality=1080p"),
        "'mpv-handler://play/abc/?quality=1080p'"
    );
    assert_eq!(shell_quote("it's"), "'it'\\''s'");
}
