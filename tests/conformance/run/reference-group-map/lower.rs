// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: reference-group-map
fn main() {
    let values: terrane_collection_support::List<terrane_int_support::Int> = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(7_i128)]);
    let grouped: &terrane_int_support::Int = {
        let __terrane_index = __terrane_raised(
            terrane_collection_support::index_from_int(
                &terrane_int_support::Int::from(0_i128),
            ),
            0 /* terrane-site: case.trn:6:26-6:35 */,
        );
        __terrane_raised(
            values
                .get(__terrane_index)
                .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                    __terrane_index,
                )),
            0 /* terrane-site: case.trn:6:26-6:35 */,
        )
    };
    let ordered: terrane_collection_support::Map<String, terrane_int_support::Int> = terrane_collection_support::Map::<
        String,
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("answer"),
            terrane_int_support::Int::from(42_i128))
        ],
    );
    let ordered_value: &terrane_int_support::Int = __terrane_raised(
        ordered
            .get(&String::from("answer"))
            .ok_or(terrane_collection_support::MissingKey),
        1 /* terrane-site: case.trn:8:31-8:48 */,
    );
    let unordered: terrane_collection_support::UnorderedMap<
        String,
        terrane_int_support::Int,
    > = terrane_collection_support::UnorderedMap::<
        String,
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("answer"),
            terrane_int_support::Int::from(43_i128))
        ],
    );
    let unordered_value: &terrane_int_support::Int = __terrane_raised(
        unordered
            .get(&String::from("answer"))
            .ok_or(terrane_collection_support::MissingKey),
        2 /* terrane-site: case.trn:10:33-10:52 */,
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&grouped.clone()),
        terrane_scalar_support::scalar_text(&ordered_value.clone()),
        terrane_scalar_support::scalar_text(&unordered_value.clone())
    );
}
