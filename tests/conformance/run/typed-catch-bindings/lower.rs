// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: typed-catch-bindings
fn main() {
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let __terrane_completion_1: TerraneCompletion<()> = (|| {
                let __terrane_try_1: TerraneCompletion<()> = (|| {
                    return TerraneCompletion::Error(
                        TerraneError::raised(
                            TerraneErrorKind::CoercionError,
                            0 /* terrane-site: case.trn:7:7-7:27 */,
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
                            let caught = __terrane_error_1.clone();
                            println!(
                                "{}", terrane_scalar_support::scalar_text(&caught.message()
                                .to_owned())
                            );
                            println!(
                                "{}", terrane_scalar_support::scalar_text(&caught.render()
                                .contains(&String::from("coercion-error: coercion has no compatible result")))
                            );
                            return TerraneCompletion::Error(
                                TerraneError::raised(
                                        TerraneErrorKind::ArithmeticOverflow,
                                        1 /* terrane-site: case.trn:11:7-11:32 */,
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
                    let replacement = __terrane_error_0.clone();
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&replacement.detail
                        .as_ref().and_then(| detail | detail.cause.as_deref()).cloned()
                        .is_none())
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
    let __terrane_completion_2: TerraneCompletion<()> = (|| {
        let __terrane_try_2: TerraneCompletion<()> = (|| {
            let __terrane_completion_3: TerraneCompletion<()> = (|| {
                let __terrane_try_3: TerraneCompletion<()> = (|| {
                    return TerraneCompletion::Error(
                        TerraneError::raised(
                            TerraneErrorKind::CoercionError,
                            2 /* terrane-site: case.trn:17:7-17:27 */,
                        ),
                    );
                })();
                match __terrane_try_3 {
                    TerraneCompletion::Return(value) => {
                        return TerraneCompletion::Return(value);
                    }
                    TerraneCompletion::Break => return TerraneCompletion::Break,
                    TerraneCompletion::Continue => return TerraneCompletion::Continue,
                    TerraneCompletion::Normal => {}
                    TerraneCompletion::Error(__terrane_error_3) => {
                        let mut __terrane_handled_3 = false;
                        if !__terrane_handled_3
                            && __terrane_error_3.kind == TerraneErrorKind::CoercionError
                        {
                            __terrane_handled_3 = true;
                            let caught = __terrane_error_3.clone();
                            return TerraneCompletion::Error(
                                caught
                                    .clone()
                                    .at(3 /* terrane-site: case.trn:19:7-19:19 */),
                            );
                        }
                        if !__terrane_handled_3 {
                            return TerraneCompletion::Error(__terrane_error_3);
                        }
                    }
                }
                TerraneCompletion::Normal
            })();
            match __terrane_completion_3 {
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
                    let reraised = __terrane_error_2.clone();
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&reraised.message()
                        .to_owned())
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
    let __terrane_completion_4: TerraneCompletion<()> = (|| {
        let __terrane_try_4: TerraneCompletion<()> = (|| {
            return TerraneCompletion::Error(
                TerraneError::raised(
                    TerraneErrorKind::ArithmeticOverflow,
                    4 /* terrane-site: case.trn:24:5-24:30 */,
                ),
            );
        })();
        match __terrane_try_4 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_4) => {
                let mut __terrane_handled_4 = false;
                if !__terrane_handled_4 {
                    __terrane_handled_4 = true;
                    let failure = __terrane_error_4.clone();
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&failure.message()
                        .to_owned())
                    );
                }
                if !__terrane_handled_4 {
                    return TerraneCompletion::Error(__terrane_error_4);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_4 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
