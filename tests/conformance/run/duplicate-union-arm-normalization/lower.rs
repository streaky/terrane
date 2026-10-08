// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-scalar-support
// Source: case.trn
// Namespace: duplicate-union-arm-normalization
fn main() {
    let value: i8;
    let empty: ();
    value = 7;
    empty = ();
    println!("{}", terrane_scalar_support::scalar_text(&value));
    println!("{}", terrane_scalar_support::scalar_text(&{ let _ = &empty; true }));
}
