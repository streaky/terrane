// Generated deterministically by Terrane 0.1.0.
include!("main.lowered.support.rs");
// Source: src/main.trn
// Namespace: benchmark-collatz-branching
fn benchmark_size() -> i64 {
    let supplied: terrane_collection_support::List<NativeString> = arguments();
    if terrane_int_support::Int::from(terrane_int_support::Int::from(supplied.length()))
        != terrane_int_support::Int::from(1_i128)
    {
        exit(make_exit_status(terrane_int_support::Int::from(2_i128)));
    }
    let limit: i64 = __terrane_raised(
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
                                        0 /* terrane-site: src/main.trn:9:18-9:29 */,
                                    ),
                                ),
                            0 /* terrane-site: src/main.trn:9:18-9:29 */,
                        )
                        .text,
                    &10,
                ),
                1 /* terrane-site: src/main.trn:9:18-9:44 */,
            ),
        ),
        2 /* terrane-site: src/main.trn:9:17-9:45 */,
    );
    if limit <= 0 {
        exit(make_exit_status(terrane_int_support::Int::from(2_i128)));
    }
    return limit;
}
fn main() {
    let limit: i64 = benchmark_size();
    let mut total: i64 = 0;
    let mut start: i64 = 1;
    while start <= limit {
        let mut value: i64 = start;
        while value != 1 {
            if value.rem_euclid(2) == 0 {
                value = __terrane_raised(
                    terrane_int_support::fixed_division(value, 2),
                    3 /* terrane-site: src/main.trn:23:17-23:26 */,
                );
            } else {
                value = __terrane_raised(
                    terrane_int_support::fixed_addition(
                        __terrane_raised(
                            terrane_int_support::fixed_multiplication(3, value),
                            4 /* terrane-site: src/main.trn:25:17-25:26 */,
                        ),
                        1,
                    ),
                    5 /* terrane-site: src/main.trn:25:17-25:30 */,
                );
            }
            total = __terrane_raised(
                terrane_int_support::fixed_addition(total, 1),
                6 /* terrane-site: src/main.trn:26:7-26:14 */,
            );
        }
        start = __terrane_raised(
            terrane_int_support::fixed_addition(start, 1),
            7 /* terrane-site: src/main.trn:27:5-27:12 */,
        );
    }
    println!("{}", terrane_scalar_support::scalar_text(&total));
}
