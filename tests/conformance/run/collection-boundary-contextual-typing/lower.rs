// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: collection-boundary-contextual-typing
fn take(values: terrane_collection_support::List<i8>) {
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(values
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        0 /* terrane-site: case.trn:4:10-4:19 */)), 0 /* terrane-site: case.trn:4:10-4:19 */))
    );
}
fn make() -> terrane_collection_support::List<i8> {
    return terrane_collection_support::List::<i8>::new(vec![5, 6]);
}
fn take_entry_map(
    values: terrane_collection_support::Map<
        String,
        terrane_collection_support::Entry<String, i8>,
    >,
) {
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(values
        .get_or_error(&String::from("a")), 1 /* terrane-site: case.trn:8:10-8:21 */)
        .key), terrane_scalar_support::scalar_text(&__terrane_raised(values
        .get_or_error(&String::from("a")), 2 /* terrane-site: case.trn:8:27-8:38 */)
        .value)
    );
}
fn make_entry_map() -> terrane_collection_support::Map<
    String,
    terrane_collection_support::Entry<String, i8>,
> {
    return terrane_collection_support::Map::<
        String,
        terrane_collection_support::Entry<String, i8>,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("a"),
            terrane_collection_support::Entry::< String, i8 >::new(String::from("c"), 8))
        ],
    );
}
fn take_nested_map(
    values: terrane_collection_support::Map<
        String,
        terrane_collection_support::Map<String, i8>,
    >,
) {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(__terrane_raised(values
        .get_or_error(&String::from("a")), 3 /* terrane-site: case.trn:12:10-12:21 */)
        .get_or_error(&String::from("b")), 4 /* terrane-site: case.trn:12:10-12:26 */))
    );
}
fn take_nested_list(
    values: terrane_collection_support::List<terrane_collection_support::List<i8>>,
) {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(__terrane_raised(values
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        5 /* terrane-site: case.trn:14:10-14:19 */)), 5 /* terrane-site: case.trn:14:10-14:19 */)
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        6 /* terrane-site: case.trn:14:10-14:22 */)), 6 /* terrane-site: case.trn:14:10-14:22 */))
    );
}
fn take_map_list(
    values: terrane_collection_support::Map<String, terrane_collection_support::List<i8>>,
) {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(__terrane_raised(values
        .get_or_error(&String::from("a")), 7 /* terrane-site: case.trn:16:10-16:21 */)
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        8 /* terrane-site: case.trn:16:10-16:24 */)), 8 /* terrane-site: case.trn:16:10-16:24 */))
    );
}
fn main() {
    let pair: terrane_collection_support::Entry<String, i8>;
    let keyed: terrane_collection_support::Map<i8, String>;
    let made: terrane_collection_support::List<i8>;
    let made_entries: terrane_collection_support::Map<
        String,
        terrane_collection_support::Entry<String, i8>,
    >;
    pair = terrane_collection_support::Entry::<String, i8>::new(String::from("a"), 6);
    keyed = terrane_collection_support::Map::<
        i8,
        String,
    >::new(
        vec![
            terrane_collection_support::Entry::< i8, String >::new(5, String::from("x"))
        ],
    );
    take(terrane_collection_support::List::<i8>::new(vec![5, 6]));
    take_entry_map(
        terrane_collection_support::Map::<
            String,
            terrane_collection_support::Entry<String, i8>,
        >::new(
            vec![
                terrane_collection_support::Entry::new(String::from("a"),
                terrane_collection_support::Entry::< String, i8 >::new(String::from("b"),
                7))
            ],
        ),
    );
    take_nested_map(
        terrane_collection_support::Map::<
            String,
            terrane_collection_support::Map<String, i8>,
        >::new(
            vec![
                terrane_collection_support::Entry::new(String::from("a"),
                terrane_collection_support::Map::< String, i8
                >::new(vec![terrane_collection_support::Entry::new(String::from("b"),
                7)]))
            ],
        ),
    );
    take_nested_list(
        terrane_collection_support::List::<
            terrane_collection_support::List<i8>,
        >::new(vec![terrane_collection_support::List::< i8 >::new(vec![5, 6])]),
    );
    take_map_list(
        terrane_collection_support::Map::<
            String,
            terrane_collection_support::List<i8>,
        >::new(
            vec![
                terrane_collection_support::Entry::new(String::from("a"),
                terrane_collection_support::List::< i8 >::new(vec![9]))
            ],
        ),
    );
    made = make();
    made_entries = make_entry_map();
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&pair.value),
        terrane_scalar_support::scalar_text(&__terrane_raised(keyed.get_or_error(&5),
        9 /* terrane-site: case.trn:27:22-27:30 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(made
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        10 /* terrane-site: case.trn:27:32-27:39 */)), 10 /* terrane-site: case.trn:27:32-27:39 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(made_entries
        .get_or_error(&String::from("a")), 11 /* terrane-site: case.trn:27:41-27:58 */).key),
        terrane_scalar_support::scalar_text(&__terrane_raised(made_entries
        .get_or_error(&String::from("a")), 12 /* terrane-site: case.trn:27:64-27:81 */).value)
    );
}
