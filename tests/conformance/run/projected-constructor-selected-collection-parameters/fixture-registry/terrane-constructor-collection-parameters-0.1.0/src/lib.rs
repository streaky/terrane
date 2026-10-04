pub struct Batch<T> { pub values: Vec<T>, pub enabled: bool }
pub fn total(batch: Batch<i64>) -> i64 { if batch.enabled { batch.values.into_iter().sum() } else { 0 } }
