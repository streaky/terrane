use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::{Arc, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Clone)]
pub struct Record { value: i64 }
impl Record {
    pub fn value(&self) -> i64 { self.value }
    pub fn set(&mut self, value: i64) { self.value = value; }
    pub fn take(self) -> i64 { self.value }
    pub fn boxed(self: Box<Self>) -> i64 { self.value }
}
pub fn record(value: i64) -> Record { Record { value } }
pub fn arc(value: i64) -> Arc<Record> { Arc::new(Record { value }) }
pub fn same_arc(left: &Arc<Record>, right: &Arc<Record>) -> bool { Arc::ptr_eq(left, right) }
pub fn rc(value: i64) -> Rc<Record> { Rc::new(Record { value }) }
pub fn boxed(value: i64) -> Box<Record> { Box::new(Record { value }) }
pub fn chain(value: i64) -> Arc<Rc<Record>> { Arc::new(Rc::new(Record { value })) }
pub fn nested_box(value: i64) -> Box<Box<Record>> { Box::new(Box::new(Record { value })) }

pub struct RecordHandle { inner: Record }
impl Deref for RecordHandle {
    type Target = Record;
    fn deref(&self) -> &Record { &self.inner }
}
impl DerefMut for RecordHandle {
    fn deref_mut(&mut self) -> &mut Record { &mut self.inner }
}
pub fn wrapped(value: i64) -> RecordHandle { RecordHandle { inner: Record { value } } }

#[derive(Clone)]
pub struct RecordView<'a> { record: &'a Record }
impl RecordView<'_> {
    pub fn value(&self) -> i64 { self.record.value }
    pub fn nested(&self) -> RecordView<'_> { RecordView { record: self.record } }
}
impl RecordHandle {
    pub fn view(&self) -> RecordView<'_> { RecordView { record: &self.inner } }
}

pub struct ReadOnlyHandle { inner: Record }
impl Deref for ReadOnlyHandle {
    type Target = Record;
    fn deref(&self) -> &Record { &self.inner }
}
pub fn readonly(value: i64) -> ReadOnlyHandle { ReadOnlyHandle { inner: Record { value } } }

pub struct Locks { mutex: Mutex<Record>, rwlock: RwLock<Record> }
impl Locks {
    pub fn new(value: i64) -> Self { Self { mutex: Mutex::new(Record { value }), rwlock: RwLock::new(Record { value }) } }
    pub fn locked(&self) -> MutexGuard<'_, Record> { self.mutex.lock().unwrap() }
    pub fn read(&self) -> RwLockReadGuard<'_, Record> { self.rwlock.read().unwrap() }
    pub fn write(&self) -> RwLockWriteGuard<'_, Record> { self.rwlock.write().unwrap() }
    pub fn available(&self) -> bool { self.mutex.try_lock().is_ok() }
    pub fn writable(&self) -> bool { self.rwlock.try_write().is_ok() }
}
