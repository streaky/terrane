// Generated deterministically by Terrane 0.1.0.
include!("main.lowered.support.rs");
// Source: src/main.trn
// Namespace: benchmark-elementwise-materialized
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
    let mut transformed: terrane_collection_support::List<f64>;
    let mut index: i64;
    let mut raw: f64;
    let mut x: f64;
    let mut total: f64;
    let mut value: f64;
    count = benchmark_size();
    transformed = terrane_collection_support::List::<f64>::new(Vec::new());
    index = 0;
    {
        let __terrane_list_append_0 = transformed.make_unique();
        let __terrane_list_start_0 = index;
        let __terrane_list_end_0 = count;
        let __terrane_list_length_0 = (__terrane_list_start_0..__terrane_list_end_0)
            .size_hint()
            .0;
        let __terrane_list_capacity_limit_0 = 268435456usize
            / std::mem::size_of::<f64>().max(1);
        if __terrane_list_length_0 <= __terrane_list_capacity_limit_0 {
            __terrane_list_append_0.reserve(__terrane_list_length_0);
            for __terrane_list_index_0 in __terrane_list_start_0..__terrane_list_end_0 {
                index = __terrane_list_index_0;
                raw = __terrane_raised(
                    terrane_int_support::exact_fixed_f64(index.rem_euclid(1000)),
                    2 /* terrane-site: src/main.trn:23:19-23:31 */,
                );
                x = raw / 100.0_f64;
                __terrane_list_append_0.push(x * x + 3.0_f64 * x - 7.0_f64);
            }
            index = std::cmp::max(__terrane_list_start_0, __terrane_list_end_0);
        } else {
            __terrane_list_append_0.reserve(__terrane_list_capacity_limit_0);
            while index < count {
                raw = __terrane_raised(
                    terrane_int_support::exact_fixed_f64(index.rem_euclid(1000)),
                    2 /* terrane-site: src/main.trn:23:19-23:31 */,
                );
                x = raw / 100.0_f64;
                {
                    let __terrane_collection_argument_0 = x * x + 3.0_f64 * x - 7.0_f64;
                    __terrane_list_append_0.push(__terrane_collection_argument_0);
                };
                index = __terrane_raised(
                    terrane_int_support::fixed_addition(index, 1),
                    3 /* terrane-site: src/main.trn:26:5-26:12 */,
                );
            }
        }
        let _ = &index;
    }
    total = 0.0_f64;
    let __terrane_iterable_1 = transformed;
    let mut __terrane_iterator_1 = terrane_collection_support::slice_iterator(
        __terrane_iterable_1.as_slice(),
    );
    loop {
        value = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        total = total + value;
    }
    println!("{}", terrane_scalar_support::scalar_text(&total));
}
