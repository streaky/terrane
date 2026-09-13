#![cfg(unix)]

#[cfg(not(target_has_atomic = "64"))]
compile_error!("process signal support requires lock-free 64-bit atomics");

use std::io;
use std::os::fd::RawFd;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU64, Ordering};

pub const SIGNAL_COUNT: usize = 4;
const SIGNALS: [libc::c_int; SIGNAL_COUNT] =
    [libc::SIGINT, libc::SIGTERM, libc::SIGHUP, libc::SIGQUIT];
static WRITE_FD: AtomicI32 = AtomicI32::new(-1);
static INSTALLED: AtomicBool = AtomicBool::new(false);
static ACTIVE_HANDLERS: AtomicU64 = AtomicU64::new(0);
static COUNTS: [AtomicU64; SIGNAL_COUNT] = [const { AtomicU64::new(0) }; SIGNAL_COUNT];
static OVERFLOWED: [AtomicBool; SIGNAL_COUNT] = [const { AtomicBool::new(false) }; SIGNAL_COUNT];

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SignalBatch {
    pub count: u64,
    pub overflowed: bool,
}

pub struct Registration {
    read_fd: RawFd,
    write_fd: RawFd,
    previous: std::sync::Mutex<[Option<libc::sigaction>; SIGNAL_COUNT]>,
}

