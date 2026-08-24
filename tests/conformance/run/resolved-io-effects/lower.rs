// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: resolved-io-effects
fn print() {}
fn writes() {
    println!("{}", terrane_scalar_support::scalar_text(&(String::from("body"))));
}
fn local_only() {
    print();
}
fn main() {
    writes();
    println!("{}", terrane_scalar_support::scalar_text(&("io".to_owned())));
    println!("{}", terrane_scalar_support::scalar_text(&("".to_owned())));
    local_only();
}
