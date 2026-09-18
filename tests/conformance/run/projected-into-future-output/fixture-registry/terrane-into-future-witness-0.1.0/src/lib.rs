use std::future::{IntoFuture, Ready, ready};

pub struct ReadyValue(u32);

impl IntoFuture for ReadyValue {
    type Output = u32;
    type IntoFuture = Ready<u32>;

    fn into_future(self) -> Self::IntoFuture {
        ready(self.0)
    }
}

pub fn ready_value(value: u32) -> ReadyValue {
    ReadyValue(value)
}
