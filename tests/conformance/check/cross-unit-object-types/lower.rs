// Generated deterministically by Terrane <version>.
// Source: app/main.trn
// Namespace: app
fn identity(value: Item) -> Item {
    return value.clone();
}
fn main() {
    let original: Item = Item::terrane_construct();
    let copied: Item = identity(original.clone());
    let _ = &copied;
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
