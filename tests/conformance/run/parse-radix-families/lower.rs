// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: parse-radix-families
fn from_hex(source: String) -> terrane_int_support::Int {
    return __terrane_raised(
        terrane_int_support::parse_radix(&source, &16),
        0 /* terrane-site: case.trn:4:10-4:26 */,
    );
}
fn fail(source: String) -> Result<terrane_int_support::Int, TerraneError> {
    println!("{}", terrane_scalar_support::scalar_text(&source));
    return Err(
        TerraneError::raised(
            TerraneErrorKind::CoercionError,
            1 /* terrane-site: case.trn:7:3-7:23 */,
        ),
    );
}
fn main() {
    let text: String = String::from("ff");
    let parsed: terrane_int_support::Int = from_hex(text);
    println!("{}", terrane_scalar_support::scalar_text(&parsed));
    let value: i64 = 255;
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::format_radix(&value,
        &16), 2 /* terrane-site: case.trn:13:11-13:26 */))
    );
    let bad: String = String::from("x");
    fail(bad).ok();
}
