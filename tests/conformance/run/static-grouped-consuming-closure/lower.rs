// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: static-grouped-consuming-closure
pub static TERRANE_STATIC_HOLDER_F: std::sync::LazyLock<
    TerraneSharedCell<Option<TerraneConsumingCallable<(), terrane_int_support::Int>>>,
> = std::sync::LazyLock::new(|| TerraneSharedCell::new(None));
pub struct Holder {}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self {}
    }
}
fn main() {
    let closure: TerraneConsumingCallable<(), terrane_int_support::Int>;
    closure = {
        TerraneConsumingCallable::new(move |(): ()| -> terrane_int_support::Int {
            return terrane_int_support::Int::from(1_i128);
        })
    };
    {
        let __terrane_static_value = Some(closure);
        *TERRANE_STATIC_HOLDER_F.lock().expect("static field lock poisoned") = __terrane_static_value;
    }
    println!("{}", terrane_scalar_support::scalar_text(&1));
}
