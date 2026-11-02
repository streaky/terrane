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
    batch = {
        let __terrane_conversion_input_0 = numbers;
        let __terrane_conversion_input_1 = true;
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(witness::Batch::<i64> {
                    values: __terrane_conversion_input_0
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
                    enabled: __terrane_conversion_input_1,
                })
            }(),
            0 /* terrane-site: src/main.trn:8:13-8:61 */,
        )
    };
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
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        batch; __terrane_raised(total(__terrane_conversion_input_0),
        1 /* terrane-site: src/main.trn:11:13-11:30 */) })
    );
}
