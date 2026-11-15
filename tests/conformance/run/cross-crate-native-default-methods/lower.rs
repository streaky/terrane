// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn update_twice(mutator: &mut Mutator) -> Result<u8, TerraneError> {
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        terrane_int_support::Int::from(2_i128); __terrane_raised_err(| | -> Result < _,
        crate ::TerraneForeignError > { match match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(< witness::Mutator as provider::MutableValue >
        ::update(&mut * mutator, terrane_int_support::coerce:: < u8 >
        (&__terrane_conversion_input_0).map_err(| error | crate
        ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ?)) })) { Ok(result) => result ?, Err(payload) => return
        Err(crate ::__terrane_dependency_panic(payload, "native-default-witness",
        "witness::Mutator::update")) } { Ok(value) =>
        Ok(terrane_int_support::Int::from_u128(value as u128)), Err(error) => Err(crate
        ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
        ::TERRANE_DEPENDENCY_ERROR,
        format!("Rust dependency `{}` member `{}` failed: {error}",
        "native-default-witness", "witness::Mutator::update"), crate ::TERRANE_NO_SITE)))
        } } (), 0 /* terrane-site: src/main.trn:7:13-7:30 */) ? })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        terrane_int_support::Int::from(3_i128); __terrane_raised_err(| | -> Result < _,
        crate ::TerraneForeignError > { match match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(< witness::Mutator as provider::MutableValue >
        ::update(&mut * mutator, terrane_int_support::coerce:: < u8 >
        (&__terrane_conversion_input_0).map_err(| error | crate
        ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ?)) })) { Ok(result) => result ?, Err(payload) => return
        Err(crate ::__terrane_dependency_panic(payload, "native-default-witness",
        "witness::Mutator::update")) } { Ok(value) =>
        Ok(terrane_int_support::Int::from_u128(value as u128)), Err(error) => Err(crate
        ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
        ::TERRANE_DEPENDENCY_ERROR,
        format!("Rust dependency `{}` member `{}` failed: {error}",
        "native-default-witness", "witness::Mutator::update"), crate ::TERRANE_NO_SITE)))
        } } (), 1 /* terrane-site: src/main.trn:8:13-8:30 */) ? })
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
                    2 /* terrane-site: src/main.trn:9:12-9:26 */,
                )?,
            ),
            2 /* terrane-site: src/main.trn:9:12-9:26 */,
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
                        3 /* terrane-site: src/main.trn:13:23-13:38 */
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
                panic_free(), 4 /* terrane-site: src/main.trn:20:9-20:20 */
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
                    (()) } } (), 5 /* terrane-site: src/main.trn:28:9-28:42 */
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
fn main() {
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
            6 /* terrane-site: src/main.trn:35:30-35:52 */,
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
        7 /* terrane-site: src/main.trn:36:19-36:37 */,
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
        8 /* terrane-site: src/main.trn:37:23-37:38 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(2); __terrane_text.push_str("");
        terrane_scalar_support::ScalarDisplay::write_scalar(&owner, &mut __terrane_text);
        __terrane_text.push_str(":");
        terrane_scalar_support::ScalarDisplay::write_scalar(&selected, &mut
        __terrane_text); __terrane_text.push_str(":");
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_raised(| | ->
        Result < _, crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(< witness::Wrapped < u8 > as provider::NativeValue >
        ::label(&value)) })) { Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "native-default-witness",
        "witness::Wrapped::label")) }; Ok:: < _, crate ::TerraneForeignError > (value
        .to_owned()) } } (), 9 /* terrane-site: src/main.trn:38:49-38:61 */), &mut
        __terrane_text); __terrane_text })
    );
    overridden = __terrane_raised(
        terrane_static_trn_4f76657272696464656e_new(),
        10 /* terrane-site: src/main.trn:40:18-40:34 */,
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
                11 /* terrane-site: src/main.trn:41:28-41:51 */,
            ),
        ),
        11 /* terrane-site: src/main.trn:41:28-41:51 */,
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
        12 /* terrane-site: src/main.trn:42:32-42:52 */,
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
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(< witness::Overridden as provider::NativeValue >
        ::label(&overridden)) })) { Ok(result) => result ?, Err(payload) => return
        Err(crate ::__terrane_dependency_panic(payload, "native-default-witness",
        "witness::Overridden::label")) }; Ok:: < _, crate ::TerraneForeignError > (value
        .to_owned()) } } (), 13 /* terrane-site: src/main.trn:43:67-43:84 */), &mut
        __terrane_text); __terrane_text })
    );
    mutator = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(10_i128);
        __terrane_raised(
            terrane_static_trn_4d757461746f72_new(__terrane_conversion_input_0),
            14 /* terrane-site: src/main.trn:45:15-45:31 */,
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced(update_twice(&mut
        mutator), 15 /* terrane-site: src/main.trn:46:13-46:34 */))
    );
    let __terrane_completion_3: TerraneCompletion<()> = (|| {
        let __terrane_try_3: TerraneCompletion<()> = (|| {
            {
                let __terrane_conversion_input_0 = terrane_int_support::Int::from(
                    0_i128,
                );
                __terrane_raised_completion!(
                    | | -> Result < _, crate ::TerraneForeignError > { match match
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result <
                    _, crate ::TerraneForeignError > { Ok(< witness::Mutator as
                    provider::MutableValue > ::update(&mut mutator,
                    terrane_int_support::coerce:: < u8 > (&__terrane_conversion_input_0)
                    .map_err(| error | crate ::TerraneForeignError(crate
                    ::TerraneRaised::raised(error, crate ::TERRANE_NO_SITE))) ?)) })) {
                    Ok(result) => result ?, Err(payload) => return Err(crate
                    ::__terrane_dependency_panic(payload, "native-default-witness",
                    "witness::Mutator::update")) } { Ok(value) =>
                    Ok(terrane_int_support::Int::from_u128(value as u128)), Err(error) =>
                    Err(crate ::TerraneForeignError(crate
                    ::TerraneError::custom_raised(crate ::TERRANE_DEPENDENCY_ERROR,
                    format!("Rust dependency `{}` member `{}` failed: {error}",
                    "native-default-witness", "witness::Mutator::update"), crate
                    ::TERRANE_NO_SITE))) } } (), 16 /* terrane-site: src/main.trn:48:9-48:26 */
                )
            };
            TerraneCompletion::Normal
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
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&{ let mut
                        __terrane_text = String::with_capacity(21); __terrane_text
                        .push_str("native error caught: ");
                        terrane_scalar_support::ScalarDisplay::write_scalar(&{ let
                        __terrane_error_text_9 = error.as_ref()
                        .expect("flow-proven available binding").message(); let
                        __terrane_string_argument_0 = "zero update rejected";
                        __terrane_error_text_9.contains(std::convert::AsRef:: < str >
                        ::as_ref(&__terrane_string_argument_0)) }, &mut __terrane_text);
                        __terrane_text })
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
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
