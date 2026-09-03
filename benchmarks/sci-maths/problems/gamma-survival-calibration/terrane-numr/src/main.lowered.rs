// Generated deterministically by Terrane 0.1.0.
include!("main.lowered.support.rs");
// Source: src/main.trn
// Namespace: benchmark-gamma-survival-calibration
fn benchmark_size() -> i64 {
    let supplied: terrane_collection_support::List<NativeString> = arguments();
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
fn main() {
    let count: i64 = benchmark_size();
    let mut total: f64 = 0.0_f64;
    let mut index: i64 = 0;
    while index < count {
        let shape_part: f64 = __terrane_raised(
            terrane_int_support::exact_fixed_f64(index.rem_euclid(17)),
            3 /* terrane-site: src/main.trn:20:26-20:36 */,
        );
        let observation_part: f64 = __terrane_raised(
            terrane_int_support::exact_fixed_f64(index.rem_euclid(101)),
            4 /* terrane-site: src/main.trn:21:32-21:43 */,
        );
        let target_part: f64 = __terrane_raised(
            terrane_int_support::exact_fixed_f64(index.rem_euclid(7)),
            5 /* terrane-site: src/main.trn:22:27-22:36 */,
        );
        let shape: f64 = 1.25 + shape_part * 0.125;
        let observation: f64 = 0.5 + observation_part * 0.05;
        let target: f64 = 0.2 + target_part * 0.1;
        let survival: f64 = __terrane_raised(
            gammaincc_scalar(shape, observation),
            6 /* terrane-site: src/main.trn:26:25-26:61 */,
        );
        let residual: f64 = survival - target;
        total = total + residual * residual;
        index = __terrane_raised(
            terrane_int_support::fixed_addition(index, 1),
            7 /* terrane-site: src/main.trn:29:5-29:12 */,
        );
    }
    let divisor: f64 = __terrane_raised(
        terrane_int_support::exact_fixed_f64(count),
        8 /* terrane-site: src/main.trn:30:21-30:26 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&(total / divisor)));
}
