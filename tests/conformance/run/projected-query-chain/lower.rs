// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[allow(dead_code)]
fn trigger_static_panic(record: Row) -> Result<Vec<u8>, TerraneError> {
    return Ok(
        __terrane_raised_err(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| <terrane_query_witness::Row as terrane_query_contract::RowAccess>::try_get(
                    &record,
                    "panic",
                )),
            ) {
                Ok(Ok(value)) => Ok(value),
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
                            "terrane_query_witness",
                            "terrane_query_witness::Row::try_get",
                        ),
                    )
                }
            },
            0 /* terrane-site: src/main.trn:8:10-8:39 */,
        )?,
    );
}
#[allow(dead_code)]
fn trigger_instance_panic(record: Row) -> Result<Vec<u8>, TerraneError> {
    return Ok(
        __terrane_raised_err(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| <terrane_query_witness::Row as terrane_query_contract::RowAccess>::try_get(
                    &record,
                    "panic",
                )),
            ) {
                Ok(Ok(value)) => Ok(value),
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
                            "terrane_query_witness",
                            "terrane_query_witness::Row::try_get",
                        ),
                    )
                }
            },
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
                            {
                                Ok(Ok(value)) => Ok(value),
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
                                            "terrane_query_witness",
                                            "terrane_query_witness::ConnectionOptions::connect",
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
        insert_op = String::from("insert");
        first_op = String::from("select");
        second_op = String::from("select-second");
        invalid_op = String::from("invalid");
        __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = terrane_query_witness::query(&insert_op)
                            .bind(Vec::<u8>::from([102, 105, 114, 115, 116]))
                            .execute(&mut connection);
                        Box::pin(async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                            {
                                Ok(Ok(value)) => Ok(value),
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
                                            "terrane_query_witness",
                                            "terrane_query_witness::Query::execute",
                                        ),
                                    )
                                }
                            }
                        })
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            4 /* terrane-site: src/main.trn:20:9-20:64 */,
                        )
                    }
                })
                .await,
            4 /* terrane-site: src/main.trn:20:9-20:64 */,
        );
        __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = terrane_query_witness::query(&insert_op)
                            .bind(Vec::<u8>::from([115, 101, 99, 111, 110, 100]))
                            .execute(&mut connection);
                        Box::pin(async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                            {
                                Ok(Ok(value)) => Ok(value),
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
                                            "terrane_query_witness",
                                            "terrane_query_witness::Query::execute",
                                        ),
                                    )
                                }
                            }
                        })
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            5 /* terrane-site: src/main.trn:21:9-21:65 */,
                        )
                    }
                })
                .await,
            5 /* terrane-site: src/main.trn:21:9-21:65 */,
        );
        rows = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = terrane_query_witness::query(&first_op)
                            .fetch_all(&mut connection);
                        Box::pin(async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                            {
                                Ok(Ok(value)) => {
                                    Ok(terrane_collection_support::List::new(value))
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
                                            "terrane_query_witness",
                                            "terrane_query_witness::Query::fetch_all",
                                        ),
                                    )
                                }
                            }
                        })
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            6 /* terrane-site: src/main.trn:22:28-22:67 */,
                        )
                    }
                })
                .await,
            6 /* terrane-site: src/main.trn:22:28-22:67 */,
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(rows
            .length()))
        );
        row = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = terrane_query_witness::query(&first_op)
                            .fetch_one(&mut connection);
                        Box::pin(async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                            {
                                Ok(Ok(value)) => Ok(value),
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
                                            "terrane_query_witness",
                                            "terrane_query_witness::Query::fetch_one",
                                        ),
                                    )
                                }
                            }
                        })
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            7 /* terrane-site: src/main.trn:24:19-24:58 */,
                        )
                    }
                })
                .await,
            7 /* terrane-site: src/main.trn:24:19-24:58 */,
        );
        body = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| <terrane_query_witness::Row as terrane_query_contract::RowAccess>::try_get(
                    &row,
                    "body",
                )),
            ) {
                Ok(Ok(value)) => Ok(value),
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
                            "terrane_query_witness",
                            "terrane_query_witness::Row::try_get",
                        ),
                    )
                }
            },
            8 /* terrane-site: src/main.trn:25:16-25:41 */,
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::byte_at(&body,
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            9 /* terrane-site: src/main.trn:26:10-26:17 */)), 9 /* terrane-site: src/main.trn:26:10-26:17 */))
        );
        second_row = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = terrane_query_witness::query(&second_op)
                            .fetch_one(&mut connection);
                        Box::pin(async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                            {
                                Ok(Ok(value)) => Ok(value),
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
                                            "terrane_query_witness",
                                            "terrane_query_witness::Query::fetch_one",
                                        ),
                                    )
                                }
                            }
                        })
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            10 /* terrane-site: src/main.trn:27:26-27:66 */,
                        )
                    }
                })
                .await,
            10 /* terrane-site: src/main.trn:27:26-27:66 */,
        );
        second_body = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| <terrane_query_witness::Row as terrane_query_contract::RowAccess>::try_get(
                    &second_row,
                    "body",
                )),
            ) {
                Ok(Ok(value)) => Ok(value),
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
                            "terrane_query_witness",
                            "terrane_query_witness::Row::try_get",
                        ),
                    )
                }
            },
            11 /* terrane-site: src/main.trn:28:23-28:49 */,
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::byte_at(&second_body,
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            12 /* terrane-site: src/main.trn:29:10-29:24 */)), 12 /* terrane-site: src/main.trn:29:10-29:24 */))
        );
        let __terrane_completion_0: TerraneCompletion<()> = async {
            let __terrane_try_0: TerraneCompletion<()> = async {
                __terrane_traced_completion!(
                    __terrane_await({ let __terrane_future = { let __terrane_call =
                    terrane_query_witness::query(&invalid_op).execute(&mut connection);
                    Box::pin(async move { match crate
                    ::__terrane_dependency_await_unwind(__terrane_call). await {
                    Ok(Ok(value)) => Ok(value), Ok(Err(error)) => Err(crate
                    ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
                    ::DescriptorId(2), error.to_string(), crate ::TERRANE_NO_SITE))),
                    Err(payload) => Err(crate ::__terrane_dependency_panic(payload,
                    "terrane_query_witness", "terrane_query_witness::Query::execute")) }
                    }) }; async move { __terrane_raised_err(__terrane_future. await,
                    13 /* terrane-site: src/main.trn:31:11-31:50 */) } }). await,
                    13 /* terrane-site: src/main.trn:31:11-31:50 */
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
                            {
                                Ok(Ok(value)) => Ok(value),
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
                                            "terrane_query_witness",
                                            "terrane_query_witness::Connection::close",
                                        ),
                                    )
                                }
                            }
                        })
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            14 /* terrane-site: src/main.trn:34:9-34:26 */,
                        )
                    }
                })
                .await,
            14 /* terrane-site: src/main.trn:34:9-34:26 */,
        );
        println!("{}", String::from("complete"));
    });
}
