// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn update_twice(mutator: &mut Mutator) -> Result<u8, TerraneError> {
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        terrane_int_support::Int::from(2_i128); let __terrane_checked_native_input_0 =
        __terrane_raised_err(| | -> Result < _, crate ::TerraneForeignError > {
        Ok(terrane_int_support::coerce:: < u8 > (&__terrane_conversion_input_0).map_err(|
        error | crate ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ?) } (), 0 /* terrane-site: src/main.trn:8:29-8:30 */) ?;
        __terrane_raised_err(| | -> Result < _, crate ::TerraneForeignError > { match
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(< witness::Mutator as provider::MutableValue >
        ::update(&mut * mutator, __terrane_checked_native_input_0)) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "native-default-witness", "witness::Mutator::update")) } { Ok(value) =>
        Ok(terrane_int_support::Int::from_u128(value as u128)), Err(error) => Err(crate
        ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
        ::TERRANE_DEPENDENCY_ERROR,
        format!("Rust dependency `{}` member `{}` failed: {error}",
        "native-default-witness", "witness::Mutator::update"), crate ::TERRANE_NO_SITE)))
        } } (), 1 /* terrane-site: src/main.trn:8:13-8:30 */) ? })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        terrane_int_support::Int::from(3_i128); let __terrane_checked_native_input_0 =
        __terrane_raised_err(| | -> Result < _, crate ::TerraneForeignError > {
        Ok(terrane_int_support::coerce:: < u8 > (&__terrane_conversion_input_0).map_err(|
        error | crate ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ?) } (), 2 /* terrane-site: src/main.trn:9:29-9:30 */) ?;
        __terrane_raised_err(| | -> Result < _, crate ::TerraneForeignError > { match
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(< witness::Mutator as provider::MutableValue >
        ::update(&mut * mutator, __terrane_checked_native_input_0)) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "native-default-witness", "witness::Mutator::update")) } { Ok(value) =>
        Ok(terrane_int_support::Int::from_u128(value as u128)), Err(error) => Err(crate
        ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
        ::TERRANE_DEPENDENCY_ERROR,
        format!("Rust dependency `{}` member `{}` failed: {error}",
        "native-default-witness", "witness::Mutator::update"), crate ::TERRANE_NO_SITE)))
        } } (), 3 /* terrane-site: src/main.trn:9:13-9:30 */) ? })
    );
    return Ok(
        __terrane_raised_err(
            terrane_int_support::coerce::<
                u8,
            >(
                &__terrane_raised_err(
                    || -> Result<_, crate::TerraneForeignError> {
                        {
                            let value = match std::panic::catch_unwind(
                                std::panic::AssertUnwindSafe(|| -> Result<
                                    _,
                                    crate::TerraneForeignError,
                                > { Ok((&*mutator).value()) }),
                            ) {
                                Ok(result) => result?,
                                Err(payload) => {
                                    return Err(
                                        crate::__terrane_dependency_panic(
                                            payload,
                                            "native-default-witness",
                                            "witness::Mutator::value",
                                        ),
                                    );
                                }
                            };
                            Ok::<
                                _,
                                crate::TerraneForeignError,
                            >(terrane_int_support::Int::from_u128(value as u128))
                        }
                    }(),
                    4 /* terrane-site: src/main.trn:10:12-10:26 */,
                )?,
            ),
            4 /* terrane-site: src/main.trn:10:12-10:26 */,
        )?,
    );
}
#[allow(dead_code)]
fn generic_panic_attributed(value: witness::Wrapped<u8>) -> bool {
    let mut wide: Option<u32> = None;
    let mut error: Option<TerraneError> = None;
    let __terrane_completion_0: TerraneCompletion<bool> = (|| {
        let __terrane_try_0: TerraneCompletion<bool> = (|| {
            let _ = wide
                .insert(
                    __terrane_raised_completion!(
                        | | -> Result < _, crate ::TerraneForeignError > { { let value =
                        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                        -> Result < _, crate ::TerraneForeignError > { Ok(<
                        witness::Wrapped < u8 > as provider::NativeValue > ::selected:: <
                        u32 > (&value)) })) { Ok(result) => result ?, Err(payload) =>
                        return Err(crate ::__terrane_dependency_panic(payload,
                        "native-default-witness", "witness::Wrapped::selected")) }; Ok::
                        < _, crate ::TerraneForeignError > (value) } } (),
                        5 /* terrane-site: src/main.trn:14:23-14:38 */
                    ),
                );
            return TerraneCompletion::Return(
                *wide.as_ref().expect("flow-proven available binding") == 0,
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
                    let _ = error.insert(__terrane_error_0.clone());
                    return TerraneCompletion::Return(
                        {
                            let __terrane_error_text_0 = error
                                .as_ref()
                                .expect("flow-proven available binding")
                                .message();
                            let __terrane_string_argument_0 = "Rust dependency `native-default-witness`";
                            __terrane_error_text_0
                                .contains(
                                    std::convert::AsRef::<
                                        str,
                                    >::as_ref(&__terrane_string_argument_0),
                                )
                        }
                            && {
                                let __terrane_error_text_1 = error
                                    .as_ref()
                                    .expect("flow-proven available binding")
                                    .message();
                                let __terrane_string_argument_0 = "member `witness::Wrapped::selected`";
                                __terrane_error_text_1
                                    .contains(
                                        std::convert::AsRef::<
                                            str,
                                        >::as_ref(&__terrane_string_argument_0),
                                    )
                            }
                            && {
                                let __terrane_error_text_2 = error
                                    .as_ref()
                                    .expect("flow-proven available binding")
                                    .message();
                                let __terrane_string_argument_0 = "generic selector panic";
                                __terrane_error_text_2
                                    .contains(
                                        std::convert::AsRef::<
                                            str,
                                        >::as_ref(&__terrane_string_argument_0),
                                    )
                            },
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
#[allow(dead_code)]
fn free_panic_attributed() -> bool {
    let mut error: Option<TerraneError> = None;
    let __terrane_completion_1: TerraneCompletion<bool> = (|| {
        let __terrane_try_1: TerraneCompletion<bool> = (|| {
            __terrane_raised_completion!(
                panic_free(), 6 /* terrane-site: src/main.trn:21:9-21:20 */
            );
            return TerraneCompletion::Return(false);
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
                    let _ = error.insert(__terrane_error_1.clone());
                    return TerraneCompletion::Return(
                        {
                            let __terrane_error_text_3 = error
                                .as_ref()
                                .expect("flow-proven available binding")
                                .message();
                            let __terrane_string_argument_0 = "Rust dependency `native-default-witness`";
                            __terrane_error_text_3
                                .contains(
                                    std::convert::AsRef::<
                                        str,
                                    >::as_ref(&__terrane_string_argument_0),
                                )
                        }
                            && {
                                let __terrane_error_text_4 = error
                                    .as_ref()
                                    .expect("flow-proven available binding")
                                    .message();
                                let __terrane_string_argument_0 = "member `witness::panic_free`";
                                __terrane_error_text_4
                                    .contains(
                                        std::convert::AsRef::<
                                            str,
                                        >::as_ref(&__terrane_string_argument_0),
                                    )
                            }
                            && {
                                let __terrane_error_text_5 = error
                                    .as_ref()
                                    .expect("flow-proven available binding")
                                    .message();
                                let __terrane_string_argument_0 = "free function panic";
                                __terrane_error_text_5
                                    .contains(
                                        std::convert::AsRef::<
                                            str,
                                        >::as_ref(&__terrane_string_argument_0),
                                    )
                            },
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
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
#[allow(dead_code)]
fn generic_free_panic_attributed() -> bool {
    let mut payload: Option<u8> = None;
    let mut error: Option<TerraneError> = None;
    let __terrane_completion_2: TerraneCompletion<bool> = (|| {
        let __terrane_try_2: TerraneCompletion<bool> = (|| {
            let _ = payload.insert(9);
            {
                let __terrane_conversion_input_0 = *payload
                    .as_ref()
                    .expect("flow-proven available binding");
                __terrane_raised_completion!(
                    | | -> Result < _, crate ::TerraneForeignError > { { let _ = match
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result <
                    _, crate ::TerraneForeignError > { Ok({ let _ =
                    witness::panic_generic(__terrane_conversion_input_0); }) })) {
                    Ok(result) => result ?, Err(payload) => return Err(crate
                    ::__terrane_dependency_panic(payload, "native-default-witness",
                    "witness::panic_generic")) }; Ok:: < _, crate ::TerraneForeignError >
                    (()) } } (), 7 /* terrane-site: src/main.trn:29:9-29:42 */
                )
            };
            return TerraneCompletion::Return(false);
        })();
        match __terrane_try_2 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_2) => {
                let mut __terrane_handled_2 = false;
                if !__terrane_handled_2 {
                    __terrane_handled_2 = true;
                    let _ = error.insert(__terrane_error_2.clone());
                    return TerraneCompletion::Return(
                        {
                            let __terrane_error_text_6 = error
                                .as_ref()
                                .expect("flow-proven available binding")
                                .message();
                            let __terrane_string_argument_0 = "Rust dependency `native-default-witness`";
                            __terrane_error_text_6
                                .contains(
                                    std::convert::AsRef::<
                                        str,
                                    >::as_ref(&__terrane_string_argument_0),
                                )
                        }
                            && {
                                let __terrane_error_text_7 = error
                                    .as_ref()
                                    .expect("flow-proven available binding")
                                    .message();
                                let __terrane_string_argument_0 = "member `witness::panic_generic`";
                                __terrane_error_text_7
                                    .contains(
                                        std::convert::AsRef::<
                                            str,
                                        >::as_ref(&__terrane_string_argument_0),
                                    )
                            }
                            && {
                                let __terrane_error_text_8 = error
                                    .as_ref()
                                    .expect("flow-proven available binding")
                                    .message();
                                let __terrane_string_argument_0 = "generic free function panic";
                                __terrane_error_text_8
                                    .contains(
                                        std::convert::AsRef::<
                                            str,
                                        >::as_ref(&__terrane_string_argument_0),
                                    )
                            },
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
#[allow(dead_code)]
fn custom_free_panic_attributed() -> bool {
    let mut error: Option<TerraneError> = None;
    let __terrane_completion_3: TerraneCompletion<bool> = (|| {
        let __terrane_try_3: TerraneCompletion<bool> = (|| {
            __terrane_raised_completion!(
                fail_free(), 8 /* terrane-site: src/main.trn:36:9-36:26 */
            );
            return TerraneCompletion::Return(false);
        })();
        match __terrane_try_3 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_3) => {
                let mut __terrane_handled_3 = false;
                if !__terrane_handled_3 {
                    __terrane_handled_3 = true;
                    let _ = error.insert(__terrane_error_3.clone());
                    return TerraneCompletion::Return(
                        {
                            let __terrane_error_text_9 = error
                                .as_ref()
                                .expect("flow-proven available binding")
                                .message();
                            let __terrane_string_argument_0 = "Rust dependency `custom-lib-panic-witness`";
                            __terrane_error_text_9
                                .contains(
                                    std::convert::AsRef::<
                                        str,
                                    >::as_ref(&__terrane_string_argument_0),
                                )
                        }
                            && {
                                let __terrane_error_text_10 = error
                                    .as_ref()
                                    .expect("flow-proven available binding")
                                    .message();
                                let __terrane_string_argument_0 = "::fail_free`";
                                __terrane_error_text_10
                                    .contains(
                                        std::convert::AsRef::<
                                            str,
                                        >::as_ref(&__terrane_string_argument_0),
                                    )
                            }
                            && {
                                let __terrane_error_text_11 = error
                                    .as_ref()
                                    .expect("flow-proven available binding")
                                    .message();
                                let __terrane_string_argument_0 = "custom library free panic";
                                __terrane_error_text_11
                                    .contains(
                                        std::convert::AsRef::<
                                            str,
                                        >::as_ref(&__terrane_string_argument_0),
                                    )
                            },
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
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
#[allow(dead_code)]
fn custom_method_panic_attributed() -> bool {
    let value: Panicker;
    let mut error: Option<TerraneError> = None;
    value = __terrane_raised(
        terrane_static_trn_50616e69636b6572_new(),
        9 /* terrane-site: src/main.trn:42:13-42:33 */,
    );
    let __terrane_completion_4: TerraneCompletion<bool> = (|| {
        let __terrane_try_4: TerraneCompletion<bool> = (|| {
            __terrane_raised_completion!(
                | | -> Result < _, crate ::TerraneForeignError > { { let _ = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok({ let _ = (&value).fail(); }) })) {
                Ok(result) => result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "custom-lib-panic-witness",
                "custom_lib_panic_witness::Panicker::fail")) }; Ok:: < _, crate
                ::TerraneForeignError > (()) } } (), 10 /* terrane-site: src/main.trn:44:9-44:20 */
            );
            return TerraneCompletion::Return(false);
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
                    let _ = error.insert(__terrane_error_4.clone());
                    return TerraneCompletion::Return(
                        {
                            let __terrane_error_text_12 = error
                                .as_ref()
                                .expect("flow-proven available binding")
                                .message();
                            let __terrane_string_argument_0 = "Rust dependency `custom-lib-panic-witness`";
                            __terrane_error_text_12
                                .contains(
                                    std::convert::AsRef::<
                                        str,
                                    >::as_ref(&__terrane_string_argument_0),
                                )
                        }
                            && {
                                let __terrane_error_text_13 = error
                                    .as_ref()
                                    .expect("flow-proven available binding")
                                    .message();
                                let __terrane_string_argument_0 = "::Panicker::fail`";
                                __terrane_error_text_13
                                    .contains(
                                        std::convert::AsRef::<
                                            str,
                                        >::as_ref(&__terrane_string_argument_0),
                                    )
                            }
                            && {
                                let __terrane_error_text_14 = error
                                    .as_ref()
                                    .expect("flow-proven available binding")
                                    .message();
                                let __terrane_string_argument_0 = "custom library instance panic";
                                __terrane_error_text_14
                                    .contains(
                                        std::convert::AsRef::<
                                            str,
                                        >::as_ref(&__terrane_string_argument_0),
                                    )
                            },
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
#[allow(dead_code)]
async fn custom_async_panic_attributed() -> bool {
    let value: Panicker;
    let mut error: Option<TerraneError> = None;
    value = __terrane_raised(
        terrane_static_trn_50616e69636b6572_new(),
        11 /* terrane-site: src/main.trn:50:13-50:33 */,
    );
    let __terrane_completion_5: TerraneCompletion<bool> = async {
        let __terrane_try_5: TerraneCompletion<bool> = async {
            __terrane_traced_completion!(
                __terrane_await({ let __terrane_future = { let __terrane_call = (&value)
                .fail_async(); Box::pin(async move { { let value = crate
                ::__terrane_dependency_await_unwind(__terrane_call). await .map_err(|
                payload | crate ::__terrane_dependency_panic(payload,
                "custom-lib-panic-witness",
                "custom_lib_panic_witness::Panicker::fail_async")) ?; Ok:: < _, crate
                ::TerraneForeignError > (value) } }) }; async move {
                __terrane_raised_err(__terrane_future. await, 12 /* terrane-site: src/main.trn:52:15-52:32 */) } }). await, 12 /* terrane-site: src/main.trn:52:15-52:32 */
            );
            return TerraneCompletion::Return(false);
        }
            .await;
        match __terrane_try_5 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_5) => {
                let mut __terrane_handled_5 = false;
                if !__terrane_handled_5 {
                    __terrane_handled_5 = true;
                    let _ = error.insert(__terrane_error_5.clone());
                    return TerraneCompletion::Return(
                        {
                            let __terrane_error_text_15 = error
                                .as_ref()
                                .expect("flow-proven available binding")
                                .message();
                            let __terrane_string_argument_0 = "Rust dependency `custom-lib-panic-witness`";
                            __terrane_error_text_15
                                .contains(
                                    std::convert::AsRef::<
                                        str,
                                    >::as_ref(&__terrane_string_argument_0),
                                )
                        }
                            && {
                                let __terrane_error_text_16 = error
                                    .as_ref()
                                    .expect("flow-proven available binding")
                                    .message();
                                let __terrane_string_argument_0 = "::Panicker::fail_async`";
                                __terrane_error_text_16
                                    .contains(
                                        std::convert::AsRef::<
                                            str,
                                        >::as_ref(&__terrane_string_argument_0),
                                    )
                            }
                            && {
                                let __terrane_error_text_17 = error
                                    .as_ref()
                                    .expect("flow-proven available binding")
                                    .message();
                                let __terrane_string_argument_0 = "custom library async panic";
                                __terrane_error_text_17
                                    .contains(
                                        std::convert::AsRef::<
                                            str,
                                        >::as_ref(&__terrane_string_argument_0),
                                    )
                            },
                    );
                }
                if !__terrane_handled_5 {
                    return TerraneCompletion::Error(__terrane_error_5);
                }
            }
        }
        TerraneCompletion::Normal
    }
        .await;
    match __terrane_completion_5 {
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
fn main() {
    __terrane_run(async move {
        let seed: u8;
        let value: witness::Wrapped<u8>;
        let owner: u8;
        let selected: u16;
        let overridden: Overridden;
        let explicit_owner: u8;
        let explicit_selected: u16;
        let mut mutator: Mutator;
        let mut error: Option<TerraneError> = None;
        seed = 9;
        value = {
            let __terrane_conversion_input_0 = seed;
            __terrane_raised(
                || -> Result<_, crate::TerraneForeignError> {
                    Ok(witness::Wrapped::<u8> {
                        0: __terrane_conversion_input_0,
                    })
                }(),
                13 /* terrane-site: src/main.trn:59:30-59:52 */,
            )
        };
        owner = __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                <witness::Wrapped<
                                    u8,
                                > as provider::NativeValue>::owner_value(&value),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "native-default-witness",
                                    "witness::Wrapped::owner_value",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),
            14 /* terrane-site: src/main.trn:60:19-60:37 */,
        );
        selected = __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                <witness::Wrapped<
                                    u8,
                                > as provider::NativeValue>::selected::<u16>(&value),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "native-default-witness",
                                    "witness::Wrapped::selected",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),
            15 /* terrane-site: src/main.trn:61:23-61:38 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
            String::with_capacity(2); __terrane_text.push_str("");
            terrane_scalar_support::ScalarDisplay::write_scalar(&owner, &mut
            __terrane_text); __terrane_text.push_str(":");
            terrane_scalar_support::ScalarDisplay::write_scalar(&selected, &mut
            __terrane_text); __terrane_text.push_str(":");
            terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_raised(| | ->
            Result < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(< witness::Wrapped < u8 > as
            provider::NativeValue > ::label(&value)) })) { Ok(result) => result ?,
            Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "native-default-witness", "witness::Wrapped::label")) }; Ok:: < _, crate
            ::TerraneForeignError > (value.to_owned()) } } (), 16 /* terrane-site: src/main.trn:62:49-62:61 */), &mut __terrane_text); __terrane_text })
        );
        overridden = __terrane_raised(
            terrane_static_trn_4f76657272696464656e_new(),
            17 /* terrane-site: src/main.trn:64:18-64:34 */,
        );
        explicit_owner = __terrane_raised(
            terrane_int_support::coerce::<
                u8,
            >(
                &__terrane_raised(
                    || -> Result<_, crate::TerraneForeignError> {
                        {
                            let value = match std::panic::catch_unwind(
                                std::panic::AssertUnwindSafe(|| -> Result<
                                    _,
                                    crate::TerraneForeignError,
                                > {
                                    Ok(
                                        <witness::Overridden as provider::NativeValue>::owner_value(
                                            &overridden,
                                        ),
                                    )
                                }),
                            ) {
                                Ok(result) => result?,
                                Err(payload) => {
                                    return Err(
                                        crate::__terrane_dependency_panic(
                                            payload,
                                            "native-default-witness",
                                            "witness::Overridden::owner_value",
                                        ),
                                    );
                                }
                            };
                            Ok::<
                                _,
                                crate::TerraneForeignError,
                            >(terrane_int_support::Int::from_u128(value as u128))
                        }
                    }(),
                    18 /* terrane-site: src/main.trn:65:28-65:51 */,
                ),
            ),
            18 /* terrane-site: src/main.trn:65:28-65:51 */,
        );
        explicit_selected = __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                <witness::Overridden as provider::NativeValue>::selected::<
                                    u16,
                                >(&overridden),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "native-default-witness",
                                    "witness::Overridden::selected",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),
            19 /* terrane-site: src/main.trn:66:32-66:52 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
            String::with_capacity(2); __terrane_text.push_str("");
            terrane_scalar_support::ScalarDisplay::write_scalar(&explicit_owner, &mut
            __terrane_text); __terrane_text.push_str(":");
            terrane_scalar_support::ScalarDisplay::write_scalar(&explicit_selected, &mut
            __terrane_text); __terrane_text.push_str(":");
            terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_raised(| | ->
            Result < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(< witness::Overridden as
            provider::NativeValue > ::label(&overridden)) })) { Ok(result) => result ?,
            Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "native-default-witness", "witness::Overridden::label")) }; Ok:: < _, crate
            ::TerraneForeignError > (value.to_owned()) } } (), 20 /* terrane-site: src/main.trn:67:67-67:84 */), &mut __terrane_text); __terrane_text })
        );
        mutator = {
            let __terrane_conversion_input_0 = terrane_int_support::Int::from(10_i128);
            __terrane_raised(
                terrane_static_trn_4d757461746f72_new(__terrane_conversion_input_0),
                21 /* terrane-site: src/main.trn:69:15-69:31 */,
            )
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(update_twice(&mut
            mutator), 22 /* terrane-site: src/main.trn:70:13-70:34 */))
        );
        let __terrane_completion_6: TerraneCompletion<()> = (|| {
            let __terrane_try_6: TerraneCompletion<()> = (|| {
                {
                    let __terrane_conversion_input_0 = terrane_int_support::Int::from(
                        0_i128,
                    );
                    let __terrane_checked_native_input_0 = __terrane_raised_completion!(
                        | | -> Result < _, crate ::TerraneForeignError > {
                        Ok(terrane_int_support::coerce:: < u8 >
                        (&__terrane_conversion_input_0).map_err(| error | crate
                        ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
                        ::TERRANE_NO_SITE))) ?) } (), 23 /* terrane-site: src/main.trn:72:25-72:26 */
                    );
                    __terrane_raised_completion!(
                        | | -> Result < _, crate ::TerraneForeignError > { match match
                        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
                        Result < _, crate ::TerraneForeignError > { Ok(< witness::Mutator
                        as provider::MutableValue > ::update(&mut mutator,
                        __terrane_checked_native_input_0)) })) { Ok(result) => result ?,
                        Err(payload) => return Err(crate
                        ::__terrane_dependency_panic(payload, "native-default-witness",
                        "witness::Mutator::update")) } { Ok(value) =>
                        Ok(terrane_int_support::Int::from_u128(value as u128)),
                        Err(error) => Err(crate ::TerraneForeignError(crate
                        ::TerraneError::custom_raised(crate ::TERRANE_DEPENDENCY_ERROR,
                        format!("Rust dependency `{}` member `{}` failed: {error}",
                        "native-default-witness", "witness::Mutator::update"), crate
                        ::TERRANE_NO_SITE))) } } (), 24 /* terrane-site: src/main.trn:72:9-72:26 */
                    )
                };
                TerraneCompletion::Normal
            })();
            match __terrane_try_6 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_6) => {
                    let mut __terrane_handled_6 = false;
                    if !__terrane_handled_6 {
                        __terrane_handled_6 = true;
                        let _ = error.insert(__terrane_error_6.clone());
                        println!(
                            "{}", terrane_scalar_support::scalar_text(&{ let mut
                            __terrane_text = String::with_capacity(21); __terrane_text
                            .push_str("native error caught: ");
                            terrane_scalar_support::ScalarDisplay::write_scalar(&{ let
                            __terrane_error_text_18 = error.as_ref()
                            .expect("flow-proven available binding").message(); let
                            __terrane_string_argument_0 = "zero update rejected";
                            __terrane_error_text_18.contains(std::convert::AsRef:: < str
                            > ::as_ref(&__terrane_string_argument_0)) }, &mut
                            __terrane_text); __terrane_text })
                        );
                    }
                    if !__terrane_handled_6 {
                        return TerraneCompletion::Error(__terrane_error_6);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        match __terrane_completion_6 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
    });
}
