// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: nested-collection-for-binding
fn main() {
    let nested: terrane_collection_support::List<
        terrane_collection_support::List<terrane_int_support::Int>,
    > = terrane_collection_support::List::<
        terrane_collection_support::List<terrane_int_support::Int>,
    >::new(
        vec![
            terrane_collection_support::List::< terrane_int_support::Int
            >::new(vec![terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128)]), terrane_collection_support::List::<
            terrane_int_support::Int >::new(vec![terrane_int_support::Int::from(3_i128)])
        ],
    );
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &nested,
    );
    loop {
        let row = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}{}",
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(row
            .length())), terrane_scalar_support::scalar_text(&__terrane_raised(row
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            0 /* terrane-site: case.trn:8:24-8:30 */)), 0 /* terrane-site: case.trn:8:24-8:30 */))
        );
    }
    let groups: terrane_collection_support::Map<
        String,
        terrane_collection_support::List<terrane_int_support::Int>,
    > = terrane_collection_support::Map::<
        String,
        terrane_collection_support::List<terrane_int_support::Int>,
    >::new(
        vec![
            terrane_collection_support::Entry::< String, terrane_collection_support::List
            < terrane_int_support::Int >>::new(String::from("a"),
            terrane_collection_support::List::< terrane_int_support::Int
            >::new(vec![terrane_int_support::Int::from(4_i128),
            terrane_int_support::Int::from(5_i128)]))
        ],
    );
    let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
        &groups,
    );
    loop {
        let group = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(group.value
            .clone()
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            1 /* terrane-site: case.trn:11:12-11:26 */)), 1 /* terrane-site: case.trn:11:12-11:26 */))
        );
    }
}
