// Generated deterministically by Terrane <version>.
// Source: app/b-main.trn
// Namespace: app
#[allow(dead_code)]
fn greet_terrane_app() -> String {
    return String::from("local");
}
fn main() {
    println!("{}", terrane_scalar_support::scalar_text(&greet_terrane_app()));
    println!("{}", terrane_scalar_support::scalar_text(&anchor()));
}
// Source: lib/value.trn
// Namespace: lib
#[allow(dead_code)]
fn greet_terrane_lib() -> String {
    return String::from("imported");
}
#[allow(dead_code)]
fn anchor() -> String {
    return greet_terrane_lib();
}
