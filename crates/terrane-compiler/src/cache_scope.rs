use std::cell::Cell;

thread_local! {
    static ENABLED: Cell<bool> = const { Cell::new(false) };
}

/// Enables persistent compilation caches for the current thread for the duration of `action`.
///
/// The previous setting is restored even if `action` unwinds.
pub fn with_compilation_cache<T>(action: impl FnOnce() -> T) -> T {
    struct Restore(bool);
    impl Drop for Restore {
        fn drop(&mut self) {
            ENABLED.set(self.0);
        }
    }

    let previous = ENABLED.get();
    ENABLED.set(true);
    let _restore = Restore(previous);
    action()
}

pub(crate) fn enabled() -> bool {
    ENABLED.get()
}
