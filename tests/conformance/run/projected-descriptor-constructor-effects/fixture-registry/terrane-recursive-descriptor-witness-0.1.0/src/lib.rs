pub struct Descriptor<'a> { pub text: &'a str, pub window: Option<(u32, u32)> }
pub struct Marked<'a> { pub text: &'a str, pub mark: char }
pub struct Rows<'a> { pub text: &'a str, pub rows: Vec<(u32, u32)> }
pub fn mark_text(value: &Marked<'_>) -> String { value.mark.to_string() }
pub fn describe(value: &Descriptor<'_>) -> String {
    format!("{}:{}", value.text, if value.window.is_some() { "some" } else { "none" })
}
pub fn describe_rows(value: &Rows<'_>) -> String {
    format!("{}:{}:{}", value.text, value.rows.len(), value.rows[0].0 + value.rows[0].1)
}
