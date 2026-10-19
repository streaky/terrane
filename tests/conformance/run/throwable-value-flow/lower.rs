// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: throwable-value-flow
fn describe(failure: TerraneError) -> String {
    return failure.message().to_owned().clone();
}
fn relay(failure: TerraneError) -> TerraneError {
    return failure;
}
fn main() {
    let mut caught: Option<TerraneError> = None;
    let mut again: Option<TerraneError> = None;
    let mut outer: Option<TerraneError> = None;
    let mut origin: Option<Option<TerraneError>> = None;
    let mut returned: Option<TerraneError> = None;
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let __terrane_completion_1: TerraneCompletion<()> = (|| {
                let __terrane_try_1: TerraneCompletion<()> = (|| {
                    return TerraneCompletion::Error(
                        TerraneError::raised(
                            TerraneErrorKind::CoercionError,
                            0 /* terrane-site: case.trn:13:7-13:27 */,
                        ),
                    );
                })();
                match __terrane_try_1 {
                    TerraneCompletion::Return(value) => {
                        return TerraneCompletion::Return(value);
                    }
                    TerraneCompletion::Break => return TerraneCompletion::Break,
                    TerraneCompletion::Continue => return TerraneCompletion::Continue,
                    TerraneCompletion::Normal => {}
                    TerraneCompletion::Error(__terrane_error_1) => {
                        let mut __terrane_handled_1 = false;
                        if !__terrane_handled_1
                            && __terrane_error_1.kind == TerraneErrorKind::CoercionError
                        {
                            __terrane_handled_1 = true;
                            let _ = caught.insert(__terrane_error_1.clone());
                            let _ = again
                                .insert(
                                    relay(
                                        caught
                                            .as_ref()
                                            .expect("flow-proven available binding")
                                            .clone(),
                                    ),
                                );
                            println!(
                                "{}", terrane_scalar_support::scalar_text(&describe(again
                                .as_ref().expect("flow-proven available binding").clone()))
                            );
                            return TerraneCompletion::Error(
                                TerraneError::raised(
                                        TerraneErrorKind::ArithmeticOverflow,
                                        1 /* terrane-site: case.trn:17:7-17:32 */,
                                    )
                                    .with_cause(__terrane_error_1.clone()),
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
                TerraneCompletion::Normal => {
                    __terrane_generated_defect("non-fallthrough try completed normally")
                }
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Error(error) => return TerraneCompletion::Error(error),
                TerraneCompletion::Break | TerraneCompletion::Continue => {
                    __terrane_generated_defect("loop control escaped a non-loop try")
                }
            }
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0
                    && __terrane_error_0.kind == TerraneErrorKind::ArithmeticOverflow
                {
                    __terrane_handled_0 = true;
                    let _ = outer.insert(__terrane_error_0.clone());
                    let _ = origin
                        .insert(
                            outer
                                .as_ref()
                                .expect("flow-proven available binding")
                                .clone()
                                .detail
                                .as_ref()
                                .and_then(|detail| detail.cause.as_deref())
                                .cloned(),
                        );
                    if origin.as_ref().expect("flow-proven available binding").is_some()
                    {
                        let _ = returned
                            .insert(
                                relay(
                                    match origin.take().expect("flow-proven availability") {
                                        Some(value) => value,
                                        _ => unreachable!("flow-proven storage refinement"),
                                    },
                                ),
                            );
                        println!(
                            "{}", terrane_scalar_support::scalar_text(&describe(returned
                            .as_ref().expect("flow-proven available binding").clone()))
                        );
                    }
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
}
