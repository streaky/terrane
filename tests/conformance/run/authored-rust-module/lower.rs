// Generated deterministically by Terrane <version>.
#[path = "lower.authored/rust/adapters.rs"]
mod adapters;
// Source: src/main.trn
// Namespace: app
fn main() {
    let answer: terrane_int_support::Int;
    answer = crate::adapters::answer();
    println!("{}", terrane_scalar_support::scalar_text(&answer));
}
