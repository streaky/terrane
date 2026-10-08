// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: for-loops
fn show(mut value: String) {
    println!("{}", terrane_scalar_support::scalar_text(&value));
    value = String::from("parameter");
    println!("{}", terrane_scalar_support::scalar_text(&value));
}
fn main() {
    let text: String;
    let mut character: String;
    let mut index: terrane_int_support::Int;
    let mut skipped: terrane_int_support::Int;
    show(String::from("original"));
    text = String::from("e\u{301}x");
    let __terrane_iterable_0 = text.clone();
    let mut __terrane_iterator_0 = terrane_collection_support::string_iterator(
        &__terrane_iterable_0,
    );
    loop {
        character = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&character));
        character = String::from("loop");
        println!("{}", terrane_scalar_support::scalar_text(&character));
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&(terrane_string_support::length(&text)
        as i128))
    );
    index = terrane_int_support::Int::from(0_i128);
    '__terrane_break_1: while index.clone() < terrane_int_support::Int::from(3_i128) {
        '__terrane_continue_1: {
            if index.clone() == terrane_int_support::Int::from(1_i128) {
                break '__terrane_continue_1;
            }
            println!("{}", terrane_scalar_support::scalar_text(&index));
        }
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    println!("{}", terrane_scalar_support::scalar_text(&index));
    skipped = terrane_int_support::Int::from(7_i128);
    '__terrane_break_2: while skipped.clone() < terrane_int_support::Int::from(7_i128) {
        '__terrane_continue_2: {
            println!(
                "{}", terrane_scalar_support::scalar_text(&String::from("unreachable"))
            );
        }
        skipped = skipped.clone() + terrane_int_support::Int::from(1_i128);
    }
    println!("{}", terrane_scalar_support::scalar_text(&skipped));
}
