// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: milestone-representative
fn main() {
    let heading_terrane_f0_s91: String;
    let joined_terrane_f0_s136: String;
    let count_terrane_f0_s192: terrane_int_support::Int;
    let converted_terrane_f0_s221: u8;
    heading_terrane_f0_s91 = format!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&String::from("Terrane")),
        terrane_scalar_support::scalar_text(&String::from(" ")),
        terrane_scalar_support::scalar_text(&4),
        terrane_scalar_support::scalar_text(&String::from(".")),
        terrane_scalar_support::scalar_text(&7)
    );
    joined_terrane_f0_s136 = vec![
        terrane_scalar_support::scalar_text(&heading_terrane_f0_s91),
        terrane_scalar_support::scalar_text(&String::from("namespaces")),
        terrane_scalar_support::scalar_text(&String::from("strings"))
    ]
        .join(&String::from(" / "));
    count_terrane_f0_s192 = terrane_int_support::Int::from(
        terrane_string_support::length(&String::from("a🇺🇳")) as i128,
    );
    converted_terrane_f0_s221 = terrane_int_support::saturating_coerce::<u8>(&300);
    if count_terrane_f0_s192.clone() == terrane_int_support::Int::from(2_i128) {
        println!("{}", terrane_scalar_support::scalar_text(&joined_terrane_f0_s136));
    }
    println!("{}", terrane_scalar_support::scalar_text(&converted_terrane_f0_s221));
}
