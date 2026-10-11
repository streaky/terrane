// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let numbers: terrane_collection_support::List<terrane_int_support::Int>;
    let batch: witness::Batch<i64>;
    let snapshot: terrane_collection_support::List<terrane_int_support::Int>;
    numbers = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128),
            terrane_int_support::Int::from(3_i128)
        ],
    );
    batch = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::Batch::<i64> {
                values: match || -> Result<_, crate::TerraneForeignError> {
                    Ok(
                        numbers
                            .into_vec()
                            .into_iter()
                            .map(|item| -> Result<_, crate::TerraneForeignError> {
                                Ok(
                                    terrane_int_support::coerce::<i64>(&item)
                                        .map_err(|error| crate::TerraneForeignError(
                                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                        ))?,
                                )
                            })
                            .collect::<Result<std::vec::Vec<i64>, _>>()?,
                    )
                }() {
                    Ok(value) => value,
                    Err(error) => std::panic::panic_any(error),
                },
                enabled: true,
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "Batch::<i64>::terrane_construct",
                    ),
                )
            }
        },
        0 /* terrane-site: src/main.trn:8:13-8:61 */,
    );
    snapshot = terrane_collection_support::List::new(
        (&batch.values)
            .iter()
            .map(|__terrane_field_item| terrane_int_support::Int::from(
                i128::from(*__terrane_field_item),
            ))
            .collect(),
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(snapshot
        .length()))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(total(batch),
        1 /* terrane-site: src/main.trn:11:13-11:30 */))
    );
}
