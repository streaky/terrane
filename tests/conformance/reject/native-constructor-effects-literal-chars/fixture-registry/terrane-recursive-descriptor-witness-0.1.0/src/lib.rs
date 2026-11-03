pub struct Mark { pub mark: char }
pub struct Count { pub n: i64 }
pub struct Nested { pub marks: Vec<Option<char>> }
pub fn mark_code(value: Mark) -> u32 { value.mark as u32 }
pub fn count_value(value: Count) -> i64 { value.n }
pub fn char_code(value: char) -> u32 { value as u32 }
pub fn int_value(value: i64) -> i64 { value }
pub struct OptionalMark { pub mark: Option<char> }
pub fn mark_or_zero(value: OptionalMark) -> u32 { value.mark.map_or(0, |mark| mark as u32) }
impl Count {
    pub fn char_value(&self, value: char) -> u32 { value as u32 }
    pub fn int_value(&self, value: i64) -> i64 { value }
    pub async fn async_char(&self, value: char) -> u32 { value as u32 }
    pub async fn async_int(&self, value: i64) -> i64 { value }
}
#[macro_export]
macro_rules! native_int {
    ($value:expr) => { $value };
}
pub fn nested_count(value: Nested) -> usize { value.marks.len() }
