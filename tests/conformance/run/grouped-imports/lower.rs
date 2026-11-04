// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: grouped-imports
fn encoded() -> String {
    let first: String;
    let second: String;
    first = encode_base64(Vec::<u8>::from([97, 98, 99]), false, true);
    second = encode_hex(Vec::<u8>::from([97, 98, 99]));
    return {
        let mut __terrane_text = String::with_capacity(1);
        terrane_scalar_support::ScalarDisplay::write_scalar(&first, &mut __terrane_text);
        __terrane_text.push_str(":");
        terrane_scalar_support::ScalarDisplay::write_scalar(
            &second,
            &mut __terrane_text,
        );
        __terrane_text
    };
}
fn main() {
    let value: i8;
    let other: i16;
    let third: i32;
    value = 7;
    other = 9;
    println!("{}", terrane_scalar_support::scalar_text(&value));
    println!("{}", terrane_scalar_support::scalar_text(&other));
    third = 42;
    println!("{}", terrane_scalar_support::scalar_text(&third));
    println!("{}", terrane_scalar_support::scalar_text(&encoded()));
}
