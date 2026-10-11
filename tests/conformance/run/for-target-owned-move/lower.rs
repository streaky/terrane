// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: for-target-owned-move
fn main() {
    let row: terrane_collection_support::List<terrane_int_support::Int>;
    let rows: terrane_collection_support::List<
        terrane_collection_support::List<terrane_int_support::Int>,
    >;
    let mut item: terrane_collection_support::List<terrane_int_support::Int>;
    let mut destination: terrane_collection_support::List<terrane_int_support::Int>;
    row = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(99_i128)]);
    rows = terrane_collection_support::List::<
        terrane_collection_support::List<terrane_int_support::Int>,
    >::new(
        vec![
            terrane_collection_support::List::< terrane_int_support::Int
            >::new(vec![terrane_int_support::Int::from(1_i128)]),
            terrane_collection_support::List::< terrane_int_support::Int
            >::new(vec![terrane_int_support::Int::from(2_i128)]),
            terrane_collection_support::List::< terrane_int_support::Int
            >::new(vec![terrane_int_support::Int::from(3_i128)])
        ],
    );
    let __terrane_iterable_0 = rows;
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        item = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        destination = item;
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(destination
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            0 /* terrane-site: case.trn:10:12-10:26 */)), 0 /* terrane-site: case.trn:10:12-10:26 */))
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(row
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        1 /* terrane-site: case.trn:11:10-11:16 */)), 1 /* terrane-site: case.trn:11:10-11:16 */))
    );
}
