// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: duplicate-union-arm-normalization
fn main() {
    let value: i8 = 7;
    let empty: () = ();
    println!("{}", terrane_scalar_support::scalar_text(&value));
    println!("{}", terrane_scalar_support::scalar_text(&{ let _ = &empty; true }));
}
