// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-scalar-support
// Source: case.trn
// Namespace: numeric-literals
fn main() {
    let adaptive: i64;
    let single: f32;
    let double: f64;
    let inferred: f64;
    let signed_value: i8;
    let unsigned_value: u8;
    let minimum: i8;
    let negative_hex: i8;
    adaptive = 16;
    single = 1.5_f32;
    double = 2.25;
    inferred = 3.5;
    signed_value = 127;
    unsigned_value = 255;
    minimum = -128;
    negative_hex = -16;
    println!("{}", terrane_scalar_support::scalar_text(&adaptive));
    println!("{}", terrane_scalar_support::scalar_text(&single));
    println!("{}", terrane_scalar_support::scalar_text(&double));
    println!("{}", terrane_scalar_support::scalar_text(&inferred));
    println!("{}", terrane_scalar_support::scalar_text(&signed_value));
    println!("{}", terrane_scalar_support::scalar_text(&unsigned_value));
    println!("{}", terrane_scalar_support::scalar_text(&minimum));
    println!("{}", terrane_scalar_support::scalar_text(&negative_hex));
}
