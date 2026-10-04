#[non_exhaustive]
pub enum OpenEvent {
    Known(String),
    Future,
}

pub fn future_event() -> OpenEvent {
    OpenEvent::Future
}
