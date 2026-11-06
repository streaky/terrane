use std::sync::Arc;
pub struct Event { pub value: i64 }
pub type SharedListener<E> = Arc<dyn Fn(&E) -> bool + Send + Sync>;
pub fn invoke_optional_direct(callback: Option<Arc<dyn Fn(&Event) -> bool + Send + Sync>>, event: &Event) -> bool { callback.map(|callback| callback(event)).unwrap_or(false) }
pub fn invoke_optional_alias(callback: Option<SharedListener<Event>>, event: &Event) -> bool { callback.map(|callback| callback(event)).unwrap_or(false) }
pub fn invoke_vec(callbacks: Vec<Arc<dyn Fn(&Event) -> bool + Send + Sync>>, event: &Event) -> usize { callbacks.iter().filter(|callback| callback(event)).count() }
pub trait Handler { fn handle(&self, value: i64) -> bool; }
impl<F: Fn(i64) -> bool> Handler for F { fn handle(&self, value: i64) -> bool { self(value) } }
pub fn use_handler<H: Handler>(handler: H) -> bool { handler.handle(7) }
#[derive(Clone)]
pub struct MutableEvent { pub active: bool }
pub fn invoke_optional_box(callback: Option<Box<dyn FnMut(&mut MutableEvent) -> bool>>, event: &mut MutableEvent) -> bool {
    callback.map(|mut callback| callback(event)).unwrap_or(false)
}
