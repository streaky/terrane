// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn advance_optional(counter: &mut Option<Counter>) {
    {
        let counter = &mut *counter;
        counter.as_mut().unwrap().increment();
    }
}
fn forward_optional(counter: &mut Option<Counter>) {
    advance_optional(&mut *counter);
}
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
fn optional_counter_value(counter: &Option<Counter>) -> i64 {
    return counter.as_ref().unwrap().value();
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
                    0 /* terrane-site: src/main.trn:41:12-41:25 */,
                )?,
            ),
            0 /* terrane-site: src/main.trn:41:12-41:25 */,
        )?,
    );
}
async fn forward_async(native: &mut Counter) {
    __terrane_await(increment_async(&mut *native)).await;
}
fn main() {
    __terrane_run(async move {
        let mut counter: Counter;
        let mut optional_counter_2: Option<Counter>;
        let bump: std::sync::Arc<dyn Fn(&mut Option<Counter>) -> () + Send + Sync>;
        let bump_native: std::sync::Arc<dyn Fn(&mut Counter) -> () + Send + Sync>;
        let mut forwarded: Counter;
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
            1 /* terrane-site: src/main.trn:47:15-47:36 */,
        );
        optional_counter_2 = __terrane_raised(
            optional_counter(),
            2 /* terrane-site: src/main.trn:48:24-48:41 */,
        );
        advance_optional(&mut optional_counter_2);
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&optional_counter_value(&optional_counter_2))
        );
        forward_optional(&mut optional_counter_2);
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&optional_counter_value(&optional_counter_2))
        );
        bump = {
            std::sync::Arc::new(move |counter: &mut Option<Counter>| -> () {
                {
                    let counter = &mut *counter;
                    counter.as_mut().unwrap().increment();
                }
                ()
            })
        };
        bump(&mut optional_counter_2);
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&optional_counter_value(&optional_counter_2))
        );
        bump_native = {
            std::sync::Arc::new(move |counter: &mut Counter| -> () {
                {
                    let counter = &mut *counter;
                    counter.increment();
                }
                ()
            })
        };
        bump_native(&mut counter);
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&counter)
            .value())) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "witness",
            "witness::Counter::value")) }, 3 /* terrane-site: src/main.trn:64:13-64:27 */))
        );
        println!("{}", terrane_scalar_support::scalar_text(&advance(&mut counter)));
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&counter)
            .value())) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "witness",
            "witness::Counter::value")) }, 4 /* terrane-site: src/main.trn:66:13-66:27 */))
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
            __terrane_raised_err(__terrane_future. await, 5 /* terrane-site: src/main.trn:68:18-68:33 */) } }). await, 5 /* terrane-site: src/main.trn:68:18-68:33 */))
        );
        forwarded = __terrane_raised(
            terrane_static_trn_436f756e746572_new(
                terrane_int_support::Int::from(80_i128),
            ),
            6 /* terrane-site: src/main.trn:69:17-69:39 */,
        );
        __terrane_await(forward_async(&mut forwarded)).await;
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&forwarded)
            .value())) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "witness",
            "witness::Counter::value")) }, 7 /* terrane-site: src/main.trn:71:13-71:29 */))
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
            8 /* terrane-site: src/main.trn:73:13-73:31 */))
        );
        empty = terrane_collection_support::List::<
            terrane_int_support::Int,
        >::new(vec![]);
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(sum(empty),
            9 /* terrane-site: src/main.trn:75:13-75:30 */))
        );
        opaque_values = terrane_collection_support::List::<
            OpaqueValue,
        >::new(
            vec![
                __terrane_raised(terrane_static_trn_4f706171756556616c7565_new(terrane_int_support::Int::from(3_i128)),
                10 /* terrane-site: src/main.trn:76:48-76:67 */),
                __terrane_raised(terrane_static_trn_4f706171756556616c7565_new(terrane_int_support::Int::from(8_i128)),
                11 /* terrane-site: src/main.trn:76:71-76:90 */)
            ],
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await({
            let __terrane_future = checksum((&opaque_values).as_vec()); async move {
            __terrane_raised_err(__terrane_future. await, 12 /* terrane-site: src/main.trn:77:18-77:41 */) } }). await, 12 /* terrane-site: src/main.trn:77:18-77:41 */))
        );
        records = __terrane_raised(
            rows(),
            13 /* terrane-site: src/main.trn:78:33-78:38 */,
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
                        14 /* terrane-site: src/main.trn:81:26-81:42 */,
                    ),
                ),
                15 /* terrane-site: src/main.trn:81:17-81:43 */,
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
                    16 /* terrane-site: src/main.trn:85:17-85:33 */))
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
// Source: src/native-forward.trn
// Namespace: app
async fn increment_async(native: &mut Counter) {
    {
        let native = &mut *native;
        native.increment();
    }
}
