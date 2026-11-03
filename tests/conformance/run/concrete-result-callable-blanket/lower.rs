// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn truth(value: terrane_int_support::Int) -> bool {
    return value == terrane_int_support::Int::from(7_i128);
}
fn main() {
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        std::sync::Arc::new(truth);
        __terrane_raised(use_handler(__terrane_conversion_input_0), 0 /* terrane-site: src/main.trn:8:13-8:31 */) })
    );
}
