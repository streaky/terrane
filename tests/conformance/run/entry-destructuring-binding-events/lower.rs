// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support
// Source: case.trn
// Namespace: entry-destructuring-binding-events
fn main() {
    let mut second_terrane_f0_s105: terrane_int_support::Int;
    let entries_terrane_f0_s118: terrane_collection_support::List<
        terrane_collection_support::Entry<String, terrane_int_support::Int>,
    >;
    let mut key_terrane_f0_s166: String;
    second_terrane_f0_s105 = terrane_int_support::Int::from(3_i128);
    let _ = &mut second_terrane_f0_s105;
    entries_terrane_f0_s118 = terrane_collection_support::List::<
        terrane_collection_support::Entry<String, terrane_int_support::Int>,
    >::new(
        vec![
            terrane_collection_support::Entry::< String, terrane_int_support::Int
            >::new(String::from("a"), terrane_int_support::Int::from(1_i128))
        ],
    );
    let __terrane_iterable_0 = entries_terrane_f0_s118;
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        let __terrane_item_0 = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        key_terrane_f0_s166 = __terrane_item_0.key;
        second_terrane_f0_s105 = __terrane_item_0.value;
        let _ = &second_terrane_f0_s105;
        second_terrane_f0_s105 = terrane_int_support::Int::from(9_i128);
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&key_terrane_f0_s166),
            terrane_scalar_support::scalar_text(&second_terrane_f0_s105)
        );
    }
}
