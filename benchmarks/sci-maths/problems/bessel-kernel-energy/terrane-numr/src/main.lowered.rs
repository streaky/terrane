// Generated deterministically by Terrane 0.1.0.
include!("main.lowered.support.rs");
// Source: src/main.trn
// Namespace: benchmark-bessel-kernel-energy
fn benchmark_size() -> i64 {
    let supplied: terrane_collection_support::List<NativeString>;
    let count: i64;
    supplied = arguments();
    if terrane_int_support::Int::from(supplied.length())
        != terrane_int_support::Int::from(1_i128)
    {
        exit(make_exit_status(terrane_int_support::Int::from(2_i128)));
    }
    count = __terrane_raised(
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
                                        0 /* terrane-site: src/main.trn:11:17-11:28 */,
                                    ),
                                ),
                            0 /* terrane-site: src/main.trn:11:17-11:28 */,
                        )
                        .text,
                    &10,
                ),
                1 /* terrane-site: src/main.trn:11:17-11:43 */,
            ),
        ),
        1 /* terrane-site: src/main.trn:11:17-11:43 */,
    );
    if count <= 0 {
        exit(make_exit_status(terrane_int_support::Int::from(2_i128)));
    }
    return count;
}
fn coordinate(index: i64) -> f64 {
    let raw: f64;
    raw = __terrane_raised(
        terrane_int_support::exact_fixed_f64(
            __terrane_raised(
                    terrane_int_support::fixed_multiplication(index, 37),
                    2 /* terrane-site: src/main.trn:17:18-17:28 */,
                )
                .rem_euclid(1009),
        ),
        3 /* terrane-site: src/main.trn:17:17-17:36 */,
    );
    return raw / 1009.0_f64;
}
fn main() {
    let count: i64;
    let mut total: f64;
    let mut left_index: i64;
    let mut left: f64;
    let mut right_index: i64;
    let mut right: f64;
    let mut distance: f64;
    let mut argument: f64;
    let mut value: f64;
    let pair_count: f64;
    count = benchmark_size();
    total = 0.0_f64;
    left_index = 0;
    while left_index < count {
        left = coordinate(left_index);
        right_index = 0;
        while right_index < count {
            right = coordinate(right_index);
            distance = left - right;
            if distance < 0.0_f64 {
                distance = 0.0_f64 - distance;
            }
            argument = 18.0_f64 * distance;
            value = {
                let __terrane_conversion_input_0 = argument;
                __terrane_raised(
                    bessel_j0_scalar(__terrane_conversion_input_0),
                    4 /* terrane-site: src/main.trn:33:23-33:49 */,
                )
            };
            total = total + value / (1.0_f64 + 4.0_f64 * distance * distance);
            right_index = __terrane_raised(
                terrane_int_support::fixed_addition(right_index, 1),
                5 /* terrane-site: src/main.trn:35:7-35:20 */,
            );
        }
        left_index = __terrane_raised(
            terrane_int_support::fixed_addition(left_index, 1),
            6 /* terrane-site: src/main.trn:36:5-36:17 */,
        );
    }
    pair_count = __terrane_raised(
        terrane_int_support::exact_fixed_f64(
            __terrane_raised(
                terrane_int_support::fixed_multiplication(count, count),
                7 /* terrane-site: src/main.trn:37:24-37:37 */,
            ),
        ),
        7 /* terrane-site: src/main.trn:37:24-37:37 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&(total / pair_count)));
}
