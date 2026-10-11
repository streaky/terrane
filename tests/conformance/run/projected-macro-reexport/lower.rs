// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[allow(dead_code)]
fn accepts_direct_owner(value: GeneratedValue) {
    let _ = &value;
}
fn main() {
    let facade_value: GeneratedValue;
    facade_value = __terrane_raised(
        terrane_static_trn_47656e65726174656456616c7565_standard(),
        0 /* terrane-site: src/main.trn:9:20-9:45 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&facade_value)
        .value())) { Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => Err(crate ::__terrane_dependency_panic(payload,
        "terrane_reexport_facade_witness",
        "terrane_reexport_facade_witness::GeneratedValue::value")) },
        1 /* terrane-site: src/main.trn:10:13-10:32 */))
    );
}
