// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-string-support
// Source: case.trn
// Namespace: structured-error-origin-and-frames
fn leaf() -> Result<(), TerraneError> {
    return Err(
        TerraneError::raised(
            TerraneErrorKind::DivisionByZero,
            0 /* terrane-site: case.trn:5:3-5:25 */,
        ),
    );
}
fn middle() -> Result<(), TerraneError> {
    __terrane_traced_err(leaf(), 1 /* terrane-site: case.trn:8:3-8:8 */)?;
    Ok(())
}
fn main() {
    __terrane_traced(middle(), 2 /* terrane-site: case.trn:11:3-11:10 */);
}
