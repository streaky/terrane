use std::future::{ready, IntoFuture, Ready};
use std::io;
use std::sync::atomic::{AtomicU32, Ordering};

static CONSTRUCTIONS: AtomicU32 = AtomicU32::new(0);
static CONVERSIONS: AtomicU32 = AtomicU32::new(0);

pub struct ReadyValue<T>(T);

impl<T> IntoFuture for ReadyValue<T> {
    type Output = T;
    type IntoFuture = Ready<T>;

    fn into_future(self) -> Self::IntoFuture {
        CONVERSIONS.fetch_add(1, Ordering::SeqCst);
        ready(self.0)
    }
}

pub fn ready_value<T>(value: T) -> ReadyValue<T> {
    CONSTRUCTIONS.fetch_add(1, Ordering::SeqCst);
    ReadyValue(value)
}

pub struct ReadyResult(bool);

impl IntoFuture for ReadyResult {
    type Output = Result<u32, io::Error>;
    type IntoFuture = Ready<Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        CONVERSIONS.fetch_add(1, Ordering::SeqCst);
        ready(if self.0 {
            Ok(7)
        } else {
            Err(io::Error::other("ready result failed"))
        })
    }
}

pub fn ready_result(succeed: bool) -> ReadyResult {
    CONSTRUCTIONS.fetch_add(1, Ordering::SeqCst);
    ReadyResult(succeed)
}

pub fn construction_count() -> u32 {
    CONSTRUCTIONS.load(Ordering::SeqCst)
}

pub fn conversion_count() -> u32 {
    CONVERSIONS.load(Ordering::SeqCst)
}
