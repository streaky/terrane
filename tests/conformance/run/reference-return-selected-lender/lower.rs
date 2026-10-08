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
    let left_value_terrane_f0_s211: terrane_int_support::Int;
    let right_value_terrane_f0_s228: terrane_int_support::Int;
    let left_terrane_f0_s246: &terrane_int_support::Int;
    let right_terrane_f0_s278: &terrane_int_support::Int;
    let chosen_terrane_f0_s312: &terrane_int_support::Int;
    left_value_terrane_f0_s211 = terrane_int_support::Int::from(1_i128);
    right_value_terrane_f0_s228 = terrane_int_support::Int::from(2_i128);
    left_terrane_f0_s246 = &left_value_terrane_f0_s211;
    right_terrane_f0_s278 = &right_value_terrane_f0_s228;
    chosen_terrane_f0_s312 = relay(left_terrane_f0_s246, right_terrane_f0_s278);
    println!("{}", terrane_scalar_support::scalar_text(&chosen_terrane_f0_s312.clone()));
}
