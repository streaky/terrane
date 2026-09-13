// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: explicit-numeric-coercions
fn divide(numerator: f64, denominator: f64) -> f64 {
    return numerator / denominator;
}
fn main() {
    let quantity: i64 = 9007199254740993;
    let rounded64: f64 = __terrane_raised(
        terrane_int_support::coerce_to_f64(&quantity),
        0 /* terrane-site: case.trn:11:23-11:38 */,
    );
    let rounded32: f32 = __terrane_raised(
        terrane_int_support::coerce_to_f32(&quantity),
        1 /* terrane-site: case.trn:12:23-12:38 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&rounded64));
    println!("{}", terrane_scalar_support::scalar_text(&rounded32));
    let signed: i8 = -7;
    let signed_float: f32 = signed as f32;
    let maximum: u128 = 340282366920938463463374607431768211455;
    let maximum_float: f64 = maximum as f64;
    println!("{}", terrane_scalar_support::scalar_text(&signed_float));
    println!("{}", terrane_scalar_support::scalar_text(&maximum_float));
    let precise: f64 = 16777217.0;
    let narrowed: f32 = __terrane_raised(
        terrane_int_support::coerce_f64_to_f32(precise),
        2 /* terrane-site: case.trn:24:22-24:36 */,
    );
    let widened: f64 = narrowed as f64;
    println!("{}", terrane_scalar_support::scalar_text(&narrowed));
    println!("{}", terrane_scalar_support::scalar_text(&widened));
    let infinity64: f64 = divide(1.0, 0.0);
    let infinity32: f32 = __terrane_raised(
        terrane_int_support::coerce_f64_to_f32(infinity64),
        3 /* terrane-site: case.trn:30:24-30:41 */,
    );
    let nan64: f64 = divide(0.0, 0.0);
    let nan32: f32 = __terrane_raised(
        terrane_int_support::coerce_f64_to_f32(nan64),
        4 /* terrane-site: case.trn:32:19-32:31 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&infinity32.is_infinite()));
    println!("{}", terrane_scalar_support::scalar_text(&nan32.is_nan()));
    let checked_integer: Option<f32> = terrane_int_support::coerce_fixed_to_f32(maximum)
        .ok();
    println!("{}", terrane_scalar_support::scalar_text(&checked_integer.is_none()));
    let base: f64 = 65536.0;
    let too_large: f64 = base * base * base * base * base * base * base * base;
    let checked_float: Option<f32> = terrane_int_support::coerce_f64_to_f32(too_large)
        .ok();
    println!("{}", terrane_scalar_support::scalar_text(&checked_float.is_none()));
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let impossible: f32 = __terrane_raised_completion!(
                terrane_int_support::coerce_fixed_to_f32(maximum), 5 /* terrane-site: case.trn:45:26-45:40 */
            );
            println!("{}", terrane_scalar_support::scalar_text(&impossible));
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
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("integer overflow caught"))
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
            let impossible: f32 = __terrane_raised_completion!(
                terrane_int_support::coerce_f64_to_f32(too_large), 6 /* terrane-site: case.trn:51:26-51:42 */
            );
            println!("{}", terrane_scalar_support::scalar_text(&impossible));
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
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("float overflow caught"))
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
