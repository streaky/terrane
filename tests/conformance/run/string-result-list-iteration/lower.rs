// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: string-result-list-iteration
fn main() {
    let parts: Vec<String>;
    let mut part: String;
    let matches: Vec<terrane_string_support::TextRange>;
    let mut found: terrane_string_support::TextRange;
    parts = {
        let __terrane_receiver_value_0 = &String::from("red,green,blue");
        let __terrane_string_argument_0 = ",";
        terrane_string_support::split(
            &__terrane_receiver_value_0,
            &std::convert::AsRef::<str>::as_ref(&__terrane_string_argument_0),
        )
    };
    let __terrane_iterable_0 = parts.clone();
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        &__terrane_iterable_0,
    );
    loop {
        part = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&part));
    }
    println!("{}", terrane_scalar_support::scalar_text(&(parts.len() as i128)));
    let __terrane_iterable_1 = parts;
    let mut __terrane_iterator_1 = terrane_collection_support::slice_iterator(
        &__terrane_iterable_1,
    );
    loop {
        part = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &part;
        println!("{}", terrane_scalar_support::scalar_text(&part));
    }
    matches = {
        let __terrane_receiver_value_1 = &String::from("banana");
        let __terrane_string_argument_0 = "an";
        terrane_string_support::find_all(
            &__terrane_receiver_value_1,
            &std::convert::AsRef::<str>::as_ref(&__terrane_string_argument_0),
        )
    };
    let __terrane_iterable_2 = matches.clone();
    let mut __terrane_iterator_2 = terrane_collection_support::slice_iterator(
        &__terrane_iterable_2,
    );
    loop {
        found = match __terrane_iterator_2.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&found.text().to_owned()));
    }
    println!("{}", terrane_scalar_support::scalar_text(&(matches.len() as i128)));
}
