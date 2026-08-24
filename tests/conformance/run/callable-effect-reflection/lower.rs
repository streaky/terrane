// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: callable-effect-reflection
fn noisy() {
    println!("{}", terrane_scalar_support::scalar_text(&(String::from("work"))));
}
fn main() {
    noisy();
    println!("{}", terrane_scalar_support::scalar_text(&("io".to_owned())));
}
