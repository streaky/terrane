// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: named-result-types
fn pass() -> terrane_int_support::OverflowResult<i8> {
    let small: i8;
    let result: terrane_int_support::OverflowResult<i8>;
    small = 120;
    result = terrane_int_support::fixed_addition_overflowing(small, 10);
    return result;
}
fn divide() -> terrane_int_support::DivRemResult<i8> {
    let small: i8;
    small = 7;
    return __terrane_raised(
        terrane_int_support::fixed_div_rem(small, 3),
        0 /* terrane-site: case.trn:9:10-9:26 */,
    );
}
fn main() {
    let result: terrane_int_support::OverflowResult<i8>;
    let pair: terrane_int_support::DivRemResult<i8>;
    let text: String;
    let found: Option<terrane_string_support::TextRange>;
    result = pass();
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&result.value),
        terrane_scalar_support::scalar_text(&result.overflowed)
    );
    pair = divide();
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&pair.quotient),
        terrane_scalar_support::scalar_text(&pair.remainder)
    );
    text = String::from("banana");
    found = terrane_string_support::find(&text, &String::from("ana"));
    if (&found).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &found { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") } .text()
            .to_owned())
        );
    }
}
