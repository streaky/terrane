// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let mut descriptor: Descriptor;
    let owned: Owned;
    let first: String;
    let defaults: Descriptor;
    let indexed: Owned;
    let noted: Owned;
    let zero: Owned;
    let mut sizes: NativeSizes;
    let observed: terrane_int_support::Int;
    let mut optional: Option<terrane_int_support::Int>;
    let mut error: Option<TerraneError> = None;
    let mut borrowed_sizes: BorrowedSizes;
    let borrowed_count: terrane_int_support::Int;
    let mut borrowed_delta: Option<terrane_int_support::Int>;
    descriptor = {
        let __terrane_conversion_input_0 = String::from("main");
        let __terrane_conversion_input_3 = Some(String::from("ready"));
        let __terrane_conversion_input_1 = terrane_collection_support::List::<
            String,
        >::new(vec![String::from("ab"), String::from("c")]);
        let __terrane_conversion_input_4 = Some(
            terrane_collection_support::List::<String>::new(vec![String::from("five")]),
        );
        let __terrane_conversion_input_2 = 35;
        __terrane_raised(
            Descriptor::terrane_construct(
                __terrane_conversion_input_0,
                __terrane_conversion_input_1,
                __terrane_conversion_input_2,
                __terrane_conversion_input_3,
                __terrane_conversion_input_4,
            ),
            0 /* terrane-site: src/main.trn:8:29-8:152 */,
        )
    };
    owned = {
        let __terrane_conversion_input_0 = String::from("owned");
        let __terrane_conversion_input_1 = 7;
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(terrane_borrowed_struct_witness::Owned {
                    label: __terrane_conversion_input_0,
                    count: __terrane_conversion_input_1,
                    note: None::<String>,
                    index: None::<i64>,
                })
            }(),
            1 /* terrane-site: src/main.trn:9:19-9:68 */,
        )
    };
    first = __terrane_raised(
        summarize(&descriptor),
        2 /* terrane-site: src/main.trn:10:20-10:41 */,
    );
    {
        let __terrane_value: u32 = 36;
        descriptor.offset = __terrane_value;
    }
    println!(
        "{}{}{}{}{}{}{}", terrane_scalar_support::scalar_text(&first), String::from(":"),
        terrane_scalar_support::scalar_text(&__terrane_raised(summarize_owned(&owned),
        3 /* terrane-site: src/main.trn:12:25-12:47 */)), String::from(":"),
        terrane_scalar_support::scalar_text(&owned.count), String::from(":"),
        terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        descriptor; __terrane_raised(consume(__terrane_conversion_input_0),
        4 /* terrane-site: src/main.trn:12:74-12:93 */) })
    );
    defaults = {
        let __terrane_conversion_input_0 = String::from("defaults");
        let __terrane_conversion_input_1 = terrane_collection_support::List::<
            String,
        >::new(vec![String::from("x")]);
        let __terrane_conversion_input_2 = 1;
        __terrane_raised(
            Descriptor::terrane_construct(
                __terrane_conversion_input_0,
                __terrane_conversion_input_1,
                __terrane_conversion_input_2,
                None::<String>,
                None::<terrane_collection_support::List<String>>,
            ),
            5 /* terrane-site: src/main.trn:13:27-13:107 */,
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(summarize(&defaults),
        6 /* terrane-site: src/main.trn:14:13-14:32 */))
    );
    indexed = {
        let __terrane_conversion_input_0 = String::from("indexed");
        let __terrane_conversion_input_1 = 8;
        let __terrane_conversion_input_3 = Some(terrane_int_support::Int::from(42_i128));
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(terrane_borrowed_struct_witness::Owned {
                    label: __terrane_conversion_input_0,
                    count: __terrane_conversion_input_1,
                    note: None::<String>,
                    index: {
                        let __terrane_field_argument: Option<terrane_int_support::Int> = __terrane_conversion_input_3;
                        __terrane_field_argument
                    }
                        .map(|value| -> Result<i64, crate::TerraneForeignError> {
                            Ok(
                                terrane_int_support::coerce::<i64>(&value)
                                    .map_err(|error| crate::TerraneForeignError(
                                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                    ))?,
                            )
                        })
                        .transpose()?
                        .into(),
                })
            }(),
            7 /* terrane-site: src/main.trn:15:21-15:84 */,
        )
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(summarize_owned(&indexed),
        8 /* terrane-site: src/main.trn:16:13-16:37 */))
    );
    noted = {
        let __terrane_conversion_input_0 = String::from("noted");
        let __terrane_conversion_input_1 = 9;
        let __terrane_conversion_input_2 = Some(String::from("retained"));
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(terrane_borrowed_struct_witness::Owned {
                    label: __terrane_conversion_input_0,
                    count: __terrane_conversion_input_1,
                    note: {
                        let __terrane_field_argument: Option<String> = __terrane_conversion_input_2;
                        __terrane_field_argument
                    }
                        .into(),
                    index: None::<i64>,
                })
            }(),
            9 /* terrane-site: src/main.trn:17:19-17:87 */,
        )
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(summarize_owned(&noted),
        10 /* terrane-site: src/main.trn:18:13-18:35 */))
    );
    zero = {
        let __terrane_conversion_input_0 = String::from("zero");
        let __terrane_conversion_input_1 = 10;
        let __terrane_conversion_input_2 = Some(String::from("both"));
        let __terrane_conversion_input_3 = Some(terrane_int_support::Int::from(0_i128));
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(terrane_borrowed_struct_witness::Owned {
                    label: __terrane_conversion_input_0,
                    count: __terrane_conversion_input_1,
                    note: {
                        let __terrane_field_argument: Option<String> = __terrane_conversion_input_2;
                        __terrane_field_argument
                    }
                        .into(),
                    index: {
                        let __terrane_field_argument: Option<terrane_int_support::Int> = __terrane_conversion_input_3;
                        __terrane_field_argument
                    }
                        .map(|value| -> Result<i64, crate::TerraneForeignError> {
                            Ok(
                                terrane_int_support::coerce::<i64>(&value)
                                    .map_err(|error| crate::TerraneForeignError(
                                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                    ))?,
                            )
                        })
                        .transpose()?
                        .into(),
                })
            }(),
            11 /* terrane-site: src/main.trn:19:18-19:93 */,
        )
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(summarize_owned(&zero),
        12 /* terrane-site: src/main.trn:20:13-20:34 */))
    );
    sizes = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(7_i128);
        let __terrane_conversion_input_1 = terrane_int_support::Int::from(-2_i128);
        let __terrane_conversion_input_3 = Some(terrane_int_support::Int::from(3_i128));
        let __terrane_conversion_input_2 = terrane_collection_support::List::<
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_int_support::Int::from(- 5_i128),
                terrane_int_support::Int::from(6_i128)
            ],
        );
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(terrane_borrowed_struct_witness::NativeSizes {
                    count: terrane_int_support::coerce::<
                        usize,
                    >(&__terrane_conversion_input_0)
                        .map_err(|error| crate::TerraneForeignError(
                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                        ))?,
                    delta: terrane_int_support::coerce::<
                        isize,
                    >(&__terrane_conversion_input_1)
                        .map_err(|error| crate::TerraneForeignError(
                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                        ))?,
                    offsets: __terrane_conversion_input_2
                        .into_iter()
                        .map(|item| -> Result<_, crate::TerraneForeignError> {
                            Ok(
                                terrane_int_support::coerce::<isize>(&item)
                                    .map_err(|error| crate::TerraneForeignError(
                                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                    ))?,
                            )
                        })
                        .collect::<Result<Vec<isize>, _>>()?,
                    optional: {
                        let __terrane_field_argument: Option<terrane_int_support::Int> = __terrane_conversion_input_3;
                        __terrane_field_argument
                    }
                        .map(|value| -> Result<usize, crate::TerraneForeignError> {
                            Ok(
                                terrane_int_support::coerce::<usize>(&value)
                                    .map_err(|error| crate::TerraneForeignError(
                                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                    ))?,
                            )
                        })
                        .transpose()?
                        .into(),
                })
            }(),
            13 /* terrane-site: src/main.trn:21:13-21:95 */,
        )
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(summarize_sizes(&sizes),
        14 /* terrane-site: src/main.trn:22:13-22:35 */))
    );
    {
        let __terrane_value: usize = __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                let __terrane_field_value: terrane_int_support::Int = terrane_int_support::Int::from(
                    9_i128,
                );
                Ok(
                    terrane_int_support::coerce::<usize>(&__terrane_field_value)
                        .map_err(|error| crate::TerraneForeignError(
                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                        ))?,
                )
            }(),
            15 /* terrane-site: src/main.trn:23:5-23:20 */,
        );
        sizes.count = __terrane_value;
    }
    {
        let __terrane_value: Option<usize> = __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                let __terrane_field_value: Option<terrane_int_support::Int> = None;
                Ok(
                    __terrane_field_value
                        .map(|value| -> Result<usize, crate::TerraneForeignError> {
                            Ok(
                                terrane_int_support::coerce::<usize>(&value)
                                    .map_err(|error| crate::TerraneForeignError(
                                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                    ))?,
                            )
                        })
                        .transpose()?,
                )
            }(),
            16 /* terrane-site: src/main.trn:24:5-24:26 */,
        );
        sizes.optional = __terrane_value;
    }
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&sizes.count),
        String::from(":"), terrane_scalar_support::scalar_text(&sizes.delta),
        String::from(":"),
        terrane_scalar_support::scalar_text(&__terrane_raised(summarize_sizes(&sizes),
        17 /* terrane-site: src/main.trn:25:49-25:71 */))
    );
    observed = terrane_int_support::Int::from_u128(*&sizes.count as u128);
    println!(
        "{}", terrane_scalar_support::scalar_text(&(observed.clone() +
        terrane_int_support::Int::from(1_i128)))
    );
    optional = (&sizes.optional)
        .as_ref()
        .map(|__terrane_field_item| terrane_int_support::Int::from_u128(
            *__terrane_field_item as u128,
        ))
        .clone();
    println!("{}", terrane_scalar_support::scalar_text(&(&optional).is_none()));
    {
        let __terrane_value: Option<usize> = __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                let __terrane_field_value: Option<terrane_int_support::Int> = Some(
                    terrane_int_support::Int::from(12_i128),
                );
                Ok(
                    __terrane_field_value
                        .map(|value| -> Result<usize, crate::TerraneForeignError> {
                            Ok(
                                terrane_int_support::coerce::<usize>(&value)
                                    .map_err(|error| crate::TerraneForeignError(
                                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                    ))?,
                            )
                        })
                        .transpose()?,
                )
            }(),
            18 /* terrane-site: src/main.trn:30:5-30:24 */,
        );
        sizes.optional = __terrane_value;
    }
    optional = (&sizes.optional)
        .as_ref()
        .map(|__terrane_field_item| terrane_int_support::Int::from_u128(
            *__terrane_field_item as u128,
        ))
        .clone();
    println!(
        "{}", { let mut __terrane_text = String::new(); match &optional {
        Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    {
        let __terrane_value: Vec<isize> = __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                let __terrane_field_value: terrane_collection_support::List<
                    terrane_int_support::Int,
                > = terrane_collection_support::List::<
                    terrane_int_support::Int,
                >::new(
                    vec![
                        terrane_int_support::Int::from(13_i128),
                        terrane_int_support::Int::from(- 14_i128)
                    ],
                );
                Ok(
                    __terrane_field_value
                        .into_iter()
                        .map(|item| -> Result<_, crate::TerraneForeignError> {
                            Ok(
                                terrane_int_support::coerce::<isize>(&item)
                                    .map_err(|error| crate::TerraneForeignError(
                                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                    ))?,
                            )
                        })
                        .collect::<Result<Vec<isize>, _>>()?,
                )
            }(),
            19 /* terrane-site: src/main.trn:33:5-33:36 */,
        );
        sizes.offsets = __terrane_value;
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(summarize_sizes(&sizes),
        20 /* terrane-site: src/main.trn:34:13-34:35 */))
    );
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            {
                let __terrane_value: usize = __terrane_raised_completion!(
                    | | -> Result < _, crate ::TerraneForeignError > { let
                    __terrane_field_value : terrane_int_support::Int =
                    terrane_int_support::Int::from(- 1_i128);
                    Ok(terrane_int_support::coerce:: < usize > (&__terrane_field_value)
                    .map_err(| error | crate ::TerraneForeignError(crate
                    ::TerraneRaised::raised(error, crate ::TERRANE_NO_SITE))) ?) } (),
                    21 /* terrane-site: src/main.trn:36:9-36:25 */
                );
                sizes.count = __terrane_value;
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
                    && __terrane_error_0.kind
                        == TerraneErrorKind::IntegerConversionOverflow
                {
                    __terrane_handled_0 = true;
                    let _ = error.insert(__terrane_error_0.clone());
                    {
                        let __terrane_error_text_0 = error
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
    println!("{}", terrane_scalar_support::scalar_text(&sizes.count));
    borrowed_sizes = {
        let __terrane_conversion_input_0 = String::from("view");
        let __terrane_conversion_input_1 = terrane_int_support::Int::from(4_i128);
        let __terrane_conversion_input_2 = Some(terrane_int_support::Int::from(-3_i128));
        __terrane_raised(
            BorrowedSizes::terrane_construct(
                __terrane_conversion_input_0,
                __terrane_conversion_input_1,
                __terrane_conversion_input_2,
            ),
            22 /* terrane-site: src/main.trn:40:22-40:90 */,
        )
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(summarize_borrowed_sizes(&borrowed_sizes),
        23 /* terrane-site: src/main.trn:41:13-41:53 */))
    );
    borrowed_count = terrane_int_support::Int::from_u128(*&borrowed_sizes.count as u128);
    borrowed_delta = (&borrowed_sizes.delta)
        .as_ref()
        .map(|__terrane_field_item| terrane_int_support::Int::from(
            *__terrane_field_item as i128,
        ))
        .clone();
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(borrowed_count.clone() +
        terrane_int_support::Int::from(1_i128))), String::from(":"), { let mut
        __terrane_text = String::new(); match &borrowed_delta { Some(__terrane_present)
        => { terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    if terrane_int_support::Int::from_u128(*&borrowed_sizes.count as u128)
        > terrane_int_support::Int::from(3_i128)
    {
        println!("{}", String::from("borrowed count compared"));
    }
    {
        let __terrane_value: usize = __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                let __terrane_field_value: terrane_int_support::Int = terrane_int_support::Int::from(
                    11_i128,
                );
                Ok(
                    terrane_int_support::coerce::<usize>(&__terrane_field_value)
                        .map_err(|error| crate::TerraneForeignError(
                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                        ))?,
                )
            }(),
            24 /* terrane-site: src/main.trn:47:5-47:30 */,
        );
        borrowed_sizes.count = __terrane_value;
    }
    {
        let __terrane_value: Option<isize> = __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                let __terrane_field_value: Option<terrane_int_support::Int> = None;
                Ok(
                    __terrane_field_value
                        .map(|value| -> Result<isize, crate::TerraneForeignError> {
                            Ok(
                                terrane_int_support::coerce::<isize>(&value)
                                    .map_err(|error| crate::TerraneForeignError(
                                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                    ))?,
                            )
                        })
                        .transpose()?,
                )
            }(),
            25 /* terrane-site: src/main.trn:48:5-48:32 */,
        );
        borrowed_sizes.delta = __terrane_value;
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(summarize_borrowed_sizes(&borrowed_sizes),
        26 /* terrane-site: src/main.trn:49:13-49:53 */))
    );
    borrowed_delta = (&borrowed_sizes.delta)
        .as_ref()
        .map(|__terrane_field_item| terrane_int_support::Int::from(
            *__terrane_field_item as i128,
        ))
        .clone();
    println!("{}", terrane_scalar_support::scalar_text(&(&borrowed_delta).is_none()));
}
