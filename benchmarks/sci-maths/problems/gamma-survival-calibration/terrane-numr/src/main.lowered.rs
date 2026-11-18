// Generated deterministically by Terrane 0.1.0.
include!("main.lowered.support.rs");
// Source: src/main.trn
// Namespace: benchmark-gamma-survival-calibration
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
fn main() {
    let count: i64;
    let mut total: f64;
    let mut index: i64;
    let mut shape_part: f64;
    let mut observation_part: f64;
    let mut target_part: f64;
    let mut shape: f64;
    let mut observation: f64;
    let mut target: f64;
    let mut survival: f64;
    let mut residual: f64;
    let divisor: f64;
    count = benchmark_size();
    total = 0.0_f64;
    index = 0;
    while index < count {
        shape_part = __terrane_raised(
            terrane_int_support::exact_fixed_f64(index.rem_euclid(17)),
            2 /* terrane-site: src/main.trn:21:26-21:36 */,
        );
        observation_part = __terrane_raised(
            terrane_int_support::exact_fixed_f64(index.rem_euclid(101)),
            3 /* terrane-site: src/main.trn:22:32-22:43 */,
        );
        target_part = __terrane_raised(
            terrane_int_support::exact_fixed_f64(index.rem_euclid(7)),
            4 /* terrane-site: src/main.trn:23:27-23:36 */,
        );
        shape = 1.25 + shape_part * 0.125;
        observation = 0.5 + observation_part * 0.05;
        target = 0.2 + target_part * 0.1;
        survival = {
            let __terrane_conversion_input_0 = shape;
            let __terrane_conversion_input_1 = observation;
            __terrane_raised(
                gammaincc_scalar(
                    __terrane_conversion_input_0,
                    __terrane_conversion_input_1,
                ),
                5 /* terrane-site: src/main.trn:27:24-27:60 */,
            )
        };
        residual = survival - target;
        total = total + residual * residual;
        index = __terrane_raised(
            terrane_int_support::fixed_addition(index, 1),
            6 /* terrane-site: src/main.trn:30:5-30:12 */,
        );
    }
    divisor = __terrane_raised(
        terrane_int_support::exact_fixed_f64(count),
        7 /* terrane-site: src/main.trn:31:21-31:26 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&(total / divisor)));
}
