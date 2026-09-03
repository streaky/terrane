// Generated deterministically by Terrane 0.1.0.
include!("main.lowered.support.rs");
// Source: src/main.trn
// Namespace: benchmark-bessel-kernel-energy
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
    if count <= 0 {
        exit(make_exit_status(terrane_int_support::Int::from(2_i128)));
    }
    return count;
}
fn bessel_j0(argument: f64) -> f64 {
    let x: f64 = argument.abs();
    if x < 8.0_f64 {
        let square: f64 = x * x;
        let numerator: f64 = 57568490574.0_f64
            + square
                * (-13362590354.0_f64
                    + square
                        * (651619640.7
                            + square
                                * (-11214424.18_f64
                                    + square * (77392.33017 + square * -184.9052456_f64))));
        let denominator: f64 = 57568490411.0_f64
            + square
                * (1029532985.0_f64
                    + square
                        * (9494680.718
                            + square * (59272.64853 + square * (267.8532712 + square))));
        return numerator / denominator;
    }
    let scale: f64 = 8.0_f64 / x;
    let square: f64 = scale * scale;
    let phase: f64 = x - 0.785398164;
    let numerator: f64 = 1.0_f64
        + square
            * (-0.001098628627_f64
                + square
                    * (0.00002734510407
                        + square * (-2.073370639e-6_f64 + square * 0.0000002093887211)));
    let denominator: f64 = -0.01562499995_f64
        + square
            * (0.0001430488765
                + square
                    * (-6.911147651e-6_f64
                        + square * (0.0000007621095161 - square * 0.0000000934935152)));
    let amplitude: f64 = (0.636619772 / x).sqrt();
    return amplitude * (phase.cos() * numerator - scale * phase.sin() * denominator);
}
fn coordinate(index: i64) -> f64 {
    let raw: f64 = __terrane_raised(
        terrane_int_support::exact_fixed_f64(
            __terrane_raised(
                    terrane_int_support::fixed_multiplication(index, 37),
                    3 /* terrane-site: src/main.trn:32:18-32:28 */,
                )
                .rem_euclid(1009),
        ),
        4 /* terrane-site: src/main.trn:32:17-32:36 */,
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
            let distance: f64 = (left - right).abs();
            let argument: f64 = 18.0_f64 * distance;
            let value: f64 = bessel_j0(argument);
            total = total + value / (1.0_f64 + 4.0_f64 * distance * distance);
            right_index = __terrane_raised(
                terrane_int_support::fixed_addition(right_index, 1),
                5 /* terrane-site: src/main.trn:48:7-48:20 */,
            );
        }
        left_index = __terrane_raised(
            terrane_int_support::fixed_addition(left_index, 1),
            6 /* terrane-site: src/main.trn:49:5-49:17 */,
        );
    }
    let pair_count: f64 = __terrane_raised(
        terrane_int_support::exact_fixed_f64(
            __terrane_raised(
                terrane_int_support::fixed_multiplication(count, count),
                7 /* terrane-site: src/main.trn:50:24-50:37 */,
            ),
        ),
        7 /* terrane-site: src/main.trn:50:24-50:37 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&(total / pair_count)));
}
