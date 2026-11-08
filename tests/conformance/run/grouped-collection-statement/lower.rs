// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: grouped-collection-statement
fn main() {
    let mut values: terrane_collection_support::List<terrane_int_support::Int>;
    values = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(1_i128)]);
    {
        let __terrane_collection_argument_0 = terrane_int_support::Int::from(2_i128);
        values.append(__terrane_collection_argument_0);
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(values
        .length()))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(values
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        0 /* terrane-site: case.trn:7:12-7:21 */)), 0 /* terrane-site: case.trn:7:12-7:21 */))
    );
}
