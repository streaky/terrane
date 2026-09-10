// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: string-result-list-iteration
fn main() {
    let parts: Vec<String> = terrane_string_support::split(
        &String::from("red,green,blue"),
        &String::from(","),
    );
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(&parts);
    loop {
        let part = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&part));
    }
    println!("{}", terrane_scalar_support::scalar_text(&(parts.len() as i128)));
    let mut __terrane_iterator_1 = terrane_collection_support::slice_iterator(&parts);
    loop {
        let part = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&part));
    }
    let matches: Vec<terrane_string_support::TextRange> = terrane_string_support::find_all(
        &String::from("banana"),
        &String::from("an"),
    );
    let mut __terrane_iterator_2 = terrane_collection_support::slice_iterator(&matches);
    loop {
        let found = match __terrane_iterator_2.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&found.text().to_owned()));
    }
    println!("{}", terrane_scalar_support::scalar_text(&(matches.len() as i128)));
}
