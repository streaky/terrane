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
}
