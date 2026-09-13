// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: named-result-types
fn pass() -> terrane_int_support::OverflowResult<i8> {
    let small: i8 = 120;
    let result: terrane_int_support::OverflowResult<i8> = terrane_int_support::fixed_addition_overflowing(
        small,
        10,
    );
    return result;
}
fn divide() -> terrane_int_support::DivRemResult<i8> {
    let small: i8 = 7;
    return __terrane_raised(
        terrane_int_support::fixed_div_rem(small, 3),
        0 /* terrane-site: case.trn:9:10-9:26 */,
    );
}
fn main() {
    let result: terrane_int_support::OverflowResult<i8> = pass();
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&result.value),
        terrane_scalar_support::scalar_text(&result.overflowed)
    );
    let pair: terrane_int_support::DivRemResult<i8> = divide();
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&pair.quotient),
        terrane_scalar_support::scalar_text(&pair.remainder)
    );
    let text: String = String::from("banana");
    let found: Option<terrane_string_support::TextRange> = terrane_string_support::find(
        &text,
        &String::from("ana"),
    );
    if found.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&found.as_ref()
            .expect("semantic optional narrowing").text().to_owned())
        );
    }
}
