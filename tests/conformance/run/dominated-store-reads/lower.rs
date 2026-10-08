// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-scalar-support
// Source: case.trn
// Namespace: dominated-store-reads
fn main() {
    let mut nested_value_terrane_f0_s49: i8;
    let mut escaping_terrane_f0_s146: i8;
    nested_value_terrane_f0_s49 = 0;
    let _ = &mut nested_value_terrane_f0_s49;
    if 1 == 1 {
        nested_value_terrane_f0_s49 = 1;
        if 1 == 1 {
            println!(
                "{}", terrane_scalar_support::scalar_text(&nested_value_terrane_f0_s49)
            );
        }
    }
    escaping_terrane_f0_s146 = 0;
    if 1 == 1 {
        escaping_terrane_f0_s146 = 2;
    }
    println!("{}", terrane_scalar_support::scalar_text(&escaping_terrane_f0_s146));
}
