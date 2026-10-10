//! Opt-in, synchronous progress for the existing compiler pipeline.
//!
//! Reporting is scoped to the calling thread so library and language-server callers stay
//! silent, including after a reported compilation returns an error or unwinds.

use std::cell::Cell;
use std::fmt::Display;
use std::time::Instant;

thread_local! {
    static REPORTING: Cell<bool> = const { Cell::new(false) };
}

struct ReportingScope(bool);

impl Drop for ReportingScope {
    fn drop(&mut self) {
        REPORTING.set(self.0);
    }
}

/// Runs an action with live internal compilation progress and wall-clock timings on stderr.
///
/// Reporting applies only to this thread and is restored when the action returns or unwinds.
/// Ordinary compilation APIs remain silent unless called inside this scope.
pub fn with_compilation_progress<T>(action: impl FnOnce() -> T) -> T {
    let _scope = ReportingScope(REPORTING.replace(true));
    action()
}

pub(crate) fn note(message: &'static str, target: impl Display) {
    if REPORTING.get() {
        eprintln!("  Compilation [{target}]: {message}");
    }
}

/// Capture an identity before mutating its owner, without allocating for silent callers.
pub(crate) fn owned_target(target: impl Display) -> String {
    if REPORTING.get() {
        target.to_string()
    } else {
        String::new()
    }
}

pub(crate) struct Stage<T: Display> {
    name: &'static str,
    target: T,
    started: Option<Instant>,
}

pub(crate) fn start<T: Display>(name: &'static str, target: T) -> Stage<T> {
    let started = REPORTING.get().then(|| {
        eprintln!("  Compiling [{target}]: {name}...");
        Instant::now()
    });
    Stage {
        name,
        target,
        started,
    }
}

impl<T: Display> Stage<T> {
    /// Complete explicitly: an early error must not print a successful phase timing.
    pub(crate) fn finish(self) {
        if let Some(started) = self.started {
            eprintln!(
                "  Compiled [{}]: {} ({:.3}s)",
                self.target,
                self.name,
                started.elapsed().as_secs_f64()
            );
        }
    }
}
