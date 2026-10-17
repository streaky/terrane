// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: contextual-constant-arithmetic
fn third() -> Result<f32, TerraneError> {
    return Ok(0.33333334_f32);
}
fn bounded() -> i32 {
    return 1;
}
fn main() {
    let reduced: i8;
    let integral: i64;
    let quotient: i64;
    let ratio: f32;
    let rounded: terrane_int_support::Int;
    reduced = 100;
    integral = 4;
    quotient = 0;
    ratio = 0.33333334_f32;
    rounded = {
        let __terrane_result = __terrane_raised(
            terrane_int_support::rounded_f32(
                ratio,
                terrane_int_support::FloatRounding::TiesEven,
            ),
            0 /* terrane-site: case.trn:14:17-14:29 */,
        );
        __terrane_result
    };
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
