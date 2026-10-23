// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct Record {
    pub value: String,
}
impl Record {
    pub fn terrane_construct() -> Self {
        Self { value: String::from("") }
    }
    pub fn load(&mut self) {
        let source: Row;
        source = __terrane_raised(
            row(),
            0 /* terrane-site: src/main.trn:8:14-8:18 */,
        );
        {
            let __terrane_value = __terrane_raised(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| (&source).decoded::<String>()),
                ) {
                    Ok(value) => Ok(value),
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane_generic_row_witness",
                                "terrane_generic_row_witness::Row::decoded",
                            ),
                        )
                    }
                },
                1 /* terrane-site: src/main.trn:9:18-9:33 */,
            );
            self.value = __terrane_value;
        }
    }
}
fn decoded_value() -> String {
    let source: Row;
    source = __terrane_raised(row(), 2 /* terrane-site: src/main.trn:12:12-12:16 */);
    return __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| (&source).decoded::<String>()),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_generic_row_witness",
                        "terrane_generic_row_witness::Row::decoded",
                    ),
                )
            }
        },
        3 /* terrane-site: src/main.trn:13:10-13:25 */,
    );
}
fn make_number() -> i64 {
    return __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| factory::default_value::<i64>()),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "factory",
                        "factory::default_value",
                    ),
                )
            }
        },
        4 /* terrane-site: src/main.trn:16:10-16:24 */,
    );
}
fn accept_by_argument(value: String) {
    println!("{}", terrane_scalar_support::scalar_text(&value));
}
fn main() {
    let number: i64;
    let optional: Option<i64>;
    let data: Vec<u8>;
    let values: terrane_collection_support::List<i64>;
    let names: terrane_collection_support::Map<String, i64>;
    let tags: terrane_collection_support::Set<String>;
    let rows: terrane_collection_support::Map<String, i64>;
    let numeric_rows: terrane_collection_support::Map<i64, i64>;
    let renamed: String;
    let source: Row;
    let returned: String;
    let mut item: Record;
    number = make_number();
    optional = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| factory::sample_value::<Option<i64>>()),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "factory",
                        "factory::sample_value",
                    ),
                )
            }
        },
        5 /* terrane-site: src/main.trn:23:25-23:38 */,
    );
    data = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| factory::sample_value::<Vec<u8>>()),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "factory",
                        "factory::sample_value",
                    ),
                )
            }
        },
        6 /* terrane-site: src/main.trn:24:16-24:29 */,
    );
    values = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| factory::sample_value::<
                std::vec::Vec<i64>,
            >()),
        ) {
            Ok(value) => Ok(terrane_collection_support::List::new(value)),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "factory",
                        "factory::sample_value",
                    ),
                )
            }
        },
        7 /* terrane-site: src/main.trn:25:26-25:39 */,
    );
    names = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| factory::sample_value::<
                std::collections::BTreeMap<String, i64>,
            >()),
        ) {
            Ok(value) => {
                Ok(
                    terrane_collection_support::Map::new(
                        value
                            .into_iter()
                            .map(|(key, item)| terrane_collection_support::Entry::new(
                                key,
                                item,
                            ))
                            .collect(),
                    ),
                )
            }
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "factory",
                        "factory::sample_value",
                    ),
                )
            }
        },
        8 /* terrane-site: src/main.trn:26:32-26:45 */,
    );
    tags = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| factory::sample_value::<
                std::collections::BTreeSet<String>,
            >()),
        ) {
            Ok(value) => {
                Ok(
                    terrane_collection_support::Set::new(
                        value.into_iter().map(|item| item).collect(),
                    ),
                )
            }
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "factory",
                        "factory::sample_value",
                    ),
                )
            }
        },
        9 /* terrane-site: src/main.trn:27:24-27:37 */,
    );
    rows = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| factory::sample_rows::<i64>()),
        ) {
            Ok(value) => {
                Ok(
                    terrane_collection_support::Map::new(
                        value
                            .into_iter()
                            .map(|(key, item)| terrane_collection_support::Entry::new(
                                key,
                                item,
                            ))
                            .collect(),
                    ),
                )
            }
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "factory",
                        "factory::sample_rows",
                    ),
                )
            }
        },
        10 /* terrane-site: src/main.trn:28:31-28:43 */,
    );
    numeric_rows = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| factory::sample_numeric_rows::<i64>()),
        ) {
            Ok(value) => {
                Ok(
                    terrane_collection_support::Map::new(
                        value
                            .into_iter()
                            .map(|(key, item)| terrane_collection_support::Entry::new(
                                key,
                                item,
                            ))
                            .collect(),
                    ),
                )
            }
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "factory",
                        "factory::sample_numeric_rows",
                    ),
                )
            }
        },
        11 /* terrane-site: src/main.trn:29:38-29:58 */,
    );
    renamed = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| factory::renamed_bound_value::<String>()),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "factory",
                        "factory::renamed_bound_value",
                    ),
                )
            }
        },
        12 /* terrane-site: src/main.trn:30:20-30:40 */,
    );
    source = __terrane_raised(row(), 13 /* terrane-site: src/main.trn:31:12-31:16 */);
    accept_by_argument(
        __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| (&source).decoded::<String>()),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "terrane_generic_row_witness",
                            "terrane_generic_row_witness::Row::decoded",
                        ),
                    )
                }
            },
            14 /* terrane-site: src/main.trn:32:24-32:39 */,
        ),
    );
    returned = decoded_value();
    println!("{}", terrane_scalar_support::scalar_text(&returned));
    item = Record::terrane_construct();
    item.load();
    println!("{}", terrane_scalar_support::scalar_text(&item.value));
    println!(
        "{}{}{}{}{}{}{}{}{}", terrane_scalar_support::scalar_text(&number),
        terrane_scalar_support::scalar_text(&(&optional).is_some()),
        terrane_scalar_support::scalar_text(&(data.len() as i128)),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(values
        .length())),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(names
        .length())),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(tags
        .length())),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(rows
        .length())),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(numeric_rows
        .length())), terrane_scalar_support::scalar_text(&renamed)
    );
}
