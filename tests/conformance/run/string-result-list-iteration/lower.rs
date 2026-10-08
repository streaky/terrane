// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: string-result-list-iteration
fn main() {
    let parts_terrane_f0_s57: Vec<String>;
    let mut part_terrane_f0_s99: String;
    let matches_terrane_f0_s191: Vec<terrane_string_support::TextRange>;
    let mut found_terrane_f0_s231: terrane_string_support::TextRange;
    parts_terrane_f0_s57 = terrane_string_support::split(
        &String::from("red,green,blue"),
        &String::from(","),
    );
    let __terrane_iterable_0 = parts_terrane_f0_s57.clone();
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        &__terrane_iterable_0,
    );
    loop {
        part_terrane_f0_s99 = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&part_terrane_f0_s99));
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&(parts_terrane_f0_s57.len() as i128))
    );
    let __terrane_iterable_1 = parts_terrane_f0_s57;
    let mut __terrane_iterator_1 = terrane_collection_support::slice_iterator(
        &__terrane_iterable_1,
    );
    loop {
        part_terrane_f0_s99 = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &part_terrane_f0_s99;
        println!("{}", terrane_scalar_support::scalar_text(&part_terrane_f0_s99));
    }
    matches_terrane_f0_s191 = terrane_string_support::find_all(
        &String::from("banana"),
        &String::from("an"),
    );
    let __terrane_iterable_2 = matches_terrane_f0_s191.clone();
    let mut __terrane_iterator_2 = terrane_collection_support::slice_iterator(
        &__terrane_iterable_2,
    );
    loop {
        found_terrane_f0_s231 = match __terrane_iterator_2.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&found_terrane_f0_s231.text()
            .to_owned())
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&(matches_terrane_f0_s191.len() as
        i128))
    );
}
