// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: app
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
fn maybe(present: bool) -> Option<Item> {
    if present {
        return Some(Item::terrane_construct());
    }
    return None;
}
fn show(present: bool) {
    let owner: Option<Item>;
    let borrowed: &Option<Item>;
    owner = maybe(present);
    borrowed = &owner;
    if borrowed.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot =
            borrowed.as_ref().expect("semantic optional narrowing").value.clone();
            __terrane_snapshot })
        );
    } else {
        println!("{}", String::from("absent"));
    }
}
fn show_shared(present: bool) {
    let owner: std::sync::Arc<TerraneSharedCell<Option<Item>>>;
    let borrowed: std::sync::Weak<TerraneSharedCell<Option<Item>>>;
    owner = std::sync::Arc::new(TerraneSharedCell::new(maybe(present)));
    borrowed = std::sync::Arc::downgrade(&owner);
    if {
        let __terrane_owner_0 = (&borrowed).upgrade().expect("reference expired");
        __terrane_owner_0.lock().expect("reference lock poisoned").is_some()
    } {
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = { let
            __terrane_owner = borrowed.upgrade().expect("reference expired");
            __terrane_owner.lock().expect("reference lock poisoned").as_ref()
            .expect("semantic optional narrowing").value.clone() }; __terrane_snapshot })
        );
    } else {
        println!("{}", String::from("absent-shared"));
    }
}
fn main() {
    show(true);
    show(false);
    show_shared(true);
    show_shared(false);
}
