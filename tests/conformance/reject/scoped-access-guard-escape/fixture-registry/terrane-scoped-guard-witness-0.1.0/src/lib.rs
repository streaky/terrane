use std::sync::{Mutex, MutexGuard};

pub struct Locks(Mutex<i64>);
pub struct Guard<'a>(MutexGuard<'a, i64>);
impl Guard<'_> {
    pub fn value(&self) -> i64 { *self.0 }
}

impl Locks {
    pub fn new(value: i64) -> Self {
        Self(Mutex::new(value))
    }

    pub fn locked(&self) -> Guard<'_> {
        Guard(self.0.lock().expect("guard witness lock is not poisoned"))
    }
}
