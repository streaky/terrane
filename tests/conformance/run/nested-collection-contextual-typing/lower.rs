// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: nested-collection-contextual-typing
fn main() {
    let outer: terrane_collection_support::List<terrane_collection_support::List<i8>>;
    outer = terrane_collection_support::List::<
        terrane_collection_support::List<i8>,
    >::new(vec![terrane_collection_support::List::< i8 >::new(vec![5, 6])]);
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_0 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        0 /* terrane-site: case.trn:5:10-5:18 */); let __terrane_index_place_1 =
        __terrane_raised(outer.get(__terrane_index_0).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_0)),
        0 /* terrane-site: case.trn:5:10-5:18 */);
        __terrane_raised(__terrane_index_place_1
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        1 /* terrane-site: case.trn:5:10-5:21 */)), 1 /* terrane-site: case.trn:5:10-5:21 */) })
    );
}
