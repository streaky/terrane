#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Text(String),
    Binary(Vec<u8>),
    Ping,
    Pair(i64, i64),
    Named { value: i64 },
}
