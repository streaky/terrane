// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: map-entry-destructuring
fn main() {
    let ordered: terrane_collection_support::Map<String, terrane_int_support::Int> = terrane_collection_support::Map::<
        String,
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("first"),
            terrane_int_support::Int::from(1_i128)),
            terrane_collection_support::Entry::new(String::from("second"),
            terrane_int_support::Int::from(2_i128))
        ],
    );
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &ordered,
    );
    loop {
        let __terrane_item_0 = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let mut key = __terrane_item_0.key;
        let mut value = __terrane_item_0.value;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&key),
            terrane_scalar_support::scalar_text(&value)
        );
        key = String::from("seen");
        value = value.clone() + terrane_int_support::Int::from(1_i128);
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&key),
            terrane_scalar_support::scalar_text(&value)
        );
    }
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(ordered
        .get_or_error(&String::from("first")), 0 /* terrane-site: case.trn:11:10-11:26 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(ordered
        .get_or_error(&String::from("second")), 1 /* terrane-site: case.trn:11:28-11:45 */))
    );
    let deterministic: terrane_collection_support::UnorderedMap<
        String,
        terrane_int_support::Int,
    > = terrane_collection_support::UnorderedMap::<
        String,
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("only"),
            terrane_int_support::Int::from(7_i128))
        ],
    );
    let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
        &deterministic,
    );
    loop {
        let __terrane_item_1 = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let key = __terrane_item_1.key;
        let value = __terrane_item_1.value;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&key),
            terrane_scalar_support::scalar_text(&value)
        );
    }
}
