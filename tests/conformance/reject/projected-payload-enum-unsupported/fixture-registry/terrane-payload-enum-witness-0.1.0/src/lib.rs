#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Text(String),
    Pair(i64, i64),
    Named { value: i64 },
}
