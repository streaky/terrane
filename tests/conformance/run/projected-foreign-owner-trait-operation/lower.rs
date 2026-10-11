// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let value: Value;
    value = __terrane_raised(
        terrane_static_trn_56616c7565_new(terrane_int_support::Int::from(37_i128)),
        0 /* terrane-site: src/main.trn:6:19-6:33 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | < provider::Value as
        contract::Doubled > ::doubled(&value))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "provider", "provider::Value::doubled")) },
        1 /* terrane-site: src/main.trn:7:13-7:27 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | < provider::Value as
        contract::Doubled > ::doubled(&provider::Value::BASE))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "provider", "provider::Value::doubled")) },
        2 /* terrane-site: src/main.trn:8:13-8:35 */))
    );
}
