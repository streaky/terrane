#[derive(Clone)]
pub struct Event { pub active: bool }
pub type EventRef = &'static Event;
pub trait Producer<T> { fn produce(&self) -> T; }
impl<T, F: Fn() -> T> Producer<T> for F { fn produce(&self) -> T { self() } }
pub trait Borrower { fn borrow(&self) -> EventRef; }
impl<F> Borrower for F where F: Fn() -> EventRef {
    fn borrow(&self) -> EventRef { self() }
}
pub fn retain(callback: Box<dyn for<'a> Fn(&'a Event) -> &'a Event>, event: &Event) -> bool { callback(event).active }
pub fn retain_alias(callback: Box<dyn Fn() -> EventRef>) -> bool { callback().active }
pub fn retain_generic(callback: Box<dyn Producer<EventRef>>) -> bool { callback.produce().active }
pub fn retain_trait(callback: Box<dyn Borrower>) -> bool { callback.borrow().active }
