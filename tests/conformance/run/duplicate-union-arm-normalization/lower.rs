// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-scalar-support
// Source: case.trn
// Namespace: duplicate-union-arm-normalization
fn main() {
    let value_terrane_f0_s98: i8;
    let empty_terrane_f0_s120: ();
    value_terrane_f0_s98 = 7;
    empty_terrane_f0_s120 = ();
    println!("{}", terrane_scalar_support::scalar_text(&value_terrane_f0_s98));
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let _ = &empty_terrane_f0_s120; true
        })
    );
}
