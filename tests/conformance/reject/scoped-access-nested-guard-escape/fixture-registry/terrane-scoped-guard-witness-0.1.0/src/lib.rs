use std::sync::{Mutex, MutexGuard};

pub struct Locks(Mutex<i64>);

impl Locks {
    pub fn new(value: i64) -> Self {
        Self(Mutex::new(value))
    }

    pub fn locked(&self) -> MutexGuard<'_, i64> {
        self.0.lock().expect("guard witness lock is not poisoned")
    }
}
