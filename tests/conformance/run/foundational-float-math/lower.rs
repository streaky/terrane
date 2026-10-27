// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: foundational-float-math
fn main() {
    let zero32: f32;
    let one32: f32;
    let nine32: f32;
    let root32: std::sync::Arc<dyn Fn() -> f32 + Send + Sync>;
    let pair32: terrane_collection_support::Tuple<f32>;
    let zero64: f64;
    let one64: f64;
    let nine64: f64;
    let pair64: terrane_collection_support::Tuple<f64>;
    let negative: f64;
    let not_a_number: f64;
    let negative_zero: f64;
    let reciprocal: f64;
    let negative_infinity: f64;
    let negative32: f32;
    let low32: f32;
    let high32: f32;
    let not_a_number32: f32;
    let infinity32: f32;
    let low64: f64;
    let high64: f64;
    let minimum64: std::sync::Arc<dyn Fn(f64) -> f64 + Send + Sync>;
    let maximum64: std::sync::Arc<dyn Fn(f64) -> f64 + Send + Sync>;
    let multiply_add64: std::sync::Arc<dyn Fn(f64, f64) -> f64 + Send + Sync>;
    let minimum_zero: f64;
    let maximum_zero: f64;
    let multiplicand: f64;
    let multiplier: f64;
    let addend: f64;
    let fused: f64;
    let unfused: f64;
    let multiplicand32: f32;
    let multiplier32: f32;
    let addend32: f32;
    let fused32: f32;
    let unfused32: f32;
    zero32 = 0.0_f32;
    one32 = 1.0_f32;
    nine32 = 9.0_f32;
    root32 = {
        let receiver = nine32;
        std::sync::Arc::new(move || receiver.sqrt())
    };
    println!("{}", terrane_scalar_support::scalar_text(&(root32() == 3.0_f32)));
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero32
        .sin(); __terrane_result } == 0.0_f32))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero32
        .cos(); __terrane_result } == 1.0_f32))
    );
    pair32 = {
        let __terrane_result = {
            let terrane_sine_cosine = zero32.sin_cos();
            terrane_collection_support::Tuple::new(
                vec![terrane_sine_cosine.0, terrane_sine_cosine.1],
            )
        };
        __terrane_result
    };
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(pair32
        .length()) == terrane_int_support::Int::from(2_i128))),
        terrane_scalar_support::scalar_text(&(__terrane_raised(pair32
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        0 /* terrane-site: case.trn:13:30-13:39 */)), 0 /* terrane-site: case.trn:13:30-13:39 */) == 0.0_f32)),
        terrane_scalar_support::scalar_text(&(__terrane_raised(pair32
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        1 /* terrane-site: case.trn:13:48-13:57 */)), 1 /* terrane-site: case.trn:13:48-13:57 */) == 1.0_f32))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = one32.ln();
        __terrane_result } == 0.0_f32))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero32
        .exp(); __terrane_result } == 1.0_f32))
    );
    zero64 = 0.0;
    one64 = 1.0;
    nine64 = 9.0;
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = nine64
        .sqrt(); __terrane_result } == 3.0))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero64
        .sin(); __terrane_result } == 0.0))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero64
        .cos(); __terrane_result } == 1.0))
    );
    pair64 = {
        let __terrane_result = {
            let terrane_sine_cosine = zero64.sin_cos();
            terrane_collection_support::Tuple::new(
                vec![terrane_sine_cosine.0, terrane_sine_cosine.1],
            )
        };
        __terrane_result
    };
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(pair64
        .length()) == terrane_int_support::Int::from(2_i128))),
        terrane_scalar_support::scalar_text(&(__terrane_raised(pair64
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        2 /* terrane-site: case.trn:24:30-24:39 */)), 2 /* terrane-site: case.trn:24:30-24:39 */) == 0.0)),
        terrane_scalar_support::scalar_text(&(__terrane_raised(pair64
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        3 /* terrane-site: case.trn:24:48-24:57 */)), 3 /* terrane-site: case.trn:24:48-24:57 */) == 1.0))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = one64.ln();
        __terrane_result } == 0.0))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = zero64
        .exp(); __terrane_result } == 1.0))
    );
    negative = -1.0_f64;
    not_a_number = {
        let __terrane_result = negative.sqrt();
        __terrane_result
    };
    println!("{}", terrane_scalar_support::scalar_text(&(not_a_number != not_a_number)));
    negative_zero = -0.0_f64;
    reciprocal = 1.0
        / {
            let __terrane_result = negative_zero.sqrt();
            __terrane_result
        };
    println!("{}", terrane_scalar_support::scalar_text(&(reciprocal < 0.0)));
    negative_infinity = {
        let __terrane_result = zero64.ln();
        __terrane_result
    };
    println!("{}", terrane_scalar_support::scalar_text(&(negative_infinity < 0.0)));
    negative32 = -3.0_f32;
    low32 = 2.0_f32;
    high32 = 5.0_f32;
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = negative32
        .abs(); __terrane_result } == 3.0_f32))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = { let
        terrane_receiver : f32 = low32; let terrane_argument : f32 = high32; if
        terrane_receiver == 0.0 &&terrane_argument == 0.0 { if terrane_receiver
        .is_sign_negative() || terrane_argument.is_sign_negative() { - 0.0 } else { 0.0 }
        } else { terrane_receiver.min(terrane_argument) } }; __terrane_result } ==
        2.0_f32))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = { let
        terrane_receiver : f32 = low32; let terrane_argument : f32 = high32; if
        terrane_receiver == 0.0 &&terrane_argument == 0.0 { if terrane_receiver
        .is_sign_positive() || terrane_argument.is_sign_positive() { 0.0 } else { - 0.0 }
        } else { terrane_receiver.max(terrane_argument) } }; __terrane_result } ==
        5.0_f32))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = low32
        .mul_add(high32, 1.0_f32); __terrane_result } == 11.0_f32))
    );
    println!("{}", terrane_scalar_support::scalar_text(&low32.is_finite()));
    not_a_number32 = {
        let __terrane_result = negative32.sqrt();
        __terrane_result
    };
    infinity32 = 1.0_f32 / zero32;
    println!("{}", terrane_scalar_support::scalar_text(&not_a_number32.is_nan()));
    println!("{}", terrane_scalar_support::scalar_text(&infinity32.is_infinite()));
    low64 = 2.0;
    high64 = 5.0;
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = negative
        .abs(); __terrane_result } == 1.0))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = { let
        terrane_receiver : f64 = low64; let terrane_argument : f64 = high64; if
        terrane_receiver == 0.0 &&terrane_argument == 0.0 { if terrane_receiver
        .is_sign_negative() || terrane_argument.is_sign_negative() { - 0.0 } else { 0.0 }
        } else { terrane_receiver.min(terrane_argument) } }; __terrane_result } == 2.0))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = { let
        terrane_receiver : f64 = low64; let terrane_argument : f64 = high64; if
        terrane_receiver == 0.0 &&terrane_argument == 0.0 { if terrane_receiver
        .is_sign_positive() || terrane_argument.is_sign_positive() { 0.0 } else { - 0.0 }
        } else { terrane_receiver.max(terrane_argument) } }; __terrane_result } == 5.0))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = low64
        .mul_add(high64, 1.0); __terrane_result } == 11.0))
    );
    minimum64 = {
        let receiver = low64;
        std::sync::Arc::new(move |argument_0: f64| {
            let terrane_receiver: f64 = receiver;
            let terrane_argument: f64 = argument_0;
            if terrane_receiver == 0.0 && terrane_argument == 0.0 {
                if terrane_receiver.is_sign_negative()
                    || terrane_argument.is_sign_negative()
                {
                    -0.0
                } else {
                    0.0
                }
            } else {
                terrane_receiver.min(terrane_argument)
            }
        })
    };
    maximum64 = {
        let receiver = low64;
        std::sync::Arc::new(move |argument_0: f64| {
            let terrane_receiver: f64 = receiver;
            let terrane_argument: f64 = argument_0;
            if terrane_receiver == 0.0 && terrane_argument == 0.0 {
                if terrane_receiver.is_sign_positive()
                    || terrane_argument.is_sign_positive()
                {
                    0.0
                } else {
                    -0.0
                }
            } else {
                terrane_receiver.max(terrane_argument)
            }
        })
    };
    multiply_add64 = {
        let receiver = low64;
        std::sync::Arc::new(move |argument_0: f64, argument_1: f64| {
            receiver.mul_add(argument_0, argument_1)
        })
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(minimum64(high64) == 2.0)),
        terrane_scalar_support::scalar_text(&(maximum64(high64) == 5.0)),
        terrane_scalar_support::scalar_text(&(multiply_add64(high64, 1.0) == 11.0))
    );
    println!("{}", terrane_scalar_support::scalar_text(&low64.is_finite()));
    println!("{}", terrane_scalar_support::scalar_text(&not_a_number.is_nan()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&negative_infinity.is_infinite())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = { let
        terrane_receiver : f64 = not_a_number; let terrane_argument : f64 = low64; if
        terrane_receiver == 0.0 &&terrane_argument == 0.0 { if terrane_receiver
        .is_sign_negative() || terrane_argument.is_sign_negative() { - 0.0 } else { 0.0 }
        } else { terrane_receiver.min(terrane_argument) } }; __terrane_result } ==
        low64))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = { let
        terrane_receiver : f64 = not_a_number; let terrane_argument : f64 = high64; if
        terrane_receiver == 0.0 &&terrane_argument == 0.0 { if terrane_receiver
        .is_sign_positive() || terrane_argument.is_sign_positive() { 0.0 } else { - 0.0 }
        } else { terrane_receiver.max(terrane_argument) } }; __terrane_result } ==
        high64))
    );
    minimum_zero = {
        let __terrane_result = {
            let terrane_receiver: f64 = negative_zero;
            let terrane_argument: f64 = zero64;
            if terrane_receiver == 0.0 && terrane_argument == 0.0 {
                if terrane_receiver.is_sign_negative()
                    || terrane_argument.is_sign_negative()
                {
                    -0.0
                } else {
                    0.0
                }
            } else {
                terrane_receiver.min(terrane_argument)
            }
        };
        __terrane_result
    };
    maximum_zero = {
        let __terrane_result = {
            let terrane_receiver: f64 = negative_zero;
            let terrane_argument: f64 = zero64;
            if terrane_receiver == 0.0 && terrane_argument == 0.0 {
                if terrane_receiver.is_sign_positive()
                    || terrane_argument.is_sign_positive()
                {
                    0.0
                } else {
                    -0.0
                }
            } else {
                terrane_receiver.max(terrane_argument)
            }
        };
        __terrane_result
    };
    println!("{}", terrane_scalar_support::scalar_text(&(1.0 / minimum_zero < 0.0)));
    println!("{}", terrane_scalar_support::scalar_text(&(1.0 / maximum_zero > 0.0)));
    multiplicand = 1.0000000000000002;
    multiplier = 1.0000000000000002;
    addend = -1.0000000000000004_f64;
    fused = {
        let __terrane_result = multiplicand.mul_add(multiplier, addend);
        __terrane_result
    };
    unfused = multiplicand * multiplier + addend;
    println!(
        "{}", terrane_scalar_support::scalar_text(&(fused ==
        0.00000000000000000000000000000004930380657631323784))
    );
    println!("{}", terrane_scalar_support::scalar_text(&(unfused == 0.0)));
    multiplicand32 = 1.0000001_f32;
    multiplier32 = 1.0000001_f32;
    addend32 = -1.0000002_f32;
    fused32 = {
        let __terrane_result = multiplicand32.mul_add(multiplier32, addend32);
        __terrane_result
    };
    unfused32 = multiplicand32 * multiplier32 + addend32;
    println!("{}", terrane_scalar_support::scalar_text(&(fused32 == 1.4210855e-14_f32)));
    println!("{}", terrane_scalar_support::scalar_text(&(unfused32 == 0.0_f32)));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = { let
        terrane_receiver : f32 = low32; let terrane_argument : f32 = 0.0_f32; if
        terrane_receiver == 0.0 &&terrane_argument == 0.0 { if terrane_receiver
        .is_sign_negative() || terrane_argument.is_sign_negative() { - 0.0 } else { 0.0 }
        } else { terrane_receiver.min(terrane_argument) } }; __terrane_result } ==
        0.0_f32)), terrane_scalar_support::scalar_text(&({ let __terrane_result = { let
        terrane_receiver : f32 = low32; let terrane_argument : f32 = 7.0_f32; if
        terrane_receiver == 0.0 &&terrane_argument == 0.0 { if terrane_receiver
        .is_sign_positive() || terrane_argument.is_sign_positive() { 0.0 } else { - 0.0 }
        } else { terrane_receiver.max(terrane_argument) } }; __terrane_result } ==
        7.0_f32))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let __terrane_result = { let
        terrane_receiver : f64 = low64; let terrane_argument : f64 = 0.0; if
        terrane_receiver == 0.0 &&terrane_argument == 0.0 { if terrane_receiver
        .is_sign_negative() || terrane_argument.is_sign_negative() { - 0.0 } else { 0.0 }
        } else { terrane_receiver.min(terrane_argument) } }; __terrane_result } == 0.0)),
        terrane_scalar_support::scalar_text(&({ let __terrane_result = { let
        terrane_receiver : f64 = low64; let terrane_argument : f64 = 7.0; if
        terrane_receiver == 0.0 &&terrane_argument == 0.0 { if terrane_receiver
        .is_sign_positive() || terrane_argument.is_sign_positive() { 0.0 } else { - 0.0 }
        } else { terrane_receiver.max(terrane_argument) } }; __terrane_result } == 7.0))
    );
}
