// Generated deterministically by Terrane 0.1.0.
include!("main.lowered.support.rs");
// Source: src/main.trn
// Namespace: benchmark-scalar-reduction
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
    let mut total: i64;
    let mut index: i64;
    let mut value: i64;
    count = benchmark_size();
    total = 0;
    index = 0;
    while index < count {
        value = __terrane_raised(
            terrane_int_support::fixed_subtraction(index.rem_euclid(1000), 500),
            2 /* terrane-site: src/main.trn:22:19-22:39 */,
        );
        total = __terrane_raised(
            terrane_int_support::fixed_addition(
                total,
                __terrane_raised(
                    terrane_int_support::fixed_multiplication(value, value),
                    3 /* terrane-site: src/main.trn:23:21-23:34 */,
                ),
            ),
            4 /* terrane-site: src/main.trn:23:13-23:34 */,
        );
        index = __terrane_raised(
            terrane_int_support::fixed_addition(index, 1),
            5 /* terrane-site: src/main.trn:24:5-24:12 */,
        );
    }
    println!("{}", terrane_scalar_support::scalar_text(&total));
}
