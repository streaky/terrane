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
    let rounded_terrane_f0_s148: f32;
    let negative_zero_terrane_f0_s203: f64;
    let narrowed_negative_zero_terrane_f0_s234: f32;
    let overflow_input_terrane_f0_s305: f64;
    let overflow_terrane_f0_s366: f32;
    let underflow_input_terrane_f0_s424: f64;
    let underflow_terrane_f0_s487: f32;
    let nan_input_terrane_f0_s547: f64;
    let nan_terrane_f0_s578: f32;
    rounded_terrane_f0_s148 = 16777217.0 as f32;
    negative_zero_terrane_f0_s203 = -0.0_f64;
    narrowed_negative_zero_terrane_f0_s234 = negative_zero_terrane_f0_s203 as f32;
    overflow_input_terrane_f0_s305 = terrane_scalar_support::scale_binary_f32(
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
    overflow_terrane_f0_s366 = overflow_input_terrane_f0_s305 as f32;
    underflow_input_terrane_f0_s424 = {
        let _ = &TerraneDescriptor {
            identity: "float64",
            name: "float64",
            kind: "type",
            inherently_identity_bearing: false,
            fields: &[],
        };
        f64::from_bits(1)
    };
    underflow_terrane_f0_s487 = underflow_input_terrane_f0_s424 as f32;
    nan_input_terrane_f0_s547 = divide(0.0, 0.0);
    nan_terrane_f0_s578 = nan_input_terrane_f0_s547 as f32;
    println!("{}", terrane_scalar_support::scalar_text(&rounded_terrane_f0_s148));
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&(narrowed_negative_zero_terrane_f0_s234 ==
        0.0)),
        terrane_scalar_support::scalar_text(&narrowed_negative_zero_terrane_f0_s234
        .is_sign_negative())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&overflow_terrane_f0_s366
        .is_infinite())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(underflow_terrane_f0_s487 == 0.0))
    );
    println!("{}", terrane_scalar_support::scalar_text(&nan_terrane_f0_s578.is_nan()));
}
