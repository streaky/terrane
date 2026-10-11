// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: collection-contextual-typing
fn main() {
    let values: terrane_collection_support::List<terrane_int_support::Int>;
    let found: Option<terrane_int_support::Int>;
    let narrow: terrane_collection_support::List<i8>;
    values = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(1_i128)]);
    found = terrane_collection_support::index_from_int(
            &terrane_int_support::Int::from(0_i128),
        )
        .ok()
        .and_then(|index| values.get(index).cloned());
    narrow = terrane_collection_support::List::<i8>::new(vec![5, 6]);
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(narrow
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        0 /* terrane-site: case.trn:7:10-7:19 */)), 0 /* terrane-site: case.trn:7:10-7:19 */))
    );
    if found.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &found { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
}
