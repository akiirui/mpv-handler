//! macOS URL scheme handler for mpv-handler.
//!
//! On macOS, a bare executable cannot register as a protocol handler. Instead,
//! an `.app` bundle with an `Info.plist` declaring `CFBundleURLTypes` claims
//! the scheme, LaunchServices delivers URLs as `GURL` Apple Events, and
//! `NSApplication`'s run loop services them.

mod ipc;

use objc2::rc::{Retained, autoreleasepool};
use objc2::runtime::AnyObject;
use objc2::{AnyThread, DefinedClass, MainThreadMarker, define_class, msg_send, sel};
use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy};
use objc2_foundation::{
    NSAppleEventDescriptor, NSAppleEventManager, NSObject, NSObjectNSDelayedPerforming,
    NSObjectProtocol, NSUserName,
};
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// Exit once no URL has arrived for this long. Covers a launch without a URL
/// and gives the next URL of a burst time to reuse this same instance.
const IDLE_TIMEOUT: f64 = 10.0;

const DEBUG_HANDOFF_TIMEOUT: Duration = Duration::from_secs(60);
const TERMINAL_SESSION: &str = "MPV_HANDLER_TERMINAL_SESSION";
const URL_SERVICE: &str = "MPV_HANDLER_URL_SERVICE";

/// `kInternetEventClass` and `kAEGetURL`, both `'GURL'`.
const K_GURL: u32 = u32::from_be_bytes(*b"GURL");

/// `keyDirectObject`, the event parameter that carries the URL itself.
const K_DIRECT_OBJECT: u32 = u32::from_be_bytes(*b"----");

/// Appended to `$PATH`: a LaunchServices process inherits only
/// `/usr/bin:/bin:/usr/sbin:/sbin`, which hides Homebrew and MacPorts.
const EXTRA_BIN_DIRS: [&str; 3] = ["/opt/homebrew/bin", "/usr/local/bin", "/opt/local/bin"];

define_class!(
    // SAFETY: NSObject has no subclassing requirements.
    #[unsafe(super(NSObject))]
    #[ivars = Arc<()>]
    struct URLHandler;

    impl URLHandler {
        /// The selector must be exactly `-handleGetURL:withReplyEvent:`,
        /// because `NSAppleEventManager` looks the method up by that name.
        #[unsafe(method(handleGetURL:withReplyEvent:))]
        fn handle_get_url(&self, event: &NSAppleEventDescriptor, _reply: &NSAppleEventDescriptor) {
            // Push the exit out first, so a URL arriving while this one is
            // being dispatched cannot race it.
            self.arm_exit_timer();

            let url = event
                .paramDescriptorForKeyword(K_DIRECT_OBJECT)
                .and_then(|desc| desc.stringValue())
                .map(|url| url.to_string());

            match url {
                Some(url) => {
                    // Playback and launching Terminal can both
                    // outlive the idle timer. Keep the event loop free for more URLs.
                    let active_job = Arc::clone(self.ivars());
                    if let Err(e) = thread::Builder::new().spawn(move || {
                        if url.starts_with("mpv-handler-debug:") {
                            if let Err(e) = autoreleasepool(|_| open_debug_terminal(&url)) {
                                eprintln!("error opening Terminal: {e}");
                            }
                        } else if let Err(e) = crate::handle_url(&url) {
                            eprintln!("error: {e}");
                        }
                        drop(active_job);
                    }) {
                        eprintln!("error starting URL handler: {e}");
                    }
                }
                None => eprintln!("error: no URL in the Apple Event"),
            }
        }

        #[unsafe(method(exitIfIdle:))]
        fn exit_if_idle(&self, _sender: Option<&AnyObject>) {
            // Only this main thread can start a job; workers release their references on exit.
            if Arc::strong_count(self.ivars()) == 1 {
                let mtm = MainThreadMarker::new().expect("AppKit requires the main thread");
                NSApplication::sharedApplication(mtm).terminate(None);
            } else {
                self.arm_exit_timer();
            }
        }
    }

    unsafe impl NSObjectProtocol for URLHandler {}
);

impl URLHandler {
    fn new() -> Retained<Self> {
        let this = Self::alloc().set_ivars(Arc::new(()));
        // SAFETY: `init` is `NSObject`'s designated initializer.
        unsafe { msg_send![super(this), init] }
    }

    fn arm_exit_timer(&self) {
        // SAFETY: exitIfIdle: takes one nullable object and runs on this run loop.
        unsafe {
            NSObject::cancelPreviousPerformRequestsWithTarget_selector_object(
                self,
                sel!(exitIfIdle:),
                None,
            );
            self.performSelector_withObject_afterDelay(sel!(exitIfIdle:), None, IDLE_TIMEOUT);
        }
    }
}