impl Registration {
    /// Installs handlers for the selected signals and creates their shared self-pipe.
    ///
    /// # Errors
    ///
    /// Returns an error when another registration is active or the host rejects pipe/handler setup.
    pub fn install(selected: [bool; SIGNAL_COUNT]) -> io::Result<Self> {
        if INSTALLED
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "process signal registration is already installed",
            ));
        }
        for index in 0..SIGNAL_COUNT {
            COUNTS[index].store(0, Ordering::Release);
            OVERFLOWED[index].store(false, Ordering::Release);
        }
        let mut pipe = [-1; 2];
        // SAFETY: `pipe` points to two writable file-descriptor slots. Both descriptors are owned
        // by the returned registration after this call succeeds.
        if unsafe { libc::pipe2(pipe.as_mut_ptr(), libc::O_CLOEXEC) } != 0 {
            INSTALLED.store(false, Ordering::Release);
            return Err(io::Error::last_os_error());
        }
        // SAFETY: the write descriptor is valid and exclusively owned here; nonblocking writes keep
        // the signal handler async-signal-safe even when wake bytes are already queued.
        if unsafe { libc::fcntl(pipe[1], libc::F_SETFL, libc::O_NONBLOCK) } != 0 {
            let error = io::Error::last_os_error();
            // SAFETY: the descriptors were created successfully and have not been transferred.
            unsafe {
                libc::close(pipe[0]);
                libc::close(pipe[1]);
            }
            INSTALLED.store(false, Ordering::Release);
            return Err(error);
        }
        let registration = Self {
            read_fd: pipe[0],
            write_fd: pipe[1],
            previous: std::sync::Mutex::new([None; SIGNAL_COUNT]),
        };
        WRITE_FD.store(pipe[1], Ordering::Release);
        if let Err(error) = registration.reconfigure(selected) {
            drop(registration);
            return Err(error);
        }
        Ok(registration)
    }

    /// Changes the installed handler set while retaining each kind's initially captured disposition.
    ///
    /// # Errors
    ///
    /// Returns an error when the host rejects installing or restoring a selected disposition.
    ///
    /// # Panics
    ///
    /// Panics only if another thread panicked while mutating this registration's disposition state.
    pub fn reconfigure(&self, selected: [bool; SIGNAL_COUNT]) -> io::Result<()> {
        let mut previous = self
            .previous
            .lock()
            .expect("signal disposition lock poisoned");
        for (index, signal) in SIGNALS.iter().copied().enumerate() {
            match (selected[index], previous[index]) {
                (true, None) => {
                    // SAFETY: zero is a valid starting representation for `sigaction`; the mask and
                    // handler fields are initialized before the value is passed to libc.
                    let mut action = unsafe { std::mem::zeroed::<libc::sigaction>() };
                    action.sa_sigaction = signal_handler as *const () as usize;
                    action.sa_flags = libc::SA_RESTART;
                    // SAFETY: `action.sa_mask` is writable storage belonging to this stack value.
                    unsafe { libc::sigemptyset(&raw mut action.sa_mask) };
                    // SAFETY: the signal is supported and `captured` receives its prior disposition.
                    let mut captured = unsafe { std::mem::zeroed::<libc::sigaction>() };
                    if unsafe { libc::sigaction(signal, &raw const action, &raw mut captured) } != 0
                    {
                        return Err(io::Error::last_os_error());
                    }
                    previous[index] = Some(captured);
                }
                (false, Some(captured)) => {
                    // SAFETY: `captured` came from the successful installation for this signal.
                    if unsafe { libc::sigaction(signal, &raw const captured, std::ptr::null_mut()) }
                        != 0
                    {
                        return Err(io::Error::last_os_error());
                    }
                    previous[index] = None;
                    COUNTS[index].store(0, Ordering::Release);
                    OVERFLOWED[index].store(false, Ordering::Release);
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Blocks until a signal or explicit wake is observed, then returns exact per-signal batches.
    ///
    /// # Errors
    ///
    /// Returns an error when the self-pipe closes or the host read fails.
    pub fn wait(&self) -> io::Result<[SignalBatch; SIGNAL_COUNT]> {
        let mut byte = 0_u8;
        loop {
            // SAFETY: `byte` is valid writable storage and `read_fd` remains owned by `self`.
            let read = unsafe { libc::read(self.read_fd, (&raw mut byte).cast(), 1) };
            if read == 1 {
                break;
            }
            if read == 0 {
                return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
            }
            let error = io::Error::last_os_error();
            if error.kind() == io::ErrorKind::Interrupted {
                continue;
            }
            return Err(error);
        }
        Ok(std::array::from_fn(|index| SignalBatch {
            count: COUNTS[index].swap(0, Ordering::AcqRel),
            overflowed: OVERFLOWED[index].swap(false, Ordering::AcqRel),
        }))
    }

    pub fn wake(&self) {
        let byte = 0_u8;
        // SAFETY: writing one byte to the registration's nonblocking self-pipe is async-signal-safe;
        // failure only means a wake is already pending in the full pipe.
        unsafe { libc::write(self.write_fd, (&raw const byte).cast(), 1) };
    }
}

impl Drop for Registration {
    fn drop(&mut self) {
        WRITE_FD.store(-1, Ordering::SeqCst);
        let previous = self
            .previous
            .get_mut()
            .expect("signal disposition lock poisoned");
        for (signal, captured) in SIGNALS.iter().copied().zip(previous.iter_mut()) {
            if let Some(captured) = captured.take() {
                // SAFETY: each saved disposition was captured by this registration.
                unsafe { libc::sigaction(signal, &raw const captured, std::ptr::null_mut()) };
            }
        }
        while ACTIVE_HANDLERS.load(Ordering::SeqCst) != 0 {
            std::hint::spin_loop();
        }
        // SAFETY: no handler can retain the write descriptor after the active-handler barrier.
        unsafe {
            libc::close(self.read_fd);
            libc::close(self.write_fd);
        }
        INSTALLED.store(false, Ordering::Release);
    }
}

#[cfg(any(target_os = "linux", target_os = "android"))]
unsafe fn errno_location() -> *mut libc::c_int {
    // SAFETY: libc exposes the calling thread's errno slot for the duration of the handler.
    unsafe { libc::__errno_location() }
}

#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "openbsd",
    target_os = "netbsd"
))]
unsafe fn errno_location() -> *mut libc::c_int {
    // SAFETY: libc exposes the calling thread's errno slot for the duration of the handler.
    unsafe { libc::__error() }
}

