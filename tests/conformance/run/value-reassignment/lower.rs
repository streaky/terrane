// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-scalar-support
// Source: case.trn
// Namespace: value-reassignment
fn main() {
    let mut value_terrane_f0_s46: terrane_int_support::Int;
    let next_terrane_f0_s73: i64;
    value_terrane_f0_s46 = terrane_int_support::Int::from(1_i128);
    println!("{}", terrane_scalar_support::scalar_text(&value_terrane_f0_s46));
    next_terrane_f0_s73 = 2;
    value_terrane_f0_s46 = terrane_int_support::Int::from(next_terrane_f0_s73 as i128);
    println!("{}", terrane_scalar_support::scalar_text(&value_terrane_f0_s46));
    value_terrane_f0_s46 = terrane_int_support::Int::from(3_i128);
    println!("{}", terrane_scalar_support::scalar_text(&value_terrane_f0_s46));
}
