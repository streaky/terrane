// Generated deterministically by Terrane <version>.
// Source: app/main.trn
// Namespace: app
#[derive(Clone)]
pub struct Fancy {
    pub value: terrane_int_support::Int,
    pub extra: terrane_int_support::Int,
}
impl Fancy {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(7_i128),
            extra: terrane_int_support::Int::from(3_i128),
        }
    }
}
fn main() {
    let value: Fancy = Fancy::terrane_construct();
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&value.value),
        terrane_scalar_support::scalar_text(&String::from(":")),
        terrane_scalar_support::scalar_text(&value.extra)
    );
}
// Source: models/item.trn
// Namespace: models
#[derive(Clone)]
pub struct Item {
    pub value: terrane_int_support::Int,
}
impl Item {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(7_i128),
        }
    }
}
