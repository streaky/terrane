// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-scalar-support
// Source: case.trn
// Namespace: reference-return-selected-lender
fn choose<'a>(
    left: &'a terrane_int_support::Int,
    right: &terrane_int_support::Int,
) -> &'a terrane_int_support::Int {
    let _ = &right;
    return left;
}
fn relay<'a>(
    left: &'a terrane_int_support::Int,
    right: &terrane_int_support::Int,
) -> &'a terrane_int_support::Int {
    return choose(left, right);
}
fn main() {
    let left_value: terrane_int_support::Int;
    let right_value: terrane_int_support::Int;
    let left: &terrane_int_support::Int;
    let right: &terrane_int_support::Int;
    let chosen: &terrane_int_support::Int;
    left_value = terrane_int_support::Int::from(1_i128);
    right_value = terrane_int_support::Int::from(2_i128);
    left = &left_value;
    right = &right_value;
    chosen = relay(left, right);
    println!("{}", terrane_scalar_support::scalar_text(&chosen.clone()));
}
