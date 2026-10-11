// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: resolved-name-effects
fn print(value: String) -> String {
    return value;
}
fn quiet() -> String {
    return print(String::from("quiet"));
}
fn main() {
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&String::from("contracts=")),
        terrane_scalar_support::scalar_text(&{ let _ = quiet; "".to_owned() })
    );
    println!("{}", terrane_scalar_support::scalar_text(&quiet()));
}
