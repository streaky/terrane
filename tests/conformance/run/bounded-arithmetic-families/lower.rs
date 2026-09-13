// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: bounded-arithmetic-families
fn main() {
    let small: i8 = 120;
    let wrapped: i8 = terrane_int_support::fixed_addition_wrap(small, 10);
    println!("{}", terrane_scalar_support::scalar_text(&wrapped));
    let overflowed: terrane_int_support::OverflowResult<i8> = terrane_int_support::fixed_addition_overflowing(
        small,
        10,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&overflowed.value),
        terrane_scalar_support::scalar_text(&overflowed.overflowed)
    );
    let pair: terrane_int_support::DivRemResult<terrane_int_support::Int> = __terrane_raised(
        terrane_int_support::Int::from(-7_i128)
            .div_rem(&terrane_int_support::Int::from(3_i128)),
        0 /* terrane-site: case.trn:8:10-8:26 */,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&pair.quotient),
        terrane_scalar_support::scalar_text(&pair.remainder)
    );
    let exact: i64 = 5;
    println!(
        "{}", terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(exact
        as i128) * terrane_int_support::Int::from(9_i128)))
    );
    terrane_int_support::fixed_subtraction_checked(small, 20);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::fixed_multiplication_saturate(small,
        2))
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_division(small,
        3), 1 /* terrane-site: case.trn:14:11-14:26 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_remainder(small,
        7), 2 /* terrane-site: case.trn:14:30-14:48 */))
    );
    let negated: terrane_int_support::OverflowResult<i8> = terrane_int_support::fixed_negation_overflowing(
        small,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&negated.value),
        terrane_scalar_support::scalar_text(&negated.overflowed)
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_shift_left_wrap(small,
        &1), 3 /* terrane-site: case.trn:17:11-17:35 */))
    );
    __terrane_raised(
        terrane_int_support::fixed_shift_right_checked(small, &2),
        4 /* terrane-site: case.trn:18:3-18:31 */,
    );
    let mut count: i8 = 1;
    count = __terrane_raised(
        terrane_int_support::fixed_addition(count, 1),
        5 /* terrane-site: case.trn:20:3-20:10 */,
    );
    count = __terrane_raised(
        terrane_int_support::fixed_subtraction(count, 1),
        6 /* terrane-site: case.trn:21:3-21:10 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&count));
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::fixed_addition_checked(small,
        10).is_none()),
        terrane_scalar_support::scalar_text(&terrane_int_support::fixed_addition_saturate(small,
        10))
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::fixed_subtraction_wrap(small,
        - 20)),
        terrane_scalar_support::scalar_text(&terrane_int_support::fixed_subtraction_saturate(small,
        - 20))
    );
    let sub_overflow: terrane_int_support::OverflowResult<i8> = terrane_int_support::fixed_subtraction_overflowing(
        small,
        -20,
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&sub_overflow.value),
        terrane_scalar_support::scalar_text(&sub_overflow.overflowed),
        terrane_scalar_support::scalar_text(&terrane_int_support::fixed_subtraction_checked(small,
        - 20).is_none())
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::fixed_multiplication_wrap(small,
        2)),
        terrane_scalar_support::scalar_text(&terrane_int_support::fixed_multiplication_checked(small,
        2).is_none())
    );
    let mul_overflow: terrane_int_support::OverflowResult<i8> = terrane_int_support::fixed_multiplication_overflowing(
        small,
        2,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&mul_overflow.value),
        terrane_scalar_support::scalar_text(&mul_overflow.overflowed)
    );
    let minimum: i8 = -128;
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_division_wrap(minimum,
        - 1), 7 /* terrane-site: case.trn:31:11-31:34 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_division_saturate(minimum,
        - 1), 8 /* terrane-site: case.trn:31:38-31:65 */))
    );
    let div_overflow: terrane_int_support::OverflowResult<i8> = __terrane_raised(
        terrane_int_support::fixed_division_overflowing(minimum, -1),
        9 /* terrane-site: case.trn:32:18-32:48 */,
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&div_overflow.value),
        terrane_scalar_support::scalar_text(&div_overflow.overflowed),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_division_checked(minimum,
        - 1), 10 /* terrane-site: case.trn:33:56-33:82 */).is_none())
    );
    let rem_overflow: terrane_int_support::OverflowResult<i8> = __terrane_raised(
        terrane_int_support::fixed_remainder_overflowing(minimum, -1),
        11 /* terrane-site: case.trn:34:18-34:51 */,
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_remainder_wrap(minimum,
        - 1), 12 /* terrane-site: case.trn:35:11-35:37 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_remainder_saturate(minimum,
        - 1), 13 /* terrane-site: case.trn:35:41-35:71 */))
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&rem_overflow.value),
        terrane_scalar_support::scalar_text(&rem_overflow.overflowed),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_remainder_checked(minimum,
        - 1), 14 /* terrane-site: case.trn:36:56-36:85 */).is_none())
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::Int::from(-
        7_i128).euclidean_div(&terrane_int_support::Int::from(3_i128)),
        15 /* terrane-site: case.trn:37:11-37:26 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::Int::from(-
        7_i128).modulo(&terrane_int_support::Int::from(3_i128)), 16 /* terrane-site: case.trn:37:30-37:48 */))
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::Int::from(7_i128)
        .euclidean_div(&- terrane_int_support::Int::from(3_i128)), 17 /* terrane-site: case.trn:38:11-38:25 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::Int::from(7_i128)
        .modulo(&- terrane_int_support::Int::from(3_i128)), 18 /* terrane-site: case.trn:38:29-38:46 */))
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::Int::from(-
        7_i128).euclidean_div(&- terrane_int_support::Int::from(3_i128)),
        19 /* terrane-site: case.trn:39:11-39:27 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::Int::from(-
        7_i128).modulo(&- terrane_int_support::Int::from(3_i128)), 20 /* terrane-site: case.trn:39:31-39:50 */))
    );
}
