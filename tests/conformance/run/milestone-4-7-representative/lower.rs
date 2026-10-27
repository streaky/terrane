// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: milestone-representative
fn main() {
    let heading: String;
    let joined: String;
    let count: terrane_int_support::Int;
    let converted: u8;
    heading = {
        let mut __terrane_text = String::with_capacity(9);
        __terrane_text.push_str("Terrane");
        __terrane_text.push_str(" ");
        terrane_scalar_support::ScalarDisplay::write_scalar(&4, &mut __terrane_text);
        __terrane_text.push_str(".");
        terrane_scalar_support::ScalarDisplay::write_scalar(&7, &mut __terrane_text);
        __terrane_text
    };
    joined = {
        let __terrane_separator = &" / ";
        let mut __terrane_text = String::with_capacity(23);
        terrane_scalar_support::ScalarDisplay::write_scalar(
            &heading,
            &mut __terrane_text,
        );
        __terrane_text.push_str(__terrane_separator);
        __terrane_text.push_str("namespaces");
        __terrane_text.push_str(__terrane_separator);
        __terrane_text.push_str("strings");
        __terrane_text
    };
    count = terrane_int_support::Int::from(
        terrane_string_support::length(&String::from("a🇺🇳")) as i128,
    );
    converted = terrane_int_support::saturating_coerce::<u8>(&300);
    if count == terrane_int_support::Int::from(2_i128) {
        println!("{}", terrane_scalar_support::scalar_text(&joined));
    }
    println!("{}", terrane_scalar_support::scalar_text(&converted));
}
