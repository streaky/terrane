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
            2 /* terrane-site: src/main.trn:14:13-14:36 */,
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
                            3 /* terrane-site: src/main.trn:15:22-15:38 */,
                        )
                    }
                })
                .await,
            3 /* terrane-site: src/main.trn:15:22-15:38 */,
        );
        insert_op = __terrane_raised(
            insert_sql(),
            4 /* terrane-site: src/main.trn:16:15-16:26 */,
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
                                    5 /* terrane-site: src/main.trn:20:40-20:56 */,
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
                                    6 /* terrane-site: src/main.trn:20:39-20:72 */,
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
                                7 /* terrane-site: src/main.trn:20:38-20:91 */,
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
                            8 /* terrane-site: src/main.trn:20:37-20:112 */,
                        )
                    }
                })
                .await,
            8 /* terrane-site: src/main.trn:20:37-20:112 */,
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
            } } (), 9 /* terrane-site: src/main.trn:21:11-21:39 */)),
            String::from(" "), terrane_scalar_support::scalar_text(&__terrane_raised(| |
            -> Result < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&insert_result).persistent()) })) {
            Ok(result) => result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::StoreResult::persistent")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 10 /* terrane-site: src/main.trn:21:48-21:73 */))
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
                                11 /* terrane-site: src/main.trn:22:11-22:27 */,
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
                                12 /* terrane-site: src/main.trn:22:10-22:44 */,
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
                            13 /* terrane-site: src/main.trn:22:9-22:65 */,
                        )
                    }
                })
                .await,
            13 /* terrane-site: src/main.trn:22:9-22:65 */,
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
                                14 /* terrane-site: src/main.trn:23:29-23:44 */,
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
                            15 /* terrane-site: src/main.trn:23:28-23:67 */,
                        )
                    }
                })
                .await,
            15 /* terrane-site: src/main.trn:23:28-23:67 */,
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
                                16 /* terrane-site: src/main.trn:25:20-25:35 */,
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
                            17 /* terrane-site: src/main.trn:25:19-25:58 */,
                        )
                    }
                })
                .await,
            17 /* terrane-site: src/main.trn:25:19-25:58 */,
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
            18 /* terrane-site: src/main.trn:26:16-26:41 */,
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::byte_at(&body,
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            19 /* terrane-site: src/main.trn:27:10-27:17 */)), 19 /* terrane-site: src/main.trn:27:10-27:17 */))
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
                                20 /* terrane-site: src/main.trn:28:27-28:43 */,
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
                            21 /* terrane-site: src/main.trn:28:26-28:66 */,
                        )
                    }
                })
                .await,
            21 /* terrane-site: src/main.trn:28:26-28:66 */,
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
            22 /* terrane-site: src/main.trn:29:23-29:49 */,
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::byte_at(&second_body,
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            23 /* terrane-site: src/main.trn:30:10-30:24 */)), 23 /* terrane-site: src/main.trn:30:10-30:24 */))
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
                    ::TerraneForeignError > (value) } } (), 24 /* terrane-site: src/main.trn:32:12-32:29 */) .execute(&mut connection);
                    Box::pin(async move { match crate
                    ::__terrane_dependency_await_unwind(__terrane_call). await .map_err(|
                    payload | crate ::__terrane_dependency_panic(payload,
                    "terrane_query_witness", "terrane_query_witness::Query::execute")) ?
                    { Ok(value) => Ok(value), Err(error) => Err(crate
                    ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
                    ::DescriptorId(2), error.to_string(), crate ::TERRANE_NO_SITE))) } })
                    }; async move { __terrane_raised_err(__terrane_future. await,
                    25 /* terrane-site: src/main.trn:32:11-32:50 */) } }). await,
                    25 /* terrane-site: src/main.trn:32:11-32:50 */
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
                            26 /* terrane-site: src/main.trn:35:9-35:26 */,
                        )
                    }
                })
                .await,
            26 /* terrane-site: src/main.trn:35:9-35:26 */,
        );
        println!("{}", String::from("complete"));
    });
}
