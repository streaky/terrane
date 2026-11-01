// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    __terrane_run(async move {
        let database: Database;
        let answer: terrane_int_support::Int;
        let prefix: String;
        let rendered: String;
        let mut builder: terrane_chain_witness::LineBuilder<'_>;
        let resumed: terrane_chain_witness::LineBuilder<'_>;
        database = __terrane_traced(
            __terrane_await({
                    let __terrane_future = memory_database();
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            0 /* terrane-site: src/main.trn:5:20-5:36 */,
                        )
                    }
                })
                .await,
            0 /* terrane-site: src/main.trn:5:20-5:36 */,
        );
        answer = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = {
                            let __terrane_native_input_0 = __terrane_raised(
                                match std::panic::catch_unwind(
                                    std::panic::AssertUnwindSafe(|| terrane_chain_witness::lookup_scalar(
                                        &database,
                                        String::from("answer"),
                                    )),
                                ) {
                                    Ok(value) => Ok(value),
                                    Err(payload) => {
                                        Err(
                                            crate::__terrane_dependency_panic(
                                                payload,
                                                "terrane-chain-witness",
                                                "terrane_chain_witness::lookup_scalar",
                                            ),
                                        )
                                    }
                                },
                                1 /* terrane-site: src/main.trn:6:24-6:57 */,
                            );
                            let __terrane_native_input_1 = match || -> Result<
                                _,
                                crate::TerraneForeignError,
                            > {
                                Ok(
                                    terrane_int_support::coerce::<
                                        i64,
                                    >(&terrane_int_support::Int::from(42_i128))
                                        .map_err(|error| crate::TerraneForeignError(
                                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                        ))?,
                                )
                            }() {
                                Ok(value) => value,
                                Err(error) => std::panic::panic_any(error),
                            };
                            __terrane_raised(
                                match std::panic::catch_unwind(
                                    std::panic::AssertUnwindSafe(|| {
                                        __terrane_native_input_0.bind(__terrane_native_input_1)
                                    }),
                                ) {
                                    Ok(value) => Ok(value),
                                    Err(payload) => {
                                        Err(
                                            crate::__terrane_dependency_panic(
                                                payload,
                                                "terrane_chain_witness",
                                                "terrane_chain_witness::ScalarLookup::bind",
                                            ),
                                        )
                                    }
                                },
                                2 /* terrane-site: src/main.trn:6:23-6:67 */,
                            )
                        }
                            .fetch_one();
                        Box::pin(async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                            {
                                Ok(Ok(value)) => {
                                    Ok(terrane_int_support::Int::from(i128::from(value)))
                                }
                                Ok(Err(error)) => {
                                    Err(
                                        crate::TerraneForeignError(
                                            crate::TerraneError::custom_raised(
                                                crate::DescriptorId(2),
                                                error.to_string(),
                                                crate::TERRANE_NO_SITE,
                                            ),
                                        ),
                                    )
                                }
                                Err(payload) => {
                                    Err(
                                        crate::__terrane_dependency_panic(
                                            payload,
                                            "terrane_chain_witness",
                                            "terrane_chain_witness::ScalarLookup::fetch_one",
                                        ),
                                    )
                                }
                            }
                        })
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            3 /* terrane-site: src/main.trn:6:22-6:79 */,
                        )
                    }
                })
                .await,
            3 /* terrane-site: src/main.trn:6:22-6:79 */,
        );
        prefix = String::from("value=");
        rendered = {
            let __terrane_native_input_0 = {
                let __terrane_native_input_0 = __terrane_raised(
                    match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| terrane_chain_witness::line(
                            &prefix,
                        )),
                    ) {
                        Ok(value) => Ok(value),
                        Err(payload) => {
                            Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane-chain-witness",
                                    "terrane_chain_witness::line",
                                ),
                            )
                        }
                    },
                    4 /* terrane-site: src/main.trn:8:23-8:35 */,
                );
                let __terrane_native_input_1 = match || -> Result<
                    _,
                    crate::TerraneForeignError,
                > {
                    Ok(
                        terrane_int_support::coerce::<i64>(&answer.clone())
                            .map_err(|error| crate::TerraneForeignError(
                                crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                            ))?,
                    )
                }() {
                    Ok(value) => value,
                    Err(error) => std::panic::panic_any(error),
                };
                __terrane_raised(
                    match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| {
                            __terrane_native_input_0.number(__terrane_native_input_1)
                        }),
                    ) {
                        Ok(value) => Ok(value),
                        Err(payload) => {
                            Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_chain_witness",
                                    "terrane_chain_witness::LineBuilder::number",
                                ),
                            )
                        }
                    },
                    5 /* terrane-site: src/main.trn:8:22-8:51 */,
                )
            };
            __terrane_raised(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| __terrane_native_input_0.finish()),
                ) {
                    Ok(value) => Ok(value),
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane_chain_witness",
                                "terrane_chain_witness::LineBuilder::finish",
                            ),
                        )
                    }
                },
                6 /* terrane-site: src/main.trn:8:21-8:60 */,
            )
        };
        println!("{}", terrane_scalar_support::scalar_text(&answer));
        println!("{}", terrane_scalar_support::scalar_text(&rendered));
        builder = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| terrane_chain_witness::line(&prefix)),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "terrane-chain-witness",
                            "terrane_chain_witness::line",
                        ),
                    )
                }
            },
            7 /* terrane-site: src/main.trn:11:13-11:25 */,
        );
        builder = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    builder
                        .number(
                            match || -> Result<_, crate::TerraneForeignError> {
                                Ok(
                                    terrane_int_support::coerce::<
                                        i64,
                                    >(
                                            &(answer.clone() + terrane_int_support::Int::from(1_i128)),
                                        )
                                        .map_err(|error| crate::TerraneForeignError(
                                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                        ))?,
                                )
                            }() {
                                Ok(value) => value,
                                Err(error) => std::panic::panic_any(error),
                            },
                        )
                }),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "terrane_chain_witness",
                            "terrane_chain_witness::LineBuilder::number",
                        ),
                    )
                }
            },
            8 /* terrane-site: src/main.trn:12:13-12:39 */,
        );
        resumed = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = builder.pause();
                        Box::pin(async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                            {
                                Ok(value) => Ok(value),
                                Err(payload) => {
                                    Err(
                                        crate::__terrane_dependency_panic(
                                            payload,
                                            "terrane_chain_witness",
                                            "terrane_chain_witness::LineBuilder::pause",
                                        ),
                                    )
                                }
                            }
                        })
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            9 /* terrane-site: src/main.trn:13:19-13:33 */,
                        )
                    }
                })
                .await,
            9 /* terrane-site: src/main.trn:13:19-13:33 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | resumed.finish()))
            { Ok(value) => Ok(value), Err(payload) => Err(crate
            ::__terrane_dependency_panic(payload, "terrane_chain_witness",
            "terrane_chain_witness::LineBuilder::finish")) }, 10 /* terrane-site: src/main.trn:14:11-14:26 */))
        );
    });
}