extern "C" fn signal_handler(signal: libc::c_int) {
    // SAFETY: the platform-specific accessor returns this thread's live errno slot.
    let errno = unsafe { errno_location() };
    // SAFETY: the slot is valid throughout this signal handler invocation.
    let saved_errno = unsafe { *errno };
    ACTIVE_HANDLERS.fetch_add(1, Ordering::SeqCst);
    if let Some(index) = SIGNALS.iter().position(|candidate| *candidate == signal) {
        let counter = &COUNTS[index];
        let mut current = counter.load(Ordering::Relaxed);
        loop {
            if current == u64::MAX {
                OVERFLOWED[index].store(true, Ordering::Relaxed);
                break;
            }
            match counter.compare_exchange_weak(
                current,
                current + 1,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(observed) => current = observed,
            }
        }
        let fd = WRITE_FD.load(Ordering::SeqCst);
        if fd >= 0 {
            let byte = match index {
                0 => 1_u8,
                1 => 2_u8,
                2 => 3_u8,
                3 => 4_u8,
                _ => 0_u8,
            };
            // SAFETY: `write` is async-signal-safe, the pointer names one readable byte, and the
            // active-handler barrier keeps the descriptor open through this call.
            unsafe { libc::write(fd, (&raw const byte).cast(), 1) };
        }
    }
    ACTIVE_HANDLERS.fetch_sub(1, Ordering::SeqCst);
    // SAFETY: restore the interrupted code's errno after every handler operation.
    unsafe { *errno = saved_errno };
}

#[cfg(test)]
mod tests {
    use super::*;
    const KERNEL_RESTORER_FLAG: libc::c_int = 0x0400_0000;

    #[test]
    fn registration_counts_signals_and_restores_the_previous_disposition() {
        let mut before = unsafe { std::mem::zeroed::<libc::sigaction>() };
        // SAFETY: `before` is valid output storage and a null action only queries the disposition.
        assert_eq!(
            unsafe { libc::sigaction(libc::SIGINT, std::ptr::null(), &raw mut before) },
            0
        );
        let mut term_before = unsafe { std::mem::zeroed::<libc::sigaction>() };
        // SAFETY: `term_before` is valid output storage and a null action only queries disposition.
        assert_eq!(
            unsafe { libc::sigaction(libc::SIGTERM, std::ptr::null(), &raw mut term_before) },
            0
        );

        let registration = Registration::install([true, false, false, false])
            .expect("install signal registration");
        let mut term_during = unsafe { std::mem::zeroed::<libc::sigaction>() };
        // SAFETY: querying an unselected disposition does not alter it.
        assert_eq!(
            unsafe { libc::sigaction(libc::SIGTERM, std::ptr::null(), &raw mut term_during) },
            0
        );
        assert_eq!(term_during.sa_sigaction, term_before.sa_sigaction);
        // SAFETY: the accessor returns this test thread's writable errno slot.
        unsafe { *errno_location() = 777 };
        // SAFETY: SIGINT is one of the signals owned by the registration during this test.
        assert_eq!(unsafe { libc::raise(libc::SIGINT) }, 0);
        // SAFETY: reading this test thread's errno slot validates handler preservation.
        assert_eq!(unsafe { *errno_location() }, 777);
        let batches = registration.wait().expect("observe raised signal");
        assert_eq!(batches[0].count, 1);
        assert!(!batches[0].overflowed);
        drop(registration);

        let mut after = unsafe { std::mem::zeroed::<libc::sigaction>() };
        // SAFETY: `after` is valid output storage and a null action only queries the disposition.
        assert_eq!(
            unsafe { libc::sigaction(libc::SIGINT, std::ptr::null(), &raw mut after) },
            0
        );
        assert_eq!(after.sa_sigaction, before.sa_sigaction);
        assert_eq!(
            after.sa_flags & !KERNEL_RESTORER_FLAG,
            before.sa_flags & !KERNEL_RESTORER_FLAG
        );
        for signal in SIGNALS {
            // SAFETY: both masks were initialized by successful `sigaction` queries/install calls.
            assert_eq!(
                unsafe { libc::sigismember(&raw const after.sa_mask, signal) },
                unsafe { libc::sigismember(&raw const before.sa_mask, signal) }
            );
        }
    }
}
