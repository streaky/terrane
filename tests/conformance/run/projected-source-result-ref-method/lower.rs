// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct Maker {}
impl Maker {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn make(&self) -> Holder {
        return __terrane_raised(
            terrane_static_trn_486f6c646572_new(),
            0 /* terrane-site: src/main.trn:7:12-7:24 */,
        );
    }
}
fn observe(maker: &Maker) {
    let y: Holder;
    y = maker.clone().make();
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&y).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 1 /* terrane-site: src/main.trn:11:11-11:23 */))
    );
}
fn main() {
    let maker: Maker;
    maker = Maker::terrane_construct();
    observe(&maker);
}
