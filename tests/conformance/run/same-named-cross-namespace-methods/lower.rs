// Generated deterministically by Terrane <version>.
// Source: app/main.trn
// Namespace: app
fn main() {
    let first: TerraneNsFirstDuplicateIdentity2F6669727374006475706C6963617465;
    let second: TerraneNsSecondDuplicateIdentity2F7365636F6E64006475706C6963617465;
    first = TerraneNsFirstDuplicateIdentity2F6669727374006475706C6963617465::terrane_construct();
    second = TerraneNsSecondDuplicateIdentity2F7365636F6E64006475706C6963617465::terrane_construct();
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&first.value()),
        terrane_scalar_support::scalar_text(&second.value())
    );
}
// Source: first/item.trn
// Namespace: first
#[derive(Clone)]
pub struct TerraneNsFirstDuplicateIdentity2F6669727374006475706C6963617465 {}
impl TerraneNsFirstDuplicateIdentity2F6669727374006475706C6963617465 {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn value(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(1_i128);
    }
}
// Source: second/item.trn
// Namespace: second
#[derive(Clone)]
pub struct TerraneNsSecondDuplicateIdentity2F7365636F6E64006475706C6963617465 {}
impl TerraneNsSecondDuplicateIdentity2F7365636F6E64006475706C6963617465 {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn value(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(2_i128);
    }
}
