// Generated deterministically by Terrane 0.1.0.
include!("main.lowered.support.rs");
// Source: src/main.trn
// Namespace: benchmark-standard-score-outliers
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
fn main() {
    let count: i64;
    let mut values: terrane_collection_support::List<f64>;
    let mut total: f64;
    let mut index: i64;
    let mut raw: f64;
    let mut periodic: f64;
    let mut value: f64;
    let divisor: f64;
    let mean: f64;
    let mut squared_total: f64;
    let mut deviation: f64;
    let variance: f64;
    let mut outliers: i64;
    count = benchmark_size();
    values = terrane_collection_support::List::<f64>::new(Vec::new());
    total = 0.0_f64;
    index = 0;
    {
        let __terrane_list_append_0 = values.make_unique();
        if let (Ok(__terrane_start), Ok(__terrane_end)) = (
            usize::try_from(index),
            usize::try_from(count),
        ) {
            let __terrane_capacity_limit = 268435456usize
                / std::mem::size_of::<f64>().max(1);
            __terrane_list_append_0
                .reserve(
                    __terrane_end
                        .saturating_sub(__terrane_start)
                        .min(__terrane_capacity_limit),
                );
        }
        while index < count {
            raw = __terrane_raised(
                terrane_int_support::exact_fixed_f64(
                    __terrane_raised(
                        terrane_int_support::fixed_subtraction(
                            index.rem_euclid(200),
                            100,
                        ),
                        2 /* terrane-site: src/main.trn:22:19-22:38 */,
                    ),
                ),
                2 /* terrane-site: src/main.trn:22:19-22:38 */,
            );
            periodic = __terrane_raised(
                terrane_int_support::exact_fixed_f64(index.rem_euclid(7)),
                3 /* terrane-site: src/main.trn:23:24-23:33 */,
            );
            value = 0.01 * raw * raw + periodic - 3.0_f64;
            {
                let __terrane_collection_argument_0 = value;
                __terrane_list_append_0.push(__terrane_collection_argument_0);
            };
            total = total + value;
            index = __terrane_raised(
                terrane_int_support::fixed_addition(index, 1),
                4 /* terrane-site: src/main.trn:27:5-27:12 */,
            );
        }
    }
    divisor = __terrane_raised(
        terrane_int_support::exact_fixed_f64(count),
        5 /* terrane-site: src/main.trn:29:21-29:26 */,
    );
    mean = total / divisor;
    squared_total = 0.0_f64;
    let __terrane_iterable_0 = values.clone();
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        __terrane_iterable_0.as_slice(),
    );
    loop {
        value = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        deviation = value - mean;
        squared_total = squared_total + deviation * deviation;
    }
    variance = squared_total / divisor;
    outliers = 0;
    let __terrane_iterable_1 = values;
    let mut __terrane_iterator_1 = terrane_collection_support::slice_iterator(
        __terrane_iterable_1.as_slice(),
    );
    loop {
        value = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        deviation = value - mean;
        if deviation * deviation > 2.5 * variance {
            outliers = __terrane_raised(
                terrane_int_support::fixed_addition(outliers, 1),
                6 /* terrane-site: src/main.trn:41:7-41:17 */,
            );
        }
    }
    println!("{}", terrane_scalar_support::scalar_text(&outliers));
}
