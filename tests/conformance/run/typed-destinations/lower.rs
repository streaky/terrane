// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: typed-destinations
fn answer() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(41_i128);
}
fn main() {
    let text_terrane_f0_s79: String;
    let mut total_terrane_f0_s98: terrane_int_support::Int;
    text_terrane_f0_s79 = String::from("Terrane");
    total_terrane_f0_s98 = terrane_int_support::Int::from(
        terrane_string_support::length(&text_terrane_f0_s79) as i128,
    );
    total_terrane_f0_s98 = total_terrane_f0_s98.clone()
        + terrane_int_support::Int::from(1_i128);
    println!("{}", terrane_scalar_support::scalar_text(&answer()));
    println!("{}", terrane_scalar_support::scalar_text(&total_terrane_f0_s98));
}
