// Generated deterministically by Terrane <version>.
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct Widget {}
impl Widget {
    pub fn terrane_construct() -> Self {
        Self {}
    }
}
fn main() {
    let value: Option<Widget>;
    value = None;
    if value.is_none() {
        println!("{}", String::from("empty"));
    }
}
