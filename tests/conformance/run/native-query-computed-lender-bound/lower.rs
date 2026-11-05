// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn lend<'a>(sql: &'a String) -> &'a String {
    return sql;
}
async fn async_sql() -> String {
    return __terrane_raised(
        insert_sql(),
        0 /* terrane-site: src/main.trn:9:10-9:21 */,
    );
}
fn main() {
    __terrane_run(async move {
        let options: ConnectionOptions;
        let mut connection: Connection;
        let sql: String;
        let computed: String;
        let result: terrane_query_witness::StoreResult;
        let borrowed_result: terrane_query_witness::StoreResult;
        options = __terrane_raised(
            terrane_static_trn_436f6e6e656374696f6e4f7074696f6e73_new(),
            1 /* terrane-site: src/main.trn:12:13-12:36 */,
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
                            2 /* terrane-site: src/main.trn:13:22-13:38 */,
                        )
                    }
                })
                .await,
            2 /* terrane-site: src/main.trn:13:22-13:38 */,
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(operation_len(&__terrane_await(async_sql())
            . await), 3 /* terrane-site: src/main.trn:14:11-14:44 */))
        );
        sql = __terrane_await(async_sql()).await;
        computed = {
            let mut __terrane_text = String::with_capacity(0);
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &sql,
                &mut __terrane_text,
            );
            __terrane_text.push_str("");
            __terrane_text
        };
        result = __terrane_traced(
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
                                            > { Ok(terrane_query_witness::query(&computed)) }),
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
                                4 /* terrane-site: src/main.trn:17:20-17:35 */,
                            );
                            let __terrane_conversion_input_0 = Vec::<
                                u8,
                            >::from([111, 107]);
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
                                5 /* terrane-site: src/main.trn:17:19-17:48 */,
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
                            6 /* terrane-site: src/main.trn:17:18-17:69 */,
                        )
                    }
                })
                .await,
            6 /* terrane-site: src/main.trn:17:18-17:69 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&result).rows_affected()) })) {
            Ok(result) => result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::StoreResult::rows_affected")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 7 /* terrane-site: src/main.trn:18:11-18:32 */))
        );
        borrowed_result = __terrane_traced(
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
                                            > { Ok(terrane_query_witness::query(&*lend(&sql))) }),
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
                                8 /* terrane-site: src/main.trn:19:29-19:47 */,
                            );
                            let __terrane_conversion_input_0 = Vec::<
                                u8,
                            >::from([114, 101, 102]);
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
                                9 /* terrane-site: src/main.trn:19:28-19:61 */,
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
                            10 /* terrane-site: src/main.trn:19:27-19:82 */,
                        )
                    }
                })
                .await,
            10 /* terrane-site: src/main.trn:19:27-19:82 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&borrowed_result).rows_affected()) })) {
            Ok(result) => result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::StoreResult::rows_affected")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 11 /* terrane-site: src/main.trn:20:11-20:41 */))
        );
        println!("{}", terrane_scalar_support::scalar_text(&computed));
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
                            12 /* terrane-site: src/main.trn:22:9-22:26 */,
                        )
                    }
                })
                .await,
            12 /* terrane-site: src/main.trn:22:9-22:26 */,
        );
    });
}
