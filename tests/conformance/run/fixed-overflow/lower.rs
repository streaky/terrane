// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: fixed-overflow
fn main() {
    let left: i8 = 120;
    let right: i8 = 10;
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_addition(left,
        right), 0 /* terrane-site: case.trn:5:11-5:23 */))
    );
}
