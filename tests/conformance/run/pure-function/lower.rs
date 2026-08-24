// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: pure-function
fn identity(value: terrane_int_support::Int) -> terrane_int_support::Int {
    return value.clone();
}
fn main() {
    println!("{}", terrane_scalar_support::scalar_text(&(identity(terrane_int_support::Int::from(12_i128)))));
}
