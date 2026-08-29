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
    let value: Option<Widget> = None;
    if value.is_none() {
        println!("{}", terrane_scalar_support::scalar_text(&String::from("empty")));
    }
}
