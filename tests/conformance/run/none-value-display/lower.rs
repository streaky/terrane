// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-scalar-support
// Source: case.trn
// Namespace: none-value-display
fn main() {
    println!("{}", terrane_scalar_support::scalar_text(&()));
    let value: () = ();
    println!("{}", terrane_scalar_support::scalar_text(&value));
}
