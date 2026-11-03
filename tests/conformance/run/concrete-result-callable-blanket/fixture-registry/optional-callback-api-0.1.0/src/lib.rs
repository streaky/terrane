pub trait Handler { fn handle(&self, value: i64) -> bool; }
impl<F: Fn(i64) -> bool> Handler for F { fn handle(&self, value: i64) -> bool { self(value) } }
pub fn use_handler<H: Handler>(handler: H) -> bool { handler.handle(7) }
