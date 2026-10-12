// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: finally-throwable-replacement
fn recovered() -> String {
    let mut __terrane_completion_0: TerraneCompletion<String> = (|| {
        let __terrane_try_0: TerraneCompletion<String> = (|| {
            return TerraneCompletion::Error(
                TerraneError::raised(
                    TerraneErrorKind::CoercionError,
                    0 /* terrane-site: case.trn:6:5-6:25 */,
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
    let __terrane_finally_0: TerraneCompletion<String> = (|| {
        return TerraneCompletion::Return(String::from("recovered"));
    })();
    match __terrane_finally_0 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_0 = replacement,
    }
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {
            __terrane_generated_defect("non-fallthrough try completed normally")
        }
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
fn replaced() -> Result<String, TerraneError> {
    let mut __terrane_completion_1: TerraneCompletion<String> = (|| {
        let __terrane_try_1: TerraneCompletion<String> = (|| {
            return TerraneCompletion::Error(
                TerraneError::raised(
                    TerraneErrorKind::CoercionError,
                    1 /* terrane-site: case.trn:12:5-12:25 */,
                ),
            );
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
    let __terrane_finally_1: TerraneCompletion<String> = (|| {
        return TerraneCompletion::Error(
            TerraneError::raised(
                TerraneErrorKind::ArithmeticOverflow,
                2 /* terrane-site: case.trn:14:5-14:30 */,
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
    let recovered_operation: std::sync::Arc<
        dyn Fn() -> Result<String, TerraneError> + Send + Sync,
    >;
    let replacement_operation: std::sync::Arc<
        dyn Fn() -> Result<String, TerraneError> + Send + Sync,
    >;
    recovered_operation = std::sync::Arc::new(move || Ok(recovered()));
    replacement_operation = std::sync::Arc::new(replaced);
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let _ = recovered_operation;
        "arithmetic-overflow".to_owned() })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let _ = recovered_operation; ""
        .to_owned() })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let _ = replacement_operation;
        "arithmetic-overflow".to_owned() })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let _ = replacement_operation;
        "arithmetic-overflow".to_owned() })
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_traced(recovered_operation(),
        3 /* terrane-site: case.trn:23:11-23:31 */))
    );
    let __terrane_completion_2: TerraneCompletion<()> = (|| {
        let __terrane_try_2: TerraneCompletion<()> = (|| {
            __terrane_traced_completion!(
                replacement_operation(), 4 /* terrane-site: case.trn:25:5-25:27 */
            );
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
                    && __terrane_error_2.kind == TerraneErrorKind::ArithmeticOverflow
                {
                    __terrane_handled_2 = true;
                    println!("{}", String::from("replaced"));
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
