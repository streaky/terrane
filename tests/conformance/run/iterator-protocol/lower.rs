// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support
// Source: case.trn
// Namespace: iterator-protocol
fn main() {
    let mut values_terrane_f0_s84: terrane_collection_support::Iterator<()>;
    let mut value_terrane_f0_s120: ();
    let mut exhausted_terrane_f0_s155: terrane_collection_support::Iterator<
        terrane_int_support::Int,
    >;
    let text_terrane_f0_s250: String;
    let mut grapheme_terrane_f0_s275: String;
    values_terrane_f0_s84 = terrane_collection_support::Iterator::<
        (),
    >::new(vec![(), ()]);
    let mut __terrane_iterator_0 = &mut values_terrane_f0_s84;
    loop {
        value_terrane_f0_s120 = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&value_terrane_f0_s120));
    }
    exhausted_terrane_f0_s155 = terrane_collection_support::Iterator::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(1_i128)]);
    {
        let _ = exhausted_terrane_f0_s155.next();
    };
    {
        let _ = exhausted_terrane_f0_s155.next();
    };
    {
        let _ = exhausted_terrane_f0_s155.next();
    };
    println!("{}", terrane_scalar_support::scalar_text(&String::from("end")));
    text_terrane_f0_s250 = String::from("A👍🏽");
    let __terrane_iterable_1 = text_terrane_f0_s250;
    let mut __terrane_iterator_1 = terrane_collection_support::string_iterator(
        &__terrane_iterable_1,
    );
    loop {
        grapheme_terrane_f0_s275 = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&grapheme_terrane_f0_s275));
    }
}
