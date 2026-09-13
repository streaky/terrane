// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: loop-carried-store-reads
fn main() {
    let mut while_value: i8 = 0;
    let mut limit: i8 = 0;
    while limit < 2 {
        println!("{}", terrane_scalar_support::scalar_text(&while_value));
        while_value = 5;
        limit = __terrane_raised(
            terrane_int_support::fixed_addition(limit, 1),
            0 /* terrane-site: case.trn:8:13-8:22 */,
        );
    }
    let mut for_value: i8 = 0;
    let mut __terrane_iterator_0 = terrane_collection_support::string_iterator(
        &String::from("ab"),
    );
    loop {
        let character = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &character;
        println!("{}", terrane_scalar_support::scalar_text(&for_value));
        for_value = 7;
    }
}
