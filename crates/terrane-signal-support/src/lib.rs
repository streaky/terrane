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
    previous: [libc::sigaction; SIGNAL_COUNT],
}

impl Registration {
    /// Installs the supported signal handlers and creates their self-pipe.
    ///
    /// # Errors
    ///
    /// Returns an error when another registration is active or the host rejects pipe/handler setup.
    pub fn install() -> io::Result<Self> {
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
            unsafe {
                libc::close(pipe[0]);
                libc::close(pipe[1]);
            }
            INSTALLED.store(false, Ordering::Release);
            return Err(error);
        }
        let mut previous =
            [const { unsafe { std::mem::zeroed::<libc::sigaction>() } }; SIGNAL_COUNT];
        for (index, signal) in SIGNALS.iter().copied().enumerate() {
            // SAFETY: zero is a valid starting representation for `sigaction`; the mask and handler
            // fields are initialized before the value is passed to libc.
            let mut action = unsafe { std::mem::zeroed::<libc::sigaction>() };
            action.sa_sigaction = signal_handler as *const () as usize;
            action.sa_flags = libc::SA_RESTART;
            // SAFETY: `action.sa_mask` is valid writable storage belonging to this stack value.
            unsafe { libc::sigemptyset(&raw mut action.sa_mask) };
            // SAFETY: signal numbers are fixed supported process signals, pointers are valid, and
            // `previous[index]` receives the exact disposition that is later restored.
            if unsafe { libc::sigaction(signal, &raw const action, &raw mut previous[index]) } != 0
            {
                for rollback in 0..index {
                    // SAFETY: these entries were initialized by successful `sigaction` calls above.
                    unsafe {
                        libc::sigaction(
                            SIGNALS[rollback],
                            &raw const previous[rollback],
                            std::ptr::null_mut(),
                        )
                    };
                }
                // SAFETY: the descriptors were created successfully and have not been transferred.
                unsafe {
                    libc::close(pipe[0]);
                    libc::close(pipe[1]);
                }
                INSTALLED.store(false, Ordering::Release);
                return Err(io::Error::last_os_error());
            }
        }
        WRITE_FD.store(pipe[1], Ordering::Release);
        Ok(Self {
            read_fd: pipe[0],
            write_fd: pipe[1],
            previous,
        })
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
        WRITE_FD.store(-1, Ordering::Release);
        for (signal, previous) in SIGNALS.iter().copied().zip(&self.previous) {
            // SAFETY: each saved disposition was initialized by `install`; restoring it is the
            // registration's exclusive shutdown responsibility.
            unsafe { libc::sigaction(signal, previous, std::ptr::null_mut()) };
        }
        // SAFETY: both descriptors are exclusively owned by this registration until drop.
        unsafe {
            libc::close(self.read_fd);
            libc::close(self.write_fd);
        }
        INSTALLED.store(false, Ordering::Release);
    }
}

extern "C" fn signal_handler(signal: libc::c_int) {
    let Some(index) = SIGNALS.iter().position(|candidate| *candidate == signal) else {
        return;
    };
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
    let fd = WRITE_FD.load(Ordering::Acquire);
    if fd >= 0 {
        let byte = u8::try_from(index + 1).expect("supported signal index fits in one byte");
        // SAFETY: `write` is async-signal-safe, the pointer names one readable byte, and a stale or
        // full descriptor can only make the best-effort wake fail; the atomic counters retain data.
        unsafe { libc::write(fd, (&raw const byte).cast(), 1) };
    }
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

        let registration = Registration::install().expect("install signal registration");
        // SAFETY: SIGINT is one of the signals owned by the registration during this test.
        assert_eq!(unsafe { libc::raise(libc::SIGINT) }, 0);
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
