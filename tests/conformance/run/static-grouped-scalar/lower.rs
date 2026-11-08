// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: static-grouped-scalar
pub static TERRANE_STATIC_HOLDER_N: std::sync::LazyLock<
    TerraneSharedCell<terrane_int_support::Int>,
> = std::sync::LazyLock::new(|| TerraneSharedCell::new(
    terrane_int_support::Int::from(0_i128),
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
        let __terrane_static_value = terrane_int_support::Int::from(4_i128);
        *TERRANE_STATIC_HOLDER_N.lock().expect("static field lock poisoned") = __terrane_static_value;
    }
    println!(
        "{}", { let __terrane_result = terrane_scalar_support::scalar_text(&*
        TERRANE_STATIC_HOLDER_N.lock().expect("static field lock poisoned"));
        __terrane_result }
    );
}
