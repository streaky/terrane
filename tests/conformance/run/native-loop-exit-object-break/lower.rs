// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let h: Holder;
    let i: i64;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),
        0 /* terrane-site: src/main.trn:7:9-7:21 */,
    );
    i = 0;
    while i < 2 {
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(h.into_label_len()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::into_label_len")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 1 /* terrane-site: src/main.trn:10:17-10:34 */))
        );
        break;
    }
}
