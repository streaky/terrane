// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-scalar-support
// Source: case.trn
// Namespace: contextual-floating-literals
fn main() {
    let hexadecimal_terrane_f0_s56: f32;
    let hexadecimal64_terrane_f0_s85: f64;
    let whole_terrane_f0_s115: f64;
    hexadecimal_terrane_f0_s56 = 255.0_f32;
    hexadecimal64_terrane_f0_s85 = 5.0_f64;
    whole_terrane_f0_s115 = 9007199254740992.0_f64;
    println!("{}", terrane_scalar_support::scalar_text(&hexadecimal_terrane_f0_s56));
    println!("{}", terrane_scalar_support::scalar_text(&hexadecimal64_terrane_f0_s85));
    println!("{}", terrane_scalar_support::scalar_text(&whole_terrane_f0_s115));
}
