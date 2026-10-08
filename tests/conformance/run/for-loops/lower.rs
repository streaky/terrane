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
    let text_terrane_f0_s137: String;
    let mut character_terrane_f0_s164: String;
    let mut index_terrane_f0_s274: terrane_int_support::Int;
    let mut skipped_terrane_f0_s375: terrane_int_support::Int;
    show(String::from("original"));
    text_terrane_f0_s137 = String::from("e\u{301}x");
    let __terrane_iterable_0 = text_terrane_f0_s137.clone();
    let mut __terrane_iterator_0 = terrane_collection_support::string_iterator(
        &__terrane_iterable_0,
    );
    loop {
        character_terrane_f0_s164 = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&character_terrane_f0_s164));
        character_terrane_f0_s164 = String::from("loop");
        println!("{}", terrane_scalar_support::scalar_text(&character_terrane_f0_s164));
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(terrane_string_support::length(&text_terrane_f0_s137)
        as i128))
    );
    index_terrane_f0_s274 = terrane_int_support::Int::from(0_i128);
    '__terrane_break_1: while index_terrane_f0_s274.clone()
        < terrane_int_support::Int::from(3_i128)
    {
        '__terrane_continue_1: {
            if index_terrane_f0_s274.clone() == terrane_int_support::Int::from(1_i128) {
                break '__terrane_continue_1;
            }
            println!("{}", terrane_scalar_support::scalar_text(&index_terrane_f0_s274));
        }
        index_terrane_f0_s274 = index_terrane_f0_s274.clone()
            + terrane_int_support::Int::from(1_i128);
    }
    println!("{}", terrane_scalar_support::scalar_text(&index_terrane_f0_s274));
    skipped_terrane_f0_s375 = terrane_int_support::Int::from(7_i128);
    '__terrane_break_2: while skipped_terrane_f0_s375.clone()
        < terrane_int_support::Int::from(7_i128)
    {
        '__terrane_continue_2: {
            println!(
                "{}", terrane_scalar_support::scalar_text(&String::from("unreachable"))
            );
        }
        skipped_terrane_f0_s375 = skipped_terrane_f0_s375.clone()
            + terrane_int_support::Int::from(1_i128);
    }
    println!("{}", terrane_scalar_support::scalar_text(&skipped_terrane_f0_s375));
}
