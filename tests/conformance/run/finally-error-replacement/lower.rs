// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: finally-error-replacement
fn error_then_return() -> Result<terrane_int_support::Int, TerraneError> {
    let mut __terrane_completion_0: TerraneCompletion<terrane_int_support::Int> = (|| {
        let __terrane_try_0: TerraneCompletion<terrane_int_support::Int> = (|| {
            return TerraneCompletion::Error(
                TerraneError::raised(
                    TerraneErrorKind::ArithmeticOverflow,
                    0 /* terrane-site: case.trn:5:5-5:30 */,
                ),
            );
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_0: TerraneCompletion<terrane_int_support::Int> = (|| {
        return TerraneCompletion::Return(terrane_int_support::Int::from(7_i128));
    })();
    match __terrane_finally_0 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_0 = replacement,
    }
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {
            __terrane_generated_defect("non-fallthrough try completed normally")
        }
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
fn return_then_error() -> Result<terrane_int_support::Int, TerraneError> {
    let mut __terrane_completion_1: TerraneCompletion<terrane_int_support::Int> = (|| {
        let __terrane_try_1: TerraneCompletion<terrane_int_support::Int> = (|| {
            return TerraneCompletion::Return(terrane_int_support::Int::from(8_i128));
        })();
        match __terrane_try_1 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_1) => {
                let mut __terrane_handled_1 = false;
                if !__terrane_handled_1 {
                    return TerraneCompletion::Error(__terrane_error_1);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_1: TerraneCompletion<terrane_int_support::Int> = (|| {
        return TerraneCompletion::Error(
            TerraneError::raised(
                TerraneErrorKind::CoercionError,
                1 /* terrane-site: case.trn:12:5-12:25 */,
            ),
        );
    })();
    match __terrane_finally_1 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_1 = replacement,
    }
    match __terrane_completion_1 {
        TerraneCompletion::Normal => {
            __terrane_generated_defect("non-fallthrough try completed normally")
        }
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
fn main() {
    let first: terrane_int_support::Int = __terrane_traced(
        error_then_return(),
        2 /* terrane-site: case.trn:14:15-14:33 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&first));
    let __terrane_completion_2: TerraneCompletion<()> = (|| {
        let __terrane_try_2: TerraneCompletion<()> = (|| {
            let second: terrane_int_support::Int = __terrane_traced_completion!(
                return_then_error(), 3 /* terrane-site: case.trn:17:18-17:36 */
            );
            println!("{}", terrane_scalar_support::scalar_text(&second));
            TerraneCompletion::Normal
        })();
        match __terrane_try_2 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_2) => {
                let mut __terrane_handled_2 = false;
                if !__terrane_handled_2
                    && __terrane_error_2.kind == TerraneErrorKind::CoercionError
                {
                    __terrane_handled_2 = true;
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("replaced"))
                    );
                }
                if !__terrane_handled_2 {
                    return TerraneCompletion::Error(__terrane_error_2);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_2 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
