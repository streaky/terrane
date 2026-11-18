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
                                        0 /* terrane-site: src/main.trn:9:17-9:28 */,
                                    ),
                                ),
                            0 /* terrane-site: src/main.trn:9:17-9:28 */,
                        )
                        .text,
                    &10,
                ),
                1 /* terrane-site: src/main.trn:9:17-9:43 */,
            ),
        ),
        1 /* terrane-site: src/main.trn:9:17-9:43 */,
    );
    if count <= 0 {
        exit(make_exit_status(terrane_int_support::Int::from(2_i128)));
    }
    return count;
}
fn log_gamma(value: f64) -> f64 {
    let mut y: f64;
    let shifted: f64;
    let adjusted: f64;
    let mut series: f64;
    y = value;
    shifted = value + 5.5;
    adjusted = shifted
        - (value + 0.5)
            * {
                let __terrane_result = shifted.ln();
                __terrane_result
            };
    series = 1.000000000190015;
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
    return 0.0 - adjusted
        + {
            let __terrane_result = (2.5066282746310005 * series / value).ln();
            __terrane_result
        };
}
fn lower_gamma_ratio(shape: f64, observation: f64, gamma_log: f64) -> f64 {
    let mut current_shape: f64;
    let mut term: f64;
    let mut total: f64;
    let mut iteration: i64;
    let exponent: f64;
    let scale: f64;
    current_shape = shape;
    term = 1.0_f64 / shape;
    total = term;
    iteration = 1;
    while iteration <= 100 {
        current_shape = current_shape + 1.0_f64;
        term = term * observation / current_shape;
        total = total + term;
        if {
            let __terrane_result = term.abs();
            __terrane_result
        }
            < {
                let __terrane_result = total.abs();
                __terrane_result
            } * 0.00000000000003
        {
            break;
        }
        iteration = __terrane_raised(
            terrane_int_support::fixed_addition(iteration, 1),
            2 /* terrane-site: src/main.trn:47:5-47:16 */,
        );
    }
    exponent = shape
        * {
            let __terrane_result = observation.ln();
            __terrane_result
        } + (0.0 - observation - gamma_log);
    scale = {
        let __terrane_result = exponent.exp();
        __terrane_result
    };
    return total * scale;
}
fn upper_gamma_ratio(shape: f64, observation: f64, gamma_log: f64) -> f64 {
    let floor: f64;
    let mut offset: f64;
    let mut reciprocal_floor: f64;
    let mut denominator: f64;
    let mut product: f64;
    let mut iteration: f64;
    let mut numerator: f64;
    let mut correction: f64;
    let exponent: f64;
    let scale: f64;
    floor = 0.000000000000000000000000000001;
    offset = observation + 1.0_f64 - shape;
    reciprocal_floor = 1.0_f64 / floor;
    denominator = 1.0_f64 / offset;
    product = denominator;
    iteration = 1.0_f64;
    while iteration <= 100.0_f64 {
        numerator = (0.0 - iteration) * (iteration - shape);
        offset = offset + 2.0_f64;
        denominator = numerator * denominator + offset;
        if {
            let __terrane_result = denominator.abs();
            __terrane_result
        } < floor
        {
            denominator = floor;
        }
        reciprocal_floor = offset + numerator / reciprocal_floor;
        if {
            let __terrane_result = reciprocal_floor.abs();
            __terrane_result
        } < floor
        {
            reciprocal_floor = floor;
        }
        denominator = 1.0_f64 / denominator;
        correction = denominator * reciprocal_floor;
        product = product * correction;
        if {
            let __terrane_result = (correction - 1.0_f64).abs();
            __terrane_result
        } < 0.00000000000003
        {
            break;
        }
        iteration = iteration + 1.0_f64;
    }
    exponent = shape
        * {
            let __terrane_result = observation.ln();
            __terrane_result
        } + (0.0 - observation - gamma_log);
    scale = {
        let __terrane_result = exponent.exp();
        __terrane_result
    };
    return scale * product;
}
fn gamma_survival(shape: f64, observation: f64) -> f64 {
    let gamma_log: f64;
    gamma_log = log_gamma(shape);
    if observation < shape + 1.0_f64 {
        return 1.0_f64 - lower_gamma_ratio(shape, observation, gamma_log);
    }
    return upper_gamma_ratio(shape, observation, gamma_log);
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
            3 /* terrane-site: src/main.trn:91:26-91:36 */,
        );
        observation_part = __terrane_raised(
            terrane_int_support::exact_fixed_f64(index.rem_euclid(101)),
            4 /* terrane-site: src/main.trn:92:32-92:43 */,
        );
        target_part = __terrane_raised(
            terrane_int_support::exact_fixed_f64(index.rem_euclid(7)),
            5 /* terrane-site: src/main.trn:93:27-93:36 */,
        );
        shape = 1.25 + shape_part * 0.125;
        observation = 0.5 + observation_part * 0.05;
        target = 0.2 + target_part * 0.1;
        survival = gamma_survival(shape, observation);
        residual = survival - target;
        total = residual * residual + total;
        index = __terrane_raised(
            terrane_int_support::fixed_addition(index, 1),
            6 /* terrane-site: src/main.trn:100:5-100:12 */,
        );
    }
    divisor = __terrane_raised(
        terrane_int_support::exact_fixed_f64(count),
        7 /* terrane-site: src/main.trn:101:21-101:26 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&(total / divisor)));
}
