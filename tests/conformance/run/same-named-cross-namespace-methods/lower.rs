// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-scalar-support
// Source: app/main.trn
// Namespace: app
fn main() {
    let first: TerraneNs5FirstDuplicate = TerraneNs5FirstDuplicate::terrane_construct();
    let second: TerraneNs6SecondDuplicate = TerraneNs6SecondDuplicate::terrane_construct();
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&first.value()),
        terrane_scalar_support::scalar_text(&second.value())
    );
}
// Source: first/item.trn
// Namespace: first
#[derive(Clone)]
pub struct TerraneNs5FirstDuplicate {}
impl TerraneNs5FirstDuplicate {
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
pub struct TerraneNs6SecondDuplicate {}
impl TerraneNs6SecondDuplicate {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn value(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(2_i128);
    }
}
