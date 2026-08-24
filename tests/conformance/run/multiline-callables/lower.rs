// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: multiline-callables
fn combine(first: terrane_int_support::Int, second: terrane_int_support::Int, third: terrane_int_support::Int) -> terrane_int_support::Int {
    return (first.clone() + second.clone()) + third.clone();
}
fn main() {
    println!("{}", terrane_scalar_support::scalar_text(&(combine(terrane_int_support::Int::from(1_i128), terrane_int_support::Int::from(2_i128), terrane_int_support::Int::from(3_i128)))));
    println!("{}", terrane_scalar_support::scalar_text(&(combine(terrane_int_support::Int::from(4_i128), terrane_int_support::Int::from(5_i128), terrane_int_support::Int::from(6_i128)))));
}
