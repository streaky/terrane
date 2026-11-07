// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: helpers/maker.trn
// Namespace: helpers
#[allow(dead_code)]
fn make() -> Holder {
    return __terrane_raised(
        terrane_static_trn_486f6c646572_new(),
        0 /* terrane-site: helpers/maker.trn:6:10-6:17 */,
    );
}
// Source: src/main.trn
// Namespace: app
fn same(h: Holder) -> Holder {
    return h;
}
fn main() {
    let y: Holder;
    y = same(
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),
            1 /* terrane-site: src/main.trn:9:14-9:26 */,
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
