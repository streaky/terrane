// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn selected<TerraneType54, TerraneType55>(
    ignored: TerraneType55,
    value: TerraneType54,
) -> TerraneType54 {
    let _ = &ignored;
    return value;
}
fn main() {
    let y: terrane_query_witness::Holder;
    y = selected(
        __terrane_raised(
            terrane_static_trn_4f74686572_new(),
            1 /* terrane-site: src/main.trn:9:52-9:63 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),
            0 /* terrane-site: src/main.trn:9:26-9:38 */,
        ),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&y).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 2 /* terrane-site: src/main.trn:10:11-10:23 */))
    );
}
