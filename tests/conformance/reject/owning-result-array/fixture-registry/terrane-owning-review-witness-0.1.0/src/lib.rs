use std::sync::Arc;
#[derive(Clone)]
pub struct Event { pub value: i64 }
pub type EventRef = &'static Event;
pub fn array_result(_: Box<dyn Fn() -> [EventRef; 1]>) {}
pub fn slice_result(_: Box<dyn Fn() -> [EventRef]>) {}
pub trait Source { type Output; }
pub struct Borrowed;
impl Source for Borrowed { type Output = EventRef; }
pub fn associated_result(_: Box<dyn Fn() -> <Borrowed as Source>::Output>) {}
pub struct Good;
pub struct Bad;
pub trait Handler<Tag> { fn handle(&self, value: &Event) -> i64; }
impl<F: Fn(&Event) -> i64 + std::fmt::Debug> Handler<Bad> for F {
    fn handle(&self, value: &Event) -> i64 { self(value) }
}
impl<F: Fn(&Event) -> i64> Handler<Good> for F {
    fn handle(&self, value: &Event) -> i64 { self(value) }
}
pub fn invoke_good(callback: Arc<dyn Handler<Good>>, event: &Event) -> i64 { callback.handle(event) }
