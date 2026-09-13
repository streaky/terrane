// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: collection-property-method-calls
fn main() {
    let values: terrane_collection_support::Map<
        String,
        terrane_collection_support::List<terrane_int_support::Int>,
    > = terrane_collection_support::Map::<
        String,
        terrane_collection_support::List<terrane_int_support::Int>,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("alpha"),
            terrane_collection_support::List::< terrane_int_support::Int
            >::new(vec![terrane_int_support::Int::from(3_i128),
            terrane_int_support::Int::from(1_i128)]))
        ],
    );
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &values.entries(),
    );
    loop {
        let pair = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let appended: terrane_collection_support::List<terrane_int_support::Int> = {
            let collection = &mut pair.value.clone();
            collection.append(terrane_int_support::Int::from(2_i128));
            collection.clone()
        };
        let sorted: terrane_collection_support::List<terrane_int_support::Int> = {
            let collection = &mut pair.value.clone();
            collection.sort_by(|left, right| left.cmp(right));
            collection.clone()
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&pair.key.clone()
            .contains(&String::from("al")))
        );
        println!(
            "{}{}",
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(appended
            .length())), terrane_scalar_support::scalar_text(&__terrane_raised(appended
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
            0 /* terrane-site: case.trn:10:29-10:40 */)), 0 /* terrane-site: case.trn:10:29-10:40 */))
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(sorted
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            1 /* terrane-site: case.trn:11:12-11:21 */)), 1 /* terrane-site: case.trn:11:12-11:21 */)),
            terrane_scalar_support::scalar_text(&__terrane_raised(sorted
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            2 /* terrane-site: case.trn:11:23-11:32 */)), 2 /* terrane-site: case.trn:11:23-11:32 */))
        );
    }
}
