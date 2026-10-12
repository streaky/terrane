// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: fixed-integer-float-destination
fn main() {
    let signed64: i64;
    let signed64_float: f64;
    let unsigned64: u64;
    let unsigned64_float: f64;
    let signed128: i128;
    let signed128_float: f64;
    let unsigned128: u128;
    let unsigned128_float: f64;
    let signed32: i32;
    let signed32_float: f32;
    let largest_float32_integer: u128;
    let largest_float32: f32;
    let mut inexact64: Option<u64> = None;
    let mut rejected64: Option<f64> = None;
    let mut inexact32: Option<u128> = None;
    let mut rejected32: Option<f32> = None;
    signed64 = 9007199254740992;
    signed64_float = __terrane_raised(
        terrane_int_support::exact_fixed_f64(signed64),
        0 /* terrane-site: case.trn:6:28-6:36 */,
    );
    unsigned64 = 9223372036854775808;
    unsigned64_float = __terrane_raised(
        terrane_int_support::exact_fixed_f64(unsigned64),
        1 /* terrane-site: case.trn:8:30-8:40 */,
    );
    signed128 = -170141183460469231731687303715884105728;
    signed128_float = __terrane_raised(
        terrane_int_support::exact_fixed_f64(signed128),
        2 /* terrane-site: case.trn:10:29-10:38 */,
    );
    unsigned128 = 170141183460469231731687303715884105728;
    unsigned128_float = __terrane_raised(
        terrane_int_support::exact_fixed_f64(unsigned128),
        3 /* terrane-site: case.trn:12:31-12:42 */,
    );
    signed32 = 16777216;
    signed32_float = __terrane_raised(
        terrane_int_support::exact_fixed_f32(signed32),
        4 /* terrane-site: case.trn:14:28-14:36 */,
    );
    largest_float32_integer = 340282346638528859811704183484516925440;
    largest_float32 = __terrane_raised(
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
            let _ = inexact64.insert(18446744073709551615);
            let _ = rejected64
                .insert(
                    __terrane_raised_completion!(
                        terrane_int_support::exact_fixed_f64(* inexact64.as_ref()
                        .expect("flow-proven available binding")), 6 /* terrane-site: case.trn:27:26-27:35 */
                    ),
                );
            println!(
                "{}", terrane_scalar_support::scalar_text(&rejected64.as_ref()
                .expect("flow-proven available binding"))
            );
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
                    println!("{}", String::from("inexact float64 caught"));
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
            let _ = inexact32.insert(340282366920938463463374607431768211455);
            let _ = rejected32
                .insert(
                    __terrane_raised_completion!(
                        terrane_int_support::exact_fixed_f32(* inexact32.as_ref()
                        .expect("flow-proven available binding")), 7 /* terrane-site: case.trn:34:26-34:35 */
                    ),
                );
            println!(
                "{}", terrane_scalar_support::scalar_text(&rejected32.as_ref()
                .expect("flow-proven available binding"))
            );
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
                    println!("{}", String::from("inexact float32 caught"));
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
