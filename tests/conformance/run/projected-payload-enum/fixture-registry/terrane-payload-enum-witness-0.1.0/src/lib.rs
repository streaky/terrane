#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Text(String),
    Binary(Vec<u8>),
    Ping,
    Pair(i64, i64),
    Named { value: i64 },
    Optional {
        value: Option<u32>,
        label: Option<String>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct NonClonePayload(String);

#[derive(Debug, PartialEq, Eq)]
pub enum OwnedEvent {
    Payload(NonClonePayload),
    Other,
}

pub fn non_clone_payload(value: String) -> NonClonePayload {
    NonClonePayload(value)
}

pub fn consume_payload(value: NonClonePayload) -> String {
    value.0
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OpenEvent {
    Known(String),
    #[doc(hidden)]
    Future,
}

pub fn future_event() -> OpenEvent {
    OpenEvent::Future
}
