// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[allow(dead_code)]
fn trigger_static_panic(record: Row) -> Result<Vec<u8>, TerraneError> {
    return Ok(
        __terrane_raised_err(
            || -> Result<_, crate::TerraneForeignError> {
                match match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            <terrane_query_witness::Row as terrane_query_contract::RowAccess>::try_get(
                                &record,
                                "panic",
                            ),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane_query_witness",
                                "terrane_query_witness::Row::try_get",
                            ),
                        );
                    }
                } {
                    Ok(value) => Ok(value),
                    Err(error) => {
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
                }
            }(),
            0 /* terrane-site: src/main.trn:8:10-8:39 */,
        )?,
    );
}
#[allow(dead_code)]
fn trigger_instance_panic(record: Row) -> Result<Vec<u8>, TerraneError> {
    return Ok(
        __terrane_raised_err(
            || -> Result<_, crate::TerraneForeignError> {
                match match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            <terrane_query_witness::Row as terrane_query_contract::RowAccess>::try_get(
                                &record,
                                "panic",
                            ),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane_query_witness",
                                "terrane_query_witness::Row::try_get",
                            ),
                        );
                    }
                } {
                    Ok(value) => Ok(value),
                    Err(error) => {
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
                }
            }(),
            1 /* terrane-site: src/main.trn:11:10-11:33 */,
        )?,
    );
}
async fn stored_chains() -> Result<(), TerraneError> {
    let options: ConnectionOptions;
    let mut connection: Connection;
    let op: String;
    let one: terrane_query_witness::Query<'_, terrane_query_witness::Store, ()>;
    let one_result: StoreResult;
    let two: terrane_query_witness::Query<'_, terrane_query_witness::Store, ()>;
    let two_result: StoreResult;
    let pending: terrane_query_witness::Query<'_, terrane_query_witness::Store, ()>;
    let continued: terrane_query_witness::Query<'_, terrane_query_witness::Store, ()>;
    let continued_result: StoreResult;
    let twice: terrane_query_witness::Query<'_, terrane_query_witness::Store, ()>;
    let twice_result: StoreResult;
    let rows: terrane_collection_support::List<Row>;
    let mut record: &Row;
    let mut body: Vec<u8>;
    options = __terrane_raised_err(
        terrane_static_trn_436f6e6e656374696f6e4f7074696f6e73_new(),
        2 /* terrane-site: src/main.trn:14:13-14:36 */,
    )?;
    connection = __terrane_traced_err(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&options).connect();
                    Box::pin(async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                            .map_err(|payload| crate::__terrane_dependency_panic(
                                payload,
                                "terrane_query_witness",
                                "terrane_query_witness::ConnectionOptions::connect",
                            ))?
                        {
                            Ok(value) => Ok(value),
                            Err(error) => {
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
                        }
                    })
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        3 /* terrane-site: src/main.trn:15:22-15:38 */,
                    )
                }
            })
            .await,
        3 /* terrane-site: src/main.trn:15:22-15:38 */,
    )?;
    op = __terrane_raised_err(
        insert_sql(),
        4 /* terrane-site: src/main.trn:16:8-16:19 */,
    )?;
    one = {
        let __terrane_native_input_0 = __terrane_raised_err(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok(terrane_query_witness::query(&op)) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane-query-witness",
                                    "terrane_query_witness::query",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),
            5 /* terrane-site: src/main.trn:17:10-17:19 */,
        )?;
        let __terrane_conversion_input_0 = Vec::<u8>::from([111, 110, 101]);
        __terrane_raised_err(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                __terrane_native_input_0.bind(__terrane_conversion_input_0),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_query_witness",
                                    "terrane_query_witness::Query::bind",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),
            6 /* terrane-site: src/main.trn:17:9-17:33 */,
        )?
    };
    one_result = __terrane_traced_err(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = one.execute(&mut connection);
                    Box::pin(async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                            .map_err(|payload| crate::__terrane_dependency_panic(
                                payload,
                                "terrane_query_witness",
                                "terrane_query_witness::Query::execute",
                            ))?
                        {
                            Ok(value) => Ok(value),
                            Err(error) => {
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
                        }
                    })
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        7 /* terrane-site: src/main.trn:18:34-18:57 */,
                    )
                }
            })
            .await,
        7 /* terrane-site: src/main.trn:18:34-18:57 */,
    )?;
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised_err(| | -> Result
        < _, crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&one_result).rows_affected()) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::StoreResult::rows_affected")) };
        Ok:: < _, crate ::TerraneForeignError >
        (terrane_int_support::Int::from_u128(value as u128)) } } (),
        8 /* terrane-site: src/main.trn:19:11-19:36 */) ?), String::from(" "),
        terrane_scalar_support::scalar_text(&__terrane_raised_err(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&one_result).persistent()) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::StoreResult::persistent")) };
        Ok:: < _, crate ::TerraneForeignError > (value) } } (), 9 /* terrane-site: src/main.trn:19:45-19:67 */) ?)
    );
    two = {
        let __terrane_native_input_0 = {
            let __terrane_native_input_0 = __terrane_raised_err(
                || -> Result<_, crate::TerraneForeignError> {
                    {
                        let value = match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| -> Result<
                                _,
                                crate::TerraneForeignError,
                            > { Ok(terrane_query_witness::query(&op)) }),
                        ) {
                            Ok(result) => result?,
                            Err(payload) => {
                                return Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "terrane-query-witness",
                                        "terrane_query_witness::query",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),
                10 /* terrane-site: src/main.trn:20:11-20:20 */,
            )?;
            let __terrane_conversion_input_0 = Vec::<u8>::from([116, 119, 111]);
            __terrane_raised_err(
                || -> Result<_, crate::TerraneForeignError> {
                    {
                        let value = match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| -> Result<
                                _,
                                crate::TerraneForeignError,
                            > {
                                Ok(
                                    __terrane_native_input_0.bind(__terrane_conversion_input_0),
                                )
                            }),
                        ) {
                            Ok(result) => result?,
                            Err(payload) => {
                                return Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "terrane_query_witness",
                                        "terrane_query_witness::Query::bind",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),
                11 /* terrane-site: src/main.trn:20:10-20:34 */,
            )?
        };
        let __terrane_conversion_input_0 = false;
        __terrane_raised_err(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                __terrane_native_input_0
                                    .persistent(__terrane_conversion_input_0),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_query_witness",
                                    "terrane_query_witness::Query::persistent",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),
            12 /* terrane-site: src/main.trn:20:9-20:53 */,
        )?
    };
    two_result = __terrane_traced_err(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = two.execute(&mut connection);
                    Box::pin(async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                            .map_err(|payload| crate::__terrane_dependency_panic(
                                payload,
                                "terrane_query_witness",
                                "terrane_query_witness::Query::execute",
                            ))?
                        {
                            Ok(value) => Ok(value),
                            Err(error) => {
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
                        }
                    })
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        13 /* terrane-site: src/main.trn:21:34-21:57 */,
                    )
                }
            })
            .await,
        13 /* terrane-site: src/main.trn:21:34-21:57 */,
    )?;
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised_err(| | -> Result
        < _, crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&two_result).rows_affected()) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::StoreResult::rows_affected")) };
        Ok:: < _, crate ::TerraneForeignError >
        (terrane_int_support::Int::from_u128(value as u128)) } } (),
        14 /* terrane-site: src/main.trn:22:11-22:36 */) ?), String::from(" "),
        terrane_scalar_support::scalar_text(&__terrane_raised_err(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&two_result).persistent()) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::StoreResult::persistent")) };
        Ok:: < _, crate ::TerraneForeignError > (value) } } (), 15 /* terrane-site: src/main.trn:22:45-22:67 */) ?)
    );
    pending = {
        let __terrane_native_input_0 = {
            let __terrane_native_input_0 = __terrane_raised_err(
                || -> Result<_, crate::TerraneForeignError> {
                    {
                        let value = match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| -> Result<
                                _,
                                crate::TerraneForeignError,
                            > { Ok(terrane_query_witness::query(&op)) }),
                        ) {
                            Ok(result) => result?,
                            Err(payload) => {
                                return Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "terrane-query-witness",
                                        "terrane_query_witness::query",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),
                16 /* terrane-site: src/main.trn:23:15-23:24 */,
            )?;
            let __terrane_conversion_input_0 = Vec::<
                u8,
            >::from([116, 104, 114, 101, 101]);
            __terrane_raised_err(
                || -> Result<_, crate::TerraneForeignError> {
                    {
                        let value = match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| -> Result<
                                _,
                                crate::TerraneForeignError,
                            > {
                                Ok(
                                    __terrane_native_input_0.bind(__terrane_conversion_input_0),
                                )
                            }),
                        ) {
                            Ok(result) => result?,
                            Err(payload) => {
                                return Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "terrane_query_witness",
                                        "terrane_query_witness::Query::bind",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),
                17 /* terrane-site: src/main.trn:23:14-23:40 */,
            )?
        };
        let __terrane_conversion_input_0 = false;
        __terrane_raised_err(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                __terrane_native_input_0
                                    .persistent(__terrane_conversion_input_0),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_query_witness",
                                    "terrane_query_witness::Query::persistent",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),
            18 /* terrane-site: src/main.trn:23:13-23:59 */,
        )?
    };
    continued = {
        let __terrane_conversion_input_0 = true;
        __terrane_raised_err(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok(pending.persistent(__terrane_conversion_input_0)) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_query_witness",
                                    "terrane_query_witness::Query::persistent",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),
            19 /* terrane-site: src/main.trn:24:15-24:39 */,
        )?
    };
    continued_result = __terrane_traced_err(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = continued.execute(&mut connection);
                    Box::pin(async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                            .map_err(|payload| crate::__terrane_dependency_panic(
                                payload,
                                "terrane_query_witness",
                                "terrane_query_witness::Query::execute",
                            ))?
                        {
                            Ok(value) => Ok(value),
                            Err(error) => {
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
                        }
                    })
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        20 /* terrane-site: src/main.trn:25:40-25:69 */,
                    )
                }
            })
            .await,
        20 /* terrane-site: src/main.trn:25:40-25:69 */,
    )?;
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised_err(| | -> Result
        < _, crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&continued_result).rows_affected()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane_query_witness",
        "terrane_query_witness::StoreResult::rows_affected")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 21 /* terrane-site: src/main.trn:26:11-26:42 */) ?), String::from(" "),
        terrane_scalar_support::scalar_text(&__terrane_raised_err(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&continued_result).persistent()) })) { Ok(result)
        => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane_query_witness",
        "terrane_query_witness::StoreResult::persistent")) }; Ok:: < _, crate
        ::TerraneForeignError > (value) } } (), 22 /* terrane-site: src/main.trn:26:51-26:79 */) ?)
    );
    twice = {
        let __terrane_native_input_0 = {
            let __terrane_native_input_0 = __terrane_raised_err(
                || -> Result<_, crate::TerraneForeignError> {
                    {
                        let value = match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| -> Result<
                                _,
                                crate::TerraneForeignError,
                            > { Ok(terrane_query_witness::query(&op)) }),
                        ) {
                            Ok(result) => result?,
                            Err(payload) => {
                                return Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "terrane-query-witness",
                                        "terrane_query_witness::query",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),
                23 /* terrane-site: src/main.trn:27:13-27:22 */,
            )?;
            let __terrane_conversion_input_0 = Vec::<
                u8,
            >::from([105, 103, 110, 111, 114, 101, 100]);
            __terrane_raised_err(
                || -> Result<_, crate::TerraneForeignError> {
                    {
                        let value = match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| -> Result<
                                _,
                                crate::TerraneForeignError,
                            > {
                                Ok(
                                    __terrane_native_input_0.bind(__terrane_conversion_input_0),
                                )
                            }),
                        ) {
                            Ok(result) => result?,
                            Err(payload) => {
                                return Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "terrane_query_witness",
                                        "terrane_query_witness::Query::bind",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),
                24 /* terrane-site: src/main.trn:27:12-27:40 */,
            )?
        };
        let __terrane_conversion_input_0 = Vec::<u8>::from([102, 111, 117, 114]);
        __terrane_raised_err(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                __terrane_native_input_0.bind(__terrane_conversion_input_0),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_query_witness",
                                    "terrane_query_witness::Query::bind",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),
            25 /* terrane-site: src/main.trn:27:11-27:55 */,
        )?
    };
    twice_result = __terrane_traced_err(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = twice.execute(&mut connection);
                    Box::pin(async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                            .map_err(|payload| crate::__terrane_dependency_panic(
                                payload,
                                "terrane_query_witness",
                                "terrane_query_witness::Query::execute",
                            ))?
                        {
                            Ok(value) => Ok(value),
                            Err(error) => {
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
                        }
                    })
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        26 /* terrane-site: src/main.trn:28:36-28:61 */,
                    )
                }
            })
            .await,
        26 /* terrane-site: src/main.trn:28:36-28:61 */,
    )?;
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised_err(| | -> Result
        < _, crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&twice_result).rows_affected()) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::StoreResult::rows_affected")) };
        Ok:: < _, crate ::TerraneForeignError >
        (terrane_int_support::Int::from_u128(value as u128)) } } (),
        27 /* terrane-site: src/main.trn:29:11-29:38 */) ?), String::from(" "),
        terrane_scalar_support::scalar_text(&__terrane_raised_err(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&twice_result).persistent()) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::StoreResult::persistent")) };
        Ok:: < _, crate ::TerraneForeignError > (value) } } (), 28 /* terrane-site: src/main.trn:29:47-29:71 */) ?)
    );
    rows = __terrane_traced_err(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = __terrane_raised_err(
                            || -> Result<_, crate::TerraneForeignError> {
                                {
                                    let value = match std::panic::catch_unwind(
                                        std::panic::AssertUnwindSafe(|| -> Result<
                                            _,
                                            crate::TerraneForeignError,
                                        > { Ok(terrane_query_witness::query("select")) }),
                                    ) {
                                        Ok(result) => result?,
                                        Err(payload) => {
                                            return Err(
                                                crate::__terrane_dependency_panic(
                                                    payload,
                                                    "terrane-query-witness",
                                                    "terrane_query_witness::query",
                                                ),
                                            );
                                        }
                                    };
                                    Ok::<_, crate::TerraneForeignError>(value)
                                }
                            }(),
                            29 /* terrane-site: src/main.trn:30:29-30:44 */,
                        )?
                        .fetch_all(&mut connection);
                    Box::pin(async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                            .map_err(|payload| crate::__terrane_dependency_panic(
                                payload,
                                "terrane_query_witness",
                                "terrane_query_witness::Query::fetch_all",
                            ))?
                        {
                            Ok(value) => Ok(terrane_collection_support::List::new(value)),
                            Err(error) => {
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
                        }
                    })
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        30 /* terrane-site: src/main.trn:30:28-30:67 */,
                    )
                }
            })
            .await,
        30 /* terrane-site: src/main.trn:30:28-30:67 */,
    )?;
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(rows
        .length()))
    );
    let __terrane_iterable_0 = &rows;
    let mut __terrane_iterator_0 = __terrane_iterable_0.terrane_borrowing_iterator();
    loop {
        record = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        body = __terrane_raised_err(
            || -> Result<_, crate::TerraneForeignError> {
                match match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            <terrane_query_witness::Row as terrane_query_contract::RowAccess>::try_get(
                                &*record,
                                "body",
                            ),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane_query_witness",
                                "terrane_query_witness::Row::try_get",
                            ),
                        );
                    }
                } {
                    Ok(value) => Ok(value),
                    Err(error) => {
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
                }
            }(),
            31 /* terrane-site: src/main.trn:33:18-33:40 */,
        )?;
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised_err(terrane_collection_support::byte_at(&body,
            __terrane_raised_err(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            32 /* terrane-site: src/main.trn:34:12-34:19 */) ?),
            32 /* terrane-site: src/main.trn:34:12-34:19 */) ?)
        );
    }
    __terrane_traced_err(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = connection.close();
                    Box::pin(async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                            .map_err(|payload| crate::__terrane_dependency_panic(
                                payload,
                                "terrane_query_witness",
                                "terrane_query_witness::Connection::close",
                            ))?
                        {
                            Ok(value) => Ok(value),
                            Err(error) => {
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
                        }
                    })
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        33 /* terrane-site: src/main.trn:35:9-35:26 */,
                    )
                }
            })
            .await,
        33 /* terrane-site: src/main.trn:35:9-35:26 */,
    )?;
    Ok(())
}
fn main() {
    __terrane_run(async move {
        let options: ConnectionOptions;
        let mut connection: Connection;
        let insert_op: String;
        let first_op: String;
        let second_op: String;
        let invalid_op: String;
        let insert_result: StoreResult;
        let rows: terrane_collection_support::List<Row>;
        let row: Row;
        let body: Vec<u8>;
        let second_row: Row;
        let second_body: Vec<u8>;
        options = __terrane_raised(
            terrane_static_trn_436f6e6e656374696f6e4f7074696f6e73_new(),
            34 /* terrane-site: src/main.trn:38:13-38:36 */,
        );
        connection = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = (&options).connect();
                        Box::pin(async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                                .map_err(|payload| crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_query_witness",
                                    "terrane_query_witness::ConnectionOptions::connect",
                                ))?
                            {
                                Ok(value) => Ok(value),
                                Err(error) => {
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
                            }
                        })
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            35 /* terrane-site: src/main.trn:39:22-39:38 */,
                        )
                    }
                })
                .await,
            35 /* terrane-site: src/main.trn:39:22-39:38 */,
        );
        insert_op = __terrane_raised(
            insert_sql(),
            36 /* terrane-site: src/main.trn:40:15-40:26 */,
        );
        first_op = String::from("select");
        second_op = String::from("select-second");
        invalid_op = String::from("invalid");
        insert_result = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = {
                            let __terrane_native_input_0 = {
                                let __terrane_native_input_0 = __terrane_raised(
                                    || -> Result<_, crate::TerraneForeignError> {
                                        {
                                            let value = match std::panic::catch_unwind(
                                                std::panic::AssertUnwindSafe(|| -> Result<
                                                    _,
                                                    crate::TerraneForeignError,
                                                > { Ok(terrane_query_witness::query(&insert_op)) }),
                                            ) {
                                                Ok(result) => result?,
                                                Err(payload) => {
                                                    return Err(
                                                        crate::__terrane_dependency_panic(
                                                            payload,
                                                            "terrane-query-witness",
                                                            "terrane_query_witness::query",
                                                        ),
                                                    );
                                                }
                                            };
                                            Ok::<_, crate::TerraneForeignError>(value)
                                        }
                                    }(),
                                    37 /* terrane-site: src/main.trn:44:40-44:56 */,
                                );
                                let __terrane_conversion_input_0 = Vec::<
                                    u8,
                                >::from([102, 105, 114, 115, 116]);
                                __terrane_raised(
                                    || -> Result<_, crate::TerraneForeignError> {
                                        {
                                            let value = match std::panic::catch_unwind(
                                                std::panic::AssertUnwindSafe(|| -> Result<
                                                    _,
                                                    crate::TerraneForeignError,
                                                > {
                                                    Ok(
                                                        __terrane_native_input_0.bind(__terrane_conversion_input_0),
                                                    )
                                                }),
                                            ) {
                                                Ok(result) => result?,
                                                Err(payload) => {
                                                    return Err(
                                                        crate::__terrane_dependency_panic(
                                                            payload,
                                                            "terrane_query_witness",
                                                            "terrane_query_witness::Query::bind",
                                                        ),
                                                    );
                                                }
                                            };
                                            Ok::<_, crate::TerraneForeignError>(value)
                                        }
                                    }(),
                                    38 /* terrane-site: src/main.trn:44:39-44:72 */,
                                )
                            };
                            let __terrane_conversion_input_0 = false;
                            __terrane_raised(
                                || -> Result<_, crate::TerraneForeignError> {
                                    {
                                        let value = match std::panic::catch_unwind(
                                            std::panic::AssertUnwindSafe(|| -> Result<
                                                _,
                                                crate::TerraneForeignError,
                                            > {
                                                Ok(
                                                    __terrane_native_input_0
                                                        .persistent(__terrane_conversion_input_0),
                                                )
                                            }),
                                        ) {
                                            Ok(result) => result?,
                                            Err(payload) => {
                                                return Err(
                                                    crate::__terrane_dependency_panic(
                                                        payload,
                                                        "terrane_query_witness",
                                                        "terrane_query_witness::Query::persistent",
                                                    ),
                                                );
                                            }
                                        };
                                        Ok::<_, crate::TerraneForeignError>(value)
                                    }
                                }(),
                                39 /* terrane-site: src/main.trn:44:38-44:91 */,
                            )
                        }
                            .execute(&mut connection);
                        Box::pin(async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                                .map_err(|payload| crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_query_witness",
                                    "terrane_query_witness::Query::execute",
                                ))?
                            {
                                Ok(value) => Ok(value),
                                Err(error) => {
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
                            }
                        })
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            40 /* terrane-site: src/main.trn:44:37-44:112 */,
                        )
                    }
                })
                .await,
            40 /* terrane-site: src/main.trn:44:37-44:112 */,
        );
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result
            < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&insert_result).rows_affected()) })) {
            Ok(result) => result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::StoreResult::rows_affected")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 41 /* terrane-site: src/main.trn:45:11-45:39 */)),
            String::from(" "), terrane_scalar_support::scalar_text(&__terrane_raised(| |
            -> Result < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&insert_result).persistent()) })) {
            Ok(result) => result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::StoreResult::persistent")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 42 /* terrane-site: src/main.trn:45:48-45:73 */))
        );
        __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = {
                            let __terrane_native_input_0 = __terrane_raised(
                                || -> Result<_, crate::TerraneForeignError> {
                                    {
                                        let value = match std::panic::catch_unwind(
                                            std::panic::AssertUnwindSafe(|| -> Result<
                                                _,
                                                crate::TerraneForeignError,
                                            > { Ok(terrane_query_witness::query(&insert_op)) }),
                                        ) {
                                            Ok(result) => result?,
                                            Err(payload) => {
                                                return Err(
                                                    crate::__terrane_dependency_panic(
                                                        payload,
                                                        "terrane-query-witness",
                                                        "terrane_query_witness::query",
                                                    ),
                                                );
                                            }
                                        };
                                        Ok::<_, crate::TerraneForeignError>(value)
                                    }
                                }(),
                                43 /* terrane-site: src/main.trn:46:11-46:27 */,
                            );
                            let __terrane_conversion_input_0 = Vec::<
                                u8,
                            >::from([115, 101, 99, 111, 110, 100]);
                            __terrane_raised(
                                || -> Result<_, crate::TerraneForeignError> {
                                    {
                                        let value = match std::panic::catch_unwind(
                                            std::panic::AssertUnwindSafe(|| -> Result<
                                                _,
                                                crate::TerraneForeignError,
                                            > {
                                                Ok(
                                                    __terrane_native_input_0.bind(__terrane_conversion_input_0),
                                                )
                                            }),
                                        ) {
                                            Ok(result) => result?,
                                            Err(payload) => {
                                                return Err(
                                                    crate::__terrane_dependency_panic(
                                                        payload,
                                                        "terrane_query_witness",
                                                        "terrane_query_witness::Query::bind",
                                                    ),
                                                );
                                            }
                                        };
                                        Ok::<_, crate::TerraneForeignError>(value)
                                    }
                                }(),
                                44 /* terrane-site: src/main.trn:46:10-46:44 */,
                            )
                        }
                            .execute(&mut connection);
                        Box::pin(async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                                .map_err(|payload| crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_query_witness",
                                    "terrane_query_witness::Query::execute",
                                ))?
                            {
                                Ok(value) => Ok(value),
                                Err(error) => {
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
                            }
                        })
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            45 /* terrane-site: src/main.trn:46:9-46:65 */,
                        )
                    }
                })
                .await,
            45 /* terrane-site: src/main.trn:46:9-46:65 */,
        );
        rows = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = __terrane_raised(
                                || -> Result<_, crate::TerraneForeignError> {
                                    {
                                        let value = match std::panic::catch_unwind(
                                            std::panic::AssertUnwindSafe(|| -> Result<
                                                _,
                                                crate::TerraneForeignError,
                                            > { Ok(terrane_query_witness::query(&first_op)) }),
                                        ) {
                                            Ok(result) => result?,
                                            Err(payload) => {
                                                return Err(
                                                    crate::__terrane_dependency_panic(
                                                        payload,
                                                        "terrane-query-witness",
                                                        "terrane_query_witness::query",
                                                    ),
                                                );
                                            }
                                        };
                                        Ok::<_, crate::TerraneForeignError>(value)
                                    }
                                }(),
                                46 /* terrane-site: src/main.trn:47:29-47:44 */,
                            )
                            .fetch_all(&mut connection);
                        Box::pin(async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                                .map_err(|payload| crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_query_witness",
                                    "terrane_query_witness::Query::fetch_all",
                                ))?
                            {
                                Ok(value) => {
                                    Ok(terrane_collection_support::List::new(value))
                                }
                                Err(error) => {
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
                            }
                        })
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            47 /* terrane-site: src/main.trn:47:28-47:67 */,
                        )
                    }
                })
                .await,
            47 /* terrane-site: src/main.trn:47:28-47:67 */,
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(rows
            .length()))
        );
        row = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = __terrane_raised(
                                || -> Result<_, crate::TerraneForeignError> {
                                    {
                                        let value = match std::panic::catch_unwind(
                                            std::panic::AssertUnwindSafe(|| -> Result<
                                                _,
                                                crate::TerraneForeignError,
                                            > { Ok(terrane_query_witness::query(&first_op)) }),
                                        ) {
                                            Ok(result) => result?,
                                            Err(payload) => {
                                                return Err(
                                                    crate::__terrane_dependency_panic(
                                                        payload,
                                                        "terrane-query-witness",
                                                        "terrane_query_witness::query",
                                                    ),
                                                );
                                            }
                                        };
                                        Ok::<_, crate::TerraneForeignError>(value)
                                    }
                                }(),
                                48 /* terrane-site: src/main.trn:49:20-49:35 */,
                            )
                            .fetch_one(&mut connection);
                        Box::pin(async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                                .map_err(|payload| crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_query_witness",
                                    "terrane_query_witness::Query::fetch_one",
                                ))?
                            {
                                Ok(value) => Ok(value),
                                Err(error) => {
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
                            }
                        })
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            49 /* terrane-site: src/main.trn:49:19-49:58 */,
                        )
                    }
                })
                .await,
            49 /* terrane-site: src/main.trn:49:19-49:58 */,
        );
        body = __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                match match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            <terrane_query_witness::Row as terrane_query_contract::RowAccess>::try_get(
                                &row,
                                "body",
                            ),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane_query_witness",
                                "terrane_query_witness::Row::try_get",
                            ),
                        );
                    }
                } {
                    Ok(value) => Ok(value),
                    Err(error) => {
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
                }
            }(),
            50 /* terrane-site: src/main.trn:50:16-50:41 */,
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::byte_at(&body,
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            51 /* terrane-site: src/main.trn:51:10-51:17 */)), 51 /* terrane-site: src/main.trn:51:10-51:17 */))
        );
        second_row = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = __terrane_raised(
                                || -> Result<_, crate::TerraneForeignError> {
                                    {
                                        let value = match std::panic::catch_unwind(
                                            std::panic::AssertUnwindSafe(|| -> Result<
                                                _,
                                                crate::TerraneForeignError,
                                            > { Ok(terrane_query_witness::query(&second_op)) }),
                                        ) {
                                            Ok(result) => result?,
                                            Err(payload) => {
                                                return Err(
                                                    crate::__terrane_dependency_panic(
                                                        payload,
                                                        "terrane-query-witness",
                                                        "terrane_query_witness::query",
                                                    ),
                                                );
                                            }
                                        };
                                        Ok::<_, crate::TerraneForeignError>(value)
                                    }
                                }(),
                                52 /* terrane-site: src/main.trn:52:27-52:43 */,
                            )
                            .fetch_one(&mut connection);
                        Box::pin(async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                                .map_err(|payload| crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_query_witness",
                                    "terrane_query_witness::Query::fetch_one",
                                ))?
                            {
                                Ok(value) => Ok(value),
                                Err(error) => {
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
                            }
                        })
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            53 /* terrane-site: src/main.trn:52:26-52:66 */,
                        )
                    }
                })
                .await,
            53 /* terrane-site: src/main.trn:52:26-52:66 */,
        );
        second_body = __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                match match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            <terrane_query_witness::Row as terrane_query_contract::RowAccess>::try_get(
                                &second_row,
                                "body",
                            ),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane_query_witness",
                                "terrane_query_witness::Row::try_get",
                            ),
                        );
                    }
                } {
                    Ok(value) => Ok(value),
                    Err(error) => {
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
                }
            }(),
            54 /* terrane-site: src/main.trn:53:23-53:49 */,
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::byte_at(&second_body,
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            55 /* terrane-site: src/main.trn:54:10-54:24 */)), 55 /* terrane-site: src/main.trn:54:10-54:24 */))
        );
        let __terrane_completion_0: TerraneCompletion<()> = async {
            let __terrane_try_0: TerraneCompletion<()> = async {
                __terrane_traced_completion!(
                    __terrane_await({ let __terrane_future = { let __terrane_call =
                    __terrane_raised_completion!(| | -> Result < _, crate
                    ::TerraneForeignError > { { let value = match
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result <
                    _, crate ::TerraneForeignError > {
                    Ok(terrane_query_witness::query(&invalid_op)) })) { Ok(result) =>
                    result ?, Err(payload) => return Err(crate
                    ::__terrane_dependency_panic(payload, "terrane-query-witness",
                    "terrane_query_witness::query")) }; Ok:: < _, crate
                    ::TerraneForeignError > (value) } } (), 56 /* terrane-site: src/main.trn:56:12-56:29 */) .execute(&mut connection);
                    Box::pin(async move { match crate
                    ::__terrane_dependency_await_unwind(__terrane_call). await .map_err(|
                    payload | crate ::__terrane_dependency_panic(payload,
                    "terrane_query_witness", "terrane_query_witness::Query::execute")) ?
                    { Ok(value) => Ok(value), Err(error) => Err(crate
                    ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
                    ::DescriptorId(2), error.to_string(), crate ::TERRANE_NO_SITE))) } })
                    }; async move { __terrane_raised_err(__terrane_future. await,
                    57 /* terrane-site: src/main.trn:56:11-56:50 */) } }). await,
                    57 /* terrane-site: src/main.trn:56:11-56:50 */
                );
                TerraneCompletion::Normal
            }
                .await;
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
                            == TerraneErrorKind::Custom(DescriptorId(2))
                    {
                        __terrane_handled_0 = true;
                        println!("{}", String::from("query-error"));
                    }
                    if !__terrane_handled_0 {
                        return TerraneCompletion::Error(__terrane_error_0);
                    }
                }
            }
            TerraneCompletion::Normal
        }
            .await;
        match __terrane_completion_0 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = connection.close();
                        Box::pin(async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                                .map_err(|payload| crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_query_witness",
                                    "terrane_query_witness::Connection::close",
                                ))?
                            {
                                Ok(value) => Ok(value),
                                Err(error) => {
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
                            }
                        })
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            58 /* terrane-site: src/main.trn:59:9-59:26 */,
                        )
                    }
                })
                .await,
            58 /* terrane-site: src/main.trn:59:9-59:26 */,
        );
        __terrane_traced(
            __terrane_await(stored_chains()).await,
            59 /* terrane-site: src/main.trn:60:9-60:23 */,
        );
        println!("{}", String::from("complete"));
    });
}
