// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: return-tail-block-strings
fn render(value: terrane_int_support::Int) -> Result<String, TerraneError> {
    if value.clone() < terrane_int_support::Int::from(0_i128) {
        return Err(
            TerraneError::raised(
                TerraneErrorKind::CoercionError,
                0 /* terrane-site: case.trn:6:5-6:25 */,
            ),
        );
    }
    return Ok(String::from("ok"));
}
fn render_lines() -> String {
    return String::from("first line\nsecond line");
}
fn main() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_traced(render(terrane_int_support::Int::from(1_i128)),
        1 /* terrane-site: case.trn:15:11-15:20 */))
    );
    println!("{}", terrane_scalar_support::scalar_text(&render_lines()));
}
