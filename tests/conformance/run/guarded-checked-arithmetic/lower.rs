// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: guarded-checked-arithmetic
fn transition(input: i8) -> i8 {
    let mut value: i8;
    value = input;
    value = if value as u8 <= 42_u8 {
        let __terrane_guarded_then = value / 2_i8;
        let __terrane_guarded_else = 3_i8 * value as i8 + 1_i8;
        let __terrane_guarded_mask = 0_i8.wrapping_sub((value.rem_euclid(2) == 0) as i8);
        __terrane_guarded_else
            ^ (__terrane_guarded_then ^ __terrane_guarded_else) & __terrane_guarded_mask
    } else if value.rem_euclid(2) == 0 {
        __terrane_raised(
            terrane_int_support::fixed_division(value, 2),
            0 /* terrane-site: case.trn:10:13-10:22 */,
        )
    } else {
        __terrane_raised(
            terrane_int_support::fixed_addition(
                __terrane_raised(
                    terrane_int_support::fixed_multiplication(3, value),
                    1 /* terrane-site: case.trn:12:13-12:22 */,
                ),
                1,
            ),
            2 /* terrane-site: case.trn:12:13-12:26 */,
        )
    };
    return value;
}
fn transition_uint8(input: u8) -> u8 {
    let mut value: u8;
    value = input;
    value = if value <= 84_u8 {
        let __terrane_guarded_then = value / 2_u8;
        let __terrane_guarded_else = 3_u8 * value as u8 + 1_u8;
        let __terrane_guarded_mask = 0_u8.wrapping_sub((value.rem_euclid(2) == 0) as u8);
        __terrane_guarded_else
            ^ (__terrane_guarded_then ^ __terrane_guarded_else) & __terrane_guarded_mask
    } else if value.rem_euclid(2) == 0 {
        __terrane_raised(
            terrane_int_support::fixed_division(value, 2),
            3 /* terrane-site: case.trn:18:13-18:22 */,
        )
    } else {
        __terrane_raised(
            terrane_int_support::fixed_addition(
                __terrane_raised(
                    terrane_int_support::fixed_multiplication(3, value),
                    4 /* terrane-site: case.trn:20:13-20:22 */,
                ),
                1,
            ),
            5 /* terrane-site: case.trn:20:13-20:26 */,
        )
    };
    return value;
}
fn transition_then(input: i8) -> i8 {
    let mut value: i8;
    value = input;
    value = if value as u8 <= 42_u8 {
        let __terrane_guarded_then = 3_i8 * value as i8 + 1_i8;
        let __terrane_guarded_else = value / 2_i8;
        let __terrane_guarded_mask = 0_i8.wrapping_sub((value.rem_euclid(2) != 0) as i8);
        __terrane_guarded_else
            ^ (__terrane_guarded_then ^ __terrane_guarded_else) & __terrane_guarded_mask
    } else if value.rem_euclid(2) != 0 {
        __terrane_raised(
            terrane_int_support::fixed_addition(
                __terrane_raised(
                    terrane_int_support::fixed_multiplication(3, value),
                    6 /* terrane-site: case.trn:26:13-26:22 */,
                ),
                1,
            ),
            7 /* terrane-site: case.trn:26:13-26:26 */,
        )
    } else {
        __terrane_raised(
            terrane_int_support::fixed_division(value, 2),
            8 /* terrane-site: case.trn:28:13-28:22 */,
        )
    };
    return value;
}
fn transition_remainder(input: i8) -> i8 {
    let mut value: i8;
    value = input;
    value = if value as u8 <= 42_u8 {
        let __terrane_guarded_then = value % 5_i8;
        let __terrane_guarded_else = 3_i8 * value as i8 + 1_i8;
        let __terrane_guarded_mask = 0_i8.wrapping_sub((value.rem_euclid(2) == 0) as i8);
        __terrane_guarded_else
            ^ (__terrane_guarded_then ^ __terrane_guarded_else) & __terrane_guarded_mask
    } else if value.rem_euclid(2) == 0 {
        value.rem_euclid(5)
    } else {
        __terrane_raised(
            terrane_int_support::fixed_addition(
                __terrane_raised(
                    terrane_int_support::fixed_multiplication(3, value),
                    9 /* terrane-site: case.trn:36:13-36:22 */,
                ),
                1,
            ),
            10 /* terrane-site: case.trn:36:13-36:26 */,
        )
    };
    return value;
}
fn transition_negative_coefficient(input: i8) -> i8 {
    let mut value: i8;
    value = input;
    value = if value as u8 <= 127_u8 {
        let __terrane_guarded_then = value / 2_i8;
        let __terrane_guarded_else = 100_i8 - value as i8;
        let __terrane_guarded_mask = 0_i8.wrapping_sub((value.rem_euclid(2) == 0) as i8);
        __terrane_guarded_else
            ^ (__terrane_guarded_then ^ __terrane_guarded_else) & __terrane_guarded_mask
    } else if value.rem_euclid(2) == 0 {
        __terrane_raised(
            terrane_int_support::fixed_division(value, 2),
            11 /* terrane-site: case.trn:42:13-42:22 */,
        )
    } else {
        __terrane_raised(
            terrane_int_support::fixed_subtraction(100, value),
            12 /* terrane-site: case.trn:44:13-44:24 */,
        )
    };
    return value;
}
fn transition_int64(input: i64) -> i64 {
    let mut value: i64;
    value = input;
    value = if value as u64 <= 3074457345618258602_u64 {
        let __terrane_guarded_then = value / 2_i64;
        let __terrane_guarded_else = 3_i64 * value as i64 + 1_i64;
        let __terrane_guarded_mask = 0_i64
            .wrapping_sub((value.rem_euclid(2) == 0) as i64);
        __terrane_guarded_else
            ^ (__terrane_guarded_then ^ __terrane_guarded_else) & __terrane_guarded_mask
    } else if value.rem_euclid(2) == 0 {
        __terrane_raised(
            terrane_int_support::fixed_division(value, 2),
            13 /* terrane-site: case.trn:50:13-50:22 */,
        )
    } else {
        __terrane_raised(
            terrane_int_support::fixed_addition(
                __terrane_raised(
                    terrane_int_support::fixed_multiplication(3, value),
                    14 /* terrane-site: case.trn:52:13-52:22 */,
                ),
                1,
            ),
            15 /* terrane-site: case.trn:52:13-52:26 */,
        )
    };
    return value;
}
fn transition_int128(input: i128) -> i128 {
    let mut value: i128;
    value = input;
    value = if value as u128 <= 56713727820156410577229101238628035242_u128 {
        let __terrane_guarded_then = value / 2_i128;
        let __terrane_guarded_else = 3_i128 * value as i128 + 1_i128;
        let __terrane_guarded_mask = 0_i128
            .wrapping_sub((value.rem_euclid(2) == 0) as i128);
        __terrane_guarded_else
            ^ (__terrane_guarded_then ^ __terrane_guarded_else) & __terrane_guarded_mask
    } else if value.rem_euclid(2) == 0 {
        __terrane_raised(
            terrane_int_support::fixed_division(value, 2),
            16 /* terrane-site: case.trn:58:13-58:22 */,
        )
    } else {
        __terrane_raised(
            terrane_int_support::fixed_addition(
                __terrane_raised(
                    terrane_int_support::fixed_multiplication(3, value),
                    17 /* terrane-site: case.trn:60:13-60:22 */,
                ),
                1,
            ),
            18 /* terrane-site: case.trn:60:13-60:26 */,
        )
    };
    return value;
}
fn both_affine(input: i8) -> i8 {
    let mut value: i8;
    value = input;
    if value > 0 {
        value = __terrane_raised(
            terrane_int_support::fixed_addition(value, 1),
            19 /* terrane-site: case.trn:66:13-66:22 */,
        );
    } else {
        value = __terrane_raised(
            terrane_int_support::fixed_addition(
                __terrane_raised(
                    terrane_int_support::fixed_multiplication(3, value),
                    20 /* terrane-site: case.trn:68:13-68:22 */,
                ),
                1,
            ),
            21 /* terrane-site: case.trn:68:13-68:26 */,
        );
    }
    return value;
}
fn nested_conditional(input: i8) -> i8 {
    let mut value: i8;
    value = input;
    if value < 0 {
        value = __terrane_raised(
            terrane_int_support::fixed_division(value, 2),
            22 /* terrane-site: case.trn:74:13-74:22 */,
        );
    } else if value.rem_euclid(2) == 0 {
        value = __terrane_raised(
            terrane_int_support::fixed_division(value, 2),
            23 /* terrane-site: case.trn:76:13-76:22 */,
        );
    } else {
        value = __terrane_raised(
            terrane_int_support::fixed_addition(
                __terrane_raised(
                    terrane_int_support::fixed_multiplication(3, value),
                    24 /* terrane-site: case.trn:78:13-78:22 */,
                ),
                1,
            ),
            25 /* terrane-site: case.trn:78:13-78:26 */,
        );
    }
    return value;
}
fn positive_lower_bound(input: i8) {
    let mut value: i8;
    value = input;
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            value = if value >= 72_i8 {
                let __terrane_guarded_then = value as i8 - 100_i8 - 100_i8;
                let __terrane_guarded_else = value / 2_i8;
                let __terrane_guarded_mask = 0_i8
                    .wrapping_sub((value.rem_euclid(2) == 0) as i8);
                __terrane_guarded_else
                    ^ (__terrane_guarded_then ^ __terrane_guarded_else)
                        & __terrane_guarded_mask
            } else if value.rem_euclid(2) == 0 {
                __terrane_raised_completion!(
                    terrane_int_support::fixed_subtraction(__terrane_raised_completion!(terrane_int_support::fixed_subtraction(value,
                    100), 26 /* terrane-site: case.trn:85:15-85:26 */), 100),
                    27 /* terrane-site: case.trn:85:15-85:32 */
                )
            } else {
                __terrane_raised_completion!(
                    terrane_int_support::fixed_division(value, 2), 28 /* terrane-site: case.trn:87:15-87:24 */
                )
            };
            println!("{}", terrane_scalar_support::scalar_text(&value));
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0
                    && __terrane_error_0.kind == TerraneErrorKind::ArithmeticOverflow
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("lower-caught"));
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
fn global_condition(input: i8) -> i8 {
    let mut value: i8;
    value = input;
    if __TERRANE_GLOBAL_PARITY
        .lock()
        .expect("program-global lock poisoned")
        .clone()
        .unwrap_or_else(|| __terrane_uninitialized_binding("parity", "case.trn", 94, 6))
        == 0
    {
        value = __terrane_raised(
            terrane_int_support::fixed_division(value, 2),
            29 /* terrane-site: case.trn:95:13-95:22 */,
        );
    } else {
        value = __terrane_raised(
            terrane_int_support::fixed_addition(
                __terrane_raised(
                    terrane_int_support::fixed_multiplication(3, value),
                    30 /* terrane-site: case.trn:97:13-97:22 */,
                ),
                1,
            ),
            31 /* terrane-site: case.trn:97:13-97:26 */,
        );
    }
    return value;
}
fn main() {
    let mut upper: i8;
    let mut intermediate: i8;
    let shared_value: std::sync::Arc<std::sync::Mutex<i8>>;
    let observer: std::sync::Arc<std::sync::Mutex<i8>>;
    println!("{}", terrane_scalar_support::scalar_text(&transition(42)));
    println!("{}", terrane_scalar_support::scalar_text(&transition(41)));
    positive_lower_bound(72);
    positive_lower_bound(10);
    println!("{}", terrane_scalar_support::scalar_text(&global_condition(41)));
    upper = 43;
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            upper = if upper as u8 <= 42_u8 {
                let __terrane_guarded_then = upper / 2_i8;
                let __terrane_guarded_else = 3_i8 * upper as i8 + 1_i8;
                let __terrane_guarded_mask = 0_i8
                    .wrapping_sub((upper.rem_euclid(2) == 0) as i8);
                __terrane_guarded_else
                    ^ (__terrane_guarded_then ^ __terrane_guarded_else)
                        & __terrane_guarded_mask
            } else if upper.rem_euclid(2) == 0 {
                __terrane_raised_completion!(
                    terrane_int_support::fixed_division(upper, 2), 32 /* terrane-site: case.trn:109:15-109:24 */
                )
            } else {
                __terrane_raised_completion!(
                    terrane_int_support::fixed_addition(__terrane_raised_completion!(terrane_int_support::fixed_multiplication(3,
                    upper), 33 /* terrane-site: case.trn:111:15-111:24 */), 1),
                    34 /* terrane-site: case.trn:111:15-111:28 */
                )
            };
            println!("{}", terrane_scalar_support::scalar_text(&upper));
            TerraneCompletion::Normal
        })();
        match __terrane_try_1 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_1) => {
                let mut __terrane_handled_1 = false;
                if !__terrane_handled_1
                    && __terrane_error_1.kind == TerraneErrorKind::ArithmeticOverflow
                {
                    __terrane_handled_1 = true;
                    println!("{}", String::from("upper-caught"));
                }
                if !__terrane_handled_1 {
                    return TerraneCompletion::Error(__terrane_error_1);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_1 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    intermediate = -43;
    let __terrane_completion_2: TerraneCompletion<()> = (|| {
        let __terrane_try_2: TerraneCompletion<()> = (|| {
            intermediate = if intermediate as u8 <= 42_u8 {
                let __terrane_guarded_then = intermediate / 2_i8;
                let __terrane_guarded_else = 3_i8 * intermediate as i8 + 1_i8;
                let __terrane_guarded_mask = 0_i8
                    .wrapping_sub((intermediate.rem_euclid(2) == 0) as i8);
                __terrane_guarded_else
                    ^ (__terrane_guarded_then ^ __terrane_guarded_else)
                        & __terrane_guarded_mask
            } else if intermediate.rem_euclid(2) == 0 {
                __terrane_raised_completion!(
                    terrane_int_support::fixed_division(intermediate, 2),
                    35 /* terrane-site: case.trn:118:22-118:38 */
                )
            } else {
                __terrane_raised_completion!(
                    terrane_int_support::fixed_addition(__terrane_raised_completion!(terrane_int_support::fixed_multiplication(3,
                    intermediate), 36 /* terrane-site: case.trn:120:22-120:38 */), 1),
                    37 /* terrane-site: case.trn:120:22-120:42 */
                )
            };
            println!("{}", terrane_scalar_support::scalar_text(&intermediate));
            TerraneCompletion::Normal
        })();
        match __terrane_try_2 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_2) => {
                let mut __terrane_handled_2 = false;
                if !__terrane_handled_2
                    && __terrane_error_2.kind == TerraneErrorKind::ArithmeticOverflow
                {
                    __terrane_handled_2 = true;
                    println!("{}", String::from("intermediate-caught"));
                }
                if !__terrane_handled_2 {
                    return TerraneCompletion::Error(__terrane_error_2);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_2 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    println!("{}", terrane_scalar_support::scalar_text(&transition_uint8(41)));
    println!("{}", terrane_scalar_support::scalar_text(&transition_then(41)));
    println!("{}", terrane_scalar_support::scalar_text(&transition_remainder(42)));
    println!(
        "{}", terrane_scalar_support::scalar_text(&transition_negative_coefficient(41))
    );
    println!("{}", terrane_scalar_support::scalar_text(&transition_int64(41)));
    println!("{}", terrane_scalar_support::scalar_text(&transition_int128(41)));
    println!("{}", terrane_scalar_support::scalar_text(&both_affine(41)));
    println!("{}", terrane_scalar_support::scalar_text(&nested_conditional(41)));
    shared_value = std::sync::Arc::new(std::sync::Mutex::new(41));
    observer = shared_value.clone();
    println!(
        "{}", terrane_scalar_support::scalar_text(&* observer.lock()
        .expect("shared reference lock poisoned"))
    );
    if {
        let __terrane_value = shared_value
            .lock()
            .expect("reference lock poisoned")
            .clone();
        __terrane_value
    }
        .rem_euclid(2) == 0
    {
        *shared_value.lock().expect("reference lock poisoned") = __terrane_raised(
            terrane_int_support::fixed_division(
                {
                    let __terrane_value = shared_value
                        .lock()
                        .expect("reference lock poisoned")
                        .clone();
                    __terrane_value
                },
                2,
            ),
            38 /* terrane-site: case.trn:136:20-136:36 */,
        );
    } else {
        *shared_value.lock().expect("reference lock poisoned") = __terrane_raised(
            terrane_int_support::fixed_addition(
                __terrane_raised(
                    terrane_int_support::fixed_multiplication(
                        3,
                        {
                            let __terrane_value = shared_value
                                .lock()
                                .expect("reference lock poisoned")
                                .clone();
                            __terrane_value
                        },
                    ),
                    39 /* terrane-site: case.trn:138:20-138:36 */,
                ),
                1,
            ),
            40 /* terrane-site: case.trn:138:20-138:40 */,
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&* shared_value.lock()
        .expect("reference lock poisoned"))
    );
}
