// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-scalar-support
// Source: case.trn
// Namespace: numeric-literals
fn main() {
    let adaptive_terrane_f0_s102: i64;
    let single_terrane_f0_s120: f32;
    let double_terrane_f0_s143: f64;
    let inferred_terrane_f0_s167: f64;
    let signed_value_terrane_f0_s184: i8;
    let unsigned_value_terrane_f0_s213: u8;
    let minimum_terrane_f0_s246: i8;
    let negative_hex_terrane_f0_s270: i8;
    adaptive_terrane_f0_s102 = 16;
    single_terrane_f0_s120 = 1.5_f32;
    double_terrane_f0_s143 = 2.25;
    inferred_terrane_f0_s167 = 3.5;
    signed_value_terrane_f0_s184 = 127;
    unsigned_value_terrane_f0_s213 = 255;
    minimum_terrane_f0_s246 = -128;
    negative_hex_terrane_f0_s270 = -16;
    println!("{}", terrane_scalar_support::scalar_text(&adaptive_terrane_f0_s102));
    println!("{}", terrane_scalar_support::scalar_text(&single_terrane_f0_s120));
    println!("{}", terrane_scalar_support::scalar_text(&double_terrane_f0_s143));
    println!("{}", terrane_scalar_support::scalar_text(&inferred_terrane_f0_s167));
    println!("{}", terrane_scalar_support::scalar_text(&signed_value_terrane_f0_s184));
    println!("{}", terrane_scalar_support::scalar_text(&unsigned_value_terrane_f0_s213));
    println!("{}", terrane_scalar_support::scalar_text(&minimum_terrane_f0_s246));
    println!("{}", terrane_scalar_support::scalar_text(&negative_hex_terrane_f0_s270));
}
