// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: complete-float-surface
fn exercise32() {
    let zero: f32 = 0.0_f32;
    let negative_zero: f32 = -0.0_f32;
    let one: f32 = 1.0_f32;
    let two: f32 = 2.0_f32;
    let three: f32 = 3.0_f32;
    let four: f32 = 4.0_f32;
    let negative_one: f32 = -1.0_f32;
    let negative_two: f32 = -2.0_f32;
    let negative_quarter: f32 = -0.25_f32;
    let fractional: f32 = -1.25_f32;
    let twelve: f32 = 12.0_f32;
    let thousand: f32 = 1000.0_f32;
    let overflow_input: f32 = 128.0_f32;
    let underflow_input: f32 = -150.0_f32;
    let eight: f32 = 8.0_f32;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(eight.cbrt() == two)),
        terrane_scalar_support::scalar_text(&(three.hypot(four) == 5.0_f32))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(four.powf(0.5_f32) == two)),
        terrane_scalar_support::scalar_text(&(two.powi(10) == 1024.0_f32))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(three.exp2() == eight)),
        terrane_scalar_support::scalar_text(&(zero.exp_m1() == zero))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(zero.ln_1p() == zero)),
        terrane_scalar_support::scalar_text(&(eight.log2() == three))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(thousand.log10() == three)),
        terrane_scalar_support::scalar_text(&(eight.log(two) == three))
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(zero.tan() == zero)),
        terrane_scalar_support::scalar_text(&(zero.asin() == zero)),
        terrane_scalar_support::scalar_text(&(one.acos() == zero))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(zero.atan() == zero)),
        terrane_scalar_support::scalar_text(&(zero.atan2(negative_one) > three))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(one.copysign(negative_zero) ==
        negative_one)), terrane_scalar_support::scalar_text(&negative_zero
        .is_sign_negative())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let terrane_value : f32 = two;
        let terrane_lower : f32 = 3.0_f32; let terrane_upper : f32 = 4.0_f32; if
        terrane_value.is_nan() { terrane_value } else if terrane_lower.is_nan() ||
        terrane_upper.is_nan() || terrane_lower > terrane_upper { f32::NAN } else { let
        terrane_lowered = if terrane_value == 0.0 &&terrane_lower == 0.0 { if
        terrane_value.is_sign_positive() || terrane_lower.is_sign_positive() { 0.0 } else
        { - 0.0 } } else { terrane_value.max(terrane_lower) }; if terrane_lowered == 0.0
        &&terrane_upper == 0.0 { if terrane_lowered.is_sign_negative() || terrane_upper
        .is_sign_negative() { - 0.0 } else { 0.0 } } else { terrane_lowered
        .min(terrane_upper) } } } == three)),
        terrane_scalar_support::scalar_text(&(fractional.fract() == negative_quarter))
    );
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&(zero == 0.0)),
        terrane_scalar_support::scalar_text(&(negative_zero == 0.0)),
        terrane_scalar_support::scalar_text(&one.is_normal()),
        terrane_scalar_support::scalar_text(&f32::from_bits(1).is_subnormal())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(zero.next_up() == f32::from_bits(1)))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(zero.next_down() == negative_one *
        f32::from_bits(1)))
    );
    let decomposition: terrane_scalar_support::FloatDecomposition<f32> = terrane_scalar_support::decompose_f32(
        twelve,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(decomposition.mantissa ==
        0.75_f32)), terrane_scalar_support::scalar_text(&(decomposition.exponent == 4))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(terrane_scalar_support::scale_binary_f32(decomposition
        .mantissa, decomposition.exponent) == twelve))
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(i128::from(f32::RADIX))
        == terrane_int_support::Int::from(2_i128))),
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(i128::from(f32::MANTISSA_DIGITS))
        == terrane_int_support::Int::from(24_i128)))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(f32::EPSILON > zero)),
        terrane_scalar_support::scalar_text(&f32::MIN_POSITIVE.is_normal())
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(f32::from_bits(1) > zero)),
        terrane_scalar_support::scalar_text(&(f32::MIN < zero)),
        terrane_scalar_support::scalar_text(&(f32::MAX > zero))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&two.asin().is_nan()),
        terrane_scalar_support::scalar_text(&negative_two.powf(0.5_f32).is_nan())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&overflow_input.exp2()
        .is_infinite()), terrane_scalar_support::scalar_text(&(underflow_input.exp2() ==
        zero))
    );
    let not_a_number: f32 = two.asin();
    let infinity: f32 = one / zero;
    let clamped_zero: f32 = {
        let terrane_value: f32 = negative_zero;
        let terrane_lower: f32 = zero;
        let terrane_upper: f32 = one;
        if terrane_value.is_nan() {
            terrane_value
        } else if terrane_lower.is_nan() || terrane_upper.is_nan()
            || terrane_lower > terrane_upper
        {
            f32::NAN
        } else {
            let terrane_lowered = if terrane_value == 0.0 && terrane_lower == 0.0 {
                if terrane_value.is_sign_positive() || terrane_lower.is_sign_positive() {
                    0.0
                } else {
                    -0.0
                }
            } else {
                terrane_value.max(terrane_lower)
            };
            if terrane_lowered == 0.0 && terrane_upper == 0.0 {
                if terrane_lowered.is_sign_negative() || terrane_upper.is_sign_negative()
                {
                    -0.0
                } else {
                    0.0
                }
            } else {
                terrane_lowered.min(terrane_upper)
            }
        }
    };
    println!("{}", terrane_scalar_support::scalar_text(&(one / clamped_zero > zero)));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let terrane_value : f32 =
        not_a_number; let terrane_lower : f32 = zero; let terrane_upper : f32 = one; if
        terrane_value.is_nan() { terrane_value } else if terrane_lower.is_nan() ||
        terrane_upper.is_nan() || terrane_lower > terrane_upper { f32::NAN } else { let
        terrane_lowered = if terrane_value == 0.0 &&terrane_lower == 0.0 { if
        terrane_value.is_sign_positive() || terrane_lower.is_sign_positive() { 0.0 } else
        { - 0.0 } } else { terrane_value.max(terrane_lower) }; if terrane_lowered == 0.0
        &&terrane_upper == 0.0 { if terrane_lowered.is_sign_negative() || terrane_upper
        .is_sign_negative() { - 0.0 } else { 0.0 } } else { terrane_lowered
        .min(terrane_upper) } } } .is_nan()), terrane_scalar_support::scalar_text(&{ let
        terrane_value : f32 = one; let terrane_lower : f32 = not_a_number; let
        terrane_upper : f32 = one; if terrane_value.is_nan() { terrane_value } else if
        terrane_lower.is_nan() || terrane_upper.is_nan() || terrane_lower > terrane_upper
        { f32::NAN } else { let terrane_lowered = if terrane_value == 0.0 &&terrane_lower
        == 0.0 { if terrane_value.is_sign_positive() || terrane_lower.is_sign_positive()
        { 0.0 } else { - 0.0 } } else { terrane_value.max(terrane_lower) }; if
        terrane_lowered == 0.0 &&terrane_upper == 0.0 { if terrane_lowered
        .is_sign_negative() || terrane_upper.is_sign_negative() { - 0.0 } else { 0.0 } }
        else { terrane_lowered.min(terrane_upper) } } } .is_nan())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&not_a_number.hypot(infinity)
        .is_infinite()), terrane_scalar_support::scalar_text(&infinity.next_up()
        .is_infinite())
    );
    let subnormal_decomposition: terrane_scalar_support::FloatDecomposition<f32> = terrane_scalar_support::decompose_f32(
        f32::from_bits(1),
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(terrane_scalar_support::scale_binary_f32(subnormal_decomposition
        .mantissa, subnormal_decomposition.exponent) == f32::from_bits(1)))
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_scalar_support::scale_binary_f32(f32::MAX,
        1).is_infinite()),
        terrane_scalar_support::scalar_text(&(terrane_scalar_support::scale_binary_f32(f32::from_bits(1),
        - 1) == zero))
    );
}
fn exercise64() {
    let zero: f64 = 0.0;
    let negative_zero: f64 = -0.0_f64;
    let one: f64 = 1.0;
    let two: f64 = 2.0;
    let three: f64 = 3.0;
    let four: f64 = 4.0;
    let eight: f64 = 8.0;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(eight.cbrt() == two)),
        terrane_scalar_support::scalar_text(&(three.hypot(four) == 5.0))
    );
    let negative_one: f64 = -1.0_f64;
    let negative_two: f64 = -2.0_f64;
    let negative_quarter: f64 = -0.25_f64;
    let fractional: f64 = -1.25_f64;
    let twelve: f64 = 12.0;
    let thousand: f64 = 1000.0;
    let overflow_input: f64 = 1024.0;
    let underflow_input: f64 = -1075.0_f64;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(four.powf(0.5) == two)),
        terrane_scalar_support::scalar_text(&(two.powi(10) == 1024.0))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(three.exp2() == eight)),
        terrane_scalar_support::scalar_text(&(zero.exp_m1() == zero))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(zero.ln_1p() == zero)),
        terrane_scalar_support::scalar_text(&(eight.log2() == three))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(thousand.log10() == three)),
        terrane_scalar_support::scalar_text(&(eight.log(two) == three))
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(zero.tan() == zero)),
        terrane_scalar_support::scalar_text(&(zero.asin() == zero)),
        terrane_scalar_support::scalar_text(&(one.acos() == zero))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(zero.atan() == zero)),
        terrane_scalar_support::scalar_text(&(zero.atan2(negative_one) > three))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(one.copysign(negative_zero) ==
        negative_one)), terrane_scalar_support::scalar_text(&negative_zero
        .is_sign_negative())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let terrane_value : f64 = two;
        let terrane_lower : f64 = 3.0; let terrane_upper : f64 = 4.0; if terrane_value
        .is_nan() { terrane_value } else if terrane_lower.is_nan() || terrane_upper
        .is_nan() || terrane_lower > terrane_upper { f64::NAN } else { let
        terrane_lowered = if terrane_value == 0.0 &&terrane_lower == 0.0 { if
        terrane_value.is_sign_positive() || terrane_lower.is_sign_positive() { 0.0 } else
        { - 0.0 } } else { terrane_value.max(terrane_lower) }; if terrane_lowered == 0.0
        &&terrane_upper == 0.0 { if terrane_lowered.is_sign_negative() || terrane_upper
        .is_sign_negative() { - 0.0 } else { 0.0 } } else { terrane_lowered
        .min(terrane_upper) } } } == three)),
        terrane_scalar_support::scalar_text(&(fractional.fract() == negative_quarter))
    );
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&(zero == 0.0)),
        terrane_scalar_support::scalar_text(&(negative_zero == 0.0)),
        terrane_scalar_support::scalar_text(&one.is_normal()),
        terrane_scalar_support::scalar_text(&f64::from_bits(1).is_subnormal())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(zero.next_up() == f64::from_bits(1)))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(zero.next_down() == negative_one *
        f64::from_bits(1)))
    );
    let decomposition: terrane_scalar_support::FloatDecomposition<f64> = terrane_scalar_support::decompose_f64(
        twelve,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(decomposition.mantissa == 0.75)),
        terrane_scalar_support::scalar_text(&(decomposition.exponent == 4))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(terrane_scalar_support::scale_binary_f64(decomposition
        .mantissa, decomposition.exponent) == twelve))
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(i128::from(f64::RADIX))
        == terrane_int_support::Int::from(2_i128))),
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(i128::from(f64::MANTISSA_DIGITS))
        == terrane_int_support::Int::from(53_i128)))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(f64::EPSILON > zero)),
        terrane_scalar_support::scalar_text(&f64::MIN_POSITIVE.is_normal())
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(f64::from_bits(1) > zero)),
        terrane_scalar_support::scalar_text(&(f64::MIN < zero)),
        terrane_scalar_support::scalar_text(&(f64::MAX > zero))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&two.asin().is_nan()),
        terrane_scalar_support::scalar_text(&negative_two.powf(0.5).is_nan())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&overflow_input.exp2()
        .is_infinite()), terrane_scalar_support::scalar_text(&(underflow_input.exp2() ==
        zero))
    );
    let not_a_number: f64 = two.asin();
    let infinity: f64 = one / zero;
    let clamped_zero: f64 = {
        let terrane_value: f64 = negative_zero;
        let terrane_lower: f64 = zero;
        let terrane_upper: f64 = one;
        if terrane_value.is_nan() {
            terrane_value
        } else if terrane_lower.is_nan() || terrane_upper.is_nan()
            || terrane_lower > terrane_upper
        {
            f64::NAN
        } else {
            let terrane_lowered = if terrane_value == 0.0 && terrane_lower == 0.0 {
                if terrane_value.is_sign_positive() || terrane_lower.is_sign_positive() {
                    0.0
                } else {
                    -0.0
                }
            } else {
                terrane_value.max(terrane_lower)
            };
            if terrane_lowered == 0.0 && terrane_upper == 0.0 {
                if terrane_lowered.is_sign_negative() || terrane_upper.is_sign_negative()
                {
                    -0.0
                } else {
                    0.0
                }
            } else {
                terrane_lowered.min(terrane_upper)
            }
        }
    };
    println!("{}", terrane_scalar_support::scalar_text(&(one / clamped_zero > zero)));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let terrane_value : f64 =
        not_a_number; let terrane_lower : f64 = zero; let terrane_upper : f64 = one; if
        terrane_value.is_nan() { terrane_value } else if terrane_lower.is_nan() ||
        terrane_upper.is_nan() || terrane_lower > terrane_upper { f64::NAN } else { let
        terrane_lowered = if terrane_value == 0.0 &&terrane_lower == 0.0 { if
        terrane_value.is_sign_positive() || terrane_lower.is_sign_positive() { 0.0 } else
        { - 0.0 } } else { terrane_value.max(terrane_lower) }; if terrane_lowered == 0.0
        &&terrane_upper == 0.0 { if terrane_lowered.is_sign_negative() || terrane_upper
        .is_sign_negative() { - 0.0 } else { 0.0 } } else { terrane_lowered
        .min(terrane_upper) } } } .is_nan()), terrane_scalar_support::scalar_text(&{ let
        terrane_value : f64 = one; let terrane_lower : f64 = not_a_number; let
        terrane_upper : f64 = one; if terrane_value.is_nan() { terrane_value } else if
        terrane_lower.is_nan() || terrane_upper.is_nan() || terrane_lower > terrane_upper
        { f64::NAN } else { let terrane_lowered = if terrane_value == 0.0 &&terrane_lower
        == 0.0 { if terrane_value.is_sign_positive() || terrane_lower.is_sign_positive()
        { 0.0 } else { - 0.0 } } else { terrane_value.max(terrane_lower) }; if
        terrane_lowered == 0.0 &&terrane_upper == 0.0 { if terrane_lowered
        .is_sign_negative() || terrane_upper.is_sign_negative() { - 0.0 } else { 0.0 } }
        else { terrane_lowered.min(terrane_upper) } } } .is_nan())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&not_a_number.hypot(infinity)
        .is_infinite()), terrane_scalar_support::scalar_text(&infinity.next_up()
        .is_infinite())
    );
    let subnormal_decomposition: terrane_scalar_support::FloatDecomposition<f64> = terrane_scalar_support::decompose_f64(
        f64::from_bits(1),
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(terrane_scalar_support::scale_binary_f64(subnormal_decomposition
        .mantissa, subnormal_decomposition.exponent) == f64::from_bits(1)))
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_scalar_support::scale_binary_f64(f64::MAX,
        1).is_infinite()),
        terrane_scalar_support::scalar_text(&(terrane_scalar_support::scale_binary_f64(f64::from_bits(1),
        - 1) == zero))
    );
}
fn main() {
    exercise32();
    exercise64();
}
