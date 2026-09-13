// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: fixed-integer-float-destination
fn main() {
    let signed64: i64 = 9007199254740992;
    let signed64_float: f64 = __terrane_raised(
        terrane_int_support::exact_fixed_f64(signed64),
        0 /* terrane-site: case.trn:6:28-6:36 */,
    );
    let unsigned64: u64 = 9223372036854775808;
    let unsigned64_float: f64 = __terrane_raised(
        terrane_int_support::exact_fixed_f64(unsigned64),
        1 /* terrane-site: case.trn:8:30-8:40 */,
    );
    let signed128: i128 = -170141183460469231731687303715884105728;
    let signed128_float: f64 = __terrane_raised(
        terrane_int_support::exact_fixed_f64(signed128),
        2 /* terrane-site: case.trn:10:29-10:38 */,
    );
    let unsigned128: u128 = 170141183460469231731687303715884105728;
    let unsigned128_float: f64 = __terrane_raised(
        terrane_int_support::exact_fixed_f64(unsigned128),
        3 /* terrane-site: case.trn:12:31-12:42 */,
    );
    let signed32: i32 = 16777216;
    let signed32_float: f32 = __terrane_raised(
        terrane_int_support::exact_fixed_f32(signed32),
        4 /* terrane-site: case.trn:14:28-14:36 */,
    );
    let largest_float32_integer: u128 = 340282346638528859811704183484516925440;
    let largest_float32: f32 = __terrane_raised(
        terrane_int_support::exact_fixed_f32(largest_float32_integer),
        5 /* terrane-site: case.trn:16:29-16:52 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&signed64_float));
    println!("{}", terrane_scalar_support::scalar_text(&unsigned64_float));
    println!("{}", terrane_scalar_support::scalar_text(&signed128_float));
    println!("{}", terrane_scalar_support::scalar_text(&unsigned128_float));
    println!("{}", terrane_scalar_support::scalar_text(&signed32_float));
    println!("{}", terrane_scalar_support::scalar_text(&largest_float32));
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let inexact64: u64 = 18446744073709551615;
            let rejected64: f64 = __terrane_raised_completion!(
                terrane_int_support::exact_fixed_f64(inexact64), 6 /* terrane-site: case.trn:27:26-27:35 */
            );
            println!("{}", terrane_scalar_support::scalar_text(&rejected64));
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0
                    && __terrane_error_0.kind
                        == TerraneErrorKind::IntegerConversionOverflow
                {
                    __terrane_handled_0 = true;
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("inexact float64 caught"))
                    );
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            let inexact32: u128 = 340282366920938463463374607431768211455;
            let rejected32: f32 = __terrane_raised_completion!(
                terrane_int_support::exact_fixed_f32(inexact32), 7 /* terrane-site: case.trn:34:26-34:35 */
            );
            println!("{}", terrane_scalar_support::scalar_text(&rejected32));
            TerraneCompletion::Normal
        })();
        match __terrane_try_1 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_1) => {
                let mut __terrane_handled_1 = false;
                if !__terrane_handled_1
                    && __terrane_error_1.kind
                        == TerraneErrorKind::IntegerConversionOverflow
                {
                    __terrane_handled_1 = true;
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("inexact float32 caught"))
                    );
                }
                if !__terrane_handled_1 {
                    return TerraneCompletion::Error(__terrane_error_1);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_1 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
