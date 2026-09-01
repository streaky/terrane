// Generated deterministically by Terrane 0.1.0.
include!("main.lowered.support.rs");
// Source: src/main.trn
// Namespace: benchmark-scalar-reduction
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
fn main() {
    let count: i64 = benchmark_size();
    let mut total: i64 = 0;
    let mut index: i64 = 0;
    while index < count {
        let value: i64 = __terrane_raised(
            terrane_int_support::fixed_subtraction(index.rem_euclid(1000), 500),
            3 /* terrane-site: src/main.trn:20:19-20:39 */,
        );
        total = __terrane_raised(
            terrane_int_support::fixed_addition(
                total,
                __terrane_raised(
                    terrane_int_support::fixed_multiplication(value, value),
                    4 /* terrane-site: src/main.trn:21:21-21:34 */,
                ),
            ),
            5 /* terrane-site: src/main.trn:21:13-21:34 */,
        );
        index = __terrane_raised(
            terrane_int_support::fixed_addition(index, 1),
            6 /* terrane-site: src/main.trn:22:5-22:12 */,
        );
    }
    println!("{}", terrane_scalar_support::scalar_text(&total));
}
