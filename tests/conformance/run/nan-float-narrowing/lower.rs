// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: nan-float-narrowing
fn main() {
    let zero: f64 = 0.0;
    let source: f64 = zero / zero;
    let target: f32 = {
        let source_value = source;
        let converted = source_value as f32;
        if converted as f64 == source_value {
            converted
        } else {
            __terrane_raised(
                Err(
                    terrane_int_support::ArithmeticError::conversion_overflow(
                        &source_value,
                        "float64",
                        "float32",
                        "the floating value is not exactly representable",
                    ),
                ),
                0 /* terrane-site: case.trn:5:20-5:26 */,
            )
        }
    };
    println!("{}", terrane_scalar_support::scalar_text(&target));
}
