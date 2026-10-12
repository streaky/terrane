// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: entry-valued-maps
fn main() {
    let inner: terrane_collection_support::Entry<String, i8>;
    let ordered_bound: terrane_collection_support::Map<
        String,
        terrane_collection_support::Entry<String, i8>,
    >;
    let ordered_inline: terrane_collection_support::Map<
        String,
        terrane_collection_support::Entry<String, i8>,
    >;
    let unordered_bound: terrane_collection_support::UnorderedMap<
        String,
        terrane_collection_support::Entry<String, i8>,
    >;
    let unordered_inline: terrane_collection_support::UnorderedMap<
        String,
        terrane_collection_support::Entry<String, i8>,
    >;
    let inferred_ordered: terrane_collection_support::Map<
        String,
        terrane_collection_support::Entry<String, terrane_int_support::Int>,
    >;
    let inferred_unordered: terrane_collection_support::UnorderedMap<
        String,
        terrane_collection_support::Entry<String, terrane_int_support::Int>,
    >;
    inner = terrane_collection_support::Entry::<String, i8>::new(String::from("b"), 7);
    ordered_bound = terrane_collection_support::Map::<
        String,
        terrane_collection_support::Entry<String, i8>,
    >::new(
        vec![terrane_collection_support::Entry::new(String::from("a"), inner.clone())],
    );
    ordered_inline = terrane_collection_support::Map::<
        String,
        terrane_collection_support::Entry<String, i8>,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("a"),
            terrane_collection_support::Entry::< String, i8 >::new(String::from("c"), 8))
        ],
    );
    unordered_bound = terrane_collection_support::UnorderedMap::<
        String,
        terrane_collection_support::Entry<String, i8>,
    >::new(vec![terrane_collection_support::Entry::new(String::from("a"), inner)]);
    unordered_inline = terrane_collection_support::UnorderedMap::<
        String,
        terrane_collection_support::Entry<String, i8>,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("a"),
            terrane_collection_support::Entry::< String, i8 >::new(String::from("d"), 9))
        ],
    );
    inferred_ordered = terrane_collection_support::Map::<
        String,
        terrane_collection_support::Entry<String, terrane_int_support::Int>,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("a"),
            terrane_collection_support::Entry::< String, terrane_int_support::Int
            >::new(String::from("e"), terrane_int_support::Int::from(6_i128)))
        ],
    );
    inferred_unordered = terrane_collection_support::UnorderedMap::<
        String,
        terrane_collection_support::Entry<String, terrane_int_support::Int>,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("a"),
            terrane_collection_support::Entry::< String, terrane_int_support::Int
            >::new(String::from("f"), terrane_int_support::Int::from(5_i128)))
        ],
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(ordered_bound
        .get_or_error(&String::from("a")), 0 /* terrane-site: case.trn:12:10-12:28 */)
        .key.clone()),
        terrane_scalar_support::scalar_text(&__terrane_raised(ordered_bound
        .get_or_error(&String::from("a")), 1 /* terrane-site: case.trn:12:34-12:52 */)
        .value.clone())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(ordered_inline
        .get_or_error(&String::from("a")), 2 /* terrane-site: case.trn:13:10-13:29 */)
        .key.clone()),
        terrane_scalar_support::scalar_text(&__terrane_raised(ordered_inline
        .get_or_error(&String::from("a")), 3 /* terrane-site: case.trn:13:35-13:54 */)
        .value.clone())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(unordered_bound
        .get_or_error(&String::from("a")), 4 /* terrane-site: case.trn:14:10-14:30 */)
        .key.clone()),
        terrane_scalar_support::scalar_text(&__terrane_raised(unordered_bound
        .get_or_error(&String::from("a")), 5 /* terrane-site: case.trn:14:36-14:56 */)
        .value.clone())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(unordered_inline
        .get_or_error(&String::from("a")), 6 /* terrane-site: case.trn:15:10-15:31 */)
        .key.clone()),
        terrane_scalar_support::scalar_text(&__terrane_raised(unordered_inline
        .get_or_error(&String::from("a")), 7 /* terrane-site: case.trn:15:37-15:58 */)
        .value.clone())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(inferred_ordered
        .get_or_error(&String::from("a")), 8 /* terrane-site: case.trn:16:10-16:31 */)
        .key.clone()),
        terrane_scalar_support::scalar_text(&__terrane_raised(inferred_ordered
        .get_or_error(&String::from("a")), 9 /* terrane-site: case.trn:16:37-16:58 */)
        .value.clone())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(inferred_unordered
        .get_or_error(&String::from("a")), 10 /* terrane-site: case.trn:17:10-17:33 */).key.clone()),
        terrane_scalar_support::scalar_text(&__terrane_raised(inferred_unordered
        .get_or_error(&String::from("a")), 11 /* terrane-site: case.trn:17:39-17:62 */).value.clone())
    );
}