/// LaunchServices supplies the bundle's LSEnvironment only when opening the app.
pub fn handle_launch() -> io::Result<()> {
    let service = std::env::var_os(URL_SERVICE).as_deref() == Some(OsStr::new("1"));
    if service {
        // SAFETY: main calls this before starting threads. Children use the normal CLI.
        unsafe { std::env::remove_var(URL_SERVICE) };
    }
    if std::env::args_os().len() == 1 {
        run_terminal_launcher()?;
        if service {
            run_url_service();
        }
    }
    Ok(())
}

fn run_url_service() -> ! {
    fix_inherited_path();

    // `NSAppleEventManager` does not retain the handler, so it has to outlive
    // the run loop: hence the binding rather than a temporary.
    let handler = URLHandler::new();

    // SAFETY: `URLHandler` implements the selector with the required signature.
    unsafe {
        NSAppleEventManager::sharedAppleEventManager()
            .setEventHandler_andSelector_forEventClass_andEventID(
                handler.as_ref() as &AnyObject,
                sel!(handleGetURL:withReplyEvent:),
                K_GURL,
                K_GURL,
            );
    }

    let mtm = MainThreadMarker::new().expect("AppKit requires the main thread");
    let app = NSApplication::sharedApplication(mtm);
    // No Dock icon and no menu bar: this bundle only reacts to URLs.
    app.setActivationPolicy(NSApplicationActivationPolicy::Accessory);

    // A bare `NSRunLoop` loop never services Apple Events and drops every URL
    // silently; `app.run()` is the only loop that does.
    handler.arm_exit_timer();
    app.run();
    std::process::exit(0);
}

/// Appends the package manager directories to `$PATH`, once, at startup.
fn fix_inherited_path() {
    let inherited = std::env::var_os("PATH").unwrap_or_default();
    let home = dirs::home_dir().filter(|home| !home.as_os_str().as_bytes().contains(&b':'));
    let paths = std::env::split_paths(&inherited)
        .chain(EXTRA_BIN_DIRS.map(PathBuf::from))
        .chain(home.map(|home| home.join(".local/bin")));
    if let Ok(path) = std::env::join_paths(paths) {
        // SAFETY: main thread, before any other thread exists.
        unsafe { std::env::set_var("PATH", path) };
    }
}

/// Dispatch the raw URL before parsing so CLI and player errors appear in Terminal.
fn open_debug_terminal(url: &str) -> io::Result<()> {
    let request = encode_debug_request(
        url,
        &std::env::var_os("PATH").unwrap_or_default(),
        std::env::var_os("XDG_CONFIG_HOME").as_deref(),
    )?;
    let exe = std::env::current_exe()?;
    ipc::serve(
        &request,
        move |session| autoreleasepool(|_| launch_terminal_instance(&exe, session)),
        DEBUG_HANDOFF_TIMEOUT,
    )
}

/// Give a fresh Terminal instance the handler executable and an opaque claim token.
#[allow(deprecated)] // The synchronous NSWorkspace launch API also works on macOS 10.13.
fn launch_terminal_instance(exe: &Path, session: &str) -> io::Result<()> {
    use objc2_app_kit::{
        NSWorkspace, NSWorkspaceLaunchConfigurationArguments,
        NSWorkspaceLaunchConfigurationEnvironment, NSWorkspaceLaunchOptions,
    };
    use objc2_foundation::{NSArray, NSDictionary, NSString};

    let workspace = NSWorkspace::sharedWorkspace();
    let terminal = workspace
        .URLForApplicationWithBundleIdentifier(&NSString::from_str("com.apple.Terminal"))
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Terminal.app was not found"))?;
    let startup = terminal_startup_path(exe)?;
    let arguments =
        NSArray::from_retained_slice(&[NSString::from_str("-Shell"), NSString::from_str(&startup)]);
    let environment = NSDictionary::<NSString, NSString>::from_slices(
        &[&*NSString::from_str(TERMINAL_SESSION)],
        &[&*NSString::from_str(session)],
    );
    // SAFETY: These AppKit keys denote an array of argument strings and a
    // dictionary of environment strings. Both values are retained by configuration.
    unsafe {
        let configuration = NSDictionary::from_slices(
            &[
                NSWorkspaceLaunchConfigurationArguments,
                NSWorkspaceLaunchConfigurationEnvironment,
            ],
            &[&*arguments as &objc2::runtime::AnyObject, &*environment],
        );
        workspace
            .launchApplicationAtURL_options_configuration_error(
                &terminal,
                NSWorkspaceLaunchOptions::NewInstance,
                &configuration,
            )
            .map(|_| ())
            .map_err(|error| io::Error::other(error.to_string()))
    }
}

