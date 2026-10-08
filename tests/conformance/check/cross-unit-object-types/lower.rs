// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support
// Source: app/main.trn
// Namespace: app
fn identity(value: Item) -> Item {
    return value;
}
fn main() {
    let original_terrane_f0_s112: Item;
    let copied_terrane_f0_s142: Item;
    original_terrane_f0_s112 = Item::terrane_construct();
    copied_terrane_f0_s142 = identity(original_terrane_f0_s112);
    let _ = &copied_terrane_f0_s142;
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
