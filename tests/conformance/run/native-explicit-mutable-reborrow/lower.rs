// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn advance(counter: &mut Counter) -> i64 {
    let result: i64;
    result = {
        let counter = &mut *counter;
        counter.increment();
        counter.increment();
        counter.value()
    };
    return result;
}
fn shadow_read(first: &Counter, second: &Counter) -> i64 {
    let result: i64;
    result = {
        let initial = first.value();
        let mut first = 1_i64;
        let borrowed = &mut first;
        *borrowed += 1;
        initial + second.value() + first
    };
    return result;
}
fn read_row(record: &OpaqueRow) -> Result<i64, TerraneError> {
    return Ok(
        __terrane_raised_err(
            terrane_int_support::coerce::<
                i64,
            >(
                &__terrane_raised_err(
                    match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| (&*record).value()),
                    ) {
                        Ok(value) => {
                            Ok(terrane_int_support::Int::from(i128::from(value)))
                        }
                        Err(payload) => {
                            Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "witness",
                                    "witness::OpaqueRow::value",
                                ),
                            )
                        }
                    },
                    0 /* terrane-site: src/main.trn:29:12-29:25 */,
                )?,
            ),
            0 /* terrane-site: src/main.trn:29:12-29:25 */,
        )?,
    );
}
fn main() {
    __terrane_run(async move {
        let mut counter: Counter;
        let values: terrane_collection_support::List<terrane_int_support::Int>;
        let empty: terrane_collection_support::List<terrane_int_support::Int>;
        let opaque_values: terrane_collection_support::List<OpaqueValue>;
        let records: terrane_collection_support::List<OpaqueRow>;
        let mut total: i64;
        let mut record: &OpaqueRow;
        let wide: terrane_collection_support::List<terrane_int_support::Int>;
        counter = __terrane_raised(
            terrane_static_trn_436f756e746572_new(
                terrane_int_support::Int::from(4_i128),
            ),
            1 /* terrane-site: src/main.trn:32:15-32:36 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&advance(&mut counter)));
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&counter)
            .value())) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "witness",
            "witness::Counter::value")) }, 2 /* terrane-site: src/main.trn:34:13-34:27 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&shadow_read(&counter, &counter))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await({
            let __terrane_future = { let __terrane_call = Box::new(counter).finish();
            Box::pin(async move { match crate
            ::__terrane_dependency_await_unwind(__terrane_call). await { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "witness",
            "witness::Counter::finish")) } }) }; async move {
            __terrane_raised_err(__terrane_future. await, 3 /* terrane-site: src/main.trn:36:18-36:33 */) } }). await, 3 /* terrane-site: src/main.trn:36:18-36:33 */))
        );
        values = terrane_collection_support::List::<
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_int_support::Int::from(2_i128),
                terrane_int_support::Int::from(3_i128)
            ],
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(sum(values),
            4 /* terrane-site: src/main.trn:38:13-38:31 */))
        );
        empty = terrane_collection_support::List::<
            terrane_int_support::Int,
        >::new(vec![]);
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(sum(empty),
            5 /* terrane-site: src/main.trn:40:13-40:30 */))
        );
        opaque_values = terrane_collection_support::List::<
            OpaqueValue,
        >::new(
            vec![
                __terrane_raised(terrane_static_trn_4f706171756556616c7565_new(terrane_int_support::Int::from(3_i128)),
                6 /* terrane-site: src/main.trn:41:48-41:67 */),
                __terrane_raised(terrane_static_trn_4f706171756556616c7565_new(terrane_int_support::Int::from(8_i128)),
                7 /* terrane-site: src/main.trn:41:71-41:90 */)
            ],
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await({
            let __terrane_future = checksum(opaque_values); async move {
            __terrane_raised_err(__terrane_future. await, 8 /* terrane-site: src/main.trn:42:18-42:41 */) } }). await, 8 /* terrane-site: src/main.trn:42:18-42:41 */))
        );
        records = __terrane_raised(
            rows(),
            9 /* terrane-site: src/main.trn:43:33-43:38 */,
        );
        total = 0;
        let __terrane_iterable_0 = &records;
        let mut __terrane_iterator_0 = __terrane_iterable_0.terrane_borrowing_iterator();
        loop {
            record = match __terrane_iterator_0.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            total = __terrane_raised(
                terrane_int_support::fixed_addition(
                    total,
                    __terrane_traced(
                        read_row(record),
                        10 /* terrane-site: src/main.trn:46:26-46:42 */,
                    ),
                ),
                11 /* terrane-site: src/main.trn:46:17-46:43 */,
            );
        }
        println!("{}", terrane_scalar_support::scalar_text(&total));
        wide = terrane_collection_support::List::<
            terrane_int_support::Int,
        >::new(vec![terrane_int_support::Int::from(2147483648_i128)]);
        let __terrane_completion_0: TerraneCompletion<()> = (|| {
            let __terrane_try_0: TerraneCompletion<()> = (|| {
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&__terrane_raised_completion!(sum(wide),
                    12 /* terrane-site: src/main.trn:50:17-50:33 */))
                );
                TerraneCompletion::Normal
            })();
            match __terrane_try_0 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_0) => {
                    let mut __terrane_handled_0 = false;
                    if !__terrane_handled_0
                        && __terrane_error_0.kind
                            == TerraneErrorKind::IntegerConversionOverflow
                    {
                        __terrane_handled_0 = true;
                        println!("{}", String::from("slice conversion caught"));
                    }
                    if !__terrane_handled_0 {
                        return TerraneCompletion::Error(__terrane_error_0);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        match __terrane_completion_0 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
    });
}
