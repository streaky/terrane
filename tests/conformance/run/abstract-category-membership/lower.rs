// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-scalar-support
// Source: case.trn
// Namespace: abstract-category-membership
fn main() {
    let signed: i8;
    let unsigned: u8;
    let adaptive: i64;
    let decimal: f32;
    signed = -1;
    unsigned = 1;
    adaptive = 1;
    decimal = 1.5_f32;
    println!("{}", terrane_scalar_support::scalar_text(&{ let _ = &signed; true }));
    println!("{}", terrane_scalar_support::scalar_text(&{ let _ = &signed; true }));
    println!("{}", terrane_scalar_support::scalar_text(&{ let _ = &signed; true }));
    println!("{}", terrane_scalar_support::scalar_text(&{ let _ = &signed; true }));
    println!("{}", terrane_scalar_support::scalar_text(&{ let _ = &signed; false }));
    println!("{}", terrane_scalar_support::scalar_text(&{ let _ = &unsigned; true }));
    println!("{}", terrane_scalar_support::scalar_text(&{ let _ = &adaptive; false }));
    println!("{}", terrane_scalar_support::scalar_text(&{ let _ = &decimal; true }));
}
