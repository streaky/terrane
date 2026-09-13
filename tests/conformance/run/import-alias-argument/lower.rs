// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: later-coercion-alias-argument
fn main() {
    let value: i64 = 100;
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::coerce::
        < i8 > (&value), 0 /* terrane-site: case.trn:6:11-6:16 */))
    );
}
