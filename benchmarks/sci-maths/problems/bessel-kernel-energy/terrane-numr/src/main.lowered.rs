// Generated deterministically by Terrane 0.1.0.
include!("main.lowered.support.rs");
// Source: src/main.trn
// Namespace: benchmark-bessel-kernel-energy
fn benchmark_size() -> i64 {
    let supplied: terrane_collection_support::List<PlatformString> = arguments();
    if terrane_int_support::Int::from(terrane_int_support::Int::from(supplied.length()))
        != terrane_int_support::Int::from(1_i128)
    {
        exit(make_exit_status(terrane_int_support::Int::from(2_i128)));
    }
    let count: i64 = __terrane_raised(
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
                                        0 /* terrane-site: src/main.trn:10:18-10:29 */,
                                    ),
                                ),
                            0 /* terrane-site: src/main.trn:10:18-10:29 */,
                        )
                        .text,
                    &10,
                ),
                1 /* terrane-site: src/main.trn:10:18-10:44 */,
            ),
        ),
        2 /* terrane-site: src/main.trn:10:17-10:45 */,
    );
    if count <= 0 {
        exit(make_exit_status(terrane_int_support::Int::from(2_i128)));
    }
    return count;
}
fn coordinate(index: i64) -> f64 {
    let raw: f64 = __terrane_raised(
        terrane_int_support::exact_fixed_f64(
            __terrane_raised(
                    terrane_int_support::fixed_multiplication(index, 37),
                    3 /* terrane-site: src/main.trn:16:18-16:28 */,
                )
                .rem_euclid(1009),
        ),
        4 /* terrane-site: src/main.trn:16:17-16:36 */,
    );
    return raw / 1009.0_f64;
}
fn main() {
    let count: i64 = benchmark_size();
    let mut total: f64 = 0.0_f64;
    let mut left_index: i64 = 0;
    while left_index < count {
        let left: f64 = coordinate(left_index);
        let mut right_index: i64 = 0;
        while right_index < count {
            let right: f64 = coordinate(right_index);
            let mut distance: f64 = left - right;
            if distance < 0.0_f64 {
                distance = 0.0_f64 - distance;
            }
            let argument: f64 = 18.0_f64 * distance;
            let value: f64 = __terrane_raised(
                bessel_j0_scalar(argument),
                5 /* terrane-site: src/main.trn:32:24-32:50 */,
            );
            total = total + value / (1.0_f64 + 4.0_f64 * distance * distance);
            right_index = __terrane_raised(
                terrane_int_support::fixed_addition(right_index, 1),
                6 /* terrane-site: src/main.trn:34:7-34:20 */,
            );
        }
        left_index = __terrane_raised(
            terrane_int_support::fixed_addition(left_index, 1),
            7 /* terrane-site: src/main.trn:35:5-35:17 */,
        );
    }
    let pair_count: f64 = __terrane_raised(
        terrane_int_support::exact_fixed_f64(
            __terrane_raised(
                terrane_int_support::fixed_multiplication(count, count),
                8 /* terrane-site: src/main.trn:36:24-36:37 */,
            ),
        ),
        8 /* terrane-site: src/main.trn:36:24-36:37 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&(total / pair_count)));
}
