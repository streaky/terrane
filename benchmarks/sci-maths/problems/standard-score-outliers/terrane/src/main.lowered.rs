// Generated deterministically by Terrane 0.1.0.
include!("main.lowered.support.rs");
// Source: src/main.trn
// Namespace: benchmark-standard-score-outliers
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
fn main() {
    let count: i64 = benchmark_size();
    let mut values: terrane_collection_support::List<f64> = terrane_collection_support::List::<
        f64,
    >::new(Vec::new());
    let mut total: f64 = 0.0_f64;
    let mut index: i64 = 0;
    while index < count {
        let raw: f64 = __terrane_raised(
            terrane_int_support::exact_fixed_f64(
                __terrane_raised(
                    terrane_int_support::fixed_subtraction(index.rem_euclid(200), 100),
                    3 /* terrane-site: src/main.trn:22:19-22:38 */,
                ),
            ),
            3 /* terrane-site: src/main.trn:22:19-22:38 */,
        );
        let periodic: f64 = __terrane_raised(
            terrane_int_support::exact_fixed_f64(index.rem_euclid(7)),
            4 /* terrane-site: src/main.trn:23:24-23:33 */,
        );
        let value: f64 = 0.01 * raw * raw + periodic - 3.0_f64;
        values.append(value);
        total = total + value;
        index = __terrane_raised(
            terrane_int_support::fixed_addition(index, 1),
            5 /* terrane-site: src/main.trn:27:5-27:12 */,
        );
    }
    let divisor: f64 = __terrane_raised(
        terrane_int_support::exact_fixed_f64(count),
        6 /* terrane-site: src/main.trn:29:21-29:26 */,
    );
    let mean: f64 = total / divisor;
    let mut squared_total: f64 = 0.0_f64;
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &values,
    );
    loop {
        let value = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let deviation: f64 = value - mean;
        squared_total = squared_total + deviation * deviation;
    }
    let variance: f64 = squared_total / divisor;
    let mut outliers: i64 = 0;
    let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
        &values,
    );
    loop {
        let value = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let deviation: f64 = value - mean;
        if deviation * deviation > 2.5 * variance {
            outliers = __terrane_raised(
                terrane_int_support::fixed_addition(outliers, 1),
                7 /* terrane-site: src/main.trn:41:7-41:17 */,
            );
        }
    }
    println!("{}", terrane_scalar_support::scalar_text(&outliers));
}
