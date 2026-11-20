//! One Terminal process gets one playback request, held in the URL service's memory.

use objc2_core_foundation::{
    CFData, CFMessagePort, CFMessagePortContext, CFRetained, CFRunLoop, CFString,
    kCFMessagePortBecameInvalidError, kCFMessagePortIsInvalid, kCFMessagePortReceiveTimeout,
    kCFMessagePortSendTimeout, kCFMessagePortSuccess, kCFMessagePortTransportError,
};
use std::cell::{Cell, RefCell};
use std::ffi::c_void;
use std::io;
use std::ptr::{self, NonNull};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

const PREFIX: &str = "org.mpv-handler.debug.";
const MODE: &str = "org.mpv-handler.debug-handoff";
const CLAIM: i32 = 1;
const RECEIVED: i32 = 2;

// Only the first random value is registered with the Mach bootstrap server.
// The other two let the launcher and service verify each other independently.
fn new_ticket() -> io::Result<String> {
    let mut parts = [0u128; 3];
    // SAFETY: the destination is writable and well below getentropy's 256-byte limit.
    if unsafe { libc::getentropy(parts.as_mut_ptr().cast(), std::mem::size_of_val(&parts)) } != 0 {
        return Err(io::Error::last_os_error());
    }
    let [name, claim, receipt] = parts;
    Ok(format!("{PREFIX}{name:032x}{claim:032x}{receipt:032x}"))
}

fn ticket_parts(ticket: &str) -> io::Result<(&str, &[u8], &[u8])> {
    if !ticket
        .strip_prefix(PREFIX)
        .is_some_and(|s| s.len() == 96 && s.bytes().all(|b| b.is_ascii_hexdigit()))
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "invalid Terminal session",
        ));
    }
    let (name, keys) = ticket.split_at(PREFIX.len() + 32);
    let (claim, receipt) = keys.as_bytes().split_at(32);
    Ok((name, claim, receipt))
}

struct Request<'a> {
    deadline: Instant,
    claim: &'a [u8],
    receipt: &'a [u8],
    reply: RefCell<Option<CFRetained<CFData>>>,
    received: Cell<bool>,
}

unsafe extern "C-unwind" fn handle_message(
    _port: *mut CFMessagePort,
    id: i32,
    data: *const CFData,
    context: *mut c_void,
) -> *const CFData {
    // SAFETY: serve keeps Request alive until this port has been invalidated.
    // Its source runs only on the worker's private run loop mode.
    let request = unsafe { &*context.cast::<Request<'_>>() };
    if Instant::now() >= request.deadline {
        return ptr::null();
    }
    // SAFETY: Core Foundation supplies immutable message data valid during the callback.
    let bytes = unsafe { data.as_ref().map(|d| d.as_bytes_unchecked()) };
    if id == CLAIM && bytes == Some(request.claim) {
        if let Some(reply) = request.reply.borrow_mut().take() {
            // The callback returns a +1 reference which Core Foundation releases after sending.
            return CFRetained::into_raw(reply).as_ptr();
        }
    } else if id == RECEIVED && bytes == Some(request.receipt) && request.reply.borrow().is_none() {
        request.received.set(true);
    }
    ptr::null()
}

struct LocalPort(CFRetained<CFMessagePort>);

impl Drop for LocalPort {
    fn drop(&mut self) {
        // Also removes its source from the run loop before the context goes away.
        self.0.invalidate();
    }
}

pub(crate) fn serve(
    payload: &[u8],
    launch: impl FnOnce(&str) -> io::Result<()> + Send + 'static,
    timeout: Duration,
) -> io::Result<()> {
    let ticket = new_ticket()?;
    let (name, claim, receipt) = ticket_parts(&ticket)?;
    let reply = CFData::from_bytes(&[receipt, payload].concat());
    let request = Request {
        deadline: Instant::now() + timeout,
        claim,
        receipt,
        reply: RefCell::new(Some(reply)),
        received: Cell::new(false),
    };
    let mut context = CFMessagePortContext {
        version: 0,
        info: ptr::from_ref(&request).cast_mut().cast(),
        retain: None,
        release: None,
        copyDescription: None,
    };
    let mut existing = 0;
    // SAFETY: CF copies the context descriptor; request outlives LocalPort on all exits.
    let port = unsafe {
        CFMessagePort::new_local(
            None,
            Some(&CFString::from_str(name)),
            Some(handle_message),
            &mut context,
            &mut existing,
        )
    }
    .ok_or_else(|| io::Error::other("could not register the Terminal message port"))?;
    if existing != 0 {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "Terminal message port already exists",
        ));
    }
    let port = LocalPort(port);
    let source = CFMessagePort::new_run_loop_source(None, Some(&port.0), 0)
        .ok_or_else(|| io::Error::other("could not create the Terminal message source"))?;
    let run_loop =
        CFRunLoop::current().ok_or_else(|| io::Error::other("no Terminal handoff run loop"))?;
    let mode = CFString::from_str(MODE);
    run_loop.add_source(Some(&source), Some(&mode));
    let launch_ticket = ticket.clone();
    launch_and_wait(
        request.deadline,
        move || launch(&launch_ticket),
        |wait| {
            CFRunLoop::run_in_mode(Some(&mode), wait.as_secs_f64(), true);
            request.received.get()
        },
    )
}

