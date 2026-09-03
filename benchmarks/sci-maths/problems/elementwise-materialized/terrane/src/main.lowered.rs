// Generated deterministically by Terrane 0.1.0.
include!("main.lowered.support.rs");
// Source: src/main.trn
// Namespace: benchmark-elementwise-materialized
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
    let mut transformed: terrane_collection_support::List<f64> = terrane_collection_support::List::<
        f64,
    >::new(Vec::new());
    let mut index: i64 = 0;
    while index < count {
        let raw: f64 = __terrane_raised(
            terrane_int_support::exact_fixed_f64(index.rem_euclid(1000)),
            3 /* terrane-site: src/main.trn:21:19-21:31 */,
        );
        let x: f64 = raw / 100.0_f64;
        transformed.append(x * x + 3.0_f64 * x - 7.0_f64);
        index = __terrane_raised(
            terrane_int_support::fixed_addition(index, 1),
            4 /* terrane-site: src/main.trn:24:5-24:12 */,
        );
    }
    let mut total: f64 = 0.0_f64;
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &transformed,
    );
    loop {
        let value = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        total = total + value;
    }
    println!("{}", terrane_scalar_support::scalar_text(&total));
}
