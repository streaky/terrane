// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: structured-assignment-this
#[derive(Clone)]
pub struct Item {
    pub x: terrane_int_support::Int,
}
impl Item {
    pub fn terrane_construct() -> Self {
        Self {
            x: terrane_int_support::Int::from(0_i128),
        }
    }
    pub fn bump(&mut self) {
        {
            let __terrane_value: terrane_int_support::Int = terrane_int_support::Int::from(
                5_i128,
            );
            self.x = __terrane_value;
        }
    }
}
fn main() {
    let mut value: Item;
    value = Item::terrane_construct();
    value.bump();
    println!("{}", terrane_scalar_support::scalar_text(&value.x));
}
