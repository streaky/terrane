// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: static-field-member-assignment
#[derive(Clone)]
pub struct Inner {
    pub value: terrane_int_support::Int,
}
impl Inner {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(0_i128),
        }
    }
}
pub static TERRANE_STATIC_HOLDER_SLOT: std::sync::LazyLock<std::sync::Mutex<Inner>> = std::sync::LazyLock::new(||
std::sync::Mutex::new(Inner::terrane_construct()));
#[derive(Clone)]
pub struct Holder {}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn terrane_static_update() {
        TERRANE_STATIC_HOLDER_SLOT.lock().expect("static field lock poisoned").value = terrane_int_support::Int::from(
            5_i128,
        );
    }
}
fn main() {
    Holder::terrane_static_update();
    println!(
        "{}", terrane_scalar_support::scalar_text(&TERRANE_STATIC_HOLDER_SLOT.lock()
        .expect("static field lock poisoned").clone().value)
    );
}
