// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: nested-collection-for-binding
fn main() {
    let nested: terrane_collection_support::List<
        terrane_collection_support::List<terrane_int_support::Int>,
    >;
    let mut row: terrane_collection_support::List<terrane_int_support::Int>;
    let groups: terrane_collection_support::Map<
        String,
        terrane_collection_support::List<terrane_int_support::Int>,
    >;
    let mut group: terrane_collection_support::Entry<
        String,
        terrane_collection_support::List<terrane_int_support::Int>,
    >;
    nested = terrane_collection_support::List::<
        terrane_collection_support::List<terrane_int_support::Int>,
    >::new(
        vec![
            terrane_collection_support::List::< terrane_int_support::Int
            >::new(vec![terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128)]), terrane_collection_support::List::<
            terrane_int_support::Int >::new(vec![terrane_int_support::Int::from(3_i128)])
        ],
    );
    let __terrane_iterable_0 = nested;
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        row = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}{}",
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(row
            .length())), terrane_scalar_support::scalar_text(&__terrane_raised(row
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            0 /* terrane-site: case.trn:7:24-7:30 */)), 0 /* terrane-site: case.trn:7:24-7:30 */))
        );
    }
    groups = terrane_collection_support::Map::<
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
    let __terrane_iterable_1 = groups;
    let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_1,
    );
    loop {
        group = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value =
            &group.value.clone(); __terrane_raised(__terrane_receiver_value
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            1 /* terrane-site: case.trn:10:12-10:26 */)), 1 /* terrane-site: case.trn:10:12-10:26 */) })
        );
    }
}
