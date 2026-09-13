// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: contextual-constant-arithmetic
fn third() -> Result<f32, TerraneError> {
    return Ok(0.33333334_f32);
}
fn bounded() -> i32 {
    return 1;
}
fn main() {
    let reduced: i8 = 100;
    let integral: i64 = 4;
    let quotient: i64 = 0;
    let ratio: f32 = 0.33333334_f32;
    let rounded: terrane_int_support::Int = __terrane_raised(
        terrane_int_support::rounded_f32(
            ratio,
            terrane_int_support::FloatRounding::TiesEven,
        ),
        0 /* terrane-site: case.trn:14:17-14:29 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&reduced));
    println!("{}", terrane_scalar_support::scalar_text(&integral));
    println!("{}", terrane_scalar_support::scalar_text(&quotient));
    println!("{}", terrane_scalar_support::scalar_text(&ratio));
    println!("{}", terrane_scalar_support::scalar_text(&rounded));
    println!("{}", terrane_scalar_support::scalar_text(&true));
    println!("{}", terrane_scalar_support::scalar_text(&false));
    println!("{}", terrane_scalar_support::scalar_text(&bounded()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced(third(),
        1 /* terrane-site: case.trn:23:11-23:17 */))
    );
}
