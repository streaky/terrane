use terrane_int_support::Int;
use terrane_scalar_support::{
    decompose_f32, decompose_f64, scalar_text, scale_binary_f32, scale_binary_f64,
};

fn assert_f32_bits(actual: f32, expected: f32) {
    assert_eq!(actual.to_bits(), expected.to_bits());
}

fn assert_f64_bits(actual: f64, expected: f64) {
    assert_eq!(actual.to_bits(), expected.to_bits());
}

#[test]
fn displays_core_scalars_canonically() {
    assert_eq!(scalar_text(&true), "true");
    assert_eq!(scalar_text(&false), "false");
    assert_eq!(scalar_text(&Int::from(i128::MAX)), i128::MAX.to_string());
    assert_eq!(scalar_text(&"Terrane".to_owned()), "Terrane");
    assert_eq!(scalar_text(&()), "none");
    let text = "borrowed".to_owned();
    assert_eq!(scalar_text(&&text), "borrowed");
}

#[test]
fn normalizes_float_text_and_preserves_negative_zero() {
    assert_eq!(scalar_text(&f64::INFINITY), "inf");
    assert_eq!(scalar_text(&f64::NEG_INFINITY), "-inf");
    assert_eq!(scalar_text(&f64::NAN), "nan");
    assert_eq!(scalar_text(&-0.0_f64), "-0");
    assert_eq!(scalar_text(&1.25_f32), "1.25");
}

#[test]
fn decomposes_normal_subnormal_and_nonfinite_values() {
    let normal32 = decompose_f32(-12.0);
    assert_f32_bits(normal32.mantissa, -0.75);
    assert_eq!(normal32.exponent, 4);
    let subnormal32 = decompose_f32(f32::from_bits(1));
    assert_f32_bits(
        scale_binary_f32(subnormal32.mantissa, subnormal32.exponent),
        f32::from_bits(1),
    );

    let normal64 = decompose_f64(20.0);
    assert_f64_bits(normal64.mantissa, 0.625);
    assert_eq!(normal64.exponent, 5);
    let subnormal64 = decompose_f64(f64::from_bits(1));
    assert_f64_bits(
        scale_binary_f64(subnormal64.mantissa, subnormal64.exponent),
        f64::from_bits(1),
    );

    assert!(decompose_f64(f64::NAN).mantissa.is_nan());
    assert_f64_bits(decompose_f64(f64::INFINITY).mantissa, f64::INFINITY);
    assert!(decompose_f64(-0.0).mantissa.is_sign_negative());
}

#[test]
fn scales_across_ieee_boundaries_with_one_rounding() {
    assert_f32_bits(scale_binary_f32(1.0, -149), f32::from_bits(1));
    assert_f32_bits(scale_binary_f32(f32::from_bits(1), 149), 1.0);
    assert_f32_bits(scale_binary_f32(f32::MAX, 1), f32::INFINITY);
    assert_f32_bits(scale_binary_f32(1.0, -150), 0.0);
    assert_f32_bits(
        scale_binary_f32(f32::from_bits(247_u32 << 23), -250),
        f32::from_bits(1_u32 << 19),
    );

    assert_f64_bits(scale_binary_f64(1.0, -1074), f64::from_bits(1));
    assert_f64_bits(scale_binary_f64(f64::from_bits(1), 1074), 1.0);
    assert_f64_bits(scale_binary_f64(f64::MAX, 1), f64::INFINITY);
    assert_f64_bits(scale_binary_f64(1.0, -1075), 0.0);
    assert_f64_bits(
        scale_binary_f64(f64::from_bits(2023_u64 << 52), -2000),
        f64::from_bits(23_u64 << 52),
    );
    assert_f64_bits(scale_binary_f64(f64::from_bits(1983_u64 << 52), -2100), 0.0);
}
