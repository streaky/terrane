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
    let mut borrowed_sizes: BorrowedSizes;
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
            0 /* terrane-site: src/main.trn:7:29-7:152 */,
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
            1 /* terrane-site: src/main.trn:8:19-8:68 */,
        )
    };
    first = __terrane_raised(
        summarize(&descriptor),
        2 /* terrane-site: src/main.trn:9:20-9:41 */,
    );
    {
        let __terrane_value: u32 = 36;
        descriptor.offset = __terrane_value;
    }
    println!(
        "{}{}{}{}{}{}{}", terrane_scalar_support::scalar_text(&first), String::from(":"),
        terrane_scalar_support::scalar_text(&__terrane_raised(summarize_owned(&owned),
        3 /* terrane-site: src/main.trn:11:25-11:47 */)), String::from(":"),
        terrane_scalar_support::scalar_text(&owned.count), String::from(":"),
        terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        descriptor; __terrane_raised(consume(__terrane_conversion_input_0),
        4 /* terrane-site: src/main.trn:11:74-11:93 */) })
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
            5 /* terrane-site: src/main.trn:12:27-12:107 */,
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(summarize(&defaults),
        6 /* terrane-site: src/main.trn:13:13-13:32 */))
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
            7 /* terrane-site: src/main.trn:14:21-14:84 */,
        )
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(summarize_owned(&indexed),
        8 /* terrane-site: src/main.trn:15:13-15:37 */))
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
            9 /* terrane-site: src/main.trn:16:19-16:87 */,
        )
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(summarize_owned(&noted),
        10 /* terrane-site: src/main.trn:17:13-17:35 */))
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
            11 /* terrane-site: src/main.trn:18:18-18:93 */,
        )
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(summarize_owned(&zero),
        12 /* terrane-site: src/main.trn:19:13-19:34 */))
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
            13 /* terrane-site: src/main.trn:20:13-20:95 */,
        )
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(summarize_sizes(&sizes),
        14 /* terrane-site: src/main.trn:21:13-21:35 */))
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
            15 /* terrane-site: src/main.trn:22:5-22:20 */,
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
            16 /* terrane-site: src/main.trn:23:5-23:26 */,
        );
        sizes.optional = __terrane_value;
    }
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&sizes.count),
        String::from(":"), terrane_scalar_support::scalar_text(&sizes.delta),
        String::from(":"),
        terrane_scalar_support::scalar_text(&__terrane_raised(summarize_sizes(&sizes),
        17 /* terrane-site: src/main.trn:24:49-24:71 */))
    );
    observed = terrane_int_support::Int::from_u128(*&sizes.count as u128);
    println!(
        "{}", terrane_scalar_support::scalar_text(&(observed.clone() +
        terrane_int_support::Int::from(1_i128)))
    );
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
            18 /* terrane-site: src/main.trn:27:22-27:90 */,
        )
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(summarize_borrowed_sizes(&borrowed_sizes),
        19 /* terrane-site: src/main.trn:28:13-28:53 */))
    );
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
            20 /* terrane-site: src/main.trn:29:5-29:30 */,
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
            21 /* terrane-site: src/main.trn:30:5-30:32 */,
        );
        borrowed_sizes.delta = __terrane_value;
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(summarize_borrowed_sizes(&borrowed_sizes),
        22 /* terrane-site: src/main.trn:31:13-31:53 */))
    );
}
