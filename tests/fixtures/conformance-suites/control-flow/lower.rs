// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: tests/integration/catch-binding-function-scope/case.trn
// Namespace: catch-binding-function-scope
fn test_scenario_terrane_catch_binding_function_scope() {
    let mut failure: Option<TerraneError> = None;
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            return TerraneCompletion::Error(
                TerraneError::raised(
                    TerraneErrorKind::ArithmeticOverflow,

                        0 /* terrane-site: tests/integration/catch-binding-function-scope/case.trn:6:5-6:30 */,
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
                if !__terrane_handled_0
                    && __terrane_error_0.kind == TerraneErrorKind::ArithmeticOverflow
                {
                    __terrane_handled_0 = true;
                    let _ = failure.insert(__terrane_error_0.clone());
                    {
                        let __terrane_error_text_0 = failure
                            .as_ref()
                            .expect("flow-proven available binding")
                            .message();
                        println!("{}", __terrane_error_text_0);
                    };
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
    {
        let __terrane_error_text_1 = failure
            .as_ref()
            .expect("flow-proven available binding")
            .message();
        println!("{}", __terrane_error_text_1);
    };
}
// Source: tests/integration/catch-finally-lexical-scopes/case.trn
// Namespace: catch-finally-lexical-scopes
fn typed_catch() {
    let mut caught: Option<i64> = None;
    let mut completed: Option<i64> = None;
    let mut __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            return TerraneCompletion::Error(
                TerraneError::raised(
                    TerraneErrorKind::ArithmeticOverflow,

                        1 /* terrane-site: tests/integration/catch-finally-lexical-scopes/case.trn:6:5-6:30 */,
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
                if !__terrane_handled_0
                    && __terrane_error_0.kind == TerraneErrorKind::ArithmeticOverflow
                {
                    __terrane_handled_0 = true;
                    let _ = caught.insert(1);
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&caught.as_ref()
                        .expect("flow-proven available binding"))
                    );
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_0: TerraneCompletion<()> = (|| {
        let _ = completed.insert(2);
        println!(
            "{}", terrane_scalar_support::scalar_text(&completed.as_ref()
            .expect("flow-proven available binding"))
        );
        TerraneCompletion::Normal
    })();
    match __terrane_finally_0 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_0 = replacement,
    }
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
fn catch_all() {
    let mut caught: Option<i64> = None;
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            return TerraneCompletion::Error(
                TerraneError::raised(
                    TerraneErrorKind::CoercionError,

                        2 /* terrane-site: tests/integration/catch-finally-lexical-scopes/case.trn:16:5-16:25 */,
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
                    __terrane_handled_1 = true;
                    let _ = caught.insert(3);
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&caught.as_ref()
                        .expect("flow-proven available binding"))
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
fn test_scenario_terrane_catch_finally_lexical_scopes() {
    typed_catch();
    catch_all();
}
// Source: tests/integration/finally-control-flow/case.trn
// Namespace: finally-control-flow
fn early() -> terrane_int_support::Int {
    let mut __terrane_completion_0: TerraneCompletion<terrane_int_support::Int> = (|| {
        let __terrane_try_0: TerraneCompletion<terrane_int_support::Int> = (|| {
            return TerraneCompletion::Return(terrane_int_support::Int::from(7_i128));
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
        println!("{}", String::from("return finally"));
        TerraneCompletion::Normal
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
fn caught() -> terrane_int_support::Int {
    let mut __terrane_completion_1: TerraneCompletion<terrane_int_support::Int> = (|| {
        let __terrane_try_1: TerraneCompletion<terrane_int_support::Int> = (|| {
            return TerraneCompletion::Error(
                TerraneError::raised(
                    TerraneErrorKind::ArithmeticOverflow,

                        3 /* terrane-site: tests/integration/finally-control-flow/case.trn:10:5-10:30 */,
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
                if !__terrane_handled_1
                    && __terrane_error_1.kind == TerraneErrorKind::ArithmeticOverflow
                {
                    __terrane_handled_1 = true;
                    return TerraneCompletion::Return(
                        terrane_int_support::Int::from(9_i128),
                    );
                }
                if !__terrane_handled_1 {
                    return TerraneCompletion::Error(__terrane_error_1);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_1: TerraneCompletion<terrane_int_support::Int> = (|| {
        println!("{}", String::from("catch finally"));
        TerraneCompletion::Normal
    })();
    match __terrane_finally_1 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_1 = replacement,
    }
    match __terrane_completion_1 {
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
fn test_scenario_terrane_finally_control_flow() {
    let value: terrane_int_support::Int;
    let caught_value: terrane_int_support::Int;
    let mut counter: terrane_int_support::Int;
    value = early();
    println!("{}", terrane_scalar_support::scalar_text(&value));
    caught_value = caught();
    println!("{}", terrane_scalar_support::scalar_text(&caught_value));
    counter = terrane_int_support::Int::from(0_i128);
    while counter < terrane_int_support::Int::from(3_i128) {
        counter = counter.clone() + terrane_int_support::Int::from(1_i128);
        let mut __terrane_completion_2: TerraneCompletion<()> = (|| {
            let __terrane_try_2: TerraneCompletion<()> = (|| {
                if counter == terrane_int_support::Int::from(1_i128) {
                    return TerraneCompletion::Continue;
                }
                if counter == terrane_int_support::Int::from(2_i128) {
                    return TerraneCompletion::Break;
                }
                TerraneCompletion::Normal
            })();
            match __terrane_try_2 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_2) => {
                    let mut __terrane_handled_2 = false;
                    if !__terrane_handled_2 {
                        return TerraneCompletion::Error(__terrane_error_2);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        let __terrane_finally_2: TerraneCompletion<()> = (|| {
            println!("{}", String::from("loop finally"));
            TerraneCompletion::Normal
        })();
        match __terrane_finally_2 {
            TerraneCompletion::Normal => {}
            replacement => __terrane_completion_2 = replacement,
        }
        match __terrane_completion_2 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break => break,
            TerraneCompletion::Continue => continue,
        }
    }
    println!("{}", String::from("done"));
}
// Source: tests/integration/finally-error-replacement/case.trn
// Namespace: finally-error-replacement
fn error_then_return() -> terrane_int_support::Int {
    let mut __terrane_completion_0: TerraneCompletion<terrane_int_support::Int> = (|| {
        let __terrane_try_0: TerraneCompletion<terrane_int_support::Int> = (|| {
            return TerraneCompletion::Error(
                TerraneError::raised(
                    TerraneErrorKind::ArithmeticOverflow,

                        4 /* terrane-site: tests/integration/finally-error-replacement/case.trn:5:5-5:30 */,
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
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
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

                    5 /* terrane-site: tests/integration/finally-error-replacement/case.trn:12:5-12:25 */,
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
fn test_scenario_terrane_finally_error_replacement() {
    let first: terrane_int_support::Int;
    let mut second: Option<terrane_int_support::Int> = None;
    first = error_then_return();
    println!("{}", terrane_scalar_support::scalar_text(&first));
    let __terrane_completion_2: TerraneCompletion<()> = (|| {
        let __terrane_try_2: TerraneCompletion<()> = (|| {
            let _ = second
                .insert(
                    __terrane_traced_completion!(
                        return_then_error(), 6 /* terrane-site: tests/integration/finally-error-replacement/case.trn:17:18-17:36 */
                    ),
                );
            println!(
                "{}", terrane_scalar_support::scalar_text(&second.as_ref()
                .expect("flow-proven available binding"))
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
                    && __terrane_error_2.kind == TerraneErrorKind::CoercionError
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
// Source: tests/integration/finally-throwable-replacement/case.trn
// Namespace: finally-throwable-replacement
fn recovered() -> String {
    let mut __terrane_completion_0: TerraneCompletion<String> = (|| {
        let __terrane_try_0: TerraneCompletion<String> = (|| {
            return TerraneCompletion::Error(
                TerraneError::raised(
                    TerraneErrorKind::CoercionError,

                        7 /* terrane-site: tests/integration/finally-throwable-replacement/case.trn:6:5-6:25 */,
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

                        8 /* terrane-site: tests/integration/finally-throwable-replacement/case.trn:12:5-12:25 */,
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

                    9 /* terrane-site: tests/integration/finally-throwable-replacement/case.trn:14:5-14:30 */,
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
fn test_scenario_terrane_finally_throwable_replacement() {
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
        10 /* terrane-site: tests/integration/finally-throwable-replacement/case.trn:23:11-23:31 */))
    );
    let __terrane_completion_2: TerraneCompletion<()> = (|| {
        let __terrane_try_2: TerraneCompletion<()> = (|| {
            __terrane_traced_completion!(
                replacement_operation(), 11 /* terrane-site: tests/integration/finally-throwable-replacement/case.trn:25:5-25:27 */
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
// Source: tests/integration/fizz-buzz/case.trn
// Namespace: fizz-buzz
fn test_scenario_terrane_fizz_buzz() -> Result<(), TerraneError> {
    let mut number: terrane_int_support::Int;
    number = terrane_int_support::Int::from(1_i128);
    while number <= terrane_int_support::Int::from(15_i128) {
        if __terrane_raised_err(
            number.clone().modulo(&terrane_int_support::Int::from(15_i128)),
            12 /* terrane-site: tests/integration/fizz-buzz/case.trn:6:8-6:19 */,
        )? == terrane_int_support::Int::from(0_i128)
        {
            println!("{}", String::from("FizzBuzz"));
        } else if __terrane_raised_err(
            number.clone().modulo(&terrane_int_support::Int::from(3_i128)),
            13 /* terrane-site: tests/integration/fizz-buzz/case.trn:8:13-8:23 */,
        )? == terrane_int_support::Int::from(0_i128)
        {
            println!("{}", String::from("Fizz"));
        } else if __terrane_raised_err(
            number.clone().modulo(&terrane_int_support::Int::from(5_i128)),
            14 /* terrane-site: tests/integration/fizz-buzz/case.trn:10:13-10:23 */,
        )? == terrane_int_support::Int::from(0_i128)
        {
            println!("{}", String::from("Buzz"));
        } else {
            println!("{}", terrane_scalar_support::scalar_text(&number));
        }
        number = number.clone() + terrane_int_support::Int::from(1_i128);
    }
    Ok(())
}
// Source: tests/integration/for-loops/case.trn
// Namespace: for-loops
fn show(mut value: String) {
    println!("{}", terrane_scalar_support::scalar_text(&value));
    value = String::from("parameter");
    println!("{}", terrane_scalar_support::scalar_text(&value));
}
fn test_scenario_terrane_for_loops() {
    let text: String;
    let mut character: String;
    let mut index: terrane_int_support::Int;
    let mut skipped: terrane_int_support::Int;
    show(String::from("original"));
    text = String::from("e\u{301}x");
    {
        let __terrane_iterable_0 = &text;
        let mut __terrane_iterator_0 = terrane_string_support::graphemes(
            &__terrane_iterable_0,
        );
        loop {
            character = match __terrane_iterator_0.next() {
                Some(item) => item,
                None => break,
            };
            println!("{}", terrane_scalar_support::scalar_text(&character));
            character = String::from("loop");
            println!("{}", terrane_scalar_support::scalar_text(&character));
        }
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&(terrane_string_support::length(&text)
        as i128))
    );
    index = terrane_int_support::Int::from(0_i128);
    '__terrane_break_1: while index < terrane_int_support::Int::from(3_i128) {
        '__terrane_continue_1: {
            if index == terrane_int_support::Int::from(1_i128) {
                break '__terrane_continue_1;
            }
            println!("{}", terrane_scalar_support::scalar_text(&index));
        }
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    println!("{}", terrane_scalar_support::scalar_text(&index));
    skipped = terrane_int_support::Int::from(7_i128);
    '__terrane_break_2: while skipped < terrane_int_support::Int::from(7_i128) {
        '__terrane_continue_2: {
            println!("{}", String::from("unreachable"));
        }
        skipped = skipped.clone() + terrane_int_support::Int::from(1_i128);
    }
    println!("{}", terrane_scalar_support::scalar_text(&skipped));
}
// Source: tests/integration/function-scope-assignment-destination/case.trn
// Namespace: function-scope-assignment-destination
fn optional_result(choose: bool) -> Option<String> {
    let mut result: Option<String>;
    let mut count: terrane_int_support::Int;
    result = None;
    count = terrane_int_support::Int::from(0_i128);
    while count < terrane_int_support::Int::from(2_i128) {
        if choose {
            result = Some(String::from("found"));
        }
        count = count.clone() + terrane_int_support::Int::from(1_i128);
    }
    return result;
}
fn fixed_width_result(choose: bool) -> u8 {
    let mut result: u8;
    result = 1;
    if choose {
        result = 2;
    }
    return result;
}
fn create_value(fail: bool) -> Result<String, TerraneError> {
    if fail {
        return Err(
            TerraneError::raised(
                TerraneErrorKind::CoercionError,

                    15 /* terrane-site: tests/integration/function-scope-assignment-destination/case.trn:18:9-18:39 */,
            ),
        );
    }
    return Ok(String::from("created"));
}
fn fallible_initialization(fail: bool) {
    let mut value: Option<String> = None;
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let _ = value
                .insert(
                    __terrane_traced_completion!(
                        create_value(fail), 16 /* terrane-site: tests/integration/function-scope-assignment-destination/case.trn:22:17-22:35 */
                    ),
                );
            println!(
                "{}", terrane_scalar_support::scalar_text(&value.as_ref()
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
                    println!("{}", String::from("failed"));
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
    return ();
}
fn test_scenario_terrane_function_scope_assignment_destination() {
    let absent: Option<String>;
    let present: Option<String>;
    absent = optional_result(false);
    if (&absent).is_none() {
        println!("{}", String::from("absent"));
    }
    present = optional_result(true);
    if (&present).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &present { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    println!("{}", terrane_scalar_support::scalar_text(&fixed_width_result(true)));
    fallible_initialization(false);
    fallible_initialization(true);
}
// Source: tests/integration/function-scope-boundaries/case.trn
// Namespace: function-scope-boundaries
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF9S183<'a> {
    Arm0(&'a terrane_int_support::Int),
    Arm1(&'a String),
}
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF9S424 {
    Arm0(terrane_int_support::Int),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF9S424 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF9S442 {
    Arm0(terrane_int_support::Int),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF9S442 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF9S897 {
    Arm0(bool),
    Arm1(terrane_int_support::Int),
    Arm2(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF9S897 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm2(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF9S1273 {
    Arm0(bool),
    Arm1(terrane_int_support::Int),
    Arm2(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF9S1273 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm2(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
#[derive(Clone)]
pub enum Choice {
    Number(terrane_int_support::Int),
    Label(String),
}
fn borrow_match(item: Choice) -> terrane_int_support::Int {
    let value: TerraneUnionF9S183;
    let value_2: &String;
    let result: terrane_int_support::Int;
    match &item {
        Choice::Number(__terrane_pattern_162_value) => {
            value = TerraneUnionF9S183::Arm0(__terrane_pattern_162_value);
            result = (*match &value {
                TerraneUnionF9S183::Arm0(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            })
                .clone();
        }
        Choice::Label(__terrane_pattern_238_value) => {
            value_2 = __terrane_pattern_238_value;
            println!("{}", terrane_scalar_support::scalar_text(&* value_2));
            result = terrane_int_support::Int::from(
                terrane_string_support::length(&value_2.clone()) as i128,
            );
        }
    }
    return result.clone();
}
fn move_match(item: Choice) {
    let value: TerraneUnionF9S424;
    let value_2: String;
    let result: TerraneUnionF9S442;
    let __terrane_match_value_379 = item;
    match __terrane_match_value_379 {
        Choice::Number(__terrane_pattern_403_value) => {
            value = TerraneUnionF9S424::Arm0(__terrane_pattern_403_value);
            result = TerraneUnionF9S442::Arm0(
                match value {
                    TerraneUnionF9S424::Arm0(value) => value,
                    _ => unreachable!("flow-proven storage refinement"),
                },
            );
        }
        Choice::Label(__terrane_pattern_469_value) => {
            value_2 = __terrane_pattern_469_value;
            result = TerraneUnionF9S442::Arm1(value_2);
            let _ = &result;
        }
    }
    println!(
        "{}", { let mut __terrane_text = String::new(); match &result {
        TerraneUnionF9S442::Arm0(__terrane_arm) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); }, TerraneUnionF9S442::Arm1(__terrane_arm) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); } } __terrane_text }
    );
}
fn closure_boundary() -> terrane_int_support::Int {
    let outer: i64;
    let hidden: i64;
    let read_outer: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    let nested: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    outer = 40;
    hidden = 9;
    read_outer = {
        let outer = outer.clone();
        std::sync::Arc::new(move || -> terrane_int_support::Int {
            return terrane_int_support::Int::from(outer as i128);
        })
    };
    nested = {
        std::sync::Arc::new(move || -> terrane_int_support::Int {
            let hidden: i64 = 7;
            let _ = &hidden;
            return terrane_int_support::Int::from(hidden as i128);
        })
    };
    println!("{}", terrane_scalar_support::scalar_text(&read_outer()));
    nested();
    println!("{}", terrane_scalar_support::scalar_text(&hidden));
    return terrane_int_support::Int::from(hidden as i128);
}
fn finite_unions(choose_first: bool, choose_second: bool) {
    let mut value: TerraneUnionF9S897;
    if choose_first {
        value = TerraneUnionF9S897::Arm1(terrane_int_support::Int::from(1_i128));
    } else {
        value = TerraneUnionF9S897::Arm2(String::from("two"));
        let _ = &value;
    }
    if choose_second {
        if matches!(&&value, TerraneUnionF9S897::Arm1(_)) {
            value = TerraneUnionF9S897::Arm0(true);
            let _ = &value;
        } else {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&(terrane_string_support::length(&match
                &value { TerraneUnionF9S897::Arm2(value) => value, _ =>
                unreachable!("flow-proven storage refinement") }) as i128))
            );
        }
    } else {
        value = TerraneUnionF9S897::Arm1(terrane_int_support::Int::from(3_i128));
        let _ = &value;
    }
    if matches!(&&value, TerraneUnionF9S897::Arm1(_)) {
        println!(
            "{}", terrane_scalar_support::scalar_text(&(match value {
            TerraneUnionF9S897::Arm1(value) => value, _ =>
            unreachable!("flow-proven storage refinement") } +
            terrane_int_support::Int::from(4_i128)))
        );
    } else if matches!(&&value, TerraneUnionF9S897::Arm2(_)) {
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&(terrane_string_support::length(&match
            &value { TerraneUnionF9S897::Arm2(value) => value, _ =>
            unreachable!("flow-proven storage refinement") }) as i128))
        );
    } else {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &value {
            TerraneUnionF9S897::Arm0(value) => value, _ =>
            unreachable!("flow-proven storage refinement") })
        );
    }
}
fn loop_union(mut count: terrane_int_support::Int) {
    let mut value: TerraneUnionF9S1273;
    value = TerraneUnionF9S1273::Arm1(terrane_int_support::Int::from(10_i128));
    while count < terrane_int_support::Int::from(3_i128) {
        if count == terrane_int_support::Int::from(0_i128) {
            value = TerraneUnionF9S1273::Arm2(String::from("loop"));
            let _ = &value;
        } else {
            value = TerraneUnionF9S1273::Arm0(true);
            let _ = &value;
        }
        count = count.clone() + terrane_int_support::Int::from(1_i128);
        if count == terrane_int_support::Int::from(2_i128) {
            break;
        }
    }
    if matches!(&&value, TerraneUnionF9S1273::Arm1(_)) {
        println!(
            "{}", terrane_scalar_support::scalar_text(&(match value {
            TerraneUnionF9S1273::Arm1(value) => value, _ =>
            unreachable!("flow-proven storage refinement") } +
            terrane_int_support::Int::from(1_i128)))
        );
    } else if matches!(&&value, TerraneUnionF9S1273::Arm2(_)) {
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&(terrane_string_support::length(&match
            &value { TerraneUnionF9S1273::Arm2(value) => value, _ =>
            unreachable!("flow-proven storage refinement") }) as i128))
        );
    } else {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &value {
            TerraneUnionF9S1273::Arm0(value) => value, _ =>
            unreachable!("flow-proven storage refinement") })
        );
    }
}
fn test_scenario_terrane_function_scope_boundaries() {
    let number: Choice;
    let other_number: Choice;
    let other_label: Choice;
    let movable_label: Choice;
    number = {
        let __terrane_enum_payload_0 = terrane_int_support::Int::from(5_i128);
        Choice::Number(__terrane_enum_payload_0)
    };
    println!("{}", terrane_scalar_support::scalar_text(&borrow_match(number.clone())));
    other_number = {
        let __terrane_enum_payload_0 = terrane_int_support::Int::from(6_i128);
        Choice::Number(__terrane_enum_payload_0)
    };
    move_match(other_number);
    other_label = {
        let __terrane_enum_payload_0 = String::from("borrowed");
        Choice::Label(__terrane_enum_payload_0)
    };
    println!("{}", terrane_scalar_support::scalar_text(&borrow_match(other_label)));
    movable_label = {
        let __terrane_enum_payload_0 = String::from("moved");
        Choice::Label(__terrane_enum_payload_0)
    };
    move_match(movable_label);
    println!("{}", terrane_scalar_support::scalar_text(&closure_boundary()));
    finite_unions(true, true);
    finite_unions(true, false);
    finite_unions(false, true);
    finite_unions(false, false);
    loop_union(terrane_int_support::Int::from(0_i128));
    loop_union(terrane_int_support::Int::from(1_i128));
}
// Source: tests/integration/function-scope-catch-value/case.trn
// Namespace: function-scope-catch-value
fn caught_value(choose: bool) -> String {
    let mut caught: Option<TerraneError> = None;
    let __terrane_completion_0: TerraneCompletion<String> = (|| {
        let __terrane_try_0: TerraneCompletion<String> = (|| {
            if choose {
                return TerraneCompletion::Error(
                    TerraneError::raised(
                        TerraneErrorKind::CoercionError,

                            17 /* terrane-site: tests/integration/function-scope-catch-value/case.trn:6:13-6:33 */,
                    ),
                );
            }
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
                    let _ = caught.insert(__terrane_error_0.clone());
                    println!("{}", String::from("caught"));
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
    return caught
        .as_ref()
        .unwrap_or_else(|| __terrane_uninitialized_binding("caught", "case.trn", 9, 12))
        .message()
        .to_owned();
}
fn test_scenario_terrane_function_scope_catch_value() {
    println!("{}", terrane_scalar_support::scalar_text(&caught_value(true)));
}
// Source: tests/integration/function-scope-control-equivalence/case.trn
// Namespace: function-scope-control-equivalence
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF11S441 {
    Arm0(terrane_int_support::Int),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF11S441 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF11S539 {
    Arm0(terrane_int_support::Int),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF11S539 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF11S1017 {
    Arm0(terrane_int_support::Int),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF11S1017 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF11S1289 {
    Arm0(terrane_int_support::Int),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF11S1289 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF11S1487 {
    Arm0(bool),
    Arm1(terrane_int_support::Int),
    Arm2(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF11S1487 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm2(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF11S1725 {
    Arm0(terrane_int_support::Int),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF11S1725 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
fn straight_replacement() {
    let x: i64;
    let x_2: String;
    x = 3;
    let _ = &x;
    x_2 = String::from("cat");
    println!("{}", terrane_scalar_support::scalar_text(&x_2));
}
fn straight_int() {
    let x: i64;
    x = 3;
    println!("{}", terrane_scalar_support::scalar_text(&x));
}
fn straight_string() {
    let x: String;
    x = String::from("cat");
    println!("{}", terrane_scalar_support::scalar_text(&x));
}
fn branched_replacement(choose: bool) {
    let x: i64;
    let x_2: String;
    if choose {
        x = 3;
        let _ = &x;
        x_2 = String::from("cat");
    } else {
        x_2 = String::from("cat");
        let _ = &x_2;
    }
    println!("{}", terrane_scalar_support::scalar_text(&x_2));
}
fn branched_single(choose: bool) {
    let x: TerraneUnionF11S441;
    if choose {
        x = TerraneUnionF11S441::Arm0(terrane_int_support::Int::from(3_i128));
    } else {
        x = TerraneUnionF11S441::Arm1(String::from("cat"));
        let _ = &x;
    }
    println!(
        "{}", { let mut __terrane_text = String::new(); match &x {
        TerraneUnionF11S441::Arm0(__terrane_arm) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); }, TerraneUnionF11S441::Arm1(__terrane_arm) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); } } __terrane_text }
    );
}
fn loop_replacement(mut count: terrane_int_support::Int) {
    let mut x: TerraneUnionF11S539;
    x = TerraneUnionF11S539::Arm0(terrane_int_support::Int::from(3_i128));
    while count < terrane_int_support::Int::from(1_i128) {
        x = TerraneUnionF11S539::Arm1(String::from("cat"));
        let _ = &x;
        count = count.clone() + terrane_int_support::Int::from(1_i128);
    }
    println!(
        "{}", { let mut __terrane_text = String::new(); match &x {
        TerraneUnionF11S539::Arm0(__terrane_arm) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); }, TerraneUnionF11S539::Arm1(__terrane_arm) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); } } __terrane_text }
    );
}
fn loop_int(mut count: terrane_int_support::Int) {
    let mut x: i64;
    x = 3;
    while count < terrane_int_support::Int::from(1_i128) {
        x = 3;
        let _ = &x;
        count = count.clone() + terrane_int_support::Int::from(1_i128);
    }
    println!("{}", terrane_scalar_support::scalar_text(&x));
}
fn loop_string(mut count: terrane_int_support::Int) {
    let mut x: String;
    x = String::from("cat");
    while count < terrane_int_support::Int::from(1_i128) {
        x = String::from("cat");
        let _ = &x;
        count = count.clone() + terrane_int_support::Int::from(1_i128);
    }
    println!("{}", terrane_scalar_support::scalar_text(&x));
}
fn both_branches(choose: bool) -> terrane_int_support::Int {
    let x: terrane_int_support::Int;
    if choose {
        x = terrane_int_support::Int::from(3_i128);
        let _ = &x;
    } else {
        x = terrane_int_support::Int::from(4_i128);
        let _ = &x;
    }
    return x.clone();
}
fn for_join(text: String) {
    let mut last: TerraneUnionF11S1017;
    last = TerraneUnionF11S1017::Arm0(terrane_int_support::Int::from(10_i128));
    {
        let __terrane_iterable_0 = &text;
        let mut __terrane_iterator_0 = terrane_string_support::graphemes(
            &__terrane_iterable_0,
        );
        loop {
            last = TerraneUnionF11S1017::Arm1(
                match __terrane_iterator_0.next() {
                    Some(item) => item,
                    None => break,
                },
            );
            let _ = &last;
            println!(
                "{}", terrane_scalar_support::scalar_text(&match &last {
                TerraneUnionF11S1017::Arm1(value) => value, _ =>
                unreachable!("flow-proven storage refinement") })
            );
        }
    }
    println!(
        "{}", { let mut __terrane_text = String::new(); match &last {
        TerraneUnionF11S1017::Arm0(__terrane_arm) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); }, TerraneUnionF11S1017::Arm1(__terrane_arm) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); } } __terrane_text }
    );
}
fn control_declared_function(choose: bool) -> terrane_int_support::Int {
    fn helper_terrane_function_scope_control_equivalence() -> terrane_int_support::Int {
        return terrane_int_support::Int::from(7_i128);
    }
    if choose {}
    return helper_terrane_function_scope_control_equivalence();
}
fn branch_type_use(choose: bool) -> terrane_int_support::Int {
    let x: TerraneUnionF11S1289;
    let result: terrane_int_support::Int;
    if choose {
        x = TerraneUnionF11S1289::Arm0(terrane_int_support::Int::from(3_i128));
        result = match x {
            TerraneUnionF11S1289::Arm0(value) => value,
            _ => unreachable!("flow-proven storage refinement"),
        } + terrane_int_support::Int::from(1_i128);
    } else {
        x = TerraneUnionF11S1289::Arm1(String::from("cat"));
        let _ = &x;
        result = terrane_int_support::Int::from(
            terrane_string_support::length(
                &match &x {
                    TerraneUnionF11S1289::Arm1(value) => value,
                    _ => unreachable!("flow-proven storage refinement"),
                },
            ) as i128,
        );
    }
    return result.clone();
}
fn nested_join(outer: bool, inner: bool) {
    let x: TerraneUnionF11S1487;
    if outer {
        if inner {
            x = TerraneUnionF11S1487::Arm1(terrane_int_support::Int::from(1_i128));
        } else {
            x = TerraneUnionF11S1487::Arm2(String::from("two"));
            let _ = &x;
        }
    } else {
        x = TerraneUnionF11S1487::Arm0(false);
        let _ = &x;
    }
    println!(
        "{}", { let mut __terrane_text = String::new(); match &x {
        TerraneUnionF11S1487::Arm0(__terrane_arm) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); }, TerraneUnionF11S1487::Arm1(__terrane_arm) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); }, TerraneUnionF11S1487::Arm2(__terrane_arm) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); } } __terrane_text }
    );
}
fn returned_branch(choose: bool) -> terrane_int_support::Int {
    let x: i64;
    if choose {
        return terrane_int_support::Int::from(9_i128);
    } else {
        x = 3;
    }
    return terrane_int_support::Int::from(x as i128);
}
fn loop_backedge() {
    let mut x: TerraneUnionF11S1725;
    let mut count: terrane_int_support::Int;
    x = TerraneUnionF11S1725::Arm0(terrane_int_support::Int::from(3_i128));
    count = terrane_int_support::Int::from(0_i128);
    while count < terrane_int_support::Int::from(2_i128) {
        println!(
            "{}", { let mut __terrane_text = String::new(); match &x {
            TerraneUnionF11S1725::Arm0(__terrane_arm) => {
            terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
            __terrane_text); }, TerraneUnionF11S1725::Arm1(__terrane_arm) => {
            terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
            __terrane_text); } } __terrane_text }
        );
        x = TerraneUnionF11S1725::Arm1(String::from("cat"));
        let _ = &x;
        count = count.clone() + terrane_int_support::Int::from(1_i128);
    }
    println!(
        "{}", { let mut __terrane_text = String::new(); match &x {
        TerraneUnionF11S1725::Arm0(__terrane_arm) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); }, TerraneUnionF11S1725::Arm1(__terrane_arm) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); } } __terrane_text }
    );
}
fn test_scenario_terrane_function_scope_control_equivalence() {
    let mut last: String;
    straight_replacement();
    straight_int();
    straight_string();
    branched_replacement(true);
    branched_replacement(false);
    branched_single(true);
    branched_single(false);
    loop_replacement(terrane_int_support::Int::from(0_i128));
    loop_replacement(terrane_int_support::Int::from(1_i128));
    loop_int(terrane_int_support::Int::from(0_i128));
    loop_int(terrane_int_support::Int::from(1_i128));
    loop_string(terrane_int_support::Int::from(0_i128));
    loop_string(terrane_int_support::Int::from(1_i128));
    println!("{}", terrane_scalar_support::scalar_text(&both_branches(true)));
    println!("{}", terrane_scalar_support::scalar_text(&both_branches(false)));
    last = String::from("before");
    {
        let __terrane_iterable_1 = &String::from("ab");
        let mut __terrane_iterator_1 = terrane_string_support::graphemes(
            &__terrane_iterable_1,
        );
        loop {
            last = match __terrane_iterator_1.next() {
                Some(item) => item,
                None => break,
            };
            let _ = &last;
            println!("{}", terrane_scalar_support::scalar_text(&last));
        }
    }
    println!("{}", terrane_scalar_support::scalar_text(&last));
    for_join(String::from(""));
    for_join(String::from("ab"));
    println!(
        "{}", terrane_scalar_support::scalar_text(&control_declared_function(false))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&control_declared_function(true))
    );
    println!("{}", terrane_scalar_support::scalar_text(&branch_type_use(true)));
    println!("{}", terrane_scalar_support::scalar_text(&branch_type_use(false)));
    nested_join(true, true);
    nested_join(true, false);
    nested_join(false, false);
    println!("{}", terrane_scalar_support::scalar_text(&returned_branch(true)));
    println!("{}", terrane_scalar_support::scalar_text(&returned_branch(false)));
    loop_backedge();
}
// Source: tests/integration/function-scope-partial-inferred/case.trn
// Namespace: function-scope-partial-inferred
fn value_terrane_function_scope_partial_inferred(
    choose: bool,
) -> terrane_int_support::Int {
    let mut x: Option<i64> = None;
    if choose {
        let _ = x.insert(3);
    }
    return terrane_int_support::Int::from(
        x
            .as_ref()
            .unwrap_or_else(|| __terrane_uninitialized_binding("x", "case.trn", 5, 12))
            .clone() as i128,
    );
}
fn test_scenario_terrane_function_scope_partial_inferred() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&value_terrane_function_scope_partial_inferred(true))
    );
}
// Source: tests/integration/function-scope-partial-typed/case.trn
// Namespace: function-scope-partial-typed
fn value_terrane_function_scope_partial_typed(choose: bool) -> terrane_int_support::Int {
    let mut x: Option<i64> = None;
    if choose {
        let _ = x.insert(3);
    }
    return terrane_int_support::Int::from(
        x
            .as_ref()
            .unwrap_or_else(|| __terrane_uninitialized_binding("x", "case.trn", 5, 12))
            .clone() as i128,
    );
}
fn test_scenario_terrane_function_scope_partial_typed() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&value_terrane_function_scope_partial_typed(true))
    );
}
// Source: tests/integration/function-scope-release/case.trn
// Namespace: function-scope-release
#[derive(Clone)]
pub struct TerraneNsFunctionScopeReleaseMarkerIdentity2F66756E6374696F6E2D73636F70652D72656C65617365006D61726B6572 {
    __terrane_lifetime: std::sync::Arc<()>,
    pub name: String,
}
impl TerraneNsFunctionScopeReleaseMarkerIdentity2F66756E6374696F6E2D73636F70652D72656C65617365006D61726B6572 {
    pub fn terrane_construct(name: String) -> Self {
        let mut __terrane_constructed_value = Self {
            name: String::new(),
            __terrane_lifetime: std::sync::Arc::new(()),
        };
        __terrane_constructed_value.construct(name);
        __terrane_constructed_value
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn construct(&mut self, name: String) {
        {
            let __terrane_value: String = name;
            self.name = __terrane_value;
        }
    }
    pub fn destruct(&mut self) {
        println!(
            "{}{}", String::from("released "), terrane_scalar_support::scalar_text(&self
            .name)
        );
    }
}
impl Drop
for TerraneNsFunctionScopeReleaseMarkerIdentity2F66756E6374696F6E2D73636F70652D72656C65617365006D61726B6572 {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) != 1 {
            return;
        }
        self.destruct();
    }
}
fn test_scenario_terrane_function_scope_release() {
    let mut value: TerraneNsFunctionScopeReleaseMarkerIdentity2F66756E6374696F6E2D73636F70652D72656C65617365006D61726B6572;
    let mut count: terrane_int_support::Int;
    let value_2: String;
    let retained: TerraneNsFunctionScopeReleaseMarkerIdentity2F66756E6374696F6E2D73636F70652D72656C65617365006D61726B6572;
    value = TerraneNsFunctionScopeReleaseMarkerIdentity2F66756E6374696F6E2D73636F70652D72656C65617365006D61726B6572::terrane_construct(
        String::from("first"),
    );
    println!("{}", terrane_scalar_support::scalar_text(&value.name));
    println!("{}", String::from("after-if"));
    count = terrane_int_support::Int::from(0_i128);
    while count < terrane_int_support::Int::from(2_i128) {
        value = TerraneNsFunctionScopeReleaseMarkerIdentity2F66756E6374696F6E2D73636F70652D72656C65617365006D61726B6572::terrane_construct(
            String::from("loop"),
        );
        let _ = &value;
        println!("{}", terrane_scalar_support::scalar_text(&value.name));
        count = count.clone() + terrane_int_support::Int::from(1_i128);
    }
    println!("{}", String::from("after-loop"));
    let __terrane_replacement = String::from("done");
    drop(value);
    value_2 = __terrane_replacement;
    println!("{}", terrane_scalar_support::scalar_text(&value_2));
    retained = TerraneNsFunctionScopeReleaseMarkerIdentity2F66756E6374696F6E2D73636F70652D72656C65617365006D61726B6572::terrane_construct(
        String::from("retained"),
    );
    println!("{}", terrane_scalar_support::scalar_text(&retained.name));
    println!("{}", String::from("function-end"));
}
// Source: tests/integration/function-scope-zero-loop/case.trn
// Namespace: function-scope-zero-loop
fn value_terrane_function_scope_zero_loop(mut choose: bool) -> terrane_int_support::Int {
    let mut x: Option<i64> = None;
    while choose {
        let _ = x.insert(3);
        choose = false;
    }
    return terrane_int_support::Int::from(
        x
            .as_ref()
            .unwrap_or_else(|| __terrane_uninitialized_binding("x", "case.trn", 6, 12))
            .clone() as i128,
    );
}
fn test_scenario_terrane_function_scope_zero_loop() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&value_terrane_function_scope_zero_loop(true))
    );
}
// Source: tests/integration/grouped-precedence/case.trn
// Namespace: grouped-precedence
fn test_scenario_terrane_grouped_precedence() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(3_i128) *
        terrane_int_support::Int::from(3_i128)))
    );
}
// Source: tests/integration/grouped-prefix-statements/case.trn
// Namespace: grouped-prefix-statements
#[derive(Clone)]
pub struct Holder {
    pub value: terrane_int_support::Int,
}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(1_i128),
        }
    }
}
fn test_scenario_terrane_grouped_prefix_statements() {
    let x: Holder;
    let flag: bool;
    x = Holder::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&x.value));
    flag = true;
    println!("{}", terrane_scalar_support::scalar_text(&flag));
    println!("{}", String::from("after"));
}
// Source: tests/integration/identity-evaluation/case.trn
// Namespace: identity-evaluation
fn observed_terrane_identity_evaluation(
    item: terrane_int_support::Int,
) -> terrane_int_support::Int {
    println!("{}", terrane_scalar_support::scalar_text(&item));
    return item.clone();
}
fn test_scenario_terrane_identity_evaluation() {
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let _ =
        observed_terrane_identity_evaluation(terrane_int_support::Int::from(1_i128)) +
        terrane_int_support::Int::from(1_i128); true })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let _ =
        observed_terrane_identity_evaluation(terrane_int_support::Int::from(2_i128)); let
        _ = observed_terrane_identity_evaluation(terrane_int_support::Int::from(3_i128));
        true })
    );
}
// Source: tests/integration/layout-calls/case.trn
// Namespace: layout-calls
fn observed_terrane_layout_calls(value: String) -> String {
    println!("{}", terrane_scalar_support::scalar_text(&value));
    return value;
}
fn combine(first: String, second: String) -> String {
    return {
        let mut __terrane_text = String::with_capacity(1);
        terrane_scalar_support::ScalarDisplay::write_scalar(&first, &mut __terrane_text);
        __terrane_text.push_str(":");
        terrane_scalar_support::ScalarDisplay::write_scalar(
            &second,
            &mut __terrane_text,
        );
        __terrane_text
    };
}
async fn deferred(value: String) -> String {
    return value;
}
#[derive(Clone)]
pub struct ScheduledMessage {
    pub text: String,
}
impl ScheduledMessage {
    pub fn terrane_construct(text: String) -> Self {
        let mut __terrane_constructed_value = Self { text: String::from("") };
        __terrane_constructed_value.construct(text);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, text: String) {
        {
            let __terrane_value: String = text;
            self.text = __terrane_value;
        }
    }
    pub fn with_text(&self, text: String) -> ScheduledMessage {
        return ScheduledMessage::terrane_construct(text);
    }
    pub async fn wait(&self) -> String {
        return self.text.clone();
    }
    pub async fn __terrane_shared_async_wait(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
    ) -> String {
        return {
            let __terrane_snapshot = __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .text
                .clone();
            __terrane_snapshot
        };
    }
}
async fn test_scenario_terrane_layout_calls() {
    let text: String;
    let result: String;
    let text_result: String;
    let literal: String;
    let message: ScheduledMessage;
    let ready: String;
    text = String::from("  hello world  ");
    result = {
        let __terrane_receiver_value_0 = &{
            let __terrane_receiver_value_1 = &terrane_string_support::trim(&text);
            let __terrane_string_argument_0 = &observed_terrane_layout_calls(
                String::from("world"),
            );
            let __terrane_string_argument_1 = &observed_terrane_layout_calls(
                String::from("Terrane"),
            );
            terrane_string_support::replace(
                &__terrane_receiver_value_1,
                &std::convert::AsRef::<str>::as_ref(&__terrane_string_argument_0),
                &std::convert::AsRef::<str>::as_ref(&__terrane_string_argument_1),
            )
        };
        terrane_string_support::upper(&__terrane_receiver_value_0)
    };
    println!("{}", terrane_scalar_support::scalar_text(&result));
    println!("{}", terrane_scalar_support::scalar_text(&text));
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&combine(observed_terrane_layout_calls(String::from("left")),
        observed_terrane_layout_calls(String::from("right"))))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&combine(String::from("nested"),
        String::from("calls")))
    );
    text_result = __terrane_await(
            deferred({
                let __terrane_receiver_value_4 = &{
                    let __terrane_receiver_value_5 = &String::from("  ready  ");
                    terrane_string_support::trim(&__terrane_receiver_value_5)
                };
                terrane_string_support::upper(&__terrane_receiver_value_4)
            }),
        )
        .await;
    println!("{}", terrane_scalar_support::scalar_text(&text_result));
    literal = {
        let __terrane_receiver_value_6 = &{
            let __terrane_receiver_value_7 = &String::from("unused");
            let __terrane_string_argument_0 = "unused";
            let __terrane_string_argument_1 = ".not-a-call;";
            terrane_string_support::replace(
                &__terrane_receiver_value_7,
                &std::convert::AsRef::<str>::as_ref(&__terrane_string_argument_0),
                &std::convert::AsRef::<str>::as_ref(&__terrane_string_argument_1),
            )
        };
        terrane_string_support::upper(&__terrane_receiver_value_6)
    };
    println!("{}", terrane_scalar_support::scalar_text(&literal));
    message = ScheduledMessage::terrane_construct(String::from(""));
    ready = __terrane_await((&message.with_text(String::from("async-chain"))).wait())
        .await;
    println!("{}", terrane_scalar_support::scalar_text(&ready));
}
// Source: tests/integration/lexical-replacement/case.trn
// Namespace: lexical-replacement
fn test_scenario_terrane_lexical_replacement() {
    let value: i8;
    let value_2: i64;
    value = 1;
    println!("{}", terrane_scalar_support::scalar_text(&value));
    value_2 = 2;
    println!("{}", terrane_scalar_support::scalar_text(&value_2));
}
// Source: tests/integration/local-function-lexical-boundaries/case.trn
// Namespace: local-function-lexical-boundaries
static __TERRANE_F21_SHARED: std::sync::LazyLock<terrane_int_support::Int> = std::sync::LazyLock::new(||
terrane_int_support::Int::from(10_i128));
fn helper_terrane_local_function_lexical_boundaries() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(1_i128);
}
fn outer() -> String {
    fn helper_terrane_local_function_lexical_boundaries() -> String {
        return String::from("local");
    }
    let value: String;
    value = helper_terrane_local_function_lexical_boundaries();
    return value;
}
fn factorial() -> terrane_int_support::Int {
    fn fact(n: terrane_int_support::Int) -> terrane_int_support::Int {
        if n <= terrane_int_support::Int::from(1_i128) {
            return terrane_int_support::Int::from(1_i128);
        }
        return n.clone() * fact(n.clone() - terrane_int_support::Int::from(1_i128));
    }
    return fact(terrane_int_support::Int::from(5_i128));
}
fn forward() -> terrane_int_support::Int {
    fn later() -> terrane_int_support::Int {
        return terrane_int_support::Int::from(2_i128);
    }
    let value: terrane_int_support::Int;
    value = later();
    return value.clone();
}
fn parity(value: terrane_int_support::Int) -> terrane_int_support::Int {
    fn even(n: terrane_int_support::Int) -> bool {
        if n == terrane_int_support::Int::from(0_i128) {
            return true;
        }
        return odd(n.clone() - terrane_int_support::Int::from(1_i128));
    }
    fn odd(n: terrane_int_support::Int) -> bool {
        if n == terrane_int_support::Int::from(0_i128) {
            return false;
        }
        return even(n.clone() - terrane_int_support::Int::from(1_i128));
    }
    if even(value.clone()) {
        return terrane_int_support::Int::from(1_i128);
    }
    return terrane_int_support::Int::from(0_i128);
}
fn own_closure() -> terrane_int_support::Int {
    fn nested(value: terrane_int_support::Int) -> terrane_int_support::Int {
        let local: terrane_int_support::Int;
        let callback: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
        local = value.clone() + (*__TERRANE_F21_SHARED).clone();
        callback = {
            let local = local.clone();
            std::sync::Arc::new(move || -> terrane_int_support::Int {
                return local.clone()
                    + helper_terrane_local_function_lexical_boundaries();
            })
        };
        return callback();
    }
    return nested(terrane_int_support::Int::from(1_i128));
}
fn test_scenario_terrane_local_function_lexical_boundaries() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&helper_terrane_local_function_lexical_boundaries())
    );
    println!("{}", terrane_scalar_support::scalar_text(&outer()));
    println!("{}", terrane_scalar_support::scalar_text(&factorial()));
    println!("{}", terrane_scalar_support::scalar_text(&own_closure()));
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&parity(terrane_int_support::Int::from(8_i128)))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&parity(terrane_int_support::Int::from(7_i128)))
    );
    println!("{}", terrane_scalar_support::scalar_text(&forward()));
}
// Source: tests/integration/local-function-results/case.trn
// Namespace: local-function-results
fn first() -> terrane_int_support::Int {
    fn helper_terrane_local_function_results() -> terrane_int_support::Int {
        return terrane_int_support::Int::from(1_i128);
    }
    let value: terrane_int_support::Int;
    value = helper_terrane_local_function_results();
    return value.clone();
}
fn second() -> String {
    fn helper_terrane_local_function_results() -> String {
        return String::from("second");
    }
    let value: String;
    value = helper_terrane_local_function_results();
    return value;
}
fn guarded() -> terrane_int_support::Int {
    fn helper_terrane_local_function_results() -> terrane_int_support::Int {
        return terrane_int_support::Int::from(3_i128);
    }
    let mut value: Option<terrane_int_support::Int> = None;
    let mut __terrane_completion_0: TerraneCompletion<terrane_int_support::Int> = (|| {
        let __terrane_try_0: TerraneCompletion<terrane_int_support::Int> = (|| {
            let _ = value.insert(helper_terrane_local_function_results());
            return TerraneCompletion::Return(
                value.as_ref().expect("flow-proven available binding").clone(),
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
        println!("{}", String::from("F"));
        TerraneCompletion::Normal
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
fn test_scenario_terrane_local_function_results() {
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&first()),
        terrane_scalar_support::scalar_text(&second()),
        terrane_scalar_support::scalar_text(&guarded())
    );
}
// Source: tests/integration/local-function-self-static-member/case.trn
// Namespace: local-function-self-static-member
pub static TERRANE_STATIC_COUNTER_TOTAL: std::sync::LazyLock<
    TerraneSharedCell<terrane_int_support::Int>,
> = std::sync::LazyLock::new(|| TerraneSharedCell::new(
    terrane_int_support::Int::from(0_i128),
));
#[derive(Clone)]
pub struct Counter {}
impl Counter {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn increment(&self) -> terrane_int_support::Int {
        pub fn read() -> terrane_int_support::Int {
            return {
                let __terrane_value = TERRANE_STATIC_COUNTER_TOTAL
                    .lock()
                    .expect("static field lock poisoned")
                    .clone();
                __terrane_value
            };
        }
        {
            let __terrane_static_value: terrane_int_support::Int = {
                let __terrane_value = TERRANE_STATIC_COUNTER_TOTAL
                    .lock()
                    .expect("static field lock poisoned")
                    .clone();
                __terrane_value
            } + terrane_int_support::Int::from(1_i128);
            *TERRANE_STATIC_COUNTER_TOTAL.lock().expect("static field lock poisoned") = __terrane_static_value;
        }
        return read();
    }
    pub fn read(&self) -> terrane_int_support::Int {
        return {
            let __terrane_value = TERRANE_STATIC_COUNTER_TOTAL
                .lock()
                .expect("static field lock poisoned")
                .clone();
            __terrane_value
        };
    }
}
pub static TERRANE_STATIC_STATICCOUNTER_TOTAL: std::sync::LazyLock<
    TerraneSharedCell<terrane_int_support::Int>,
> = std::sync::LazyLock::new(|| TerraneSharedCell::new(
    terrane_int_support::Int::from(7_i128),
));
#[derive(Clone)]
pub struct StaticCounter {}
impl StaticCounter {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn nested(&self) -> terrane_int_support::Int {
        return {
            let __terrane_value = TERRANE_STATIC_STATICCOUNTER_TOTAL
                .lock()
                .expect("static field lock poisoned")
                .clone();
            __terrane_value
        };
    }
    pub fn terrane_static_read() -> terrane_int_support::Int {
        pub fn nested() -> terrane_int_support::Int {
            return {
                let __terrane_value = TERRANE_STATIC_STATICCOUNTER_TOTAL
                    .lock()
                    .expect("static field lock poisoned")
                    .clone();
                __terrane_value
            };
        }
        return nested();
    }
}
pub static TERRANE_STATIC_INHERITEDCOUNTER_TOTAL: std::sync::LazyLock<
    TerraneSharedCell<terrane_int_support::Int>,
> = std::sync::LazyLock::new(|| TerraneSharedCell::new(
    terrane_int_support::Int::from(2_i128),
));
#[derive(Clone)]
pub struct InheritedCounterStorage {}
impl InheritedCounterStorage {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn nested(&self) -> terrane_int_support::Int {
        return {
            let __terrane_value = TERRANE_STATIC_INHERITEDCOUNTER_TOTAL
                .lock()
                .expect("static field lock poisoned")
                .clone();
            __terrane_value
        };
    }
}
#[derive(Clone)]
pub enum InheritedCounter {
    Own(InheritedCounterStorage),
    DerivedCounter(DerivedCounter),
}
impl InheritedCounter {
    pub fn terrane_construct() -> Self {
        Self::Own(InheritedCounterStorage::terrane_construct())
    }
    pub fn terrane_static_read() -> terrane_int_support::Int {
        pub fn nested() -> terrane_int_support::Int {
            return {
                let __terrane_value = TERRANE_STATIC_INHERITEDCOUNTER_TOTAL
                    .lock()
                    .expect("static field lock poisoned")
                    .clone();
                __terrane_value
            };
        }
        return nested();
    }
    pub fn nested(&self) -> terrane_int_support::Int {
        match self {
            Self::Own(value) => value.nested(),
            Self::DerivedCounter(value) => value.nested(),
        }
    }
}
pub static TERRANE_STATIC_DERIVEDCOUNTER_TOTAL: std::sync::LazyLock<
    TerraneSharedCell<terrane_int_support::Int>,
> = std::sync::LazyLock::new(|| TerraneSharedCell::new(
    terrane_int_support::Int::from(2_i128),
));
#[derive(Clone)]
pub struct DerivedCounter {}
impl DerivedCounter {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn nested(&self) -> terrane_int_support::Int {
        return {
            let __terrane_value = TERRANE_STATIC_DERIVEDCOUNTER_TOTAL
                .lock()
                .expect("static field lock poisoned")
                .clone();
            __terrane_value
        };
    }
    pub fn terrane_static_read() -> terrane_int_support::Int {
        pub fn nested() -> terrane_int_support::Int {
            return {
                let __terrane_value = TERRANE_STATIC_DERIVEDCOUNTER_TOTAL
                    .lock()
                    .expect("static field lock poisoned")
                    .clone();
                __terrane_value
            };
        }
        return nested();
    }
}
fn test_scenario_terrane_local_function_self_static_member() {
    let item: Counter;
    item = Counter::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&item.increment()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&StaticCounter::terrane_static_read())
    );
    {
        let __terrane_static_value: terrane_int_support::Int = terrane_int_support::Int::from(
            9_i128,
        );
        *TERRANE_STATIC_DERIVEDCOUNTER_TOTAL
            .lock()
            .expect("static field lock poisoned") = __terrane_static_value;
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&InheritedCounter::terrane_static_read())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&DerivedCounter::terrane_static_read())
    );
}
// Source: tests/integration/loop-binder-owner-rebind/case.trn
// Namespace: loop-binder-owner-rebind
fn shared_shared() {
    let mut values: terrane_collection_support::List<terrane_int_support::Int>;
    let mut value: &terrane_int_support::Int;
    values = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(1_i128)]);
    let __terrane_iterable_0 = &values;
    let mut __terrane_iterator_0 = __terrane_iterable_0.terrane_borrowing_iterator();
    loop {
        value = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&* value));
    }
    values = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(2_i128),
            terrane_int_support::Int::from(3_i128)
        ],
    );
    let __terrane_iterable_1 = &values;
    let mut __terrane_iterator_1 = __terrane_iterable_1.terrane_borrowing_iterator();
    loop {
        value = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &value;
        println!("{}", terrane_scalar_support::scalar_text(&* value));
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(values
        .length()))
    );
}
fn shared_exclusive() {
    let mut values: terrane_collection_support::List<terrane_int_support::Int>;
    let mut value: &terrane_int_support::Int;
    values = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(1_i128)]);
    let __terrane_iterable_2 = &values;
    let mut __terrane_iterator_2 = __terrane_iterable_2.terrane_borrowing_iterator();
    loop {
        value = match __terrane_iterator_2.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&* value));
    }
    values = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(2_i128),
            terrane_int_support::Int::from(3_i128)
        ],
    );
    let __terrane_iterable_3 = &mut values;
    let mut __terrane_iterator_3 = __terrane_iterable_3.terrane_borrowing_iterator();
    loop {
        value = match __terrane_iterator_3.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &value;
        println!("{}", terrane_scalar_support::scalar_text(&* value));
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(values
        .length()))
    );
}
fn exclusive_shared() {
    let mut values: terrane_collection_support::List<terrane_int_support::Int>;
    let mut value: &terrane_int_support::Int;
    values = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(1_i128)]);
    let __terrane_iterable_4 = &mut values;
    let mut __terrane_iterator_4 = __terrane_iterable_4.terrane_borrowing_iterator();
    loop {
        value = match __terrane_iterator_4.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&* value));
    }
    values = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(2_i128),
            terrane_int_support::Int::from(3_i128)
        ],
    );
    let __terrane_iterable_5 = &values;
    let mut __terrane_iterator_5 = __terrane_iterable_5.terrane_borrowing_iterator();
    loop {
        value = match __terrane_iterator_5.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &value;
        println!("{}", terrane_scalar_support::scalar_text(&* value));
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(values
        .length()))
    );
}
fn exclusive_exclusive() {
    let mut values: terrane_collection_support::List<terrane_int_support::Int>;
    let mut value: &terrane_int_support::Int;
    values = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(1_i128)]);
    let __terrane_iterable_6 = &mut values;
    let mut __terrane_iterator_6 = __terrane_iterable_6.terrane_borrowing_iterator();
    loop {
        value = match __terrane_iterator_6.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&* value));
    }
    values = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(2_i128),
            terrane_int_support::Int::from(3_i128)
        ],
    );
    let __terrane_iterable_7 = &mut values;
    let mut __terrane_iterator_7 = __terrane_iterable_7.terrane_borrowing_iterator();
    loop {
        value = match __terrane_iterator_7.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &value;
        println!("{}", terrane_scalar_support::scalar_text(&* value));
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(values
        .length()))
    );
}
fn test_scenario_terrane_loop_binder_owner_rebind() {
    shared_shared();
    shared_exclusive();
    exclusive_shared();
    exclusive_exclusive();
}
// Source: tests/integration/loop-body-backedge-arithmetic/case.trn
// Namespace: conformance/loop-body-backedge-arithmetic
fn inferred() {
    let mut count: terrane_int_support::Int;
    let mut total: terrane_int_support::Int;
    let mut result: terrane_int_support::Int;
    let mut typed_result: terrane_int_support::Int;
    let mut step: terrane_int_support::Int;
    let mut composed: terrane_int_support::Int;
    let mut chained: terrane_int_support::Int;
    let mut comparison: bool;
    let mut previous: Option<terrane_int_support::Int> = None;
    count = terrane_int_support::Int::from(0_i128);
    total = terrane_int_support::Int::from(0_i128);
    while count < terrane_int_support::Int::from(3_i128) {
        count = count.clone() + terrane_int_support::Int::from(1_i128);
        if count > terrane_int_support::Int::from(1_i128) {
            result = previous
                .as_ref()
                .unwrap_or_else(|| __terrane_uninitialized_binding(
                    "previous",
                    "case.trn",
                    10,
                    22,
                ))
                .clone() + terrane_int_support::Int::from(1_i128);
            typed_result = previous
                .as_ref()
                .unwrap_or_else(|| __terrane_uninitialized_binding(
                    "previous",
                    "case.trn",
                    11,
                    32,
                ))
                .clone() + terrane_int_support::Int::from(1_i128);
            step = previous
                .as_ref()
                .unwrap_or_else(|| __terrane_uninitialized_binding(
                    "previous",
                    "case.trn",
                    12,
                    20,
                ))
                .clone();
            composed = step.clone() * terrane_int_support::Int::from(2_i128)
                + terrane_int_support::Int::from(1_i128);
            chained = step.clone() + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128);
            comparison = *previous
                .as_ref()
                .unwrap_or_else(|| __terrane_uninitialized_binding(
                    "previous",
                    "case.trn",
                    15,
                    26,
                )) > terrane_int_support::Int::from(1_i128);
            total = total.clone() + step.clone();
            println!("{}", terrane_scalar_support::scalar_text(&result));
            println!("{}", terrane_scalar_support::scalar_text(&typed_result));
            println!("{}", terrane_scalar_support::scalar_text(&composed));
            println!("{}", terrane_scalar_support::scalar_text(&chained));
            println!("{}", terrane_scalar_support::scalar_text(&comparison));
        }
        let _ = previous.insert(count.clone());
    }
    println!("{}", terrane_scalar_support::scalar_text(&total));
}
fn typed() {
    let mut total: terrane_int_support::Int;
    let mut count: terrane_int_support::Int;
    let mut previous: Option<terrane_int_support::Int> = None;
    total = terrane_int_support::Int::from(0_i128);
    count = terrane_int_support::Int::from(1_i128);
    '__terrane_break_0: while count < terrane_int_support::Int::from(4_i128) {
        '__terrane_continue_0: {
            if count > terrane_int_support::Int::from(1_i128) {
                total = total.clone()
                    + previous
                        .as_ref()
                        .unwrap_or_else(|| __terrane_uninitialized_binding(
                            "previous",
                            "case.trn",
                            29,
                            29,
                        ))
                        .clone();
            }
            let _ = previous.insert(count.clone());
        }
        count = count.clone() + terrane_int_support::Int::from(1_i128);
    }
    println!("{}", terrane_scalar_support::scalar_text(&total));
}
fn test_scenario_terrane_conformance_loop_body_backedge_arithmetic() {
    inferred();
    typed();
}
// Source: tests/integration/loop-body-backedge-for/case.trn
// Namespace: conformance/loop-body-backedge-for
fn test_scenario_terrane_conformance_loop_body_backedge_for() {
    let mut first: bool;
    let mut item: String;
    let mut previous: Option<String> = None;
    first = true;
    {
        let __terrane_iterable_0 = &String::from("abc");
        let mut __terrane_iterator_0 = terrane_string_support::graphemes(
            &__terrane_iterable_0,
        );
        loop {
            item = match __terrane_iterator_0.next() {
                Some(item) => item,
                None => break,
            };
            if !first {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&previous.as_ref()
                    .unwrap_or_else(| | __terrane_uninitialized_binding("previous",
                    "case.trn", 8, 20)))
                );
            }
            let _ = previous.insert(item);
            first = false;
        }
    }
}
// Source: tests/integration/loop-body-backedge-inferred-rhs/case.trn
// Namespace: conformance/loop-body-backedge-inferred-rhs
fn test_scenario_terrane_conformance_loop_body_backedge_inferred_rhs() {
    let mut first: bool;
    let mut current: String;
    let mut observed: String;
    let mut item: Option<String> = None;
    first = true;
    {
        let __terrane_iterable_0 = &String::from("abc");
        let mut __terrane_iterator_0 = terrane_string_support::graphemes(
            &__terrane_iterable_0,
        );
        loop {
            current = match __terrane_iterator_0.next() {
                Some(item) => item,
                None => break,
            };
            if !first {
                observed = item
                    .as_ref()
                    .unwrap_or_else(|| __terrane_uninitialized_binding(
                        "item",
                        "case.trn",
                        8,
                        24,
                    ))
                    .clone();
                println!("{}", terrane_scalar_support::scalar_text(&observed));
            }
            let _ = item.insert(current);
            first = false;
        }
    }
}
// Source: tests/integration/loop-body-backedge-while/case.trn
// Namespace: conformance/loop-body-backedge-while
fn test_scenario_terrane_conformance_loop_body_backedge_while() {
    let mut index: terrane_int_support::Int;
    let mut item: Option<String> = None;
    index = terrane_int_support::Int::from(0_i128);
    while index < terrane_int_support::Int::from(3_i128) {
        if index > terrane_int_support::Int::from(0_i128) {
            println!(
                "{}", terrane_scalar_support::scalar_text(&item.as_ref().unwrap_or_else(|
                | __terrane_uninitialized_binding("item", "case.trn", 8, 20)))
            );
        }
        if index == terrane_int_support::Int::from(0_i128) {
            let _ = item.insert(String::from("a"));
        } else {
            let _ = item.insert(String::from("b"));
        }
        index = index.clone() + terrane_int_support::Int::from(1_i128);
        continue;
    }
}
// Source: tests/integration/loop-shadowing-scope/case.trn
// Namespace: loop-shadowing-scope
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF29S130 {
    Arm0(terrane_collection_support::List<String>),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF29S130 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(_) => {
                unreachable!("semantic display refinement excludes this union arm")
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF29S316 {
    Arm0(
        terrane_collection_support::Entry<
            String,
            terrane_collection_support::List<terrane_int_support::Int>,
        >,
    ),
    Arm1(terrane_int_support::Int),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF29S316 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(_) => {
                unreachable!("semantic display refinement excludes this union arm")
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
fn value_terrane_loop_shadowing_scope() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(1_i128);
}
fn test_scenario_terrane_loop_shadowing_scope() {
    let mut items: TerraneUnionF29S130;
    let groups: terrane_collection_support::Map<
        String,
        terrane_collection_support::List<terrane_int_support::Int>,
    >;
    let mut entry: Option<TerraneUnionF29S316> = None;
    let numbers: terrane_collection_support::List<terrane_int_support::Int>;
    let numbers_2: String;
    let mut value_2: i64;
    let copy: terrane_int_support::Int;
    items = TerraneUnionF29S130::Arm0(
        terrane_collection_support::List::<
            String,
        >::new(vec![String::from("ab"), String::from("c")]),
    );
    let __terrane_iterable_0 = match &items {
        TerraneUnionF29S130::Arm0(value) => value,
        _ => unreachable!("flow-proven storage refinement"),
    }
        .clone();
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        __terrane_iterable_0.as_slice(),
    );
    loop {
        items = TerraneUnionF29S130::Arm1(
            match __terrane_iterator_0.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            },
        );
        let _ = &items;
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &items {
            TerraneUnionF29S130::Arm1(value) => value, _ =>
            unreachable!("flow-proven storage refinement") })
        );
    }
    if matches!(&&items, TerraneUnionF29S130::Arm1(_)) {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &items {
            TerraneUnionF29S130::Arm1(value) => value, _ =>
            unreachable!("flow-proven storage refinement") })
        );
    }
    groups = terrane_collection_support::Map::<
        String,
        terrane_collection_support::List<terrane_int_support::Int>,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("first"),
            terrane_collection_support::List::< terrane_int_support::Int
            >::new(vec![terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128)]))
        ],
    );
    let __terrane_iterable_1 = groups;
    let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_1,
    );
    loop {
        let _ = entry
            .insert(
                TerraneUnionF29S316::Arm0(
                    match __terrane_iterator_1.next() {
                        terrane_collection_support::IterationStep::Item(item) => item,
                        terrane_collection_support::IterationStep::End => break,
                    },
                ),
            );
        let __terrane_iterable_2 = match entry
            .as_ref()
            .expect("flow-proven available binding")
        {
            TerraneUnionF29S316::Arm0(value) => value,
            _ => unreachable!("flow-proven storage refinement"),
        }
            .value
            .clone();
        let mut __terrane_iterator_2 = terrane_collection_support::slice_iterator(
            __terrane_iterable_2.as_slice(),
        );
        loop {
            let _ = entry
                .insert(
                    TerraneUnionF29S316::Arm1(
                        match __terrane_iterator_2.next() {
                            terrane_collection_support::IterationStep::Item(item) => item,
                            terrane_collection_support::IterationStep::End => break,
                        },
                    ),
                );
            let _ = &entry;
            println!(
                "{}", terrane_scalar_support::scalar_text(&match entry.as_ref()
                .expect("flow-proven available binding") {
                TerraneUnionF29S316::Arm1(value) => value, _ =>
                unreachable!("flow-proven storage refinement") })
            );
        }
    }
    if matches!(
        &entry.as_ref().unwrap_or_else(|| __terrane_uninitialized_binding("entry",
        "case.trn", 18, 6)), TerraneUnionF29S316::Arm1(_)
    ) {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match entry.as_ref()
            .unwrap_or_else(| | __terrane_uninitialized_binding("entry", "case.trn", 19,
            12)) { TerraneUnionF29S316::Arm1(value) => value, _ =>
            unreachable!("flow-proven storage refinement") })
        );
    }
    numbers = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128)
        ],
    );
    let _ = &numbers;
    let __terrane_replacement = String::from("ab");
    drop(numbers);
    numbers_2 = __terrane_replacement;
    println!("{}", terrane_scalar_support::scalar_text(&numbers_2));
    println!("{}", terrane_scalar_support::scalar_text(&numbers_2));
    println!(
        "{}", terrane_scalar_support::scalar_text(&value_terrane_loop_shadowing_scope())
    );
    value_2 = 5;
    copy = terrane_int_support::Int::from(value_2 as i128);
    println!("{}", terrane_scalar_support::scalar_text(&copy));
    value_2 = __terrane_raised(
        terrane_int_support::coerce::<
            i64,
        >(
            &(terrane_int_support::Int::from(value_2 as i128)
                + terrane_int_support::Int::from(1_i128)),
        ),

            18 /* terrane-site: tests/integration/loop-shadowing-scope/case.trn:31:15-31:24 */,
    );
    let _ = &value_2;
    println!("{}", terrane_scalar_support::scalar_text(&value_2));
}
// Source: tests/integration/mutable-loop-loan-boundaries/case.trn
// Namespace: mutable-loop-loan-boundaries
fn test_scenario_terrane_mutable_loop_loan_boundaries() {
    let mut values: terrane_collection_support::List<terrane_int_support::Int>;
    let mut other: terrane_collection_support::List<terrane_int_support::Int>;
    let mut value: &terrane_int_support::Int;
    values = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128)
        ],
    );
    other = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(3_i128)]);
    let __terrane_iterable_0 = &mut values;
    let mut __terrane_iterator_0 = __terrane_iterable_0.terrane_borrowing_iterator();
    {
        let __terrane_list_append_0 = other.make_unique();
        loop {
            value = match __terrane_iterator_0.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            println!("{}", terrane_scalar_support::scalar_text(&* value));
            {
                let __terrane_collection_argument_0 = terrane_int_support::Int::from(
                    4_i128,
                );
                __terrane_list_append_0.push(__terrane_collection_argument_0);
            };
        }
    }
    {
        let __terrane_collection_argument_0 = terrane_int_support::Int::from(5_i128);
        values.append(__terrane_collection_argument_0);
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(values
        .length()))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(other
        .length()))
    );
    let __terrane_iterable_1 = &mut values;
    let mut __terrane_iterator_1 = __terrane_iterable_1.terrane_borrowing_iterator();
    loop {
        value = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &value;
        println!("{}", terrane_scalar_support::scalar_text(&* value));
        break;
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(values
        .length()))
    );
}
// Source: tests/integration/name-aliasing/case.trn
// Namespace: name-aliasing
static __TERRANE_F31_ORIGINAL: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| String::from(
    "namespace",
));
static __TERRANE_F31_ALIAS: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    (*__TERRANE_F31_ORIGINAL).clone()
});
fn test_scenario_terrane_name_aliasing() {
    let local: String;
    let copy: String;
    local = String::from("function");
    copy = local;
    println!("{}", terrane_scalar_support::scalar_text(&&* __TERRANE_F31_ALIAS));
    println!("{}", terrane_scalar_support::scalar_text(&copy));
}
// Source: tests/integration/nested-collection-for-binding/case.trn
// Namespace: nested-collection-for-binding
fn test_scenario_terrane_nested_collection_for_binding() {
    let nested: terrane_collection_support::List<
        terrane_collection_support::List<terrane_int_support::Int>,
    >;
    let mut row: terrane_collection_support::List<terrane_int_support::Int>;
    let groups: terrane_collection_support::Map<
        String,
        terrane_collection_support::List<terrane_int_support::Int>,
    >;
    let mut group: terrane_collection_support::Entry<
        String,
        terrane_collection_support::List<terrane_int_support::Int>,
    >;
    nested = terrane_collection_support::List::<
        terrane_collection_support::List<terrane_int_support::Int>,
    >::new(
        vec![
            terrane_collection_support::List::< terrane_int_support::Int
            >::new(vec![terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128)]), terrane_collection_support::List::<
            terrane_int_support::Int >::new(vec![terrane_int_support::Int::from(3_i128)])
        ],
    );
    let __terrane_iterable_0 = nested;
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        __terrane_iterable_0.as_slice(),
    );
    loop {
        row = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}{}",
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(row
            .length())), terrane_scalar_support::scalar_text(&__terrane_raised(row
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            19 /* terrane-site: tests/integration/nested-collection-for-binding/case.trn:7:24-7:30 */)),
            19 /* terrane-site: tests/integration/nested-collection-for-binding/case.trn:7:24-7:30 */))
        );
    }
    groups = terrane_collection_support::Map::<
        String,
        terrane_collection_support::List<terrane_int_support::Int>,
    >::new(
        vec![
            terrane_collection_support::Entry::< String, terrane_collection_support::List
            < terrane_int_support::Int >>::new(String::from("a"),
            terrane_collection_support::List::< terrane_int_support::Int
            >::new(vec![terrane_int_support::Int::from(4_i128),
            terrane_int_support::Int::from(5_i128)]))
        ],
    );
    let __terrane_iterable_1 = groups;
    let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_1,
    );
    loop {
        group = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_0 =
            &group.value.clone(); __terrane_raised(__terrane_receiver_value_0
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            20 /* terrane-site: tests/integration/nested-collection-for-binding/case.trn:10:12-10:26 */)),
            20 /* terrane-site: tests/integration/nested-collection-for-binding/case.trn:10:12-10:26 */) })
        );
    }
}
// Source: tests/integration/reference-reassign-reaching-assignments/case.trn
// Namespace: reference-reassign-reaching-assignments
fn test_scenario_terrane_reference_reassign_reaching_assignments() {
    let mut a: terrane_int_support::Int;
    let mut b: terrane_int_support::Int;
    let mut c: terrane_int_support::Int;
    let mut r: &terrane_int_support::Int;
    let mutable_value: &mut terrane_int_support::Int;
    a = terrane_int_support::Int::from(1_i128);
    b = terrane_int_support::Int::from(2_i128);
    c = terrane_int_support::Int::from(3_i128);
    r = &a;
    println!("{}", terrane_scalar_support::scalar_text(&* r));
    r = &b;
    a = terrane_int_support::Int::from(4_i128);
    println!("{}", terrane_scalar_support::scalar_text(&* r));
    r = &c;
    b = terrane_int_support::Int::from(5_i128);
    println!("{}", terrane_scalar_support::scalar_text(&b));
    println!("{}", terrane_scalar_support::scalar_text(&* r));
    r = &a;
    println!("{}", terrane_scalar_support::scalar_text(&* r));
    mutable_value = &mut c;
    *mutable_value = terrane_int_support::Int::from(6_i128);
    println!("{}", terrane_scalar_support::scalar_text(&c));
}
// Source: tests/integration/reference-return-reassigned-parameter/case.trn
// Namespace: reference-return-reassigned-parameter
fn pick<'a>(
    a: &terrane_int_support::Int,
    b: &'a terrane_int_support::Int,
) -> &'a terrane_int_support::Int {
    let r: &terrane_int_support::Int;
    r = a;
    let _ = &r;
    let r = b;
    return r;
}
fn pick_observed<'a>(
    a: &terrane_int_support::Int,
    b: &'a terrane_int_support::Int,
) -> &'a terrane_int_support::Int {
    let r: &terrane_int_support::Int;
    r = a;
    println!("{}", terrane_scalar_support::scalar_text(&* r));
    let r = b;
    return r;
}
fn test_scenario_terrane_reference_return_reassigned_parameter() {
    let a: terrane_int_support::Int;
    let b: terrane_int_support::Int;
    let r: &terrane_int_support::Int;
    let observed: &terrane_int_support::Int;
    a = terrane_int_support::Int::from(1_i128);
    b = terrane_int_support::Int::from(2_i128);
    r = pick(&a, &b);
    println!("{}", terrane_scalar_support::scalar_text(&* r));
    observed = pick_observed(&a, &b);
    println!("{}", terrane_scalar_support::scalar_text(&* observed));
}
// Source: tests/integration/return-catch-class-in-loop/case.trn
// Namespace: return-catch-class-in-loop
#[derive(Clone)]
pub struct TerraneNsReturnCatchClassInLoopErrIdentity2F72657475726E2D63617463682D636C6173732D696E2D6C6F6F7000457272 {
    pub message: String,
}
impl TerraneNsReturnCatchClassInLoopErrIdentity2F72657475726E2D63617463682D636C6173732D696E2D6C6F6F7000457272 {
    pub fn terrane_construct() -> Self {
        Self { message: String::from("x") }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
fn bad_terrane_return_catch_class_in_loop() -> Result<
    terrane_int_support::Int,
    TerraneError,
> {
    return Err({
        let value = TerraneNsReturnCatchClassInLoopErrIdentity2F72657475726E2D63617463682D636C6173732D696E2D6C6F6F7000457272::terrane_construct();
        let rendered = value.render();
        let original = &value.message;
        let (message, rendered) = if &rendered == original {
            (rendered, None)
        } else {
            (original.clone(), Some(rendered))
        };
        TerraneError::raised_with_rendered_message(
            TerraneErrorKind::Custom(DescriptorId(0)),
            message,
            rendered,

                21 /* terrane-site: tests/integration/return-catch-class-in-loop/case.trn:11:5-11:24 */,
        )
    });
}
#[derive(Clone)]
pub struct TerraneNsReturnCatchClassInLoopKIdentity2F72657475726E2D63617463682D636C6173732D696E2D6C6F6F70004B {
    pub n: terrane_int_support::Int,
}
impl TerraneNsReturnCatchClassInLoopKIdentity2F72657475726E2D63617463682D636C6173732D696E2D6C6F6F70004B {
    pub fn terrane_construct() -> Self {
        Self {
            n: terrane_int_support::Int::from(4_i128),
        }
    }
}
fn eatk_terrane_return_catch_class_in_loop(
    x: TerraneNsReturnCatchClassInLoopKIdentity2F72657475726E2D63617463682D636C6173732D696E2D6C6F6F70004B,
) -> Result<
    TerraneNsReturnCatchClassInLoopKIdentity2F72657475726E2D63617463682D636C6173732D696E2D6C6F6F70004B,
    TerraneError,
> {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_traced_err(bad_terrane_return_catch_class_in_loop(),
        22 /* terrane-site: tests/integration/return-catch-class-in-loop/case.trn:17:13-17:17 */) ?)
    );
    return Ok(x);
}
fn f_terrane_return_catch_class_in_loop(
    o: TerraneNsReturnCatchClassInLoopKIdentity2F72657475726E2D63617463682D636C6173732D696E2D6C6F6F70004B,
) -> TerraneNsReturnCatchClassInLoopKIdentity2F72657475726E2D63617463682D636C6173732D696E2D6C6F6F70004B {
    let mut i: terrane_int_support::Int;
    let __terrane_iterable_0 = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128)
        ],
    );
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        __terrane_iterable_0.as_slice(),
    );
    loop {
        i = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let __terrane_completion_0: TerraneCompletion<
            TerraneNsReturnCatchClassInLoopKIdentity2F72657475726E2D63617463682D636C6173732D696E2D6C6F6F70004B,
        > = (|| {
            let __terrane_try_0: TerraneCompletion<
                TerraneNsReturnCatchClassInLoopKIdentity2F72657475726E2D63617463682D636C6173732D696E2D6C6F6F70004B,
            > = (|| {
                return TerraneCompletion::Return(
                    __terrane_traced_completion!(
                        eatk_terrane_return_catch_class_in_loop(o.clone()),
                        23 /* terrane-site: tests/integration/return-catch-class-in-loop/case.trn:23:21-23:28 */
                    ),
                );
            })();
            match __terrane_try_0 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_0) => {
                    let mut __terrane_handled_0 = false;
                    if !__terrane_handled_0 {
                        __terrane_handled_0 = true;
                        println!("{}", terrane_scalar_support::scalar_text(&i));
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
            TerraneCompletion::Break => break,
            TerraneCompletion::Continue => {
                __terrane_generated_defect("non-continuing try produced continue")
            }
        }
    }
    return o;
}
fn test_scenario_terrane_return_catch_class_in_loop() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&f_terrane_return_catch_class_in_loop(TerraneNsReturnCatchClassInLoopKIdentity2F72657475726E2D63617463682D636C6173732D696E2D6C6F6F70004B::terrane_construct())
        .n)
    );
}
// Source: tests/integration/return-catch-class-no-later-read/case.trn
// Namespace: return-catch-class-no-later-read
#[derive(Clone)]
pub struct TerraneNsReturnCatchClassNoLaterReadErrIdentity2F72657475726E2D63617463682D636C6173732D6E6F2D6C617465722D7265616400457272 {
    pub message: String,
}
impl TerraneNsReturnCatchClassNoLaterReadErrIdentity2F72657475726E2D63617463682D636C6173732D6E6F2D6C617465722D7265616400457272 {
    pub fn terrane_construct() -> Self {
        Self { message: String::from("x") }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
fn bad_terrane_return_catch_class_no_later_read() -> Result<
    terrane_int_support::Int,
    TerraneError,
> {
    return Err({
        let value = TerraneNsReturnCatchClassNoLaterReadErrIdentity2F72657475726E2D63617463682D636C6173732D6E6F2D6C617465722D7265616400457272::terrane_construct();
        let rendered = value.render();
        let original = &value.message;
        let (message, rendered) = if &rendered == original {
            (rendered, None)
        } else {
            (original.clone(), Some(rendered))
        };
        TerraneError::raised_with_rendered_message(
            TerraneErrorKind::Custom(DescriptorId(1)),
            message,
            rendered,

                24 /* terrane-site: tests/integration/return-catch-class-no-later-read/case.trn:11:5-11:24 */,
        )
    });
}
#[derive(Clone)]
pub struct TerraneNsReturnCatchClassNoLaterReadKIdentity2F72657475726E2D63617463682D636C6173732D6E6F2D6C617465722D72656164004B {
    pub n: terrane_int_support::Int,
}
impl TerraneNsReturnCatchClassNoLaterReadKIdentity2F72657475726E2D63617463682D636C6173732D6E6F2D6C617465722D72656164004B {
    pub fn terrane_construct() -> Self {
        Self {
            n: terrane_int_support::Int::from(4_i128),
        }
    }
}
fn eatk_terrane_return_catch_class_no_later_read(
    x: TerraneNsReturnCatchClassNoLaterReadKIdentity2F72657475726E2D63617463682D636C6173732D6E6F2D6C617465722D72656164004B,
) -> Result<
    TerraneNsReturnCatchClassNoLaterReadKIdentity2F72657475726E2D63617463682D636C6173732D6E6F2D6C617465722D72656164004B,
    TerraneError,
> {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_traced_err(bad_terrane_return_catch_class_no_later_read(),
        25 /* terrane-site: tests/integration/return-catch-class-no-later-read/case.trn:17:13-17:17 */) ?)
    );
    return Ok(x);
}
fn f_terrane_return_catch_class_no_later_read(
    o: TerraneNsReturnCatchClassNoLaterReadKIdentity2F72657475726E2D63617463682D636C6173732D6E6F2D6C617465722D72656164004B,
    d: TerraneNsReturnCatchClassNoLaterReadKIdentity2F72657475726E2D63617463682D636C6173732D6E6F2D6C617465722D72656164004B,
) -> TerraneNsReturnCatchClassNoLaterReadKIdentity2F72657475726E2D63617463682D636C6173732D6E6F2D6C617465722D72656164004B {
    let __terrane_completion_0: TerraneCompletion<
        TerraneNsReturnCatchClassNoLaterReadKIdentity2F72657475726E2D63617463682D636C6173732D6E6F2D6C617465722D72656164004B,
    > = (|| {
        let __terrane_try_0: TerraneCompletion<
            TerraneNsReturnCatchClassNoLaterReadKIdentity2F72657475726E2D63617463682D636C6173732D6E6F2D6C617465722D72656164004B,
        > = (|| {
            return TerraneCompletion::Return(
                __terrane_traced_completion!(
                    eatk_terrane_return_catch_class_no_later_read(o),
                    26 /* terrane-site: tests/integration/return-catch-class-no-later-read/case.trn:22:17-22:24 */
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
                    __terrane_handled_0 = true;
                    println!("{}", terrane_scalar_support::scalar_text(&0));
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
    return d;
}
fn test_scenario_terrane_return_catch_class_no_later_read() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&f_terrane_return_catch_class_no_later_read(TerraneNsReturnCatchClassNoLaterReadKIdentity2F72657475726E2D63617463682D636C6173732D6E6F2D6C617465722D72656164004B::terrane_construct(),
        TerraneNsReturnCatchClassNoLaterReadKIdentity2F72657475726E2D63617463682D636C6173732D6E6F2D6C617465722D72656164004B::terrane_construct())
        .n)
    );
}
// Source: tests/integration/return-catch-class-read-in-catch/case.trn
// Namespace: return-catch-class-read-in-catch
#[derive(Clone)]
pub struct TerraneNsReturnCatchClassReadInCatchErrIdentity2F72657475726E2D63617463682D636C6173732D726561642D696E2D636174636800457272 {
    pub message: String,
}
impl TerraneNsReturnCatchClassReadInCatchErrIdentity2F72657475726E2D63617463682D636C6173732D726561642D696E2D636174636800457272 {
    pub fn terrane_construct() -> Self {
        Self { message: String::from("x") }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
fn bad_terrane_return_catch_class_read_in_catch() -> Result<
    terrane_int_support::Int,
    TerraneError,
> {
    return Err({
        let value = TerraneNsReturnCatchClassReadInCatchErrIdentity2F72657475726E2D63617463682D636C6173732D726561642D696E2D636174636800457272::terrane_construct();
        let rendered = value.render();
        let original = &value.message;
        let (message, rendered) = if &rendered == original {
            (rendered, None)
        } else {
            (original.clone(), Some(rendered))
        };
        TerraneError::raised_with_rendered_message(
            TerraneErrorKind::Custom(DescriptorId(2)),
            message,
            rendered,

                27 /* terrane-site: tests/integration/return-catch-class-read-in-catch/case.trn:11:5-11:24 */,
        )
    });
}
#[derive(Clone)]
pub struct TerraneNsReturnCatchClassReadInCatchKIdentity2F72657475726E2D63617463682D636C6173732D726561642D696E2D6361746368004B {
    pub n: terrane_int_support::Int,
}
impl TerraneNsReturnCatchClassReadInCatchKIdentity2F72657475726E2D63617463682D636C6173732D726561642D696E2D6361746368004B {
    pub fn terrane_construct() -> Self {
        Self {
            n: terrane_int_support::Int::from(4_i128),
        }
    }
}
fn eatk_terrane_return_catch_class_read_in_catch(
    x: TerraneNsReturnCatchClassReadInCatchKIdentity2F72657475726E2D63617463682D636C6173732D726561642D696E2D6361746368004B,
) -> Result<
    TerraneNsReturnCatchClassReadInCatchKIdentity2F72657475726E2D63617463682D636C6173732D726561642D696E2D6361746368004B,
    TerraneError,
> {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_traced_err(bad_terrane_return_catch_class_read_in_catch(),
        28 /* terrane-site: tests/integration/return-catch-class-read-in-catch/case.trn:17:13-17:17 */) ?)
    );
    return Ok(x);
}
fn f_terrane_return_catch_class_read_in_catch(
    o: TerraneNsReturnCatchClassReadInCatchKIdentity2F72657475726E2D63617463682D636C6173732D726561642D696E2D6361746368004B,
) -> terrane_int_support::Int {
    let __terrane_completion_0: TerraneCompletion<terrane_int_support::Int> = (|| {
        let __terrane_try_0: TerraneCompletion<terrane_int_support::Int> = (|| {
            return TerraneCompletion::Return(
                __terrane_traced_completion!(
                    eatk_terrane_return_catch_class_read_in_catch(o.clone()),
                    29 /* terrane-site: tests/integration/return-catch-class-read-in-catch/case.trn:22:17-22:24 */
                )
                    .n
                    .clone(),
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
                    __terrane_handled_0 = true;
                    println!("{}", terrane_scalar_support::scalar_text(&o.n));
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
    return terrane_int_support::Int::from(0_i128);
}
fn test_scenario_terrane_return_catch_class_read_in_catch() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&f_terrane_return_catch_class_read_in_catch(TerraneNsReturnCatchClassReadInCatchKIdentity2F72657475726E2D63617463682D636C6173732D726561642D696E2D6361746368004B::terrane_construct()))
    );
}
// Source: tests/integration/return-catch-class-then-read/case.trn
// Namespace: return-catch-class-then-read
#[derive(Clone)]
pub struct TerraneNsReturnCatchClassThenReadErrIdentity2F72657475726E2D63617463682D636C6173732D7468656E2D7265616400457272 {
    pub message: String,
}
impl TerraneNsReturnCatchClassThenReadErrIdentity2F72657475726E2D63617463682D636C6173732D7468656E2D7265616400457272 {
    pub fn terrane_construct() -> Self {
        Self { message: String::from("x") }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
fn bad_terrane_return_catch_class_then_read() -> Result<
    terrane_int_support::Int,
    TerraneError,
> {
    return Err({
        let value = TerraneNsReturnCatchClassThenReadErrIdentity2F72657475726E2D63617463682D636C6173732D7468656E2D7265616400457272::terrane_construct();
        let rendered = value.render();
        let original = &value.message;
        let (message, rendered) = if &rendered == original {
            (rendered, None)
        } else {
            (original.clone(), Some(rendered))
        };
        TerraneError::raised_with_rendered_message(
            TerraneErrorKind::Custom(DescriptorId(3)),
            message,
            rendered,

                30 /* terrane-site: tests/integration/return-catch-class-then-read/case.trn:11:5-11:24 */,
        )
    });
}
#[derive(Clone)]
pub struct TerraneNsReturnCatchClassThenReadKIdentity2F72657475726E2D63617463682D636C6173732D7468656E2D72656164004B {
    pub n: terrane_int_support::Int,
}
impl TerraneNsReturnCatchClassThenReadKIdentity2F72657475726E2D63617463682D636C6173732D7468656E2D72656164004B {
    pub fn terrane_construct() -> Self {
        Self {
            n: terrane_int_support::Int::from(4_i128),
        }
    }
}
fn eatk_terrane_return_catch_class_then_read(
    x: TerraneNsReturnCatchClassThenReadKIdentity2F72657475726E2D63617463682D636C6173732D7468656E2D72656164004B,
) -> Result<
    TerraneNsReturnCatchClassThenReadKIdentity2F72657475726E2D63617463682D636C6173732D7468656E2D72656164004B,
    TerraneError,
> {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_traced_err(bad_terrane_return_catch_class_then_read(),
        31 /* terrane-site: tests/integration/return-catch-class-then-read/case.trn:17:13-17:17 */) ?)
    );
    return Ok(x);
}
fn f_terrane_return_catch_class_then_read(
    o: TerraneNsReturnCatchClassThenReadKIdentity2F72657475726E2D63617463682D636C6173732D7468656E2D72656164004B,
) -> TerraneNsReturnCatchClassThenReadKIdentity2F72657475726E2D63617463682D636C6173732D7468656E2D72656164004B {
    let __terrane_completion_0: TerraneCompletion<
        TerraneNsReturnCatchClassThenReadKIdentity2F72657475726E2D63617463682D636C6173732D7468656E2D72656164004B,
    > = (|| {
        let __terrane_try_0: TerraneCompletion<
            TerraneNsReturnCatchClassThenReadKIdentity2F72657475726E2D63617463682D636C6173732D7468656E2D72656164004B,
        > = (|| {
            return TerraneCompletion::Return(
                __terrane_traced_completion!(
                    eatk_terrane_return_catch_class_then_read(o.clone()),
                    32 /* terrane-site: tests/integration/return-catch-class-then-read/case.trn:22:17-22:24 */
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
                    __terrane_handled_0 = true;
                    println!("{}", terrane_scalar_support::scalar_text(&0));
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
    return o;
}
fn test_scenario_terrane_return_catch_class_then_read() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&f_terrane_return_catch_class_then_read(TerraneNsReturnCatchClassThenReadKIdentity2F72657475726E2D63617463682D636C6173732D7468656E2D72656164004B::terrane_construct())
        .n)
    );
}
// Source: tests/integration/return-catch-list-then-read/case.trn
// Namespace: return-catch-list-then-read
#[derive(Clone)]
pub struct TerraneNsReturnCatchListThenReadErrIdentity2F72657475726E2D63617463682D6C6973742D7468656E2D7265616400457272 {
    pub message: String,
}
impl TerraneNsReturnCatchListThenReadErrIdentity2F72657475726E2D63617463682D6C6973742D7468656E2D7265616400457272 {
    pub fn terrane_construct() -> Self {
        Self { message: String::from("x") }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
fn bad_terrane_return_catch_list_then_read() -> Result<
    terrane_int_support::Int,
    TerraneError,
> {
    return Err({
        let value = TerraneNsReturnCatchListThenReadErrIdentity2F72657475726E2D63617463682D6C6973742D7468656E2D7265616400457272::terrane_construct();
        let rendered = value.render();
        let original = &value.message;
        let (message, rendered) = if &rendered == original {
            (rendered, None)
        } else {
            (original.clone(), Some(rendered))
        };
        TerraneError::raised_with_rendered_message(
            TerraneErrorKind::Custom(DescriptorId(4)),
            message,
            rendered,

                33 /* terrane-site: tests/integration/return-catch-list-then-read/case.trn:11:5-11:24 */,
        )
    });
}
fn eatl(
    x: terrane_collection_support::List<terrane_int_support::Int>,
) -> Result<terrane_collection_support::List<terrane_int_support::Int>, TerraneError> {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_traced_err(bad_terrane_return_catch_list_then_read(),
        34 /* terrane-site: tests/integration/return-catch-list-then-read/case.trn:14:13-14:17 */) ?)
    );
    return Ok(x);
}
fn f_terrane_return_catch_list_then_read(
    o: terrane_collection_support::List<terrane_int_support::Int>,
) -> terrane_collection_support::List<terrane_int_support::Int> {
    let __terrane_completion_0: TerraneCompletion<
        terrane_collection_support::List<terrane_int_support::Int>,
    > = (|| {
        let __terrane_try_0: TerraneCompletion<
            terrane_collection_support::List<terrane_int_support::Int>,
        > = (|| {
            return TerraneCompletion::Return(
                __terrane_traced_completion!(
                    eatl(o.clone()), 35 /* terrane-site: tests/integration/return-catch-list-then-read/case.trn:19:17-19:24 */
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
                    __terrane_handled_0 = true;
                    println!("{}", terrane_scalar_support::scalar_text(&0));
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
    return o;
}
fn test_scenario_terrane_return_catch_list_then_read() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(f_terrane_return_catch_list_then_read(terrane_collection_support::List::
        < terrane_int_support::Int > ::new(vec![terrane_int_support::Int::from(1_i128),
        terrane_int_support::Int::from(2_i128)])).length()))
    );
}
// Source: tests/integration/return-catch-string-then-read/case.trn
// Namespace: return-catch-string-then-read
#[derive(Clone)]
pub struct TerraneNsReturnCatchStringThenReadErrIdentity2F72657475726E2D63617463682D737472696E672D7468656E2D7265616400457272 {
    pub message: String,
}
impl TerraneNsReturnCatchStringThenReadErrIdentity2F72657475726E2D63617463682D737472696E672D7468656E2D7265616400457272 {
    pub fn terrane_construct() -> Self {
        Self { message: String::from("x") }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
fn bad_terrane_return_catch_string_then_read() -> Result<
    terrane_int_support::Int,
    TerraneError,
> {
    return Err({
        let value = TerraneNsReturnCatchStringThenReadErrIdentity2F72657475726E2D63617463682D737472696E672D7468656E2D7265616400457272::terrane_construct();
        let rendered = value.render();
        let original = &value.message;
        let (message, rendered) = if &rendered == original {
            (rendered, None)
        } else {
            (original.clone(), Some(rendered))
        };
        TerraneError::raised_with_rendered_message(
            TerraneErrorKind::Custom(DescriptorId(5)),
            message,
            rendered,

                36 /* terrane-site: tests/integration/return-catch-string-then-read/case.trn:11:5-11:24 */,
        )
    });
}
fn eats(x: String) -> Result<String, TerraneError> {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_traced_err(bad_terrane_return_catch_string_then_read(),
        37 /* terrane-site: tests/integration/return-catch-string-then-read/case.trn:14:13-14:17 */) ?)
    );
    return Ok(x);
}
fn f_terrane_return_catch_string_then_read(o: String) -> String {
    let __terrane_completion_0: TerraneCompletion<String> = (|| {
        let __terrane_try_0: TerraneCompletion<String> = (|| {
            return TerraneCompletion::Return(
                __terrane_traced_completion!(
                    eats(o.clone()), 38 /* terrane-site: tests/integration/return-catch-string-then-read/case.trn:19:17-19:24 */
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
                    __terrane_handled_0 = true;
                    println!("{}", terrane_scalar_support::scalar_text(&0));
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
    return o;
}
fn test_scenario_terrane_return_catch_string_then_read() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&f_terrane_return_catch_string_then_read(String::from("a")))
    );
}
// Source: tests/integration/return-finally-class-read/case.trn
// Namespace: return-finally-class-read
#[derive(Clone)]
pub struct TerraneNsReturnFinallyClassReadErrIdentity2F72657475726E2D66696E616C6C792D636C6173732D7265616400457272 {
    pub message: String,
}
impl TerraneNsReturnFinallyClassReadErrIdentity2F72657475726E2D66696E616C6C792D636C6173732D7265616400457272 {
    pub fn terrane_construct() -> Self {
        Self { message: String::from("x") }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
fn bad_terrane_return_finally_class_read() -> Result<
    terrane_int_support::Int,
    TerraneError,
> {
    return Err({
        let value = TerraneNsReturnFinallyClassReadErrIdentity2F72657475726E2D66696E616C6C792D636C6173732D7265616400457272::terrane_construct();
        let rendered = value.render();
        let original = &value.message;
        let (message, rendered) = if &rendered == original {
            (rendered, None)
        } else {
            (original.clone(), Some(rendered))
        };
        TerraneError::raised_with_rendered_message(
            TerraneErrorKind::Custom(DescriptorId(6)),
            message,
            rendered,

                39 /* terrane-site: tests/integration/return-finally-class-read/case.trn:11:5-11:24 */,
        )
    });
}
#[derive(Clone)]
pub struct TerraneNsReturnFinallyClassReadKIdentity2F72657475726E2D66696E616C6C792D636C6173732D72656164004B {
    pub n: terrane_int_support::Int,
}
impl TerraneNsReturnFinallyClassReadKIdentity2F72657475726E2D66696E616C6C792D636C6173732D72656164004B {
    pub fn terrane_construct() -> Self {
        Self {
            n: terrane_int_support::Int::from(4_i128),
        }
    }
}
fn eatk_terrane_return_finally_class_read(
    x: TerraneNsReturnFinallyClassReadKIdentity2F72657475726E2D66696E616C6C792D636C6173732D72656164004B,
) -> Result<
    TerraneNsReturnFinallyClassReadKIdentity2F72657475726E2D66696E616C6C792D636C6173732D72656164004B,
    TerraneError,
> {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_traced_err(bad_terrane_return_finally_class_read(),
        40 /* terrane-site: tests/integration/return-finally-class-read/case.trn:17:13-17:17 */) ?)
    );
    return Ok(x);
}
fn f_terrane_return_finally_class_read(
    o: TerraneNsReturnFinallyClassReadKIdentity2F72657475726E2D66696E616C6C792D636C6173732D72656164004B,
) -> Result<
    TerraneNsReturnFinallyClassReadKIdentity2F72657475726E2D66696E616C6C792D636C6173732D72656164004B,
    TerraneError,
> {
    let mut __terrane_completion_0: TerraneCompletion<
        TerraneNsReturnFinallyClassReadKIdentity2F72657475726E2D66696E616C6C792D636C6173732D72656164004B,
    > = (|| {
        let __terrane_try_0: TerraneCompletion<
            TerraneNsReturnFinallyClassReadKIdentity2F72657475726E2D66696E616C6C792D636C6173732D72656164004B,
        > = (|| {
            return TerraneCompletion::Return(
                __terrane_traced_completion!(
                    eatk_terrane_return_finally_class_read(o.clone()),
                    41 /* terrane-site: tests/integration/return-finally-class-read/case.trn:22:17-22:24 */
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
    let __terrane_finally_0: TerraneCompletion<
        TerraneNsReturnFinallyClassReadKIdentity2F72657475726E2D66696E616C6C792D636C6173732D72656164004B,
    > = (|| {
        println!("{}", terrane_scalar_support::scalar_text(&o.n));
        TerraneCompletion::Normal
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
fn test_scenario_terrane_return_finally_class_read() {
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_traced_completion!(f_terrane_return_finally_class_read(TerraneNsReturnFinallyClassReadKIdentity2F72657475726E2D66696E616C6C792D636C6173732D72656164004B::terrane_construct()),
                42 /* terrane-site: tests/integration/return-finally-class-read/case.trn:28:17-28:33 */) .n)
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
                if !__terrane_handled_1 {
                    __terrane_handled_1 = true;
                    println!("{}", terrane_scalar_support::scalar_text(&0));
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
// Source: tests/integration/return-plain-return-in-try-class/case.trn
// Namespace: return-plain-return-in-try-class
#[derive(Clone)]
pub struct TerraneNsReturnPlainReturnInTryClassErrIdentity2F72657475726E2D706C61696E2D72657475726E2D696E2D7472792D636C61737300457272 {
    pub message: String,
}
impl TerraneNsReturnPlainReturnInTryClassErrIdentity2F72657475726E2D706C61696E2D72657475726E2D696E2D7472792D636C61737300457272 {
    pub fn terrane_construct() -> Self {
        Self { message: String::from("x") }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
fn bad_terrane_return_plain_return_in_try_class() -> Result<
    terrane_int_support::Int,
    TerraneError,
> {
    return Err({
        let value = TerraneNsReturnPlainReturnInTryClassErrIdentity2F72657475726E2D706C61696E2D72657475726E2D696E2D7472792D636C61737300457272::terrane_construct();
        let rendered = value.render();
        let original = &value.message;
        let (message, rendered) = if &rendered == original {
            (rendered, None)
        } else {
            (original.clone(), Some(rendered))
        };
        TerraneError::raised_with_rendered_message(
            TerraneErrorKind::Custom(DescriptorId(7)),
            message,
            rendered,

                43 /* terrane-site: tests/integration/return-plain-return-in-try-class/case.trn:11:5-11:24 */,
        )
    });
}
#[derive(Clone)]
pub struct TerraneNsReturnPlainReturnInTryClassKIdentity2F72657475726E2D706C61696E2D72657475726E2D696E2D7472792D636C617373004B {
    pub n: terrane_int_support::Int,
}
impl TerraneNsReturnPlainReturnInTryClassKIdentity2F72657475726E2D706C61696E2D72657475726E2D696E2D7472792D636C617373004B {
    pub fn terrane_construct() -> Self {
        Self {
            n: terrane_int_support::Int::from(4_i128),
        }
    }
}
fn f_terrane_return_plain_return_in_try_class(
    o: TerraneNsReturnPlainReturnInTryClassKIdentity2F72657475726E2D706C61696E2D72657475726E2D696E2D7472792D636C617373004B,
    b: bool,
) -> TerraneNsReturnPlainReturnInTryClassKIdentity2F72657475726E2D706C61696E2D72657475726E2D696E2D7472792D636C617373004B {
    let __terrane_completion_0: TerraneCompletion<
        TerraneNsReturnPlainReturnInTryClassKIdentity2F72657475726E2D706C61696E2D72657475726E2D696E2D7472792D636C617373004B,
    > = (|| {
        let __terrane_try_0: TerraneCompletion<
            TerraneNsReturnPlainReturnInTryClassKIdentity2F72657475726E2D706C61696E2D72657475726E2D696E2D7472792D636C617373004B,
        > = (|| {
            if b {
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&__terrane_traced_completion!(bad_terrane_return_plain_return_in_try_class(),
                    44 /* terrane-site: tests/integration/return-plain-return-in-try-class/case.trn:19:21-19:25 */))
                );
            }
            return TerraneCompletion::Return(o.clone());
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    __terrane_handled_0 = true;
                    println!("{}", terrane_scalar_support::scalar_text(&0));
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
    return o;
}
fn test_scenario_terrane_return_plain_return_in_try_class() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&f_terrane_return_plain_return_in_try_class(TerraneNsReturnPlainReturnInTryClassKIdentity2F72657475726E2D706C61696E2D72657475726E2D696E2D7472792D636C617373004B::terrane_construct(),
        true).n)
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&f_terrane_return_plain_return_in_try_class(TerraneNsReturnPlainReturnInTryClassKIdentity2F72657475726E2D706C61696E2D72657475726E2D696E2D7472792D636C617373004B::terrane_construct(),
        false).n)
    );
}
// Source: tests/integration/return-tail-block-strings/case.trn
// Namespace: return-tail-block-strings
fn render(value: terrane_int_support::Int) -> Result<String, TerraneError> {
    if value < terrane_int_support::Int::from(0_i128) {
        return Err(
            TerraneError::raised(
                TerraneErrorKind::CoercionError,

                    45 /* terrane-site: tests/integration/return-tail-block-strings/case.trn:6:5-6:25 */,
            ),
        );
    }
    return Ok(String::from("ok"));
}
fn render_lines() -> String {
    return String::from("first line\nsecond line");
}
fn test_scenario_terrane_return_tail_block_strings() -> Result<(), TerraneError> {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_traced_err(render(terrane_int_support::Int::from(1_i128)),
        46 /* terrane-site: tests/integration/return-tail-block-strings/case.trn:15:11-15:20 */) ?)
    );
    println!("{}", terrane_scalar_support::scalar_text(&render_lines()));
    Ok(())
}
// Source: tests/integration/returned-callable-local-loop/case.trn
// Namespace: returned-callable-local-loop
fn count(
    values: terrane_collection_support::List<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    return terrane_int_support::Int::from(values.length());
}
fn make_terrane_returned_callable_local_loop() -> std::sync::Arc<
    dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
> {
    return {
        std::sync::Arc::new(move |
            result: terrane_int_support::Int,
        | -> terrane_int_support::Int {
            let values: terrane_collection_support::List<terrane_int_support::Int>;
            let mut i: terrane_int_support::Int;
            values = terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(
                vec![
                    terrane_int_support::Int::from(1_i128),
                    terrane_int_support::Int::from(2_i128)
                ],
            );
            let __terrane_iterable_0 = terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(
                vec![
                    terrane_int_support::Int::from(1_i128),
                    terrane_int_support::Int::from(2_i128)
                ],
            );
            let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
                __terrane_iterable_0.as_slice(),
            );
            loop {
                i = match __terrane_iterator_0.next() {
                    terrane_collection_support::IterationStep::Item(item) => item,
                    terrane_collection_support::IterationStep::End => break,
                };
                let _ = &i;
                println!(
                    "{}", terrane_scalar_support::scalar_text(&count(values.clone()))
                );
            }
            return result.clone();
        })
    };
}
fn test_scenario_terrane_returned_callable_local_loop() {
    let f: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
    >;
    f = make_terrane_returned_callable_local_loop();
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&f(terrane_int_support::Int::from(7_i128)))
    );
}
// Source: tests/integration/static-call-throw-inference/case.trn
// Namespace: static-call-throw-inference
fn make_terrane_static_call_throw_inference(
    value: terrane_int_support::Int,
) -> Result<terrane_int_support::Int, TerraneError> {
    let d: Duration;
    d = __terrane_traced_err(
        Duration::terrane_static_seconds(value.clone()),

            47 /* terrane-site: tests/integration/static-call-throw-inference/case.trn:5:9-5:33 */,
    )?;
    return Ok(d.total_nanoseconds.clone());
}
#[derive(Clone)]
pub struct Maker {}
impl Maker {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn terrane_static_build(
        value: terrane_int_support::Int,
    ) -> Result<terrane_int_support::Int, TerraneError> {
        let d: Duration;
        d = __terrane_traced_err(
            Duration::terrane_static_seconds(value.clone()),

                48 /* terrane-site: tests/integration/static-call-throw-inference/case.trn:10:13-10:37 */,
        )?;
        return Ok(d.total_nanoseconds.clone());
    }
    pub fn terrane_static_accept(
        value: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        return value.clone();
    }
}
fn argument() -> Result<terrane_int_support::Int, TerraneError> {
    return Ok(
        __terrane_traced_err(
            make_terrane_static_call_throw_inference(
                terrane_int_support::Int::from(-1_i128),
            ),

                49 /* terrane-site: tests/integration/static-call-throw-inference/case.trn:17:12-17:20 */,
        )?,
    );
}
fn forward_argument() -> Result<terrane_int_support::Int, TerraneError> {
    return Ok(
        Maker::terrane_static_accept(
            __terrane_traced_err(
                argument(),

                    50 /* terrane-site: tests/integration/static-call-throw-inference/case.trn:20:28-20:37 */,
            )?,
        ),
    );
}
fn test_scenario_terrane_static_call_throw_inference() -> Result<(), TerraneError> {
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_traced_completion!(make_terrane_static_call_throw_inference(terrane_int_support::Int::from(-
                1_i128)), 51 /* terrane-site: tests/integration/static-call-throw-inference/case.trn:24:17-24:25 */))
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
                        == TerraneErrorKind::Custom(DescriptorId(8))
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("caught-free"));
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
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_traced_completion!(Maker::terrane_static_build(terrane_int_support::Int::from(-
                1_i128)), 52 /* terrane-site: tests/integration/static-call-throw-inference/case.trn:28:17-28:33 */))
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
                        == TerraneErrorKind::Custom(DescriptorId(8))
                {
                    __terrane_handled_1 = true;
                    println!("{}", String::from("caught-static"));
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
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    let __terrane_completion_2: TerraneCompletion<()> = (|| {
        let __terrane_try_2: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_traced_completion!(forward_argument(),
                53 /* terrane-site: tests/integration/static-call-throw-inference/case.trn:32:17-32:34 */))
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
                    && __terrane_error_2.kind
                        == TerraneErrorKind::Custom(DescriptorId(8))
                {
                    __terrane_handled_2 = true;
                    println!("{}", String::from("caught-argument"));
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
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_traced_err(Maker::terrane_static_build(terrane_int_support::Int::from(2_i128)),
        54 /* terrane-site: tests/integration/static-call-throw-inference/case.trn:35:13-35:28 */) ?)
    );
    Ok(())
}
// Source: tests/integration/switch-boundaries/case.trn
// Namespace: switch-boundaries
#[derive(Clone)]
pub struct TerraneNsSwitchBoundariesMarkerIdentity2F7377697463682D626F756E646172696573006D61726B6572 {
    __terrane_lifetime: std::sync::Arc<()>,
    pub name: String,
}
impl TerraneNsSwitchBoundariesMarkerIdentity2F7377697463682D626F756E646172696573006D61726B6572 {
    pub fn terrane_construct(name: String) -> Self {
        let mut __terrane_constructed_value = Self {
            name: String::new(),
            __terrane_lifetime: std::sync::Arc::new(()),
        };
        __terrane_constructed_value.construct(name);
        __terrane_constructed_value
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn construct(&mut self, name: String) {
        {
            let __terrane_value: String = name;
            self.name = __terrane_value;
        }
    }
    pub fn to_string(&self) -> String {
        return self.name.clone();
    }
    pub fn destruct(&mut self) {
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
            String::with_capacity(5); __terrane_text.push_str("drop:");
            terrane_scalar_support::ScalarDisplay::write_scalar(&self.name, &mut
            __terrane_text); __terrane_text })
        );
    }
}
impl Drop
for TerraneNsSwitchBoundariesMarkerIdentity2F7377697463682D626F756E646172696573006D61726B6572 {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) != 1 {
            return;
        }
        self.destruct();
    }
}
#[derive(Clone)]
pub struct RequiredValue {
    pub text: String,
}
impl RequiredValue {
    pub fn terrane_construct(text: String) -> Self {
        let mut __terrane_constructed_value = Self { text: String::new() };
        __terrane_constructed_value.construct(text);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, text: String) {
        {
            let __terrane_value: String = text;
            self.text = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct Configured {
    pub slot: Option<RequiredValue>,
}
impl Configured {
    pub fn terrane_construct(value: String) -> Self {
        let mut __terrane_constructed_value = Self { slot: None };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: String) {
        let __terrane_switch_value_429 = &value;
        if __terrane_switch_value_429.as_str() == "a" {
            {
                let __terrane_value: RequiredValue = RequiredValue::terrane_construct(
                    String::from("first"),
                );
                self.slot = Some(__terrane_value);
            }
        } else {
            {
                let __terrane_value: RequiredValue = RequiredValue::terrane_construct(
                    String::from("other"),
                );
                self.slot = Some(__terrane_value);
            }
        }
    }
}
fn consume(
    item: TerraneNsSwitchBoundariesMarkerIdentity2F7377697463682D626F756E646172696573006D61726B6572,
) -> bool {
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(9); __terrane_text.push_str("consumed:"); __terrane_text
        .push_str(&item.to_string()); __terrane_text })
    );
    return false;
}
fn absent() -> Option<String> {
    println!("{}", String::from("none-expression"));
    return None;
}
fn resource_case(ready: bool) {
    let item: TerraneNsSwitchBoundariesMarkerIdentity2F7377697463682D626F756E646172696573006D61726B6572;
    item = TerraneNsSwitchBoundariesMarkerIdentity2F7377697463682D626F756E646172696573006D61726B6572::terrane_construct(
        String::from("retained"),
    );
    if ready {
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
            String::with_capacity(9); __terrane_text.push_str("resource:");
            __terrane_text.push_str(&item.to_string()); __terrane_text })
        );
        println!("{}", item.to_string());
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
            String::with_capacity(0); __terrane_text.push_str(""); __terrane_text
            .push_str(&item.to_string()); __terrane_text })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_separator = &"|";
            let mut __terrane_text = String::with_capacity(1); __terrane_text
            .push_str(&item.to_string()); __terrane_text.push_str(__terrane_separator);
            __terrane_text.push_str(&item.to_string()); __terrane_text })
        );
    } else if consume(item) {
        println!("{}", String::from("wrong"));
    }
}
fn test_scenario_terrane_switch_boundaries() {
    let selector: u8;
    let other: u16;
    let first: Configured;
    let second: Configured;
    selector = 255;
    other = 511;
    let __terrane_switch_value_1149 = &selector;
    if &(*__terrane_switch_value_1149 as u16) == &other {
        println!("{}", String::from("wrong-truncated"));
    } else if __terrane_switch_value_1149 == &255 {
        println!("{}", String::from("wide-comparison"));
    }
    let __terrane_switch_value_1279 = &absent();
    if {
        let _ = ();
        __terrane_switch_value_1279.is_none()
    } {
        println!("{}", String::from("optional-none"));
    }
    let __terrane_switch_value_1353 = &();
    if absent().is_none() {
        println!("{}", String::from("none-both"));
    }
    first = Configured::terrane_construct(String::from("a"));
    second = Configured::terrane_construct(String::from("b"));
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&first.slot.as_ref()
        .expect("required field initialized").text), String::from("|"),
        terrane_scalar_support::scalar_text(&second.slot.as_ref()
        .expect("required field initialized").text)
    );
    resource_case(true);
}
// Source: <compiler-generated test registry>
// Namespace: core/testing/runner
fn __terrane_test_hex(value: &str) -> String {
    use std::fmt::Write as _;
    let mut encoded = String::with_capacity(value.len() * 2);
    for byte in value.as_bytes() {
        write!(encoded, "{byte:02x}").expect("writing to a string cannot fail");
    }
    encoded
}
fn __terrane_test_record(outcome: &str, error: &TerraneError) {
    let Ok(path) = std::env::var("TERRANE_TEST_RESULT") else {
        return;
    };
    let details = error.structured_details();
    let frames = error.source_frames();
    let mut record = format!(
        "1\n{}\n{}\n{}\n{}\n{}\n", outcome, __terrane_test_hex(&error.descriptor_name()),
        __terrane_test_hex(error.message()), details.len(), frames.len(),
    );
    for detail in details {
        record.push_str(&__terrane_test_hex(detail));
        record.push('\n');
    }
    for frame in frames {
        record.push_str(&__terrane_test_hex(&frame));
        record.push('\n');
    }
    let _ = std::fs::write(path, record);
}
fn main() {
    let selected = std::env::args().nth(1).unwrap_or_default();
    match selected.as_str() {
        "0" => {
            test_scenario_terrane_catch_binding_function_scope();
        }
        "1" => {
            test_scenario_terrane_catch_finally_lexical_scopes();
        }
        "2" => {
            test_scenario_terrane_finally_control_flow();
        }
        "3" => {
            test_scenario_terrane_finally_error_replacement();
        }
        "4" => {
            test_scenario_terrane_finally_throwable_replacement();
        }
        "5" => {
            if let Err(error) = test_scenario_terrane_fizz_buzz() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "6" => {
            test_scenario_terrane_for_loops();
        }
        "7" => {
            test_scenario_terrane_function_scope_assignment_destination();
        }
        "8" => {
            test_scenario_terrane_function_scope_boundaries();
        }
        "9" => {
            test_scenario_terrane_function_scope_catch_value();
        }
        "10" => {
            test_scenario_terrane_function_scope_control_equivalence();
        }
        "11" => {
            test_scenario_terrane_function_scope_partial_inferred();
        }
        "12" => {
            test_scenario_terrane_function_scope_partial_typed();
        }
        "13" => {
            test_scenario_terrane_function_scope_release();
        }
        "14" => {
            test_scenario_terrane_function_scope_zero_loop();
        }
        "15" => {
            test_scenario_terrane_grouped_precedence();
        }
        "16" => {
            test_scenario_terrane_grouped_prefix_statements();
        }
        "17" => {
            test_scenario_terrane_identity_evaluation();
        }
        "18" => {
            __terrane_run(async move {
                test_scenario_terrane_layout_calls().await;
            });
        }
        "19" => {
            test_scenario_terrane_lexical_replacement();
        }
        "20" => {
            test_scenario_terrane_local_function_lexical_boundaries();
        }
        "21" => {
            test_scenario_terrane_local_function_results();
        }
        "22" => {
            test_scenario_terrane_local_function_self_static_member();
        }
        "23" => {
            test_scenario_terrane_loop_binder_owner_rebind();
        }
        "24" => {
            test_scenario_terrane_conformance_loop_body_backedge_arithmetic();
        }
        "25" => {
            test_scenario_terrane_conformance_loop_body_backedge_for();
        }
        "26" => {
            test_scenario_terrane_conformance_loop_body_backedge_inferred_rhs();
        }
        "27" => {
            test_scenario_terrane_conformance_loop_body_backedge_while();
        }
        "28" => {
            test_scenario_terrane_loop_shadowing_scope();
        }
        "29" => {
            test_scenario_terrane_mutable_loop_loan_boundaries();
        }
        "30" => {
            test_scenario_terrane_name_aliasing();
        }
        "31" => {
            test_scenario_terrane_nested_collection_for_binding();
        }
        "32" => {
            test_scenario_terrane_reference_reassign_reaching_assignments();
        }
        "33" => {
            test_scenario_terrane_reference_return_reassigned_parameter();
        }
        "34" => {
            test_scenario_terrane_return_catch_class_in_loop();
        }
        "35" => {
            test_scenario_terrane_return_catch_class_no_later_read();
        }
        "36" => {
            test_scenario_terrane_return_catch_class_read_in_catch();
        }
        "37" => {
            test_scenario_terrane_return_catch_class_then_read();
        }
        "38" => {
            test_scenario_terrane_return_catch_list_then_read();
        }
        "39" => {
            test_scenario_terrane_return_catch_string_then_read();
        }
        "40" => {
            test_scenario_terrane_return_finally_class_read();
        }
        "41" => {
            test_scenario_terrane_return_plain_return_in_try_class();
        }
        "42" => {
            if let Err(error) = test_scenario_terrane_return_tail_block_strings() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "43" => {
            test_scenario_terrane_returned_callable_local_loop();
        }
        "44" => {
            if let Err(error) = test_scenario_terrane_static_call_throw_inference() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "45" => {
            test_scenario_terrane_switch_boundaries();
        }
        _ => {
            eprintln!("unknown Terrane test case");
            std::process::exit(103);
        }
    }
}
