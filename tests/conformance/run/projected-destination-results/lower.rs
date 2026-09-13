// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
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
        let source: Row = __terrane_raised(
            row(),
            0 /* terrane-site: src/main.trn:9:14-9:18 */,
        );
        self.value = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| source.decoded::<String>()),
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
            1 /* terrane-site: src/main.trn:10:18-10:33 */,
        );
    }
}
fn decoded_value() -> String {
    let source: Row = __terrane_raised(
        row(),
        2 /* terrane-site: src/main.trn:13:12-13:16 */,
    );
    return __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| source.decoded::<String>()),
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
        3 /* terrane-site: src/main.trn:14:10-14:25 */,
    );
}
fn make_number() -> i64 {
    return __terrane_raised(
        default_value::<i64>(),
        4 /* terrane-site: src/main.trn:17:10-17:24 */,
    );
}
fn accept_by_argument(value: String) {
    println!("{}", terrane_scalar_support::scalar_text(&value));
}
fn main() {
    let number: i64 = make_number();
    let optional: Option<i64> = __terrane_raised(
        sample_value::<Option<i64>>(),
        5 /* terrane-site: src/main.trn:24:25-24:38 */,
    );
    let data: Vec<u8> = __terrane_raised(
        sample_value::<Vec<u8>>(),
        6 /* terrane-site: src/main.trn:25:16-25:29 */,
    );
    let values: terrane_collection_support::List<i64> = terrane_collection_support::List::new(
        __terrane_raised(
            sample_value::<std::vec::Vec<i64>>(),
            7 /* terrane-site: src/main.trn:26:26-26:39 */,
        ),
    );
    let names: terrane_collection_support::Map<String, i64> = terrane_collection_support::Map::new(
        __terrane_raised(
                sample_value::<std::collections::BTreeMap<String, i64>>(),
                8 /* terrane-site: src/main.trn:27:32-27:45 */,
            )
            .into_iter()
            .map(|(key, item)| terrane_collection_support::Entry::new(key, item))
            .collect(),
    );
    let tags: terrane_collection_support::Set<String> = terrane_collection_support::Set::new(
        __terrane_raised(
                sample_value::<std::collections::BTreeSet<String>>(),
                9 /* terrane-site: src/main.trn:28:24-28:37 */,
            )
            .into_iter()
            .map(|item| item)
            .collect(),
    );
    let rows: terrane_collection_support::Map<String, i64> = terrane_collection_support::Map::new(
        __terrane_raised(
                sample_rows::<i64>(),
                10 /* terrane-site: src/main.trn:29:31-29:43 */,
            )
            .into_iter()
            .map(|(key, item)| terrane_collection_support::Entry::new(key, item))
            .collect(),
    );
    let numeric_rows: terrane_collection_support::Map<i64, i64> = terrane_collection_support::Map::new(
        __terrane_raised(
                sample_numeric_rows::<i64>(),
                11 /* terrane-site: src/main.trn:30:38-30:58 */,
            )
            .into_iter()
            .map(|(key, item)| terrane_collection_support::Entry::new(key, item))
            .collect(),
    );
    let renamed: String = __terrane_raised(
        renamed_bound_value::<String>(),
        12 /* terrane-site: src/main.trn:31:20-31:40 */,
    );
    let source: Row = __terrane_raised(
        row(),
        13 /* terrane-site: src/main.trn:32:12-32:16 */,
    );
    accept_by_argument(
        __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| source.decoded::<String>()),
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
            14 /* terrane-site: src/main.trn:33:24-33:39 */,
        ),
    );
    let returned: String = decoded_value();
    println!("{}", terrane_scalar_support::scalar_text(&returned));
    let mut item: Record = Record::terrane_construct();
    item.load();
    println!("{}", terrane_scalar_support::scalar_text(&item.value));
    println!(
        "{}{}{}{}{}{}{}{}{}", terrane_scalar_support::scalar_text(&number),
        terrane_scalar_support::scalar_text(&optional.is_some()),
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
// Source: <terrane>/projected/deps/factory.trn
// Namespace: deps/factory
pub fn default_value<T: core::default::Default>() -> Result<
    T,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| factory::default_value::<T>()),
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
    }
}
pub fn renamed_bound_value<T: for<'value> factory::Decode<'value>>() -> Result<
    T,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| factory::renamed_bound_value::<T>()),
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
    }
}
pub fn sample_numeric_rows<T: factory::Sample>() -> Result<
    std::collections::BTreeMap<i64, T>,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| factory::sample_numeric_rows::<T>()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "factory",
                    "factory::sample_numeric_rows",
                ),
            )
        }
    }
}
pub fn sample_rows<T: factory::Sample>() -> Result<
    std::collections::BTreeMap<String, T>,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| factory::sample_rows::<T>()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "factory",
                    "factory::sample_rows",
                ),
            )
        }
    }
}
pub fn sample_value<T: factory::Sample>() -> Result<T, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| factory::sample_value::<T>()),
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
    }
}
// Source: <terrane>/projected/deps/terrane-generic-row-witness.trn
// Namespace: deps/terrane-generic-row-witness
pub use terrane_generic_row_witness::Row;
pub fn row() -> Result<Row, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_generic_row_witness::row()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-generic-row-witness",
                    "terrane_generic_row_witness::row",
                ),
            )
        }
    }
}
