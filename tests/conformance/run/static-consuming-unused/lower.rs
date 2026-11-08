// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: static-consuming-unused
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
    println!("{}", terrane_scalar_support::scalar_text(&1));
}
