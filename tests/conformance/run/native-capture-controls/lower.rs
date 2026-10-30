// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct SourceHolder {
    pub value: i64,
}
impl SourceHolder {
    pub fn terrane_construct() -> Self {
        Self { value: 0 }
    }
    pub fn bump(&mut self) {
        {
            let __terrane_value = __terrane_raised(
                terrane_int_support::fixed_addition(self.value, 1),
                0 /* terrane-site: src/main.trn:8:22-8:36 */,
            );
            self.value = __terrane_value;
        }
    }
}
fn increment(counter: &mut Counter) {
    {
        let counter = &mut *counter;
        counter.increment();
    }
}
fn main() {
    __terrane_run(async move {
        let read_counter: Counter;
        let read_only: std::sync::Arc<
            dyn Fn() -> Result<i64, TerraneError> + Send + Sync,
        >;
        let async_values: terrane_collection_support::List<OpaqueValue>;
        let async_read_only: std::sync::Arc<
            dyn Fn(
            ) -> std::pin::Pin<
                    Box<dyn Future<Output = Result<i64, TerraneError>> + Send>,
                > + Send + Sync,
        >;
        let async_value: i64;
        let mixed_counter: OpaqueValue;
        let holder: SourceHolder;
        let mixed: TerraneMutableCallable<(), Result<i64, TerraneError>>;
        let mut callback_counter: Counter;
        let callback: std::sync::Arc<dyn Fn(&mut Counter) -> () + Send + Sync>;
        let owned: Counter;
        let consume: TerraneConsumingCallable<(), Result<i64, TerraneError>>;
        read_counter = __terrane_raised(
            terrane_static_trn_436f756e746572_new(
                terrane_int_support::Int::from(11_i128),
            ),
            1 /* terrane-site: src/main.trn:16:20-16:42 */,
        );
        read_only = {
            let read_counter = read_counter;
            std::sync::Arc::new(move || -> Result<i64, TerraneError> {
                return Ok(
                    __terrane_raised_err(
                        terrane_int_support::coerce::<
                            i64,
                        >(
                            &__terrane_raised_err(
                                match std::panic::catch_unwind(
                                    std::panic::AssertUnwindSafe(|| (&read_counter).value()),
                                ) {
                                    Ok(value) => {
                                        Ok(terrane_int_support::Int::from(i128::from(value)))
                                    }
                                    Err(payload) => {
                                        Err(
                                            crate::__terrane_dependency_panic(
                                                payload,
                                                "witness",
                                                "witness::Counter::value",
                                            ),
                                        )
                                    }
                                },
                                2 /* terrane-site: src/main.trn:18:17-18:36 */,
                            )?,
                        ),
                        2 /* terrane-site: src/main.trn:18:17-18:36 */,
                    )?,
                );
            })
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(read_only(),
            3 /* terrane-site: src/main.trn:19:13-19:23 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(read_only(),
            4 /* terrane-site: src/main.trn:20:13-20:23 */))
        );
        async_values = terrane_collection_support::List::<
            OpaqueValue,
        >::new(
            vec![
                __terrane_raised(terrane_static_trn_4f706171756556616c7565_new(terrane_int_support::Int::from(23_i128)),
                5 /* terrane-site: src/main.trn:22:47-22:67 */)
            ],
        );
        async_read_only = {
            let async_values = async_values.clone();
            std::sync::Arc::new(move || -> std::pin::Pin<
                Box<dyn Future<Output = Result<i64, TerraneError>> + Send>,
            > {
                let async_values = async_values.clone();
                Box::pin(async move {
                    return Ok(
                        __terrane_raised_err(
                            terrane_int_support::coerce::<
                                i64,
                            >(
                                &__terrane_traced_err(
                                    __terrane_await({
                                            let __terrane_future = checksum((&async_values).as_vec());
                                            async move {
                                                __terrane_raised_err(
                                                    __terrane_future.await,
                                                    6 /* terrane-site: src/main.trn:24:22-24:44 */,
                                                )
                                            }
                                        })
                                        .await,
                                    6 /* terrane-site: src/main.trn:24:22-24:44 */,
                                )?,
                            ),
                            7 /* terrane-site: src/main.trn:24:16-24:44 */,
                        )?,
                    );
                })
            })
        };
        async_value = __terrane_traced(
            __terrane_await(async_read_only()).await,
            8 /* terrane-site: src/main.trn:25:25-25:41 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&async_value));
        mixed_counter = __terrane_raised(
            terrane_static_trn_4f706171756556616c7565_new(
                terrane_int_support::Int::from(17_i128),
            ),
            9 /* terrane-site: src/main.trn:28:21-28:41 */,
        );
        holder = SourceHolder::terrane_construct();
        mixed = {
            let mut holder = holder.clone();
            let mixed_counter = mixed_counter.clone();
            TerraneMutableCallable::new(move |(): ()| -> Result<i64, TerraneError> {
                holder.bump();
                return Ok(
                    __terrane_raised_err(
                        terrane_int_support::coerce::<
                            i64,
                        >(
                            &(terrane_int_support::Int::from(holder.value as i128)
                                + __terrane_raised_err(
                                    match std::panic::catch_unwind(
                                        std::panic::AssertUnwindSafe(|| (&mixed_counter).value()),
                                    ) {
                                        Ok(value) => {
                                            Ok(terrane_int_support::Int::from(i128::from(value)))
                                        }
                                        Err(payload) => {
                                            Err(
                                                crate::__terrane_dependency_panic(
                                                    payload,
                                                    "witness",
                                                    "witness::OpaqueValue::value",
                                                ),
                                            )
                                        }
                                    },
                                    10 /* terrane-site: src/main.trn:32:32-32:52 */,
                                )?),
                        ),
                        11 /* terrane-site: src/main.trn:32:16-32:53 */,
                    )?,
                );
            })
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(mixed.call(()),
            12 /* terrane-site: src/main.trn:33:13-33:19 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(mixed.call(()),
            13 /* terrane-site: src/main.trn:34:13-34:19 */))
        );
        callback_counter = __terrane_raised(
            terrane_static_trn_436f756e746572_new(
                terrane_int_support::Int::from(20_i128),
            ),
            14 /* terrane-site: src/main.trn:36:24-36:46 */,
        );
        callback = {
            std::sync::Arc::new(move |value: &mut Counter| -> () {
                increment(&mut *value);
                ()
            })
        };
        callback(&mut callback_counter);
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&callback_counter)
            .value())) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "witness",
            "witness::Counter::value")) }, 15 /* terrane-site: src/main.trn:40:13-40:36 */))
        );
        owned = __terrane_raised(
            terrane_static_trn_436f756e746572_new(
                terrane_int_support::Int::from(40_i128),
            ),
            16 /* terrane-site: src/main.trn:42:13-42:35 */,
        );
        consume = {
            let mut owned = owned;
            TerraneConsumingCallable::new(move |(): ()| -> Result<i64, TerraneError> {
                increment(&mut owned);
                return Ok(
                    __terrane_raised_err(
                        terrane_int_support::coerce::<
                            i64,
                        >(
                            &__terrane_raised_err(
                                match std::panic::catch_unwind(
                                    std::panic::AssertUnwindSafe(|| (&owned).value()),
                                ) {
                                    Ok(value) => {
                                        Ok(terrane_int_support::Int::from(i128::from(value)))
                                    }
                                    Err(payload) => {
                                        Err(
                                            crate::__terrane_dependency_panic(
                                                payload,
                                                "witness",
                                                "witness::Counter::value",
                                            ),
                                        )
                                    }
                                },
                                17 /* terrane-site: src/main.trn:45:17-45:29 */,
                            )?,
                        ),
                        17 /* terrane-site: src/main.trn:45:17-45:29 */,
                    )?,
                );
            })
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(consume.call(()),
            18 /* terrane-site: src/main.trn:46:13-46:21 */))
        );
    });
}
