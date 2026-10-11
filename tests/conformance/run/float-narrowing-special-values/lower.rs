// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: float-narrowing-special-values
fn main() {
    let zero: f64;
    let one: f64;
    let negative_one: f64;
    let negative_zero: f64;
    let maximum: f64;
    let positive_infinity: f64;
    let negative_infinity: f64;
    let narrowed_zero: f32;
    let narrowed_positive: f32;
    let narrowed_negative: f32;
    let narrowed_maximum: f32;
    zero = 0.0;
    one = 1.0;
    negative_one = -1.0_f64;
    negative_zero = -0.0_f64;
    maximum = 340282346638528859811704183484516925440.0;
    positive_infinity = one / zero;
    negative_infinity = negative_one / zero;
    narrowed_zero = {
        let source_value = negative_zero;
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
                0 /* terrane-site: case.trn:10:27-10:40 */,
            )
        }
    };
    narrowed_positive = {
        let source_value = positive_infinity;
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
                1 /* terrane-site: case.trn:11:31-11:48 */,
            )
        }
    };
    narrowed_negative = {
        let source_value = negative_infinity;
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
                2 /* terrane-site: case.trn:12:31-12:48 */,
            )
        }
    };
    narrowed_maximum = {
        let source_value = maximum;
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
                3 /* terrane-site: case.trn:13:30-13:37 */,
            )
        }
    };
    println!("{}", terrane_scalar_support::scalar_text(&narrowed_zero));
    println!("{}", terrane_scalar_support::scalar_text(&narrowed_positive));
    println!("{}", terrane_scalar_support::scalar_text(&narrowed_negative));
    println!("{}", terrane_scalar_support::scalar_text(&narrowed_maximum));
}
