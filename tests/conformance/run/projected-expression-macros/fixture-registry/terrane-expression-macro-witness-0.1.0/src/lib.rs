use std::sync::atomic::{AtomicI64, Ordering};

static EVALUATIONS: AtomicI64 = AtomicI64::new(0);

#[derive(Clone)]
pub struct Packet {
    pub value: i64,
    pub enabled: bool,
}

pub fn sum() -> i64 { 99 }
pub fn next() -> i64 { EVALUATIONS.fetch_add(1, Ordering::SeqCst) + 1 }
pub fn evaluations() -> i64 { EVALUATIONS.load(Ordering::SeqCst) }

#[macro_export]
macro_rules! sum {
    ($($value:expr),* $(,)?) => { 0_i64 $(+ $value)* };
}

#[macro_export]
macro_rules! packet {
    ($value:expr, $enabled:expr) => { $crate::Packet { value: $value, enabled: $enabled } };
}

#[macro_export]
macro_rules! message {
    ($format:literal $(, $value:expr)* $(,)?) => { format!($format $(, $value)*) };
}

#[macro_export]
macro_rules! values {
    ($($value:expr),* $(,)?) => { vec![$($value),*] };
}

#[macro_export]
macro_rules! twice {
    ($value:expr) => { ($value) + ($value) };
}

#[macro_export]
macro_rules! selected {
    ($condition:expr, $yes:expr, $no:expr) => { if $condition { $yes } else { $no } };
}

pub mod layout {
    pub use crate::sum as combine;
}

#[macro_export]
macro_rules! disable {
    ($packet:expr) => {{
        let packet = $packet;
        packet.enabled = false;
        packet.value
    }};
}
