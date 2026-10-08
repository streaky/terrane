// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-scalar-support
// Source: case.trn
// Namespace: coercion-single-evaluation
fn observed() -> terrane_int_support::Int {
    println!("{}", terrane_scalar_support::scalar_text(&String::from("once")));
    return terrane_int_support::Int::from(300_i128);
}
fn main() {
    let value_terrane_f0_s108: u8;
    value_terrane_f0_s108 = terrane_int_support::wrapping_coerce::<u8>(&observed());
    println!("{}", terrane_scalar_support::scalar_text(&value_terrane_f0_s108));
}
