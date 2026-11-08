// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: static-bare-none
pub static TERRANE_STATIC_HOLDER_N: std::sync::LazyLock<
    TerraneSharedCell<Option<terrane_int_support::Int>>,
> = std::sync::LazyLock::new(|| TerraneSharedCell::new(
    Some(terrane_int_support::Int::from(3_i128)),
));
#[derive(Clone)]
pub struct Holder {}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self {}
    }
}
fn main() {
    {
        let __terrane_static_value: Option<terrane_int_support::Int> = None;
        *TERRANE_STATIC_HOLDER_N.lock().expect("static field lock poisoned") = __terrane_static_value;
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_value =
        TERRANE_STATIC_HOLDER_N.lock().expect("static field lock poisoned").clone();
        __terrane_value } .is_none())
    );
}
