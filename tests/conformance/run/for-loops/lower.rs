// Generated deterministically by Terrane <version>.
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
    {
        let __terrane_iterable_0 = &text;
        let mut __terrane_iterator_0 = terrane_string_support::graphemes(
            &__terrane_iterable_0,
        );
        loop {
            character = match __terrane_iterator_0.next() {
                Some(item) => item,
                None => break,
            };
            println!("{}", terrane_scalar_support::scalar_text(&character));
            character = String::from("loop");
            println!("{}", terrane_scalar_support::scalar_text(&character));
        }
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&(terrane_string_support::length(&text)
        as i128))
    );
    index = terrane_int_support::Int::from(0_i128);
    '__terrane_break_1: while index < terrane_int_support::Int::from(3_i128) {
        '__terrane_continue_1: {
            if index == terrane_int_support::Int::from(1_i128) {
                break '__terrane_continue_1;
            }
            println!("{}", terrane_scalar_support::scalar_text(&index));
        }
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    println!("{}", terrane_scalar_support::scalar_text(&index));
    skipped = terrane_int_support::Int::from(7_i128);
    '__terrane_break_2: while skipped < terrane_int_support::Int::from(7_i128) {
        '__terrane_continue_2: {
            println!("{}", String::from("unreachable"));
        }
        skipped = skipped.clone() + terrane_int_support::Int::from(1_i128);
    }
    println!("{}", terrane_scalar_support::scalar_text(&skipped));
}
