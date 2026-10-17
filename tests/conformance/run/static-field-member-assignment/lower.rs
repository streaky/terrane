// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
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
pub static TERRANE_STATIC_HOLDER_SLOT: std::sync::LazyLock<TerraneSharedCell<Inner>> = std::sync::LazyLock::new(||
TerraneSharedCell::new(Inner::terrane_construct()));
#[derive(Clone)]
pub struct Holder {}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn terrane_static_update() {
        {
            let __terrane_value = terrane_int_support::Int::from(5_i128);
            TERRANE_STATIC_HOLDER_SLOT
                .lock()
                .expect("static field lock poisoned")
                .value = __terrane_value;
        }
    }
}
fn main() {
    Holder::terrane_static_update();
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = { let
        __terrane_result = TERRANE_STATIC_HOLDER_SLOT.lock()
        .expect("static field lock poisoned").value.clone(); __terrane_result };
        __terrane_snapshot })
    );
}
