// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: entry-destructuring-binding-events
fn main() {
    let second: i64 = 3;
    let _ = &second;
    let entries: terrane_collection_support::List<
        terrane_collection_support::Entry<String, terrane_int_support::Int>,
    > = terrane_collection_support::List::<
        terrane_collection_support::Entry<String, terrane_int_support::Int>,
    >::new(
        vec![
            terrane_collection_support::Entry::< String, terrane_int_support::Int
            >::new(String::from("a"), terrane_int_support::Int::from(1_i128))
        ],
    );
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &entries,
    );
    loop {
        let __terrane_item_0 = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let key = __terrane_item_0.key;
        let mut second = __terrane_item_0.value;
        let _ = &second;
        second = terrane_int_support::Int::from(9_i128);
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&key),
            terrane_scalar_support::scalar_text(&second)
        );
    }
}
