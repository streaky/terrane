// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn pick<'a, TerraneType54>(value: &'a TerraneType54) -> &'a TerraneType54 {
    return value;
}
fn main() {
    let h: Holder;
    let y: &terrane_query_witness::Holder;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),
        0 /* terrane-site: src/main.trn:9:7-9:19 */,
    );
    y = pick(&h);
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&* y).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 1 /* terrane-site: src/main.trn:11:11-11:23 */))
    );
}
