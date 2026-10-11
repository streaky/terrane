// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let values: terrane_collection_support::Map<
        String,
        terrane_collection_support::List<terrane_collection_support::Map<i64, String>>,
    >;
    values = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::correlated_values::<
                String,
                i64,
                String,
            >()),
        ) {
            Ok(value) => {
                Ok(
                    terrane_collection_support::Map::new(
                        value
                            .into_iter()
                            .map(|(key, item)| terrane_collection_support::Entry::new(
                                key,
                                terrane_collection_support::List::new(
                                    item
                                        .into_iter()
                                        .map(|item| terrane_collection_support::Map::new(
                                            item
                                                .into_iter()
                                                .map(|(key, item)| terrane_collection_support::Entry::new(
                                                    key,
                                                    item,
                                                ))
                                                .collect(),
                                        ))
                                        .collect(),
                                ),
                            ))
                            .collect(),
                    ),
                )
            }
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::correlated_values::<String, i64, String>",
                    ),
                )
            }
        },
        0 /* terrane-site: src/main.trn:6:56-6:74 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(values
        .length()))
    );
}
