// Generated deterministically by Terrane <version>.
// Source: app/b-main.trn
// Namespace: app
fn greet_terrane_app() -> String {
    return String::from("local");
}
fn main() {
    println!("{}", terrane_scalar_support::scalar_text(&greet_terrane_app()));
    println!("{}", terrane_scalar_support::scalar_text(&anchor()));
}
// Source: lib/value.trn
// Namespace: lib
fn greet_terrane_lib() -> String {
    return String::from("imported");
}
fn anchor() -> String {
    return greet_terrane_lib();
}
