use std::sync::{Arc, Weak};
use std::sync::atomic::{AtomicUsize, Ordering};

static EVENT_CLONES: AtomicUsize = AtomicUsize::new(0);
static SIGNAL_DROPS: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
pub struct Event { pub value: i64 }
impl Clone for Event {
    fn clone(&self) -> Self {
        EVENT_CLONES.fetch_add(1, Ordering::SeqCst);
        Self { value: self.value }
    }
}
pub fn event_clones() -> usize { EVENT_CLONES.load(Ordering::SeqCst) }

pub trait EventListener<E>: Send + Sync {
    fn on_event(&self, event: &E);
}
impl<E, F> EventListener<E> for F where F: Fn(&E) + Send + Sync {
    fn on_event(&self, event: &E) { self(event) }
}
#[derive(Clone, Debug)]
pub struct MutableEvent { pub active: bool }
pub type SharedListener<E> = Arc<dyn EventListener<E> + Send + Sync>;
pub type WeakListener<E> = Weak<dyn EventListener<E> + Send + Sync>;
pub type MutableListener = Box<dyn FnMut(&mut MutableEvent)>;
pub type OnceListener = Box<dyn FnOnce(Event)>;
pub type ListenerAlias<E> = SharedListener<E>;
pub fn invoke_mut(mut callback: MutableListener, event: &mut MutableEvent) { callback(event); callback(event); }
pub fn invoke_once(callback: OnceListener, event: Event) { callback(event); }
pub fn invoke_shared(callback: Arc<dyn Fn(&Event) + Send + Sync>, event: &Event) { callback(event); }

pub trait Sink: Send + Sync {
    fn accept(&self, event: &Event) -> i64;
}
pub fn invoke_sink(sink: Arc<dyn Sink + Send + Sync>, event: &Event) -> i64 { sink.accept(event) }

pub struct Signal;
impl Signal {
    pub fn new() -> Self { Self }
    pub fn inspect(&self) -> i64 { 1 }
    pub fn drops() -> usize { SIGNAL_DROPS.load(Ordering::SeqCst) }
}
impl Drop for Signal {
    fn drop(&mut self) { SIGNAL_DROPS.fetch_add(1, Ordering::SeqCst); }
}

#[derive(Clone)]
pub struct SendOnly { value: std::cell::Cell<u32> }
impl SendOnly {
    pub fn new() -> Self { Self { value: std::cell::Cell::new(0) } }
    pub fn bump(&self) -> u32 { let value = self.value.get() + 1; self.value.set(value); value }
    pub fn get(&self) -> u32 { self.value.get() }
}
#[derive(Clone)]
pub struct LocalOnly { value: std::rc::Rc<()> }
impl LocalOnly {
    pub fn new() -> Self { Self { value: std::rc::Rc::new(()) } }
    pub fn get(&self) -> u32 { std::rc::Rc::strong_count(&self.value) as u32 }
}
pub fn invoke_send_mut(mut callback: Box<dyn FnMut(&mut MutableEvent) + Send>, event: &mut MutableEvent) {
    callback(event);
    callback(event);
}
pub fn invoke_sync(callback: Box<dyn Fn(&Event) + Sync>, event: &Event) { callback(event); }
pub fn invoke_borrowed_once(callback: Box<dyn FnOnce(&Event)>, event: &Event) { callback(event); }
pub type ObjectListener<E> = dyn EventListener<E> + Send + Sync;
pub type ObjectCarrier<E> = Arc<ObjectListener<E>>;
pub fn invoke_object_alias(callback: ObjectCarrier<Event>, event: &Event) { callback.on_event(event); }
pub fn invoke_text(mut callback: Box<dyn FnMut(&str)>) { callback("first"); callback("second"); }
pub fn invoke_number(mut callback: Box<dyn FnMut(&mut i64)>) -> i64 { let mut value = 6; callback(&mut value); value }