// Terminal splits the Shell setting into words. For an installation path with
// spaces or punctuation, give it a single safe pathname in Darwin's private
// user temp directory. Keep this alias usable by later windows of the instance.
fn terminal_startup_path(exe: &Path) -> io::Result<String> {
    use std::hash::{DefaultHasher, Hash, Hasher};
    use std::os::unix::fs::symlink;

    let single_word = |s: &str| {
        s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"/-._".contains(&b))
    };
    if let Some(path) = exe.to_str().filter(|s| single_word(s)) {
        return Ok(path.to_owned());
    }
    let result = Command::new("/usr/bin/getconf")
        .arg("DARWIN_USER_TEMP_DIR")
        .output()?;
    let root = std::str::from_utf8(&result.stdout)
        .ok()
        .map(str::trim_end)
        .filter(|s| result.status.success() && s.starts_with('/') && single_word(s))
        .ok_or_else(|| io::Error::other("could not locate the macOS user temporary directory"))?;
    let meta = fs::symlink_metadata(root)?;
    // SAFETY: geteuid has no preconditions.
    if !meta.is_dir() || meta.uid() != unsafe { libc::geteuid() } || meta.mode() & 0o777 != 0o700 {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "invalid macOS user temporary directory",
        ));
    }
    let mut hash = DefaultHasher::new();
    exe.hash(&mut hash);
    let alias = Path::new(root).join(format!("mpv-handler-{:016x}", hash.finish()));
    match symlink(exe, &alias) {
        Ok(()) => (),
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists && fs::read_link(&alias)? == exe => (),
        Err(e) => return Err(e),
    }
    Ok(alias.to_string_lossy().into_owned())
}

/// Keep OS-native bytes for PATH and XDG_CONFIG_HOME. An absent final field
/// means XDG_CONFIG_HOME was unset; an empty final field means it was set to "".
fn encode_debug_request(
    url: &str,
    path: &OsStr,
    config_home: Option<&OsStr>,
) -> io::Result<Vec<u8>> {
    if url.as_bytes().contains(&0) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "NUL in debug URL",
        ));
    }
    // A process environment cannot contain embedded NULs.
    let mut data = url.as_bytes().to_vec();
    data.push(0);
    data.extend_from_slice(path.as_bytes());
    if let Some(home) = config_home {
        data.push(0);
        data.extend_from_slice(home.as_bytes());
    }
    Ok(data)
}

fn playback_command(data: &[u8], exe: &Path) -> io::Result<Command> {
    let mut fields = data.split(|&b| b == 0);
    let (Some(url), Some(path), config_home, None) =
        (fields.next(), fields.next(), fields.next(), fields.next())
    else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid debug request",
        ));
    };
    let url =
        std::str::from_utf8(url).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let mut command = Command::new(exe);
    command
        .arg(url)
        .env("PATH", OsStr::from_bytes(path))
        .env_remove(TERMINAL_SESSION)
        .env_remove("XDG_CONFIG_HOME");
    if let Some(value) = config_home {
        command.env("XDG_CONFIG_HOME", OsStr::from_bytes(value));
    }
    Ok(command)
}

/// Terminal may use a login-style argv[0]. Its per-process environment
/// identifies this playback independently of the executable name.
fn run_terminal_launcher() -> io::Result<()> {
    use std::os::unix::process::CommandExt;

    let Some(session) = std::env::var_os(TERMINAL_SESSION) else {
        return Ok(());
    };
    let session = session
        .to_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid Terminal session"))?;
    // Terminal instances opened by earlier releases retain their old session path.
    // Their later windows should still start a shell after the app is upgraded.
    if !session.starts_with("/tmp/mpv-handler.")
        && !session.starts_with("/private/tmp/mpv-handler.")
        && let Some(data) = ipc::claim(session)?
    {
        return Err(playback_command(&data, &std::env::current_exe()?)?.exec());
    }
    Err(Command::new("/usr/bin/login")
        .arg("-fpq")
        .arg(NSUserName().to_string())
        .env_remove(TERMINAL_SESSION)
        .exec())
}

#[cfg(test)]
mod debug_session_tests {
    use super::*;
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    #[test]
    fn preserves_url_and_environment_in_one_cli_invocation() {
        let exe = Path::new("/bin/echo");
        let path = OsStr::from_bytes(b"/usr/bin:/test path/\xff");
        let url = format!(
            "mpv-handler-debug://play/A/?profile='&$x\n{}",
            "q".repeat(8000)
        );
        for xdg in [
            None,
            Some(OsStr::new("")),
            Some(OsStr::new("/other config")),
        ] {
            let data = encode_debug_request(&url, path, xdg).unwrap();
            let command = playback_command(&data, exe).unwrap();
            assert_eq!(command.get_args().collect::<Vec<_>>(), [OsStr::new(&url)]);
            let env: std::collections::HashMap<_, _> = command.get_envs().collect();
            assert_eq!(env.get(OsStr::new("PATH")), Some(&Some(path)));
            assert_eq!(env.get(OsStr::new("XDG_CONFIG_HOME")), Some(&xdg));
            assert_eq!(env.get(OsStr::new(TERMINAL_SESSION)), Some(&None));
        }
    }

    #[test]
    fn embedded_nul_cannot_change_the_argument_layout() {
        assert!(encode_debug_request("a\0b", OsStr::new("/bin"), None).is_err());
    }
}
