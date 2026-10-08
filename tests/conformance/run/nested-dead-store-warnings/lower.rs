// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-scalar-support
// Source: case.trn
// Namespace: nested-dead-store-warnings
fn main() {
    let mut nested_terrane_f0_s68: i8;
    let mut top_terrane_f0_s119: i8;
    if 1 == 1 {
        nested_terrane_f0_s68 = 1;
        let _ = &mut nested_terrane_f0_s68;
        nested_terrane_f0_s68 = 2;
        println!("{}", terrane_scalar_support::scalar_text(&nested_terrane_f0_s68));
    }
    top_terrane_f0_s119 = 3;
    let _ = &mut top_terrane_f0_s119;
    top_terrane_f0_s119 = 4;
    println!("{}", terrane_scalar_support::scalar_text(&top_terrane_f0_s119));
}
