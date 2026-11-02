#[derive(Clone)]
pub struct Plain { pub size: (u32, u32) }
impl Plain { pub fn add(&self, pair: (u32, u32)) -> u32 { self.size.0 + self.size.1 + pair.0 + pair.1 } }
pub struct Nested { pub size: Option<(u32, u32)>, pub rows: Vec<(u32, u32)> }
pub fn plain_sum(value: Plain) -> u32 { value.size.0 + value.size.1 }
pub fn nested_sum(value: Nested) -> u32 { value.size.map_or(0, |pair| pair.0 + pair.1) + value.rows.iter().map(|pair| pair.0 + pair.1).sum::<u32>() }
pub fn pair_sum(value: (u32, u32)) -> u32 { value.0 + value.1 }
