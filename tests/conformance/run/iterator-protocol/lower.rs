// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: iterator-protocol
fn main() {
    let mut values: terrane_collection_support::Iterator<()> = terrane_collection_support::Iterator::<
        (),
    >::new(vec![(), ()]);
    let mut __terrane_iterator_0 = &mut values;
    loop {
        let value = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&value));
    }
    let mut exhausted: terrane_collection_support::Iterator<terrane_int_support::Int> = terrane_collection_support::Iterator::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(1_i128)]);
    {
        let _ = exhausted.next();
    };
    {
        let _ = exhausted.next();
    };
    {
        let _ = exhausted.next();
    };
    println!("{}", terrane_scalar_support::scalar_text(&String::from("end")));
    let text: String = String::from("A👍🏽");
    let mut __terrane_iterator_1 = terrane_collection_support::string_iterator(&text);
    loop {
        let grapheme = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&grapheme));
    }
}
