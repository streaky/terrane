// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-scalar-support
// Source: app/main.trn
// Namespace: app
fn main() {
    let first_terrane_f0_s129: TerraneNs5FirstDuplicate;
    let second_terrane_f0_s165: TerraneNs6SecondDuplicate;
    first_terrane_f0_s129 = TerraneNs5FirstDuplicate::terrane_construct();
    second_terrane_f0_s165 = TerraneNs6SecondDuplicate::terrane_construct();
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&first_terrane_f0_s129.value()),
        terrane_scalar_support::scalar_text(&second_terrane_f0_s165.value())
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
