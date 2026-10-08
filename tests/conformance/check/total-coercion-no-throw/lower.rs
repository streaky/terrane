// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-scalar-support
// Source: case.trn
// Namespace: total-coercion-no-throw
fn widen(value: i8) -> i16 {
    return value as i16;
}
fn main() {
    let value_terrane_f0_s177: i8;
    let widened_terrane_f0_s195: i16;
    value_terrane_f0_s177 = 12;
    widened_terrane_f0_s195 = widen(value_terrane_f0_s177);
    println!("{}", terrane_scalar_support::scalar_text(&widened_terrane_f0_s195));
}
