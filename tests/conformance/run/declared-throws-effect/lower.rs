// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: declared-throws-effect
fn declared() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(1_i128);
}
fn middle() -> terrane_int_support::Int {
    return declared();
}
fn main() {
    let value: terrane_int_support::Int = middle();
    println!("{}", terrane_scalar_support::scalar_text(& (value)));
}
