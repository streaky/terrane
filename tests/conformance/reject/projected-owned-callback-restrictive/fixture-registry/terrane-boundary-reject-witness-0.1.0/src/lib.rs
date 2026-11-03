use std::fmt::Debug;
use std::sync::Arc;
pub fn consume_nested(callback: Box<dyn Fn(Box<dyn Fn() -> i64>) -> i64>) -> i64 { callback(Box::new(|| 7)) }
pub trait DebugListener { fn on_event(&self) -> i64; }
impl<F: Fn() -> i64 + Debug> DebugListener for F { fn on_event(&self) -> i64 { self() } }
pub fn consume_restricted(callback: Arc<dyn DebugListener>) -> i64 { callback.on_event() }
