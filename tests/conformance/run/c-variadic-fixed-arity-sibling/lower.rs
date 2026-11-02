// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: app
fn main() {
    let value: terrane_int_support::Int;
    value = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(4_i128);
        __terrane_raised(
            fixed_value(__terrane_conversion_input_0),
            0 /* terrane-site: case.trn:6:13-6:27 */,
        )
    };
    println!("{}", terrane_scalar_support::scalar_text(&value));
}
