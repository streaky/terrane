// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn make_mark(text: String) -> Result<Mark, TerraneError> {
    return Ok({
        let __terrane_conversion_input_0 = text;
        __terrane_raised_err(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(terrane_recursive_descriptor_witness::Mark {
                    mark: __terrane_conversion_input_0
                        .parse::<char>()
                        .map_err(|_| crate::TerraneForeignError(
                            crate::TerraneError::raised_with_message(
                                crate::TerraneErrorKind::CoercionError,
                                "projected `char` requires exactly one Unicode scalar",
                                crate::TERRANE_NO_SITE,
                            ),
                        ))?,
                })
            }(),
            0 /* terrane-site: src/main.trn:10:12-10:38 */,
        )?
    });
}
fn make_count(value: terrane_int_support::Int) -> Result<Count, TerraneError> {
    return Ok({
        let __terrane_conversion_input_0 = value.clone();
        __terrane_raised_err(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(terrane_recursive_descriptor_witness::Count {
                    n: terrane_int_support::coerce::<i64>(&__terrane_conversion_input_0)
                        .map_err(|error| crate::TerraneForeignError(
                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                        ))?,
                })
            }(),
            1 /* terrane-site: src/main.trn:12:12-12:37 */,
        )?
    });
}
fn make_nested(
    marks: terrane_collection_support::List<Option<String>>,
) -> Result<Nested, TerraneError> {
    return Ok({
        let __terrane_conversion_input_0 = marks;
        __terrane_raised_err(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(terrane_recursive_descriptor_witness::Nested {
                    marks: __terrane_conversion_input_0
                        .into_iter()
                        .map(|item| -> Result<_, crate::TerraneForeignError> {
                            Ok(
                                item
                                    .map(|value| -> Result<char, crate::TerraneForeignError> {
                                        Ok(
                                            value
                                                .parse::<char>()
                                                .map_err(|_| crate::TerraneForeignError(
                                                    crate::TerraneError::raised_with_message(
                                                        crate::TerraneErrorKind::CoercionError,
                                                        "projected `char` requires exactly one Unicode scalar",
                                                        crate::TERRANE_NO_SITE,
                                                    ),
                                                ))?,
                                        )
                                    })
                                    .transpose()?,
                            )
                        })
                        .collect::<Result<Vec<Option<char>>, _>>()?,
                })
            }(),
            2 /* terrane-site: src/main.trn:14:12-14:42 */,
        )?
    });
}
fn empty_mark() -> OptionalMark {
    return {
        let __terrane_conversion_input_0 = None;
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(terrane_recursive_descriptor_witness::OptionalMark {
                    mark: {
                        let __terrane_field_argument: Option<String> = __terrane_conversion_input_0;
                        __terrane_field_argument
                    }
                        .map(|value| -> Result<char, crate::TerraneForeignError> {
                            Ok(
                                value
                                    .parse::<char>()
                                    .map_err(|_| crate::TerraneForeignError(
                                        crate::TerraneError::raised_with_message(
                                            crate::TerraneErrorKind::CoercionError,
                                            "projected `char` requires exactly one Unicode scalar",
                                            crate::TERRANE_NO_SITE,
                                        ),
                                    ))?,
                            )
                        })
                        .transpose()?
                        .into(),
                })
            }(),
            3 /* terrane-site: src/main.trn:16:12-16:46 */,
        )
    };
}
fn missing_label() -> Result<String, TerraneError> {
    return Err(
        TerraneError::raised(
            TerraneErrorKind::CoercionError,
            4 /* terrane-site: src/main.trn:18:5-18:41 */,
        ),
    );
}
fn macro_value(
    value: terrane_int_support::Int,
) -> Result<terrane_int_support::Int, TerraneError> {
    return Ok(
        __terrane_raised_err(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok({
                                let __terrane_macro_result: i64 = core::convert::Into::into(
                                    terrane_recursive_descriptor_witness::native_int!(
                                        terrane_int_support::coerce::< i64 > (&value).map_err(|
                                        error | crate ::TerraneForeignError(crate
                                        ::TerraneRaised::raised(error, crate ::TERRANE_NO_SITE))) ?
                                    ),
                                );
                                __terrane_macro_result
                            })
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane-recursive-descriptor-witness",
                                    "terrane_recursive_descriptor_witness::native_int",
                                ),
                            );
                        }
                    };
                    Ok::<
                        _,
                        crate::TerraneForeignError,
                    >(terrane_int_support::Int::from(i128::from(value)))
                }
            }(),
            5 /* terrane-site: src/main.trn:20:12-20:29 */,
        )?,
    );
}
fn literal_count() -> Count {
    return {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(7_i128);
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(terrane_recursive_descriptor_witness::Count {
                    n: terrane_int_support::coerce::<i64>(&__terrane_conversion_input_0)
                        .map_err(|error| crate::TerraneForeignError(
                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                        ))?,
                })
            }(),
            6 /* terrane-site: src/main.trn:22:12-22:39 */,
        )
    };
}
fn literal_mark() -> Mark {
    return {
        let __terrane_conversion_input_0 = String::from("A");
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(terrane_recursive_descriptor_witness::Mark {
                    mark: __terrane_conversion_input_0
                        .parse::<char>()
                        .map_err(|_| crate::TerraneForeignError(
                            crate::TerraneError::raised_with_message(
                                crate::TerraneErrorKind::CoercionError,
                                "projected `char` requires exactly one Unicode scalar",
                                crate::TERRANE_NO_SITE,
                            ),
                        ))?,
                })
            }(),
            7 /* terrane-site: src/main.trn:24:12-24:39 */,
        )
    };
}
fn escaped_mark() -> Mark {
    return {
        let __terrane_conversion_input_0 = String::from("\n");
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(terrane_recursive_descriptor_witness::Mark {
                    mark: __terrane_conversion_input_0
                        .parse::<char>()
                        .map_err(|_| crate::TerraneForeignError(
                            crate::TerraneError::raised_with_message(
                                crate::TerraneErrorKind::CoercionError,
                                "projected `char` requires exactly one Unicode scalar",
                                crate::TERRANE_NO_SITE,
                            ),
                        ))?,
                })
            }(),
            8 /* terrane-site: src/main.trn:26:12-26:38 */,
        )
    };
}
fn literal_optional() -> OptionalMark {
    return {
        let __terrane_conversion_input_0 = Some(String::from("Ω"));
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(terrane_recursive_descriptor_witness::OptionalMark {
                    mark: {
                        let __terrane_field_argument: Option<String> = __terrane_conversion_input_0;
                        __terrane_field_argument
                    }
                        .map(|value| -> Result<char, crate::TerraneForeignError> {
                            Ok(
                                value
                                    .parse::<char>()
                                    .map_err(|_| crate::TerraneForeignError(
                                        crate::TerraneError::raised_with_message(
                                            crate::TerraneErrorKind::CoercionError,
                                            "projected `char` requires exactly one Unicode scalar",
                                            crate::TERRANE_NO_SITE,
                                        ),
                                    ))?,
                            )
                        })
                        .transpose()?
                        .into(),
                })
            }(),
            9 /* terrane-site: src/main.trn:28:12-28:45 */,
        )
    };
}
fn main() {
    __terrane_run(async move {
        let empty: Nested;
        let owner: Count;
        let mut converted: Option<terrane_int_support::Int> = None;
        let mut options: NativeOptions;
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = __terrane_traced(make_mark(String::from("A")), 10 /* terrane-site: src/main.trn:30:25-30:39 */);
            __terrane_raised(mark_code(__terrane_conversion_input_0),
            11 /* terrane-site: src/main.trn:30:13-30:40 */) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = __terrane_traced(make_count(terrane_int_support::Int::from(42_i128)),
            12 /* terrane-site: src/main.trn:31:27-31:41 */);
            __terrane_raised(count_value(__terrane_conversion_input_0),
            13 /* terrane-site: src/main.trn:31:13-31:42 */) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = empty_mark(); __terrane_raised(mark_or_zero(__terrane_conversion_input_0),
            14 /* terrane-site: src/main.trn:32:13-32:40 */) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = literal_count();
            __terrane_raised(count_value(__terrane_conversion_input_0),
            15 /* terrane-site: src/main.trn:33:13-33:42 */) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = literal_mark(); __terrane_raised(mark_code(__terrane_conversion_input_0),
            16 /* terrane-site: src/main.trn:34:13-34:39 */) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = escaped_mark(); __terrane_raised(mark_code(__terrane_conversion_input_0),
            17 /* terrane-site: src/main.trn:35:13-35:39 */) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = literal_optional();
            __terrane_raised(mark_or_zero(__terrane_conversion_input_0),
            18 /* terrane-site: src/main.trn:36:13-36:46 */) })
        );
        let __terrane_completion_0: TerraneCompletion<()> = (|| {
            let __terrane_try_0: TerraneCompletion<()> = (|| {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let
                    __terrane_conversion_input_0 = String::from("ab");
                    __terrane_raised_completion!(char_code(__terrane_conversion_input_0),
                    19 /* terrane-site: src/main.trn:38:17-38:32 */) })
                );
                TerraneCompletion::Normal
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
                    if !__terrane_handled_0
                        && __terrane_error_0.kind == TerraneErrorKind::CoercionError
                    {
                        __terrane_handled_0 = true;
                        println!("{}", String::from("parameter-coercion-caught"));
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
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let
                    __terrane_conversion_input_0 =
                    __terrane_traced_completion!(make_mark(String::from("ab")),
                    20 /* terrane-site: src/main.trn:42:29-42:44 */);
                    __terrane_raised_completion!(mark_code(__terrane_conversion_input_0),
                    21 /* terrane-site: src/main.trn:42:17-42:45 */) })
                );
                TerraneCompletion::Normal
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
                        println!("{}", String::from("record-coercion-caught"));
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
        let __terrane_completion_2: TerraneCompletion<()> = (|| {
            let __terrane_try_2: TerraneCompletion<()> = (|| {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let
                    __terrane_conversion_input_0 =
                    __terrane_traced_completion!(make_count(terrane_int_support::Int::from(36893488147419103228_i128)),
                    22 /* terrane-site: src/main.trn:46:31-46:66 */);
                    __terrane_raised_completion!(count_value(__terrane_conversion_input_0),
                    23 /* terrane-site: src/main.trn:46:17-46:67 */) })
                );
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
                    if !__terrane_handled_2
                        && __terrane_error_2.kind
                            == TerraneErrorKind::IntegerConversionOverflow
                    {
                        __terrane_handled_2 = true;
                        println!("{}", String::from("record-overflow-caught"));
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
        let __terrane_completion_3: TerraneCompletion<()> = (|| {
            let __terrane_try_3: TerraneCompletion<()> = (|| {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let
                    __terrane_conversion_input_0 =
                    terrane_int_support::Int::from(36893488147419103228_i128);
                    __terrane_raised_completion!(int_value(__terrane_conversion_input_0),
                    24 /* terrane-site: src/main.trn:50:17-50:51 */) })
                );
                TerraneCompletion::Normal
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
                        && __terrane_error_3.kind
                            == TerraneErrorKind::IntegerConversionOverflow
                    {
                        __terrane_handled_3 = true;
                        println!("{}", String::from("parameter-overflow-caught"));
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
        empty = __terrane_traced(
            make_nested(
                terrane_collection_support::List::<Option<String>>::new(vec![None]),
            ),
            25 /* terrane-site: src/main.trn:53:13-53:38 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = empty; __terrane_raised(nested_count(__terrane_conversion_input_0),
            26 /* terrane-site: src/main.trn:54:13-54:32 */) })
        );
        let __terrane_completion_4: TerraneCompletion<()> = (|| {
            let __terrane_try_4: TerraneCompletion<()> = (|| {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let
                    __terrane_conversion_input_0 =
                    __terrane_traced_completion!(make_nested(terrane_collection_support::List::<
                    Option < String >>::new(vec![Some(String::from("ab"))])),
                    27 /* terrane-site: src/main.trn:56:32-56:57 */);
                    __terrane_raised_completion!(nested_count(__terrane_conversion_input_0),
                    28 /* terrane-site: src/main.trn:56:17-56:58 */) })
                );
                TerraneCompletion::Normal
            })();
            match __terrane_try_4 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_4) => {
                    let mut __terrane_handled_4 = false;
                    if !__terrane_handled_4
                        && __terrane_error_4.kind == TerraneErrorKind::CoercionError
                    {
                        __terrane_handled_4 = true;
                        println!("{}", String::from("nested-coercion-caught"));
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
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = __terrane_traced(make_mark(String::from("Z")), 29 /* terrane-site: src/main.trn:59:25-59:39 */);
            __terrane_raised(mark_code(__terrane_conversion_input_0),
            30 /* terrane-site: src/main.trn:59:13-59:40 */) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = terrane_int_support::Int::from(9223372036854775807_i128);
            __terrane_raised(int_value(__terrane_conversion_input_0),
            31 /* terrane-site: src/main.trn:60:13-60:43 */) })
        );
        owner = __terrane_traced(
            make_count(terrane_int_support::Int::from(0_i128)),
            32 /* terrane-site: src/main.trn:61:19-61:32 */,
        );
        let __terrane_completion_5: TerraneCompletion<()> = (|| {
            let __terrane_try_5: TerraneCompletion<()> = (|| {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let
                    __terrane_conversion_input_0 = String::from("ab");
                    __terrane_raised_completion!(| | -> Result < _, crate
                    ::TerraneForeignError > { { let value = match
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result <
                    _, crate ::TerraneForeignError > { Ok((&owner)
                    .char_value(__terrane_conversion_input_0.parse:: < char > ()
                    .map_err(| _ | crate ::TerraneForeignError(crate
                    ::TerraneError::raised_with_message(crate
                    ::TerraneErrorKind::CoercionError,
                    "projected `char` requires exactly one Unicode scalar", crate
                    ::TERRANE_NO_SITE))) ?)) })) { Ok(result) => result ?, Err(payload)
                    => return Err(crate ::__terrane_dependency_panic(payload,
                    "terrane_recursive_descriptor_witness",
                    "terrane_recursive_descriptor_witness::Count::char_value")) }; Ok:: <
                    _, crate ::TerraneForeignError >
                    (terrane_int_support::Int::from_u128(value as u128)) } } (),
                    33 /* terrane-site: src/main.trn:63:17-63:39 */) })
                );
                TerraneCompletion::Normal
            })();
            match __terrane_try_5 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_5) => {
                    let mut __terrane_handled_5 = false;
                    if !__terrane_handled_5
                        && __terrane_error_5.kind == TerraneErrorKind::CoercionError
                    {
                        __terrane_handled_5 = true;
                        println!("{}", String::from("method-coercion-caught"));
                    }
                    if !__terrane_handled_5 {
                        return TerraneCompletion::Error(__terrane_error_5);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        match __terrane_completion_5 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        let __terrane_completion_6: TerraneCompletion<()> = (|| {
            let __terrane_try_6: TerraneCompletion<()> = (|| {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let
                    __terrane_conversion_input_0 =
                    terrane_int_support::Int::from(36893488147419103228_i128);
                    __terrane_raised_completion!(| | -> Result < _, crate
                    ::TerraneForeignError > { { let value = match
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result <
                    _, crate ::TerraneForeignError > { Ok((&owner)
                    .int_value(terrane_int_support::coerce:: < i64 >
                    (&__terrane_conversion_input_0).map_err(| error | crate
                    ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
                    ::TERRANE_NO_SITE))) ?)) })) { Ok(result) => result ?, Err(payload)
                    => return Err(crate ::__terrane_dependency_panic(payload,
                    "terrane_recursive_descriptor_witness",
                    "terrane_recursive_descriptor_witness::Count::int_value")) }; Ok:: <
                    _, crate ::TerraneForeignError >
                    (terrane_int_support::Int::from(i128::from(value))) } } (),
                    34 /* terrane-site: src/main.trn:67:17-67:57 */) })
                );
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
                    if !__terrane_handled_6
                        && __terrane_error_6.kind
                            == TerraneErrorKind::IntegerConversionOverflow
                    {
                        __terrane_handled_6 = true;
                        println!("{}", String::from("method-overflow-caught"));
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
        let __terrane_completion_7: TerraneCompletion<()> = async {
            let __terrane_try_7: TerraneCompletion<()> = async {
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&__terrane_traced_completion!(__terrane_await({
                    let __terrane_source_input_0 = String::from("ab"); { let
                    __terrane_future = { let __terrane_call = (&owner)
                    .async_char(__terrane_raised_completion!(| | -> Result < _, crate
                    ::TerraneForeignError > { Ok(__terrane_source_input_0.parse:: < char
                    > ().map_err(| _ | crate ::TerraneForeignError(crate
                    ::TerraneError::raised_with_message(crate
                    ::TerraneErrorKind::CoercionError,
                    "projected `char` requires exactly one Unicode scalar", crate
                    ::TERRANE_NO_SITE))) ?) } (), 35 /* terrane-site: src/main.trn:71:41-71:45 */)); Box::pin(async move { { let value =
                    crate ::__terrane_dependency_await_unwind(__terrane_call). await
                    .map_err(| payload | crate ::__terrane_dependency_panic(payload,
                    "terrane_recursive_descriptor_witness",
                    "terrane_recursive_descriptor_witness::Count::async_char")) ?; Ok:: <
                    _, crate ::TerraneForeignError >
                    (terrane_int_support::Int::from_u128(value as u128)) } }) }; async
                    move { __terrane_raised_err(__terrane_future. await,
                    36 /* terrane-site: src/main.trn:71:23-71:45 */) } } }). await,
                    36 /* terrane-site: src/main.trn:71:23-71:45 */))
                );
                TerraneCompletion::Normal
            }
                .await;
            match __terrane_try_7 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_7) => {
                    let mut __terrane_handled_7 = false;
                    if !__terrane_handled_7
                        && __terrane_error_7.kind == TerraneErrorKind::CoercionError
                    {
                        __terrane_handled_7 = true;
                        println!("{}", String::from("async-coercion-caught"));
                    }
                    if !__terrane_handled_7 {
                        return TerraneCompletion::Error(__terrane_error_7);
                    }
                }
            }
            TerraneCompletion::Normal
        }
            .await;
        match __terrane_completion_7 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        let __terrane_completion_8: TerraneCompletion<()> = async {
            let __terrane_try_8: TerraneCompletion<()> = async {
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&__terrane_traced_completion!(__terrane_await({
                    let __terrane_source_input_0 =
                    terrane_int_support::Int::from(36893488147419103228_i128); { let
                    __terrane_future = { let __terrane_call = (&owner)
                    .async_int(__terrane_raised_completion!(| | -> Result < _, crate
                    ::TerraneForeignError > { Ok(terrane_int_support::coerce:: < i64 >
                    (&__terrane_source_input_0).map_err(| error | crate
                    ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
                    ::TERRANE_NO_SITE))) ?) } (), 37 /* terrane-site: src/main.trn:75:40-75:63 */)); Box::pin(async move { { let value =
                    crate ::__terrane_dependency_await_unwind(__terrane_call). await
                    .map_err(| payload | crate ::__terrane_dependency_panic(payload,
                    "terrane_recursive_descriptor_witness",
                    "terrane_recursive_descriptor_witness::Count::async_int")) ?; Ok:: <
                    _, crate ::TerraneForeignError >
                    (terrane_int_support::Int::from(i128::from(value))) } }) }; async
                    move { __terrane_raised_err(__terrane_future. await,
                    38 /* terrane-site: src/main.trn:75:23-75:63 */) } } }). await,
                    38 /* terrane-site: src/main.trn:75:23-75:63 */))
                );
                TerraneCompletion::Normal
            }
                .await;
            match __terrane_try_8 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_8) => {
                    let mut __terrane_handled_8 = false;
                    if !__terrane_handled_8
                        && __terrane_error_8.kind
                            == TerraneErrorKind::IntegerConversionOverflow
                    {
                        __terrane_handled_8 = true;
                        println!("{}", String::from("async-overflow-caught"));
                    }
                    if !__terrane_handled_8 {
                        return TerraneCompletion::Error(__terrane_error_8);
                    }
                }
            }
            TerraneCompletion::Normal
        }
            .await;
        match __terrane_completion_8 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        let __terrane_completion_9: TerraneCompletion<()> = (|| {
            let __terrane_try_9: TerraneCompletion<()> = (|| {
                let _ = converted
                    .insert(
                        __terrane_raised_completion!(
                            | | -> Result < _, crate ::TerraneForeignError > { { let
                            value = match
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
                            Result < _, crate ::TerraneForeignError > { Ok({ let
                            __terrane_macro_result : i64 =
                            core::convert::Into::into(terrane_recursive_descriptor_witness::native_int!(terrane_int_support::coerce::
                            < i64 >
                            (&(terrane_int_support::Int::from(9223372036854775807_i128) *
                            terrane_int_support::Int::from(4_i128))).map_err(| error |
                            crate ::TerraneForeignError(crate
                            ::TerraneRaised::raised(error, crate ::TERRANE_NO_SITE)))
                            ?)); __terrane_macro_result }) })) { Ok(result) => result ?,
                            Err(payload) => return Err(crate
                            ::__terrane_dependency_panic(payload,
                            "terrane-recursive-descriptor-witness",
                            "terrane_recursive_descriptor_witness::native_int")) }; Ok::
                            < _, crate ::TerraneForeignError >
                            (terrane_int_support::Int::from(i128::from(value))) } } (),
                            39 /* terrane-site: src/main.trn:79:25-79:60 */
                        ),
                    );
                println!(
                    "{}", terrane_scalar_support::scalar_text(&converted.as_ref()
                    .expect("flow-proven available binding"))
                );
                TerraneCompletion::Normal
            })();
            match __terrane_try_9 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_9) => {
                    let mut __terrane_handled_9 = false;
                    if !__terrane_handled_9
                        && __terrane_error_9.kind
                            == TerraneErrorKind::IntegerConversionOverflow
                    {
                        __terrane_handled_9 = true;
                        println!("{}", String::from("macro-overflow-caught"));
                    }
                    if !__terrane_handled_9 {
                        return TerraneCompletion::Error(__terrane_error_9);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        match __terrane_completion_9 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        let __terrane_completion_10: TerraneCompletion<()> = (|| {
            let __terrane_try_10: TerraneCompletion<()> = (|| {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let
                    __terrane_conversion_input_0 = { let __terrane_conversion_input_0 =
                    __terrane_traced_completion!(missing_label(), 40 /* terrane-site: src/main.trn:84:52-84:66 */); __terrane_raised_completion!(| | ->
                    Result < _, crate ::TerraneForeignError > {
                    Ok(terrane_recursive_descriptor_witness::Mark { mark :
                    __terrane_conversion_input_0.parse:: < char > ().map_err(| _ | crate
                    ::TerraneForeignError(crate ::TerraneError::raised_with_message(crate
                    ::TerraneErrorKind::CoercionError,
                    "projected `char` requires exactly one Unicode scalar", crate
                    ::TERRANE_NO_SITE))) ? }) } (), 41 /* terrane-site: src/main.trn:84:29-84:67 */) };
                    __terrane_raised_completion!(mark_code(__terrane_conversion_input_0),
                    42 /* terrane-site: src/main.trn:84:17-84:68 */) })
                );
                TerraneCompletion::Normal
            })();
            match __terrane_try_10 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_10) => {
                    let mut __terrane_handled_10 = false;
                    if !__terrane_handled_10
                        && __terrane_error_10.kind == TerraneErrorKind::CoercionError
                    {
                        __terrane_handled_10 = true;
                        println!("{}", String::from("source-error-caught"));
                    }
                    if !__terrane_handled_10 {
                        return TerraneCompletion::Error(__terrane_error_10);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        match __terrane_completion_10 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        let __terrane_completion_11: TerraneCompletion<()> = (|| {
            let __terrane_try_11: TerraneCompletion<()> = (|| {
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&__terrane_traced_completion!(macro_value(terrane_int_support::Int::from(36893488147419103228_i128)),
                    43 /* terrane-site: src/main.trn:88:17-88:53 */))
                );
                TerraneCompletion::Normal
            })();
            match __terrane_try_11 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_11) => {
                    let mut __terrane_handled_11 = false;
                    if !__terrane_handled_11
                        && __terrane_error_11.kind
                            == TerraneErrorKind::IntegerConversionOverflow
                    {
                        __terrane_handled_11 = true;
                        println!("{}", String::from("macro-helper-overflow-caught"));
                    }
                    if !__terrane_handled_11 {
                        return TerraneCompletion::Error(__terrane_error_11);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        match __terrane_completion_11 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        options = __terrane_raised(
            native_options(),
            44 /* terrane-site: src/main.trn:91:15-91:30 */,
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(native_options_status(&options),
            45 /* terrane-site: src/main.trn:92:13-92:43 */))
        );
        {
            let __terrane_value: Option<
                terrane_recursive_descriptor_witness::ConfigurationMode,
            > = None;
            options.mode = __terrane_value;
        }
        {
            let __terrane_value: char = __terrane_raised(
                || -> Result<_, crate::TerraneForeignError> {
                    let __terrane_field_value: String = String::from("B");
                    Ok(
                        __terrane_field_value
                            .parse::<char>()
                            .map_err(|_| crate::TerraneForeignError(
                                crate::TerraneError::raised_with_message(
                                    crate::TerraneErrorKind::CoercionError,
                                    "projected `char` requires exactly one Unicode scalar",
                                    crate::TERRANE_NO_SITE,
                                ),
                            ))?,
                    )
                }(),
                46 /* terrane-site: src/main.trn:95:9-95:19 */,
            );
            options.mark = __terrane_value;
        }
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(native_options_status(&options),
            47 /* terrane-site: src/main.trn:97:13-97:43 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(native_options_mark(&options),
            48 /* terrane-site: src/main.trn:98:13-98:41 */))
        );
    });
}
