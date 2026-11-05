// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn make() -> Holder {
    return __terrane_raised(
        terrane_static_trn_486f6c646572_new(),
        0 /* terrane-site: src/main.trn:7:12-7:24 */,
    );
}
fn apply<TerraneType54>(
    factory: std::sync::Arc<dyn Fn() -> TerraneType54 + Send + Sync>,
) -> TerraneType54 {
    return factory();
}
fn main() {
    let value: Holder;
    value = apply(std::sync::Arc::new(make));
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&value).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 1 /* terrane-site: src/main.trn:14:13-14:29 */))
    );
}
