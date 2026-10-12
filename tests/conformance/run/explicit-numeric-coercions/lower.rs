// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: explicit-numeric-coercions
fn divide(numerator: f64, denominator: f64) -> f64 {
    return numerator / denominator;
}
fn main() {
    let quantity: i64;
    let rounded64: f64;
    let rounded32: f32;
    let signed: i8;
    let signed_float: f32;
    let maximum: u128;
    let maximum_float: f64;
    let precise: f64;
    let narrowed: f32;
    let widened: f64;
    let infinity64: f64;
    let infinity32: f32;
    let nan64: f64;
    let nan32: f32;
    let checked_integer: Option<f32>;
    let base: f64;
    let too_large: f64;
    let checked_float: Option<f32>;
    let mut impossible: Option<f32> = None;
    quantity = 9007199254740993;
    rounded64 = __terrane_raised(
        terrane_int_support::coerce_to_f64(&quantity),
        0 /* terrane-site: case.trn:10:23-10:38 */,
    );
    rounded32 = __terrane_raised(
        terrane_int_support::coerce_to_f32(&quantity),
        1 /* terrane-site: case.trn:11:23-11:38 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&rounded64));
    println!("{}", terrane_scalar_support::scalar_text(&rounded32));
    signed = -7;
    signed_float = signed as f32;
    maximum = 340282366920938463463374607431768211455;
    maximum_float = maximum as f64;
    println!("{}", terrane_scalar_support::scalar_text(&signed_float));
    println!("{}", terrane_scalar_support::scalar_text(&maximum_float));
    precise = 16777217.0;
    narrowed = __terrane_raised(
        terrane_int_support::coerce_f64_to_f32(precise),
        2 /* terrane-site: case.trn:23:22-23:36 */,
    );
    widened = narrowed as f64;
    println!("{}", terrane_scalar_support::scalar_text(&narrowed));
    println!("{}", terrane_scalar_support::scalar_text(&widened));
    infinity64 = divide(1.0, 0.0);
    infinity32 = __terrane_raised(
        terrane_int_support::coerce_f64_to_f32(infinity64),
        3 /* terrane-site: case.trn:29:24-29:41 */,
    );
    nan64 = divide(0.0, 0.0);
    nan32 = __terrane_raised(
        terrane_int_support::coerce_f64_to_f32(nan64),
        4 /* terrane-site: case.trn:31:19-31:31 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&infinity32.is_infinite()));
    println!("{}", terrane_scalar_support::scalar_text(&nan32.is_nan()));
    checked_integer = terrane_int_support::coerce_fixed_to_f32(maximum).ok();
    println!("{}", terrane_scalar_support::scalar_text(&checked_integer.is_none()));
    base = 65536.0;
    too_large = base * base * base * base * base * base * base * base;
    checked_float = terrane_int_support::coerce_f64_to_f32(too_large).ok();
    println!("{}", terrane_scalar_support::scalar_text(&checked_float.is_none()));
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let _ = impossible
                .insert(
                    __terrane_raised_completion!(
                        terrane_int_support::coerce_fixed_to_f32(maximum),
                        5 /* terrane-site: case.trn:44:26-44:40 */
                    ),
                );
            println!(
                "{}", terrane_scalar_support::scalar_text(&impossible.as_ref()
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
                    && __terrane_error_0.kind == TerraneErrorKind::CoercionError
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("integer overflow caught"));
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
            let _ = impossible
                .insert(
                    __terrane_raised_completion!(
                        terrane_int_support::coerce_f64_to_f32(too_large),
                        6 /* terrane-site: case.trn:50:26-50:42 */
                    ),
                );
            println!(
                "{}", terrane_scalar_support::scalar_text(&impossible.as_ref()
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
                    && __terrane_error_1.kind == TerraneErrorKind::CoercionError
                {
                    __terrane_handled_1 = true;
                    println!("{}", String::from("float overflow caught"));
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
