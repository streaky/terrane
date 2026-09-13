// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: tuple-type-boundaries
fn echo(
    values: terrane_collection_support::Tuple<terrane_int_support::Int>,
) -> terrane_collection_support::Tuple<terrane_int_support::Int> {
    return values.clone();
}
fn nested(
    values: terrane_collection_support::List<
        terrane_collection_support::Tuple<terrane_int_support::Int>,
    >,
) -> terrane_collection_support::List<
    terrane_collection_support::Tuple<terrane_int_support::Int>,
> {
    return values.clone();
}
fn main() {
    let pair: terrane_collection_support::Tuple<terrane_int_support::Int> = terrane_collection_support::Tuple::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128)
        ],
    );
    let returned: terrane_collection_support::Tuple<terrane_int_support::Int> = echo(
        pair,
    );
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(returned
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(returned
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        0 /* terrane-site: case.trn:13:27-13:38 */)), 0 /* terrane-site: case.trn:13:27-13:38 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(returned
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        1 /* terrane-site: case.trn:13:40-13:51 */)), 1 /* terrane-site: case.trn:13:40-13:51 */))
    );
    let empty: terrane_collection_support::Tuple<terrane_int_support::Int> = terrane_collection_support::Tuple::<
        terrane_int_support::Int,
    >::new(Vec::new());
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(empty
        .length()))
    );
    let groups: terrane_collection_support::List<
        terrane_collection_support::Tuple<terrane_int_support::Int>,
    > = terrane_collection_support::List::<
        terrane_collection_support::Tuple<terrane_int_support::Int>,
    >::new(
        vec![
            terrane_collection_support::Tuple::< terrane_int_support::Int
            >::new(vec![terrane_int_support::Int::from(3_i128),
            terrane_int_support::Int::from(4_i128)]),
            terrane_collection_support::Tuple::< terrane_int_support::Int
            >::new(vec![terrane_int_support::Int::from(5_i128)])
        ],
    );
    let echoed: terrane_collection_support::List<
        terrane_collection_support::Tuple<terrane_int_support::Int>,
    > = nested(groups);
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(echoed
        .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(__terrane_raised(echoed
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        2 /* terrane-site: case.trn:18:25-18:34 */)), 2 /* terrane-site: case.trn:18:25-18:34 */)
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        3 /* terrane-site: case.trn:18:25-18:37 */)), 3 /* terrane-site: case.trn:18:25-18:37 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(__terrane_raised(echoed
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        4 /* terrane-site: case.trn:18:39-18:48 */)), 4 /* terrane-site: case.trn:18:39-18:48 */)
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        5 /* terrane-site: case.trn:18:39-18:51 */)), 5 /* terrane-site: case.trn:18:39-18:51 */))
    );
}