fn launch_and_wait(
    deadline: Instant,
    launch: impl FnOnce() -> io::Result<()> + Send + 'static,
    mut wait_for_ack: impl FnMut(Duration) -> bool,
) -> io::Result<()> {
    // Terminal can start its child before LaunchServices returns. Keep this thread
    // free to serve it. The launch thread holds only the ticket and executable path.
    let (finished, result) = mpsc::channel();
    thread::Builder::new().spawn(move || {
        let _ = finished.send(launch());
    })?;
    let mut launch_pending = true;
    loop {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "Terminal did not acknowledge the debug request",
            ));
        }
        // Incoming messages wake the run loop immediately. While launching, also
        // check for a launch error without waiting for the full handoff timeout.
        let wait = if launch_pending {
            match result.try_recv() {
                Ok(outcome) => {
                    outcome?;
                    launch_pending = false;
                    remaining
                }
                Err(mpsc::TryRecvError::Empty) => remaining.min(Duration::from_millis(100)),
                Err(mpsc::TryRecvError::Disconnected) => {
                    return Err(io::Error::other(
                        "Terminal launch thread exited without a result",
                    ));
                }
            }
        } else {
            remaining
        };
        if wait_for_ack(wait) {
            return Ok(());
        }
    }
}

pub(crate) fn claim(ticket: &str) -> io::Result<Option<Vec<u8>>> {
    let (name, claim, receipt) = ticket_parts(ticket)?;
    let Some(remote) = CFMessagePort::new_remote(None, Some(&CFString::from_str(name))) else {
        return Ok(None);
    };
    let mode = CFString::from_str(MODE);
    let mut reply = ptr::null();
    // SAFETY: the non-null reply mode requests a reply and CF writes a +1 reference.
    let status = unsafe {
        remote.send_request(
            CLAIM,
            Some(&CFData::from_bytes(claim)),
            5.0,
            30.0,
            Some(&mode),
            &mut reply,
        )
    };
    #[allow(non_upper_case_globals)]
    match status {
        kCFMessagePortSuccess => (),
        // Later or simultaneous windows can observe the server closing this endpoint.
        kCFMessagePortIsInvalid
        | kCFMessagePortBecameInvalidError
        | kCFMessagePortTransportError => return Ok(None),
        kCFMessagePortSendTimeout | kCFMessagePortReceiveTimeout => {
            if !remote.is_valid() {
                return Ok(None);
            }
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "Terminal request timed out",
            ));
        }
        _ => {
            return Err(io::Error::other(format!(
                "Terminal message port returned {status}"
            )));
        }
    }
    let Some(reply) = NonNull::new(reply.cast_mut()) else {
        return Ok(None);
    };
    // SAFETY: CFMessagePortSendRequest transferred this reference to the caller.
    let reply = unsafe { CFRetained::from_raw(reply) };
    // SAFETY: the immutable response belongs to this call.
    let bytes = unsafe { reply.as_bytes_unchecked() };
    if bytes.is_empty() {
        return Ok(None);
    }
    let payload = bytes
        .strip_prefix(receipt)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "unrecognized Terminal reply"))?;
    // SAFETY: this is a one-way acknowledgement; no reply storage or mode is required.
    unsafe {
        remote.send_request(
            RECEIVED,
            Some(&CFData::from_bytes(receipt)),
            5.0,
            0.0,
            None,
            ptr::null_mut(),
        )
    };
    Ok(Some(payload.to_vec()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_window_can_claim_while_the_launch_call_is_waiting() {
        let (claim, claims) = mpsc::channel();
        let (finish_launch, allow_return) = mpsc::channel();
        let result = launch_and_wait(
            Instant::now() + Duration::from_secs(2),
            move || {
                claim.send(()).unwrap();
                allow_return
                    .recv_timeout(Duration::from_secs(4))
                    .map_err(io::Error::other)
            },
            |wait| claims.recv_timeout(wait).is_ok(),
        );
        // The launch call is still blocked when the completed handoff returns.
        finish_launch.send(()).unwrap();
        result.unwrap();
    }

    #[test]
    fn an_unfinished_launch_cannot_extend_the_handoff_deadline() {
        let (finish_launch, allow_return) = mpsc::channel();
        let error = launch_and_wait(
            Instant::now() + Duration::from_millis(50),
            move || {
                allow_return
                    .recv_timeout(Duration::from_secs(4))
                    .map_err(io::Error::other)
            },
            |wait| {
                thread::sleep(wait);
                false
            },
        )
        .unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::TimedOut);
        finish_launch.send(()).unwrap();
    }

    #[test]
    fn tickets_are_independent_and_malformed_tickets_are_rejected() {
        let first = new_ticket().unwrap();
        let second = new_ticket().unwrap();
        let (name, claim_key, receipt) = ticket_parts(&first).unwrap();
        let (second_name, second_claim, second_receipt) = ticket_parts(&second).unwrap();
        assert_ne!(name, second_name);
        assert_ne!(claim_key, second_claim);
        assert_ne!(receipt, second_receipt);
        assert_ne!(claim_key, receipt);
        for invalid in [
            "",
            PREFIX,
            &first[1..],
            &first[..first.len() - 1],
            &format!("{PREFIX}{}", "å".repeat(48)),
        ] {
            assert_eq!(
                claim(invalid).unwrap_err().kind(),
                io::ErrorKind::InvalidInput
            );
        }
    }

    #[test]
    fn a_valid_claim_takes_the_payload_once_and_acknowledges_receipt() {
        let request = Request {
            deadline: Instant::now() + Duration::from_secs(60),
            claim: b"claim",
            receipt: b"receipt",
            reply: RefCell::new(Some(CFData::from_bytes(b"receiptplayback"))),
            received: Cell::new(false),
        };
        let send = |id, bytes: &[u8]| {
            let data = CFData::from_bytes(bytes);
            // SAFETY: the live context and immutable data are used on this thread.
            let reply = unsafe {
                handle_message(
                    ptr::null_mut(),
                    id,
                    &*data,
                    ptr::from_ref(&request).cast_mut().cast(),
                )
            };
            NonNull::new(reply.cast_mut()).map(|reply| {
                // SAFETY: the callback transfers a +1 reference to the caller.
                unsafe { CFRetained::from_raw(reply) }.to_vec()
            })
        };
        assert_eq!(send(RECEIVED, b"receipt"), None);
        assert!(!request.received.get());
        assert_eq!(send(CLAIM, b"incorrect"), None);
        assert_eq!(
            send(CLAIM, b"claim").as_deref(),
            Some(&b"receiptplayback"[..])
        );
        assert_eq!(send(CLAIM, b"claim"), None);
        assert_eq!(send(RECEIVED, b"incorrect"), None);
        assert!(!request.received.get());
        assert_eq!(send(RECEIVED, b"receipt"), None);
        assert!(request.received.get());
    }
}
