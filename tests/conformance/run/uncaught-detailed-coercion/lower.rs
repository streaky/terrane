// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: uncaught-detailed-coercion
fn narrow(value: terrane_int_support::Int) -> Result<i8, TerraneError> {
    return Ok(
        __terrane_raised_err(
            terrane_int_support::coerce::<i8>(&value),
            0 /* terrane-site: case.trn:4:10-4:15 */,
        )?,
    );
}
fn main() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_traced(narrow(terrane_int_support::Int::from(300_i128)),
        1 /* terrane-site: case.trn:6:11-6:22 */))
    );
}
