// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn make(present: bool) -> Option<Holder> {
    if present {
        return Some(
            __terrane_raised(
                terrane_static_trn_486f6c646572_new(),
                0 /* terrane-site: src/main.trn:7:12-7:24 */,
            ),
        );
    }
    return None;
}
fn main() {
    let y: Option<Holder>;
    y = make(true);
    if (&y).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(match &y { Some(value) => value, _ =>
            unreachable!("flow-proven storage refinement") } .label_len()) })) {
            Ok(result) => result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::label_len")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 1 /* terrane-site: src/main.trn:13:13-13:25 */))
        );
    }
}
