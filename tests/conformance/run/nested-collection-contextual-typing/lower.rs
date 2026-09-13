// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: nested-collection-contextual-typing
fn main() {
    let outer: terrane_collection_support::List<terrane_collection_support::List<i8>> = terrane_collection_support::List::<
        terrane_collection_support::List<i8>,
    >::new(vec![terrane_collection_support::List::< i8 >::new(vec![5, 6])]);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(__terrane_raised(outer
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        0 /* terrane-site: case.trn:6:10-6:18 */)), 0 /* terrane-site: case.trn:6:10-6:18 */)
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        1 /* terrane-site: case.trn:6:10-6:21 */)), 1 /* terrane-site: case.trn:6:10-6:21 */))
    );
}
