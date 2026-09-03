// Generated deterministically by Terrane <version>.
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct TerraneDescriptor {
    identity: &'static str,
    name: &'static str,
    kind: &'static str,
}
// Source: case.trn
// Namespace: method-without-receiver-use
#[derive(Clone)]
pub struct Answerer {}
impl Answerer {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn answer(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(42_i128);
    }
}
fn main() {
    let value: Answerer = Answerer::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&value.answer()));
}
