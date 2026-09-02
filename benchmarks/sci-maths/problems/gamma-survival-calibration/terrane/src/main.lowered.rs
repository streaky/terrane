// Generated deterministically by Terrane 0.1.0.
include!("main.lowered.support.rs");
// Source: src/main.trn
// Namespace: benchmark-gamma-survival-calibration
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
fn log_gamma(value: f64) -> f64 {
    let mut y: f64 = value;
    let shifted: f64 = value + 5.5;
    let adjusted: f64 = shifted - (value + 0.5) * shifted.ln();
    let mut series: f64 = 1.000000000190015;
    y = y + 1.0_f64;
    series = series + 76.18009172947146 / y;
    y = y + 1.0_f64;
    series = series + -86.50532032941678_f64 / y;
    y = y + 1.0_f64;
    series = series + 24.01409824083091 / y;
    y = y + 1.0_f64;
    series = series + -1.231739572450155_f64 / y;
    y = y + 1.0_f64;
    series = series + 0.001208650973866179 / y;
    y = y + 1.0_f64;
    series = series + -5.395239384953e-6_f64 / y;
    return 0.0 - adjusted + (2.5066282746310005 * series / value).ln();
}
fn lower_gamma_ratio(shape: f64, observation: f64, gamma_log: f64) -> f64 {
    let mut current_shape: f64 = shape;
    let mut term: f64 = 1.0_f64 / shape;
    let mut total: f64 = term;
    let mut iteration: i64 = 1;
    while iteration <= 100 {
        current_shape = current_shape + 1.0_f64;
        term = term * observation / current_shape;
        total = total + term;
        if term.abs() < total.abs() * 0.00000000000003 {
            break;
        }
        iteration = __terrane_raised(
            terrane_int_support::fixed_addition(iteration, 1),
            3 /* terrane-site: src/main.trn:47:5-47:16 */,
        );
    }
    let exponent: f64 = shape.mul_add(observation.ln(), 0.0 - observation - gamma_log);
    let scale: f64 = exponent.exp();
    return total * scale;
}
fn upper_gamma_ratio(shape: f64, observation: f64, gamma_log: f64) -> f64 {
    let floor: f64 = 0.000000000000000000000000000001;
    let mut offset: f64 = observation + 1.0_f64 - shape;
    let mut reciprocal_floor: f64 = 1.0_f64 / floor;
    let mut denominator: f64 = 1.0_f64 / offset;
    let mut product: f64 = denominator;
    let mut iteration: f64 = 1.0_f64;
    while iteration <= 100.0_f64 {
        let numerator: f64 = (0.0 - iteration) * (iteration - shape);
        offset = offset + 2.0_f64;
        denominator = numerator.mul_add(denominator, offset);
        if denominator.abs() < floor {
            denominator = floor;
        }
        reciprocal_floor = offset + numerator / reciprocal_floor;
        if reciprocal_floor.abs() < floor {
            reciprocal_floor = floor;
        }
        denominator = 1.0_f64 / denominator;
        let correction: f64 = denominator * reciprocal_floor;
        product = product * correction;
        if (correction - 1.0_f64).abs() < 0.00000000000003 {
            break;
        }
        iteration = iteration + 1.0_f64;
    }
    let exponent: f64 = shape.mul_add(observation.ln(), 0.0 - observation - gamma_log);
    let scale: f64 = exponent.exp();
    return scale * product;
}
fn gamma_survival(shape: f64, observation: f64) -> f64 {
    let gamma_log: f64 = log_gamma(shape);
    if observation < shape + 1.0_f64 {
        return 1.0_f64 - lower_gamma_ratio(shape, observation, gamma_log);
    }
    return upper_gamma_ratio(shape, observation, gamma_log);
}
fn main() {
    let count: i64 = benchmark_size();
    let mut total: f64 = 0.0_f64;
    let mut index: i64 = 0;
    while index < count {
        let shape_part: f64 = __terrane_raised(
            terrane_int_support::exact_fixed_f64(index.rem_euclid(17)),
            4 /* terrane-site: src/main.trn:91:26-91:36 */,
        );
        let observation_part: f64 = __terrane_raised(
            terrane_int_support::exact_fixed_f64(index.rem_euclid(101)),
            5 /* terrane-site: src/main.trn:92:32-92:43 */,
        );
        let target_part: f64 = __terrane_raised(
            terrane_int_support::exact_fixed_f64(index.rem_euclid(7)),
            6 /* terrane-site: src/main.trn:93:27-93:36 */,
        );
        let shape: f64 = 1.25 + shape_part * 0.125;
        let observation: f64 = 0.5 + observation_part * 0.05;
        let target: f64 = 0.2 + target_part * 0.1;
        let survival: f64 = gamma_survival(shape, observation);
        let residual: f64 = survival - target;
        total = residual.mul_add(residual, total);
        index = __terrane_raised(
            terrane_int_support::fixed_addition(index, 1),
            7 /* terrane-site: src/main.trn:100:5-100:12 */,
        );
    }
    let divisor: f64 = __terrane_raised(
        terrane_int_support::exact_fixed_f64(count),
        8 /* terrane-site: src/main.trn:101:21-101:26 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&(total / divisor)));
}
