// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: complete-float-surface
fn exercise32() {
    let zero: f32;
    let negative_zero: f32;
    let one: f32;
    let two: f32;
    let three: f32;
    let four: f32;
    let negative_one: f32;
    let negative_two: f32;
    let negative_quarter: f32;
    let fractional: f32;
    let twelve: f32;
    let thousand: f32;
    let overflow_input: f32;
    let underflow_input: f32;
    let eight: f32;
    let decomposition: terrane_scalar_support::FloatDecomposition<f32>;
    let not_a_number: f32;
    let infinity: f32;
    let clamped_zero: f32;
    let subnormal_decomposition: terrane_scalar_support::FloatDecomposition<f32>;
    let wide_four: f64;
    let floating_exponent: f64;
    let nan_decomposition: terrane_scalar_support::FloatDecomposition<f32>;
    let infinity_decomposition: terrane_scalar_support::FloatDecomposition<f32>;
    let negative_zero_decomposition: terrane_scalar_support::FloatDecomposition<f32>;
    let negative_infinity: f32;
    let large_exponent: f32;
    let large: f32;
    let descriptor: TerraneDescriptor;
    zero = 0.0_f32;
    negative_zero = -0.0_f32;
    one = 1.0_f32;
    two = 2.0_f32;
    three = 3.0_f32;
    four = 4.0_f32;
    negative_one = -1.0_f32;
    negative_two = -2.0_f32;
    negative_quarter = -0.25_f32;
    fractional = -1.25_f32;
    twelve = 12.0_f32;
    thousand = 1000.0_f32;
    overflow_input = 128.0_f32;
    underflow_input = -150.0_f32;
    eight = 8.0_f32;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = eight
        .cbrt(); __terrane_result } == two)), terrane_scalar_support::scalar_text(&({ let
        __terrane_result = three.hypot(four); __terrane_result } == 5.0_f32))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = four
        .powf(0.5_f32); __terrane_result } == two)),
        terrane_scalar_support::scalar_text(&({ let __terrane_result = two.powi(10);
        __terrane_result } == 1024.0_f32))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = three
        .exp2(); __terrane_result } == eight)), terrane_scalar_support::scalar_text(&({
        let __terrane_result = zero.exp_m1(); __terrane_result } == zero))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero
        .ln_1p(); __terrane_result } == zero)), terrane_scalar_support::scalar_text(&({
        let __terrane_result = eight.log2(); __terrane_result } == three))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = thousand
        .log10(); __terrane_result } == three)), terrane_scalar_support::scalar_text(&({
        let __terrane_result = eight.log(two); __terrane_result } == three))
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero
        .tan(); __terrane_result } == zero)), terrane_scalar_support::scalar_text(&({ let
        __terrane_result = zero.asin(); __terrane_result } == zero)),
        terrane_scalar_support::scalar_text(&({ let __terrane_result = one.acos();
        __terrane_result } == zero))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero
        .atan(); __terrane_result } == zero)), terrane_scalar_support::scalar_text(&({
        let __terrane_result = zero.atan2(negative_one); __terrane_result } > three))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = one
        .copysign(negative_zero); __terrane_result } == negative_one)),
        terrane_scalar_support::scalar_text(&negative_zero.is_sign_negative())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = { let
        terrane_value : f32 = two; let terrane_lower : f32 = 3.0_f32; let terrane_upper :
        f32 = 4.0_f32; if terrane_value.is_nan() { terrane_value } else if terrane_lower
        .is_nan() || terrane_upper.is_nan() || terrane_lower > terrane_upper { f32::NAN }
        else { let terrane_lowered = if terrane_value == 0.0 &&terrane_lower == 0.0 { if
        terrane_value.is_sign_positive() || terrane_lower.is_sign_positive() { 0.0 } else
        { - 0.0 } } else { terrane_value.max(terrane_lower) }; if terrane_lowered == 0.0
        &&terrane_upper == 0.0 { if terrane_lowered.is_sign_negative() || terrane_upper
        .is_sign_negative() { - 0.0 } else { 0.0 } } else { terrane_lowered
        .min(terrane_upper) } } }; __terrane_result } == three)),
        terrane_scalar_support::scalar_text(&({ let __terrane_result = { let
        terrane_receiver : f32 = fractional; let terrane_fraction = terrane_receiver
        .fract(); if terrane_fraction == 0.0 { 0.0_f32.copysign(terrane_receiver) } else
        { terrane_fraction } }; __terrane_result } == negative_quarter))
    );
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&(zero == 0.0)),
        terrane_scalar_support::scalar_text(&(negative_zero == 0.0)),
        terrane_scalar_support::scalar_text(&one.is_normal()),
        terrane_scalar_support::scalar_text(&{ let _ = &TerraneDescriptor { identity :
        "float32", name : "float32", kind : "type", inherently_identity_bearing : false,
        fields : &[] }; f32::from_bits(1) } .is_subnormal())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero
        .next_up(); __terrane_result } == { let _ = &TerraneDescriptor { identity :
        "float32", name : "float32", kind : "type", inherently_identity_bearing : false,
        fields : &[] }; f32::from_bits(1) }))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero
        .next_down(); __terrane_result } == negative_one * { let _ = &TerraneDescriptor {
        identity : "float32", name : "float32", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f32::from_bits(1) }))
    );
    decomposition = {
        let __terrane_result = terrane_scalar_support::decompose_f32(twelve);
        __terrane_result
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(decomposition.mantissa ==
        0.75_f32)), terrane_scalar_support::scalar_text(&(decomposition.exponent == 4))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result =
        terrane_scalar_support::scale_binary_f32(decomposition.mantissa, decomposition
        .exponent); __terrane_result } == twelve))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor {
        identity : "float32", name : "float32", kind : "type",
        inherently_identity_bearing : false, fields : &[] };
        terrane_int_support::Int::from(i128::from(f32::RADIX)) } ==
        terrane_int_support::Int::from(2_i128))), terrane_scalar_support::scalar_text(&({
        let _ = &TerraneDescriptor { identity : "float32", name : "float32", kind :
        "type", inherently_identity_bearing : false, fields : &[] };
        terrane_int_support::Int::from(i128::from(f32::MANTISSA_DIGITS)) } ==
        terrane_int_support::Int::from(24_i128)))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor {
        identity : "float32", name : "float32", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f32::EPSILON } > zero)),
        terrane_scalar_support::scalar_text(&{ let _ = &TerraneDescriptor { identity :
        "float32", name : "float32", kind : "type", inherently_identity_bearing : false,
        fields : &[] }; f32::MIN_POSITIVE } .is_normal())
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor {
        identity : "float32", name : "float32", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f32::from_bits(1) } >
        zero)), terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor {
        identity : "float32", name : "float32", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f32::MIN } < zero)),
        terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor { identity :
        "float32", name : "float32", kind : "type", inherently_identity_bearing : false,
        fields : &[] }; f32::MAX } > zero))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_result = two.asin();
        __terrane_result } .is_nan()), terrane_scalar_support::scalar_text(&{ let
        __terrane_result = negative_two.powf(0.5_f32); __terrane_result } .is_nan())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_result =
        overflow_input.exp2(); __terrane_result } .is_infinite()),
        terrane_scalar_support::scalar_text(&({ let __terrane_result = underflow_input
        .exp2(); __terrane_result } == zero))
    );
    not_a_number = {
        let __terrane_result = two.asin();
        __terrane_result
    };
    infinity = one / zero;
    clamped_zero = {
        let __terrane_result = {
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
                    if terrane_value.is_sign_positive()
                        || terrane_lower.is_sign_positive()
                    {
                        0.0
                    } else {
                        -0.0
                    }
                } else {
                    terrane_value.max(terrane_lower)
                };
                if terrane_lowered == 0.0 && terrane_upper == 0.0 {
                    if terrane_lowered.is_sign_negative()
                        || terrane_upper.is_sign_negative()
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
        __terrane_result
    };
    println!("{}", terrane_scalar_support::scalar_text(&(one / clamped_zero > zero)));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_result = { let
        terrane_value : f32 = not_a_number; let terrane_lower : f32 = zero; let
        terrane_upper : f32 = one; if terrane_value.is_nan() { terrane_value } else if
        terrane_lower.is_nan() || terrane_upper.is_nan() || terrane_lower > terrane_upper
        { f32::NAN } else { let terrane_lowered = if terrane_value == 0.0 &&terrane_lower
        == 0.0 { if terrane_value.is_sign_positive() || terrane_lower.is_sign_positive()
        { 0.0 } else { - 0.0 } } else { terrane_value.max(terrane_lower) }; if
        terrane_lowered == 0.0 &&terrane_upper == 0.0 { if terrane_lowered
        .is_sign_negative() || terrane_upper.is_sign_negative() { - 0.0 } else { 0.0 } }
        else { terrane_lowered.min(terrane_upper) } } }; __terrane_result } .is_nan()),
        terrane_scalar_support::scalar_text(&{ let __terrane_result = { let terrane_value
        : f32 = one; let terrane_lower : f32 = not_a_number; let terrane_upper : f32 =
        one; if terrane_value.is_nan() { terrane_value } else if terrane_lower.is_nan()
        || terrane_upper.is_nan() || terrane_lower > terrane_upper { f32::NAN } else {
        let terrane_lowered = if terrane_value == 0.0 &&terrane_lower == 0.0 { if
        terrane_value.is_sign_positive() || terrane_lower.is_sign_positive() { 0.0 } else
        { - 0.0 } } else { terrane_value.max(terrane_lower) }; if terrane_lowered == 0.0
        &&terrane_upper == 0.0 { if terrane_lowered.is_sign_negative() || terrane_upper
        .is_sign_negative() { - 0.0 } else { 0.0 } } else { terrane_lowered
        .min(terrane_upper) } } }; __terrane_result } .is_nan())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_result =
        not_a_number.hypot(infinity); __terrane_result } .is_infinite()),
        terrane_scalar_support::scalar_text(&{ let __terrane_result = infinity.next_up();
        __terrane_result } .is_infinite())
    );
    subnormal_decomposition = {
        let __terrane_result = terrane_scalar_support::decompose_f32({
            let _ = &TerraneDescriptor {
                identity: "float32",
                name: "float32",
                kind: "type",
                inherently_identity_bearing: false,
                fields: &[],
            };
            f32::from_bits(1)
        });
        __terrane_result
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result =
        terrane_scalar_support::scale_binary_f32(subnormal_decomposition.mantissa,
        subnormal_decomposition.exponent); __terrane_result } == { let _ =
        &TerraneDescriptor { identity : "float32", name : "float32", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f32::from_bits(1) }))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_result =
        terrane_scalar_support::scale_binary_f32({ let _ = &TerraneDescriptor { identity
        : "float32", name : "float32", kind : "type", inherently_identity_bearing :
        false, fields : &[] }; f32::MAX }, 1); __terrane_result } .is_infinite()),
        terrane_scalar_support::scalar_text(&({ let __terrane_result =
        terrane_scalar_support::scale_binary_f32({ let _ = &TerraneDescriptor { identity
        : "float32", name : "float32", kind : "type", inherently_identity_bearing :
        false, fields : &[] }; f32::from_bits(1) }, - 1); __terrane_result } == zero))
    );
    wide_four = 4.0;
    floating_exponent = 2.0;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = three
        .hypot({ let source_value = wide_four; let converted = source_value as f32; if
        converted as f64 == source_value { converted } else {
        __terrane_raised(Err(terrane_int_support::ArithmeticError::conversion_overflow(&source_value,
        "float64", "float32", "the floating value is not exactly representable")),
        0 /* terrane-site: case.trn:50:29-50:38 */) } }); __terrane_result } ==
        5.0_f32)), terrane_scalar_support::scalar_text(&({ let __terrane_result =
        terrane_scalar_support::scale_binary_f32(one,
        __terrane_raised(terrane_int_support::exact_from_f64:: < i32 >
        (floating_exponent), 1 /* terrane-site: case.trn:50:67-50:84 */));
        __terrane_result } == four))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_result = { let
        terrane_receiver : f32 = negative_one; let terrane_fraction = terrane_receiver
        .fract(); if terrane_fraction == 0.0 { 0.0_f32.copysign(terrane_receiver) } else
        { terrane_fraction } }; __terrane_result } .is_sign_negative())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_result = { let
        terrane_value : f32 = one; let terrane_lower : f32 = two; let terrane_upper : f32
        = one; if terrane_value.is_nan() { terrane_value } else if terrane_lower.is_nan()
        || terrane_upper.is_nan() || terrane_lower > terrane_upper { f32::NAN } else {
        let terrane_lowered = if terrane_value == 0.0 &&terrane_lower == 0.0 { if
        terrane_value.is_sign_positive() || terrane_lower.is_sign_positive() { 0.0 } else
        { - 0.0 } } else { terrane_value.max(terrane_lower) }; if terrane_lowered == 0.0
        &&terrane_upper == 0.0 { if terrane_lowered.is_sign_negative() || terrane_upper
        .is_sign_negative() { - 0.0 } else { 0.0 } } else { terrane_lowered
        .min(terrane_upper) } } }; __terrane_result } .is_nan())
    );
    nan_decomposition = {
        let __terrane_result = terrane_scalar_support::decompose_f32(not_a_number);
        __terrane_result
    };
    infinity_decomposition = {
        let __terrane_result = terrane_scalar_support::decompose_f32(infinity);
        __terrane_result
    };
    negative_zero_decomposition = {
        let __terrane_result = terrane_scalar_support::decompose_f32(negative_zero);
        __terrane_result
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&nan_decomposition.mantissa
        .is_nan()), terrane_scalar_support::scalar_text(&(nan_decomposition.exponent ==
        0))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&infinity_decomposition.mantissa
        .is_infinite()), terrane_scalar_support::scalar_text(&(infinity_decomposition
        .exponent == 0))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&negative_zero_decomposition.mantissa
        .is_sign_negative()),
        terrane_scalar_support::scalar_text(&(negative_zero_decomposition.exponent == 0))
    );
    negative_infinity = negative_one / zero;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_result = { let
        terrane_receiver : f32 = infinity; let terrane_fraction = terrane_receiver
        .fract(); if terrane_fraction == 0.0 { 0.0_f32.copysign(terrane_receiver) } else
        { terrane_fraction } }; __terrane_result } .is_nan()),
        terrane_scalar_support::scalar_text(&{ let __terrane_result = { let
        terrane_receiver : f32 = negative_infinity; let terrane_fraction =
        terrane_receiver.fract(); if terrane_fraction == 0.0 { 0.0_f32
        .copysign(terrane_receiver) } else { terrane_fraction } }; __terrane_result }
        .is_nan())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_result =
        negative_infinity.next_down(); __terrane_result } .is_infinite()),
        terrane_scalar_support::scalar_text(&{ let __terrane_result = negative_infinity
        .next_down(); __terrane_result } .is_sign_negative())
    );
    large_exponent = 120.0_f32;
    large = {
        let __terrane_result = large_exponent.exp2();
        __terrane_result
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result =
        terrane_scalar_support::scale_binary_f32(large, - 250); __terrane_result } == {
        let __terrane_result = terrane_scalar_support::scale_binary_f32(one, - 130);
        __terrane_result }))
    );
    descriptor = TerraneDescriptor {
        identity: "float32",
        name: "float32",
        kind: "type",
        inherently_identity_bearing: false,
        fields: &[],
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let _ = &descriptor; f32::EPSILON }
        == { let _ = &TerraneDescriptor { identity : "float32", name : "float32", kind :
        "type", inherently_identity_bearing : false, fields : &[] }; f32::EPSILON }))
    );
}
fn exercise64() {
    let zero: f64;
    let negative_zero: f64;
    let one: f64;
    let two: f64;
    let three: f64;
    let four: f64;
    let eight: f64;
    let negative_one: f64;
    let negative_two: f64;
    let negative_quarter: f64;
    let fractional: f64;
    let twelve: f64;
    let thousand: f64;
    let overflow_input: f64;
    let underflow_input: f64;
    let decomposition: terrane_scalar_support::FloatDecomposition<f64>;
    let not_a_number: f64;
    let infinity: f64;
    let clamped_zero: f64;
    let subnormal_decomposition: terrane_scalar_support::FloatDecomposition<f64>;
    let narrow_four: f32;
    let floating_exponent: f64;
    let nan_decomposition: terrane_scalar_support::FloatDecomposition<f64>;
    let infinity_decomposition: terrane_scalar_support::FloatDecomposition<f64>;
    let negative_zero_decomposition: terrane_scalar_support::FloatDecomposition<f64>;
    let negative_infinity: f64;
    let large_exponent: f64;
    let large: f64;
    let descriptor: TerraneDescriptor;
    zero = 0.0;
    negative_zero = -0.0_f64;
    one = 1.0;
    two = 2.0;
    three = 3.0;
    four = 4.0;
    eight = 8.0;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = eight
        .cbrt(); __terrane_result } == two)), terrane_scalar_support::scalar_text(&({ let
        __terrane_result = three.hypot(four); __terrane_result } == 5.0))
    );
    negative_one = -1.0_f64;
    negative_two = -2.0_f64;
    negative_quarter = -0.25_f64;
    fractional = -1.25_f64;
    twelve = 12.0;
    thousand = 1000.0;
    overflow_input = 1024.0;
    underflow_input = -1075.0_f64;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = four
        .powf(0.5); __terrane_result } == two)), terrane_scalar_support::scalar_text(&({
        let __terrane_result = two.powi(10); __terrane_result } == 1024.0))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = three
        .exp2(); __terrane_result } == eight)), terrane_scalar_support::scalar_text(&({
        let __terrane_result = zero.exp_m1(); __terrane_result } == zero))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero
        .ln_1p(); __terrane_result } == zero)), terrane_scalar_support::scalar_text(&({
        let __terrane_result = eight.log2(); __terrane_result } == three))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = thousand
        .log10(); __terrane_result } == three)), terrane_scalar_support::scalar_text(&({
        let __terrane_result = eight.log(two); __terrane_result } == three))
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero
        .tan(); __terrane_result } == zero)), terrane_scalar_support::scalar_text(&({ let
        __terrane_result = zero.asin(); __terrane_result } == zero)),
        terrane_scalar_support::scalar_text(&({ let __terrane_result = one.acos();
        __terrane_result } == zero))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero
        .atan(); __terrane_result } == zero)), terrane_scalar_support::scalar_text(&({
        let __terrane_result = zero.atan2(negative_one); __terrane_result } > three))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = one
        .copysign(negative_zero); __terrane_result } == negative_one)),
        terrane_scalar_support::scalar_text(&negative_zero.is_sign_negative())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = { let
        terrane_value : f64 = two; let terrane_lower : f64 = 3.0; let terrane_upper : f64
        = 4.0; if terrane_value.is_nan() { terrane_value } else if terrane_lower.is_nan()
        || terrane_upper.is_nan() || terrane_lower > terrane_upper { f64::NAN } else {
        let terrane_lowered = if terrane_value == 0.0 &&terrane_lower == 0.0 { if
        terrane_value.is_sign_positive() || terrane_lower.is_sign_positive() { 0.0 } else
        { - 0.0 } } else { terrane_value.max(terrane_lower) }; if terrane_lowered == 0.0
        &&terrane_upper == 0.0 { if terrane_lowered.is_sign_negative() || terrane_upper
        .is_sign_negative() { - 0.0 } else { 0.0 } } else { terrane_lowered
        .min(terrane_upper) } } }; __terrane_result } == three)),
        terrane_scalar_support::scalar_text(&({ let __terrane_result = { let
        terrane_receiver : f64 = fractional; let terrane_fraction = terrane_receiver
        .fract(); if terrane_fraction == 0.0 { 0.0_f64.copysign(terrane_receiver) } else
        { terrane_fraction } }; __terrane_result } == negative_quarter))
    );
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&(zero == 0.0)),
        terrane_scalar_support::scalar_text(&(negative_zero == 0.0)),
        terrane_scalar_support::scalar_text(&one.is_normal()),
        terrane_scalar_support::scalar_text(&{ let _ = &TerraneDescriptor { identity :
        "float64", name : "float64", kind : "type", inherently_identity_bearing : false,
        fields : &[] }; f64::from_bits(1) } .is_subnormal())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero
        .next_up(); __terrane_result } == { let _ = &TerraneDescriptor { identity :
        "float64", name : "float64", kind : "type", inherently_identity_bearing : false,
        fields : &[] }; f64::from_bits(1) }))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero
        .next_down(); __terrane_result } == negative_one * { let _ = &TerraneDescriptor {
        identity : "float64", name : "float64", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f64::from_bits(1) }))
    );
    decomposition = {
        let __terrane_result = terrane_scalar_support::decompose_f64(twelve);
        __terrane_result
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(decomposition.mantissa == 0.75)),
        terrane_scalar_support::scalar_text(&(decomposition.exponent == 4))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result =
        terrane_scalar_support::scale_binary_f64(decomposition.mantissa, decomposition
        .exponent); __terrane_result } == twelve))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor {
        identity : "float64", name : "float64", kind : "type",
        inherently_identity_bearing : false, fields : &[] };
        terrane_int_support::Int::from(i128::from(f64::RADIX)) } ==
        terrane_int_support::Int::from(2_i128))), terrane_scalar_support::scalar_text(&({
        let _ = &TerraneDescriptor { identity : "float64", name : "float64", kind :
        "type", inherently_identity_bearing : false, fields : &[] };
        terrane_int_support::Int::from(i128::from(f64::MANTISSA_DIGITS)) } ==
        terrane_int_support::Int::from(53_i128)))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor {
        identity : "float64", name : "float64", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f64::EPSILON } > zero)),
        terrane_scalar_support::scalar_text(&{ let _ = &TerraneDescriptor { identity :
        "float64", name : "float64", kind : "type", inherently_identity_bearing : false,
        fields : &[] }; f64::MIN_POSITIVE } .is_normal())
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor {
        identity : "float64", name : "float64", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f64::from_bits(1) } >
        zero)), terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor {
        identity : "float64", name : "float64", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f64::MIN } < zero)),
        terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor { identity :
        "float64", name : "float64", kind : "type", inherently_identity_bearing : false,
        fields : &[] }; f64::MAX } > zero))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_result = two.asin();
        __terrane_result } .is_nan()), terrane_scalar_support::scalar_text(&{ let
        __terrane_result = negative_two.powf(0.5); __terrane_result } .is_nan())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_result =
        overflow_input.exp2(); __terrane_result } .is_infinite()),
        terrane_scalar_support::scalar_text(&({ let __terrane_result = underflow_input
        .exp2(); __terrane_result } == zero))
    );
    not_a_number = {
        let __terrane_result = two.asin();
        __terrane_result
    };
    infinity = one / zero;
    clamped_zero = {
        let __terrane_result = {
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
                    if terrane_value.is_sign_positive()
                        || terrane_lower.is_sign_positive()
                    {
                        0.0
                    } else {
                        -0.0
                    }
                } else {
                    terrane_value.max(terrane_lower)
                };
                if terrane_lowered == 0.0 && terrane_upper == 0.0 {
                    if terrane_lowered.is_sign_negative()
                        || terrane_upper.is_sign_negative()
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
        __terrane_result
    };
    println!("{}", terrane_scalar_support::scalar_text(&(one / clamped_zero > zero)));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_result = { let
        terrane_value : f64 = not_a_number; let terrane_lower : f64 = zero; let
        terrane_upper : f64 = one; if terrane_value.is_nan() { terrane_value } else if
        terrane_lower.is_nan() || terrane_upper.is_nan() || terrane_lower > terrane_upper
        { f64::NAN } else { let terrane_lowered = if terrane_value == 0.0 &&terrane_lower
        == 0.0 { if terrane_value.is_sign_positive() || terrane_lower.is_sign_positive()
        { 0.0 } else { - 0.0 } } else { terrane_value.max(terrane_lower) }; if
        terrane_lowered == 0.0 &&terrane_upper == 0.0 { if terrane_lowered
        .is_sign_negative() || terrane_upper.is_sign_negative() { - 0.0 } else { 0.0 } }
        else { terrane_lowered.min(terrane_upper) } } }; __terrane_result } .is_nan()),
        terrane_scalar_support::scalar_text(&{ let __terrane_result = { let terrane_value
        : f64 = one; let terrane_lower : f64 = not_a_number; let terrane_upper : f64 =
        one; if terrane_value.is_nan() { terrane_value } else if terrane_lower.is_nan()
        || terrane_upper.is_nan() || terrane_lower > terrane_upper { f64::NAN } else {
        let terrane_lowered = if terrane_value == 0.0 &&terrane_lower == 0.0 { if
        terrane_value.is_sign_positive() || terrane_lower.is_sign_positive() { 0.0 } else
        { - 0.0 } } else { terrane_value.max(terrane_lower) }; if terrane_lowered == 0.0
        &&terrane_upper == 0.0 { if terrane_lowered.is_sign_negative() || terrane_upper
        .is_sign_negative() { - 0.0 } else { 0.0 } } else { terrane_lowered
        .min(terrane_upper) } } }; __terrane_result } .is_nan())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_result =
        not_a_number.hypot(infinity); __terrane_result } .is_infinite()),
        terrane_scalar_support::scalar_text(&{ let __terrane_result = infinity.next_up();
        __terrane_result } .is_infinite())
    );
    subnormal_decomposition = {
        let __terrane_result = terrane_scalar_support::decompose_f64({
            let _ = &TerraneDescriptor {
                identity: "float64",
                name: "float64",
                kind: "type",
                inherently_identity_bearing: false,
                fields: &[],
            };
            f64::from_bits(1)
        });
        __terrane_result
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result =
        terrane_scalar_support::scale_binary_f64(subnormal_decomposition.mantissa,
        subnormal_decomposition.exponent); __terrane_result } == { let _ =
        &TerraneDescriptor { identity : "float64", name : "float64", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f64::from_bits(1) }))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_result =
        terrane_scalar_support::scale_binary_f64({ let _ = &TerraneDescriptor { identity
        : "float64", name : "float64", kind : "type", inherently_identity_bearing :
        false, fields : &[] }; f64::MAX }, 1); __terrane_result } .is_infinite()),
        terrane_scalar_support::scalar_text(&({ let __terrane_result =
        terrane_scalar_support::scale_binary_f64({ let _ = &TerraneDescriptor { identity
        : "float64", name : "float64", kind : "type", inherently_identity_bearing :
        false, fields : &[] }; f64::from_bits(1) }, - 1); __terrane_result } == zero))
    );
    narrow_four = 4.0_f32;
    floating_exponent = 2.0;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = three
        .hypot(narrow_four as f64); __terrane_result } == 5.0)),
        terrane_scalar_support::scalar_text(&({ let __terrane_result =
        terrane_scalar_support::scale_binary_f64(one,
        __terrane_raised(terrane_int_support::exact_from_f64:: < i32 >
        (floating_exponent), 2 /* terrane-site: case.trn:115:69-115:86 */));
        __terrane_result } == four))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_result = { let
        terrane_receiver : f64 = negative_one; let terrane_fraction = terrane_receiver
        .fract(); if terrane_fraction == 0.0 { 0.0_f64.copysign(terrane_receiver) } else
        { terrane_fraction } }; __terrane_result } .is_sign_negative())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_result = { let
        terrane_value : f64 = one; let terrane_lower : f64 = two; let terrane_upper : f64
        = one; if terrane_value.is_nan() { terrane_value } else if terrane_lower.is_nan()
        || terrane_upper.is_nan() || terrane_lower > terrane_upper { f64::NAN } else {
        let terrane_lowered = if terrane_value == 0.0 &&terrane_lower == 0.0 { if
        terrane_value.is_sign_positive() || terrane_lower.is_sign_positive() { 0.0 } else
        { - 0.0 } } else { terrane_value.max(terrane_lower) }; if terrane_lowered == 0.0
        &&terrane_upper == 0.0 { if terrane_lowered.is_sign_negative() || terrane_upper
        .is_sign_negative() { - 0.0 } else { 0.0 } } else { terrane_lowered
        .min(terrane_upper) } } }; __terrane_result } .is_nan())
    );
    nan_decomposition = {
        let __terrane_result = terrane_scalar_support::decompose_f64(not_a_number);
        __terrane_result
    };
    infinity_decomposition = {
        let __terrane_result = terrane_scalar_support::decompose_f64(infinity);
        __terrane_result
    };
    negative_zero_decomposition = {
        let __terrane_result = terrane_scalar_support::decompose_f64(negative_zero);
        __terrane_result
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&nan_decomposition.mantissa
        .is_nan()), terrane_scalar_support::scalar_text(&(nan_decomposition.exponent ==
        0))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&infinity_decomposition.mantissa
        .is_infinite()), terrane_scalar_support::scalar_text(&(infinity_decomposition
        .exponent == 0))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&negative_zero_decomposition.mantissa
        .is_sign_negative()),
        terrane_scalar_support::scalar_text(&(negative_zero_decomposition.exponent == 0))
    );
    negative_infinity = negative_one / zero;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_result = { let
        terrane_receiver : f64 = infinity; let terrane_fraction = terrane_receiver
        .fract(); if terrane_fraction == 0.0 { 0.0_f64.copysign(terrane_receiver) } else
        { terrane_fraction } }; __terrane_result } .is_nan()),
        terrane_scalar_support::scalar_text(&{ let __terrane_result = { let
        terrane_receiver : f64 = negative_infinity; let terrane_fraction =
        terrane_receiver.fract(); if terrane_fraction == 0.0 { 0.0_f64
        .copysign(terrane_receiver) } else { terrane_fraction } }; __terrane_result }
        .is_nan())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_result =
        negative_infinity.next_down(); __terrane_result } .is_infinite()),
        terrane_scalar_support::scalar_text(&{ let __terrane_result = negative_infinity
        .next_down(); __terrane_result } .is_sign_negative())
    );
    large_exponent = 1000.0;
    large = {
        let __terrane_result = large_exponent.exp2();
        __terrane_result
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result =
        terrane_scalar_support::scale_binary_f64(large, - 2000); __terrane_result } == {
        let __terrane_result = terrane_scalar_support::scale_binary_f64(one, - 1000);
        __terrane_result }))
    );
    descriptor = TerraneDescriptor {
        identity: "float64",
        name: "float64",
        kind: "type",
        inherently_identity_bearing: false,
        fields: &[],
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let _ = &descriptor; f64::EPSILON }
        == { let _ = &TerraneDescriptor { identity : "float64", name : "float64", kind :
        "type", inherently_identity_bearing : false, fields : &[] }; f64::EPSILON }))
    );
}
fn main() {
    exercise32();
    exercise64();
}
