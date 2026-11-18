// Generated deterministically by Terrane 0.1.0.
include!("main.lowered.support.rs");
// Source: src/main.trn
// Namespace: benchmark-collatz-branching
fn benchmark_size() -> i64 {
    let supplied: terrane_collection_support::List<NativeString>;
    let limit: i64;
    supplied = arguments();
    if terrane_int_support::Int::from(supplied.length())
        != terrane_int_support::Int::from(1_i128)
    {
        exit(make_exit_status(terrane_int_support::Int::from(2_i128)));
    }
    limit = __terrane_raised(
        terrane_int_support::coerce::<
            i64,
        >(
            &__terrane_raised(
                terrane_int_support::parse_radix(
                    &__terrane_raised(
                            supplied
                                .get_or_error(
                                    __terrane_raised(
                                        terrane_collection_support::index_from_int(
                                            &terrane_int_support::Int::from(0_i128),
                                        ),
                                        0 /* terrane-site: src/main.trn:10:17-10:28 */,
                                    ),
                                ),
                            0 /* terrane-site: src/main.trn:10:17-10:28 */,
                        )
                        .text,
                    &10,
                ),
                1 /* terrane-site: src/main.trn:10:17-10:43 */,
            ),
        ),
        1 /* terrane-site: src/main.trn:10:17-10:43 */,
    );
    if limit <= 0 {
        exit(make_exit_status(terrane_int_support::Int::from(2_i128)));
    }
    return limit;
}
fn main() {
    let limit: i64;
    let mut total: i64;
    let mut start: i64;
    let mut value: i64;
    limit = benchmark_size();
    total = 0;
    start = 1;
    while start <= limit {
        value = start;
        while value != 1 {
            value = if value as u64 <= 3074457345618258602_u64 {
                let __terrane_guarded_then = value / 2_i64;
                let __terrane_guarded_else = 3_i64 * value as i64 + 1_i64;
                let __terrane_guarded_mask = 0_i64
                    .wrapping_sub((value.rem_euclid(2) == 0) as i64);
                __terrane_guarded_else
                    ^ (__terrane_guarded_then ^ __terrane_guarded_else)
                        & __terrane_guarded_mask
            } else if value.rem_euclid(2) == 0 {
                __terrane_raised(
                    terrane_int_support::fixed_division(value, 2),
                    2 /* terrane-site: src/main.trn:25:17-25:26 */,
                )
            } else {
                __terrane_raised(
                    terrane_int_support::fixed_addition(
                        __terrane_raised(
                            terrane_int_support::fixed_multiplication(3, value),
                            3 /* terrane-site: src/main.trn:27:17-27:26 */,
                        ),
                        1,
                    ),
                    4 /* terrane-site: src/main.trn:27:17-27:30 */,
                )
            };
            total = __terrane_raised(
                terrane_int_support::fixed_addition(total, 1),
                5 /* terrane-site: src/main.trn:28:7-28:14 */,
            );
        }
        start = __terrane_raised(
            terrane_int_support::fixed_addition(start, 1),
            6 /* terrane-site: src/main.trn:29:5-29:12 */,
        );
    }
    println!("{}", terrane_scalar_support::scalar_text(&total));
}
