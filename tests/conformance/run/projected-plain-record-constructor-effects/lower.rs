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
            0 /* terrane-site: src/main.trn:9:12-9:38 */,
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
            1 /* terrane-site: src/main.trn:11:12-11:37 */,
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
                                    .map(|value| -> Result<_, crate::TerraneForeignError> {
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
            2 /* terrane-site: src/main.trn:13:12-13:42 */,
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
                        .map(|value| -> Result<_, crate::TerraneForeignError> {
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
            3 /* terrane-site: src/main.trn:15:12-15:46 */,
        )
    };
}
fn missing_label() -> Result<String, TerraneError> {
    return Err(
        TerraneError::raised(
            TerraneErrorKind::CoercionError,
            4 /* terrane-site: src/main.trn:17:5-17:41 */,
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
            5 /* terrane-site: src/main.trn:19:12-19:29 */,
        )?,
    );
}
fn main() {
    __terrane_run(async move {
        let empty: Nested;
        let owner: Count;
        let mut converted: Option<terrane_int_support::Int> = None;
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = __terrane_traced(make_mark(String::from("A")), 6 /* terrane-site: src/main.trn:21:25-21:39 */);
            __terrane_raised(mark_code(__terrane_conversion_input_0),
            7 /* terrane-site: src/main.trn:21:13-21:40 */) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = __terrane_traced(make_count(terrane_int_support::Int::from(42_i128)),
            8 /* terrane-site: src/main.trn:22:27-22:41 */);
            __terrane_raised(count_value(__terrane_conversion_input_0),
            9 /* terrane-site: src/main.trn:22:13-22:42 */) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = empty_mark(); __terrane_raised(mark_or_zero(__terrane_conversion_input_0),
            10 /* terrane-site: src/main.trn:23:13-23:40 */) })
        );
        let __terrane_completion_0: TerraneCompletion<()> = (|| {
            let __terrane_try_0: TerraneCompletion<()> = (|| {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let
                    __terrane_conversion_input_0 = String::from("ab");
                    __terrane_raised_completion!(char_code(__terrane_conversion_input_0),
                    11 /* terrane-site: src/main.trn:25:17-25:32 */) })
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
                    12 /* terrane-site: src/main.trn:29:29-29:44 */);
                    __terrane_raised_completion!(mark_code(__terrane_conversion_input_0),
                    13 /* terrane-site: src/main.trn:29:17-29:45 */) })
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
                    14 /* terrane-site: src/main.trn:33:31-33:66 */);
                    __terrane_raised_completion!(count_value(__terrane_conversion_input_0),
                    15 /* terrane-site: src/main.trn:33:17-33:67 */) })
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
                    16 /* terrane-site: src/main.trn:37:17-37:51 */) })
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
            17 /* terrane-site: src/main.trn:40:13-40:38 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = empty; __terrane_raised(nested_count(__terrane_conversion_input_0),
            18 /* terrane-site: src/main.trn:41:13-41:32 */) })
        );
        let __terrane_completion_4: TerraneCompletion<()> = (|| {
            let __terrane_try_4: TerraneCompletion<()> = (|| {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let
                    __terrane_conversion_input_0 =
                    __terrane_traced_completion!(make_nested(terrane_collection_support::List::<
                    Option < String >>::new(vec![Some(String::from("ab"))])),
                    19 /* terrane-site: src/main.trn:43:32-43:57 */);
                    __terrane_raised_completion!(nested_count(__terrane_conversion_input_0),
                    20 /* terrane-site: src/main.trn:43:17-43:58 */) })
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
            = __terrane_traced(make_mark(String::from("Z")), 21 /* terrane-site: src/main.trn:46:25-46:39 */);
            __terrane_raised(mark_code(__terrane_conversion_input_0),
            22 /* terrane-site: src/main.trn:46:13-46:40 */) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = terrane_int_support::Int::from(9223372036854775807_i128);
            __terrane_raised(int_value(__terrane_conversion_input_0),
            23 /* terrane-site: src/main.trn:47:13-47:43 */) })
        );
        owner = __terrane_traced(
            make_count(terrane_int_support::Int::from(0_i128)),
            24 /* terrane-site: src/main.trn:48:19-48:32 */,
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
                    25 /* terrane-site: src/main.trn:50:17-50:39 */) })
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
                    26 /* terrane-site: src/main.trn:54:17-54:57 */) })
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
                    ::TERRANE_NO_SITE))) ?) } (), 27 /* terrane-site: src/main.trn:58:41-58:45 */)); Box::pin(async move { { let value =
                    crate ::__terrane_dependency_await_unwind(__terrane_call). await
                    .map_err(| payload | crate ::__terrane_dependency_panic(payload,
                    "terrane_recursive_descriptor_witness",
                    "terrane_recursive_descriptor_witness::Count::async_char")) ?; Ok:: <
                    _, crate ::TerraneForeignError >
                    (terrane_int_support::Int::from_u128(value as u128)) } }) }; async
                    move { __terrane_raised_err(__terrane_future. await,
                    28 /* terrane-site: src/main.trn:58:23-58:45 */) } } }). await,
                    28 /* terrane-site: src/main.trn:58:23-58:45 */))
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
                    ::TERRANE_NO_SITE))) ?) } (), 29 /* terrane-site: src/main.trn:62:40-62:63 */)); Box::pin(async move { { let value =
                    crate ::__terrane_dependency_await_unwind(__terrane_call). await
                    .map_err(| payload | crate ::__terrane_dependency_panic(payload,
                    "terrane_recursive_descriptor_witness",
                    "terrane_recursive_descriptor_witness::Count::async_int")) ?; Ok:: <
                    _, crate ::TerraneForeignError >
                    (terrane_int_support::Int::from(i128::from(value))) } }) }; async
                    move { __terrane_raised_err(__terrane_future. await,
                    30 /* terrane-site: src/main.trn:62:23-62:63 */) } } }). await,
                    30 /* terrane-site: src/main.trn:62:23-62:63 */))
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
                            31 /* terrane-site: src/main.trn:66:25-66:60 */
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
                    __terrane_traced_completion!(missing_label(), 32 /* terrane-site: src/main.trn:71:52-71:66 */); __terrane_raised_completion!(| | ->
                    Result < _, crate ::TerraneForeignError > {
                    Ok(terrane_recursive_descriptor_witness::Mark { mark :
                    __terrane_conversion_input_0.parse:: < char > ().map_err(| _ | crate
                    ::TerraneForeignError(crate ::TerraneError::raised_with_message(crate
                    ::TerraneErrorKind::CoercionError,
                    "projected `char` requires exactly one Unicode scalar", crate
                    ::TERRANE_NO_SITE))) ? }) } (), 33 /* terrane-site: src/main.trn:71:29-71:67 */) };
                    __terrane_raised_completion!(mark_code(__terrane_conversion_input_0),
                    34 /* terrane-site: src/main.trn:71:17-71:68 */) })
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
                    35 /* terrane-site: src/main.trn:75:17-75:53 */))
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
    });
}
