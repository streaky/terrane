#[derive(Clone)]
pub struct Address {
    pub port: i64,
}

pub struct Envelope<T> {
    pub payload: T,
}

pub struct Frame<T>(pub T);

pub fn address_port(envelope: &Envelope<Address>) -> i64 {
    envelope.payload.port
}

pub fn text_length(envelope: &Envelope<String>) -> i64 {
    envelope.payload.len() as i64
}

pub fn frame_port(frame: &Frame<Address>) -> i64 {
    frame.0.port
}

pub fn into_address(envelope: Envelope<Address>) -> Address {
    envelope.payload
}

impl<T> Envelope<T> {
    pub fn into_payload(self) -> T {
        self.payload
    }
}

pub struct Pair<T, U> {
    pub first: T,
    pub second: U,
}

pub fn pair_summary(pair: &Pair<String, Address>) -> String {
    format!("{}:{}", pair.first, pair.second.port)
}

pub struct Maybe<T> {
    pub payload: Option<T>,
    pub label: String,
}

pub fn maybe_port(value: &Maybe<Address>) -> i64 {
    value.payload.as_ref().map_or(-1, |address| address.port)
}

pub struct Printable<T: std::fmt::Display> {
    pub payload: T,
}

pub fn printable_text(value: &Printable<String>) -> String {
    value.payload.to_string()
}
