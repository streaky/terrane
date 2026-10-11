// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: fractional-numeric-destination
fn main() {
    let ratio: f64;
    let count: terrane_int_support::Int;
    ratio = 4.2;
    count = __terrane_raised(
        terrane_int_support::exact_int_f64(ratio),
        0 /* terrane-site: case.trn:4:15-4:20 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&count));
}
