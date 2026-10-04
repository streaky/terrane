#[derive(Clone)]
pub enum Choice<T> { Empty, Value(T), Named { value: T, enabled: bool } }
pub fn score(value: &Choice<i64>) -> i64 { match value { Choice::Empty => 0, Choice::Value(value) => *value, Choice::Named { value, enabled } => if *enabled { *value } else { -*value } } }
