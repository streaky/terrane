use std::sync::{Mutex, MutexGuard};
pub struct Record;
impl Record {
    pub fn value(&self) -> i64 {
        // Exercise the panic payload boundary without a process-specific panic-hook log.
        std::panic::resume_unwind(Box::new(String::from("record value panic")))
    }
}
pub struct Locks { value: Mutex<Record> }
impl Locks {
    pub fn new() -> Self { Self { value: Mutex::new(Record) } }
    pub fn locked(&self) -> MutexGuard<'_, Record> { self.value.lock().unwrap() }
}
