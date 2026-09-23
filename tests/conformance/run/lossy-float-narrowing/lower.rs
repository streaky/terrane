// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-scalar-support
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct TerraneFieldMetadata {
    name: &'static str,
    external_name: &'static str,
    defaulted: bool,
    optional: bool,
    secret: bool,
}
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct TerraneDescriptor {
    identity: &'static str,
    name: &'static str,
    kind: &'static str,
    inherently_identity_bearing: bool,
    fields: &'static [TerraneFieldMetadata],
}
// Source: case.trn
// Namespace: lossy-float-narrowing
fn divide(numerator: f64, denominator: f64) -> f64 {
    return numerator / denominator;
}
fn main() {
    let rounded: f32 = 16777217.0 as f32;
    let negative_zero: f64 = -0.0_f64;
    let narrowed_negative_zero: f32 = negative_zero as f32;
    let overflow_input: f64 = terrane_scalar_support::scale_binary_f32(
        {
            let _ = &TerraneDescriptor {
                identity: "float32",
                name: "float32",
                kind: "type",
                inherently_identity_bearing: false,
                fields: &[],
            };
            f32::MAX
        },
        1,
    ) as f64;
    let overflow: f32 = overflow_input as f32;
    let underflow_input: f64 = {
        let _ = &TerraneDescriptor {
            identity: "float64",
            name: "float64",
            kind: "type",
            inherently_identity_bearing: false,
            fields: &[],
        };
        f64::from_bits(1)
    };
    let underflow: f32 = underflow_input as f32;
    let nan_input: f64 = divide(0.0, 0.0);
    let nan: f32 = nan_input as f32;
    println!("{}", terrane_scalar_support::scalar_text(&rounded));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(narrowed_negative_zero == 0.0)),
        terrane_scalar_support::scalar_text(&narrowed_negative_zero.is_sign_negative())
    );
    println!("{}", terrane_scalar_support::scalar_text(&overflow.is_infinite()));
    println!("{}", terrane_scalar_support::scalar_text(&(underflow == 0.0)));
    println!("{}", terrane_scalar_support::scalar_text(&nan.is_nan()));
}
