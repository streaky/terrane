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
    descriptor = __terrane_raised(
        Descriptor::terrane_construct(
            String::from("main"),
            terrane_collection_support::List::<
                String,
            >::new(vec![String::from("ab"), String::from("c")]),
            35,
            Some(String::from("ready")),
            Some(
                terrane_collection_support::List::<
                    String,
                >::new(vec![String::from("five")]),
            ),
        ),
        0 /* terrane-site: src/main.trn:7:29-7:152 */,
    );
    owned = terrane_borrowed_struct_witness::Owned {
        label: String::from("owned"),
        count: 7,
        note: None::<String>,
        index: None::<i64>,
    };
    first = __terrane_raised(
        summarize(&descriptor),
        1 /* terrane-site: src/main.trn:9:20-9:41 */,
    );
    {
        let __terrane_value = 36;
        descriptor.offset = __terrane_value;
    }
    println!(
        "{}{}{}{}{}{}{}", terrane_scalar_support::scalar_text(&first), String::from(":"),
        terrane_scalar_support::scalar_text(&__terrane_raised(summarize_owned(&owned),
        2 /* terrane-site: src/main.trn:11:25-11:47 */)), String::from(":"),
        terrane_scalar_support::scalar_text(&owned.count), String::from(":"),
        terrane_scalar_support::scalar_text(&__terrane_raised(consume(descriptor),
        3 /* terrane-site: src/main.trn:11:74-11:93 */))
    );
    defaults = __terrane_raised(
        Descriptor::terrane_construct(
            String::from("defaults"),
            terrane_collection_support::List::<String>::new(vec![String::from("x")]),
            1,
            None::<String>,
            None::<terrane_collection_support::List<String>>,
        ),
        4 /* terrane-site: src/main.trn:12:27-12:107 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(summarize(&defaults),
        5 /* terrane-site: src/main.trn:13:13-13:32 */))
    );
    indexed = terrane_borrowed_struct_witness::Owned {
        label: String::from("indexed"),
        count: 8,
        note: None::<String>,
        index: match || -> Result<_, crate::TerraneForeignError> {
            Ok(
                {
                    let __terrane_field_argument: Option<terrane_int_support::Int> = Some(
                        terrane_int_support::Int::from(42_i128),
                    );
                    __terrane_field_argument
                }
                    .map(|value| -> Result<_, crate::TerraneForeignError> {
                        Ok(
                            terrane_int_support::coerce::<i64>(&value)
                                .map_err(|error| crate::TerraneForeignError(
                                    crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                ))?,
                        )
                    })
                    .transpose()?,
            )
        }() {
            Ok(value) => value,
            Err(error) => std::panic::panic_any(error),
        }
            .into(),
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(summarize_owned(&indexed),
        6 /* terrane-site: src/main.trn:15:13-15:37 */))
    );
    noted = terrane_borrowed_struct_witness::Owned {
        label: String::from("noted"),
        count: 9,
        note: {
            let __terrane_field_argument: Option<String> = Some(
                String::from("retained"),
            );
            __terrane_field_argument
        }
            .into(),
        index: None::<i64>,
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(summarize_owned(&noted),
        7 /* terrane-site: src/main.trn:17:13-17:35 */))
    );
    zero = terrane_borrowed_struct_witness::Owned {
        label: String::from("zero"),
        count: 10,
        note: {
            let __terrane_field_argument: Option<String> = Some(String::from("both"));
            __terrane_field_argument
        }
            .into(),
        index: match || -> Result<_, crate::TerraneForeignError> {
            Ok(
                {
                    let __terrane_field_argument: Option<terrane_int_support::Int> = Some(
                        terrane_int_support::Int::from(0_i128),
                    );
                    __terrane_field_argument
                }
                    .map(|value| -> Result<_, crate::TerraneForeignError> {
                        Ok(
                            terrane_int_support::coerce::<i64>(&value)
                                .map_err(|error| crate::TerraneForeignError(
                                    crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                ))?,
                        )
                    })
                    .transpose()?,
            )
        }() {
            Ok(value) => value,
            Err(error) => std::panic::panic_any(error),
        }
            .into(),
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(summarize_owned(&zero),
        8 /* terrane-site: src/main.trn:19:13-19:34 */))
    );
}
