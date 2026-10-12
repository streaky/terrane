// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: string-join
fn separator() -> String {
    println!("{}", String::from("side effect"));
    return String::from("--");
}
fn main() {
    let empty: String;
    let single: String;
    let many: String;
    empty = {
        let __terrane_separator = &separator();
        let mut __terrane_text = String::with_capacity(0);
        __terrane_text
    };
    single = {
        let __terrane_separator = &"--";
        let mut __terrane_text = String::with_capacity(3);
        __terrane_text.push_str("one");
        __terrane_text
    };
    many = {
        let __terrane_separator = &"--";
        let mut __terrane_text = String::with_capacity(7);
        __terrane_text.push_str("one");
        __terrane_text.push_str(__terrane_separator);
        terrane_scalar_support::ScalarDisplay::write_scalar(&2, &mut __terrane_text);
        __terrane_text.push_str(__terrane_separator);
        terrane_scalar_support::ScalarDisplay::write_scalar(&true, &mut __terrane_text);
        __terrane_text
    };
    println!("{}", terrane_scalar_support::scalar_text(&empty));
    println!("{}", terrane_scalar_support::scalar_text(&single));
    println!("{}", terrane_scalar_support::scalar_text(&many));
}
