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
            let __terrane_value: i64 = __terrane_raised(
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
fn wrap_async(
    handler: std::sync::Arc<
        dyn Fn(
            terrane_int_support::Int,
        ) -> std::pin::Pin<
                Box<dyn Future<Output = terrane_int_support::Int> + Send>,
            > + Send + Sync,
    >,
) -> std::sync::Arc<
    dyn Fn(
        terrane_int_support::Int,
    ) -> std::pin::Pin<
            Box<dyn Future<Output = terrane_int_support::Int> + Send>,
        > + Send + Sync,
> {
    return {
        let handler = handler.clone();
        std::sync::Arc::new(move |
            value: terrane_int_support::Int,
        | -> std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>> {
            let handler = handler.clone();
            Box::pin(async move {
                return __terrane_await(handler(value.clone())).await;
            })
        })
    };
}
async fn times_two(value: terrane_int_support::Int) -> terrane_int_support::Int {
    return value.clone() * terrane_int_support::Int::from(2_i128);
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
        let wrapped: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> std::pin::Pin<
                    Box<dyn Future<Output = terrane_int_support::Int> + Send>,
                > + Send + Sync,
        >;
        read_counter = {
            let __terrane_conversion_input_0 = terrane_int_support::Int::from(11_i128);
            __terrane_raised(
                terrane_static_trn_436f756e746572_new(__terrane_conversion_input_0),
                1 /* terrane-site: src/main.trn:23:20-23:42 */,
            )
        };
        read_only = {
            let read_counter = read_counter;
            std::sync::Arc::new(move || -> Result<i64, TerraneError> {
                return Ok(
                    __terrane_raised_err(
                        terrane_int_support::coerce::<
                            i64,
                        >(
                            &__terrane_raised_err(
                                || -> Result<_, crate::TerraneForeignError> {
                                    {
                                        let value = match std::panic::catch_unwind(
                                            std::panic::AssertUnwindSafe(|| -> Result<
                                                _,
                                                crate::TerraneForeignError,
                                            > { Ok((&read_counter).value()) }),
                                        ) {
                                            Ok(result) => result?,
                                            Err(payload) => {
                                                return Err(
                                                    crate::__terrane_dependency_panic(
                                                        payload,
                                                        "witness",
                                                        "witness::Counter::value",
                                                    ),
                                                );
                                            }
                                        };
                                        Ok::<
                                            _,
                                            crate::TerraneForeignError,
                                        >(terrane_int_support::Int::from(i128::from(value)))
                                    }
                                }(),
                                2 /* terrane-site: src/main.trn:25:17-25:36 */,
                            )?,
                        ),
                        2 /* terrane-site: src/main.trn:25:17-25:36 */,
                    )?,
                );
            })
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(read_only(),
            3 /* terrane-site: src/main.trn:26:13-26:23 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(read_only(),
            4 /* terrane-site: src/main.trn:27:13-27:23 */))
        );
        async_values = terrane_collection_support::List::<
            OpaqueValue,
        >::new(
            vec![
                { let __terrane_conversion_input_0 =
                terrane_int_support::Int::from(23_i128);
                __terrane_raised(terrane_static_trn_4f706171756556616c7565_new(__terrane_conversion_input_0),
                5 /* terrane-site: src/main.trn:29:47-29:67 */) }
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
                                                    6 /* terrane-site: src/main.trn:31:22-31:44 */,
                                                )
                                            }
                                        })
                                        .await,
                                    6 /* terrane-site: src/main.trn:31:22-31:44 */,
                                )?,
                            ),
                            7 /* terrane-site: src/main.trn:31:16-31:44 */,
                        )?,
                    );
                })
            })
        };
        async_value = __terrane_traced(
            __terrane_await(async_read_only()).await,
            8 /* terrane-site: src/main.trn:32:25-32:41 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&async_value));
        mixed_counter = {
            let __terrane_conversion_input_0 = terrane_int_support::Int::from(17_i128);
            __terrane_raised(
                terrane_static_trn_4f706171756556616c7565_new(
                    __terrane_conversion_input_0,
                ),
                9 /* terrane-site: src/main.trn:35:21-35:41 */,
            )
        };
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
                                    || -> Result<_, crate::TerraneForeignError> {
                                        {
                                            let value = match std::panic::catch_unwind(
                                                std::panic::AssertUnwindSafe(|| -> Result<
                                                    _,
                                                    crate::TerraneForeignError,
                                                > { Ok((&mixed_counter).value()) }),
                                            ) {
                                                Ok(result) => result?,
                                                Err(payload) => {
                                                    return Err(
                                                        crate::__terrane_dependency_panic(
                                                            payload,
                                                            "witness",
                                                            "witness::OpaqueValue::value",
                                                        ),
                                                    );
                                                }
                                            };
                                            Ok::<
                                                _,
                                                crate::TerraneForeignError,
                                            >(terrane_int_support::Int::from(i128::from(value)))
                                        }
                                    }(),
                                    10 /* terrane-site: src/main.trn:39:32-39:52 */,
                                )?),
                        ),
                        11 /* terrane-site: src/main.trn:39:16-39:53 */,
                    )?,
                );
            })
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(mixed.call(()),
            12 /* terrane-site: src/main.trn:40:13-40:19 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(mixed.call(()),
            13 /* terrane-site: src/main.trn:41:13-41:19 */))
        );
        callback_counter = {
            let __terrane_conversion_input_0 = terrane_int_support::Int::from(20_i128);
            __terrane_raised(
                terrane_static_trn_436f756e746572_new(__terrane_conversion_input_0),
                14 /* terrane-site: src/main.trn:43:24-43:46 */,
            )
        };
        callback = {
            std::sync::Arc::new(move |value: &mut Counter| -> () {
                increment(&mut *value);
                ()
            })
        };
        callback(&mut callback_counter);
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&callback_counter).value()) })) {
            Ok(result) => result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "witness", "witness::Counter::value"))
            }; Ok:: < _, crate ::TerraneForeignError >
            (terrane_int_support::Int::from(i128::from(value))) } } (),
            15 /* terrane-site: src/main.trn:47:13-47:36 */))
        );
        owned = {
            let __terrane_conversion_input_0 = terrane_int_support::Int::from(40_i128);
            __terrane_raised(
                terrane_static_trn_436f756e746572_new(__terrane_conversion_input_0),
                16 /* terrane-site: src/main.trn:49:13-49:35 */,
            )
        };
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
                                || -> Result<_, crate::TerraneForeignError> {
                                    {
                                        let value = match std::panic::catch_unwind(
                                            std::panic::AssertUnwindSafe(|| -> Result<
                                                _,
                                                crate::TerraneForeignError,
                                            > { Ok((&owned).value()) }),
                                        ) {
                                            Ok(result) => result?,
                                            Err(payload) => {
                                                return Err(
                                                    crate::__terrane_dependency_panic(
                                                        payload,
                                                        "witness",
                                                        "witness::Counter::value",
                                                    ),
                                                );
                                            }
                                        };
                                        Ok::<
                                            _,
                                            crate::TerraneForeignError,
                                        >(terrane_int_support::Int::from(i128::from(value)))
                                    }
                                }(),
                                17 /* terrane-site: src/main.trn:52:17-52:29 */,
                            )?,
                        ),
                        17 /* terrane-site: src/main.trn:52:17-52:29 */,
                    )?,
                );
            })
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(consume.call(()),
            18 /* terrane-site: src/main.trn:53:13-53:21 */))
        );
        wrapped = wrap_async(
            std::sync::Arc::new(move |
                argument_0: terrane_int_support::Int,
            | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
                Box::pin(times_two(argument_0))
            }),
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_await(wrapped(terrane_int_support::Int::from(7_i128)))
            . await)
        );
    });
}
