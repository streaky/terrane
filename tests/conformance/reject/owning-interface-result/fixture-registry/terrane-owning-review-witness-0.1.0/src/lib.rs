use std::sync::Arc;
pub trait Worker { fn value(&self) -> i64; }
pub fn arc_worker() -> Arc<dyn Worker> { panic!("projection-only") }
