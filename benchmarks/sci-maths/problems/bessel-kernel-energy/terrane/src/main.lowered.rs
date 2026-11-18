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
    if count <= 0 {
        exit(make_exit_status(terrane_int_support::Int::from(2_i128)));
    }
    return count;
}
fn bessel_j0(argument: f64) -> f64 {
    let x: f64;
    let square: f64;
    let numerator: f64;
    let denominator: f64;
    let scale: f64;
    let square_2: f64;
    let phase: f64;
    let numerator_2: f64;
    let denominator_2: f64;
    let amplitude: f64;
    x = {
        let __terrane_result = argument.abs();
        __terrane_result
    };
    if x < 8.0_f64 {
        square = x * x;
        numerator = 57568490574.0_f64
            + square
                * (-13362590354.0_f64
                    + square
                        * (651619640.7
                            + square
                                * (-11214424.18_f64
                                    + square * (77392.33017 + square * -184.9052456_f64))));
        denominator = 57568490411.0_f64
            + square
                * (1029532985.0_f64
                    + square
                        * (9494680.718
                            + square * (59272.64853 + square * (267.8532712 + square))));
        return numerator / denominator;
    }
    scale = 8.0_f64 / x;
    square_2 = scale * scale;
    phase = x - 0.785398164;
    numerator_2 = 1.0_f64
        + square_2
            * (-0.001098628627_f64
                + square_2
                    * (0.00002734510407
                        + square_2
                            * (-2.073370639e-6_f64 + square_2 * 0.0000002093887211)));
    denominator_2 = -0.01562499995_f64
        + square_2
            * (0.0001430488765
                + square_2
                    * (-6.911147651e-6_f64
                        + square_2
                            * (0.0000007621095161 - square_2 * 0.0000000934935152)));
    amplitude = {
        let __terrane_result = (0.636619772 / x).sqrt();
        __terrane_result
    };
    return amplitude
        * ({
            let __terrane_result = phase.cos();
            __terrane_result
        } * numerator_2
            - scale
                * {
                    let __terrane_result = phase.sin();
                    __terrane_result
                } * denominator_2);
}
fn coordinate(index: i64) -> f64 {
    let raw: f64;
    raw = __terrane_raised(
        terrane_int_support::exact_fixed_f64(
            __terrane_raised(
                    terrane_int_support::fixed_multiplication(index, 37),
                    2 /* terrane-site: src/main.trn:33:18-33:28 */,
                )
                .rem_euclid(1009),
        ),
        3 /* terrane-site: src/main.trn:33:17-33:36 */,
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
            distance = {
                let __terrane_result = (left - right).abs();
                __terrane_result
            };
            argument = 18.0_f64 * distance;
            value = bessel_j0(argument);
            total = total + value / (1.0_f64 + 4.0_f64 * distance * distance);
            right_index = __terrane_raised(
                terrane_int_support::fixed_addition(right_index, 1),
                4 /* terrane-site: src/main.trn:49:7-49:20 */,
            );
        }
        left_index = __terrane_raised(
            terrane_int_support::fixed_addition(left_index, 1),
            5 /* terrane-site: src/main.trn:50:5-50:17 */,
        );
    }
    pair_count = __terrane_raised(
        terrane_int_support::exact_fixed_f64(
            __terrane_raised(
                terrane_int_support::fixed_multiplication(count, count),
                6 /* terrane-site: src/main.trn:51:24-51:37 */,
            ),
        ),
        6 /* terrane-site: src/main.trn:51:24-51:37 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&(total / pair_count)));
}
