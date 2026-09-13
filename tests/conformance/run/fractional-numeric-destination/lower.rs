// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: fractional-numeric-destination
fn main() {
    let ratio: f64 = 4.2;
    let count: terrane_int_support::Int = __terrane_raised(
        terrane_int_support::exact_int_f64(ratio),
        0 /* terrane-site: case.trn:4:15-4:20 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&count));
}
