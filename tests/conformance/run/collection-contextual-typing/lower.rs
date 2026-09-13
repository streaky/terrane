// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: collection-contextual-typing
fn main() {
    let values: terrane_collection_support::List<terrane_int_support::Int> = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(1_i128)]);
    let found: Option<terrane_int_support::Int> = terrane_collection_support::index_from_int(
            &terrane_int_support::Int::from(0_i128),
        )
        .ok()
        .and_then(|index| values.get(index).cloned());
    let narrow: terrane_collection_support::List<i8> = terrane_collection_support::List::<
        i8,
    >::new(vec![5, 6]);
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(narrow
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        0 /* terrane-site: case.trn:8:10-8:19 */)), 0 /* terrane-site: case.trn:8:10-8:19 */))
    );
    if found.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* found.as_ref()
            .expect("semantic optional narrowing"))
        );
    }
}
