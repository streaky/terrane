// Generated deterministically by Terrane <version>.
// Source: app/child/child.trn
// Namespace: app/child
fn use_double(value: terrane_int_support::Int) -> terrane_int_support::Int {
    return double(value.clone());
}
// Source: app/main.trn
// Namespace: app
fn double(value: terrane_int_support::Int) -> terrane_int_support::Int {
    return value.clone() * terrane_int_support::Int::from(2_i128);
}
fn main() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&use_double(terrane_int_support::Int::from(2_i128)))
    );
}
