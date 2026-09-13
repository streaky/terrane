// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: integer-coercion-failure
fn main() {
    let value: i64 = 128;
    let narrow: i8 = __terrane_raised(
        terrane_int_support::coerce::<i8>(&value),
        0 /* terrane-site: case.trn:5:17-5:22 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&narrow));
}
