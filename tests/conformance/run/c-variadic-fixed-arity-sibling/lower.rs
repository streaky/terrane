// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: app
fn main() {
    let value: terrane_int_support::Int;
    value = __terrane_raised(
        fixed_value(terrane_int_support::Int::from(4_i128)),
        0 /* terrane-site: case.trn:6:13-6:27 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&value));
}
