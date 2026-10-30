// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: loop-carried-store-reads
fn main() {
    let mut while_value: i8;
    let mut limit: i8;
    let mut for_value: i8;
    let mut character: String;
    while_value = 0;
    limit = 0;
    while limit < 2 {
        println!("{}", terrane_scalar_support::scalar_text(&while_value));
        while_value = 5;
        limit = __terrane_raised(
            terrane_int_support::fixed_addition(limit, 1),
            0 /* terrane-site: case.trn:8:13-8:22 */,
        );
    }
    for_value = 0;
    {
        let __terrane_iterable_0 = &String::from("ab");
        let mut __terrane_iterator_0 = terrane_string_support::graphemes(
            &__terrane_iterable_0,
        );
        loop {
            character = match __terrane_iterator_0.next() {
                Some(item) => item,
                None => break,
            };
            let _ = &character;
            println!("{}", terrane_scalar_support::scalar_text(&for_value));
            for_value = 7;
        }
    }
}
