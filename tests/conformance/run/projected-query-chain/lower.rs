// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
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
            0 /* terrane-site: src/main.trn:7:13-7:36 */,
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
                            1 /* terrane-site: src/main.trn:8:22-8:38 */,
                        )
                    }
                })
                .await,
            1 /* terrane-site: src/main.trn:8:22-8:38 */,
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
                            2 /* terrane-site: src/main.trn:13:9-13:68 */,
                        )
                    }
                })
                .await,
            2 /* terrane-site: src/main.trn:13:9-13:68 */,
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
                            3 /* terrane-site: src/main.trn:14:9-14:69 */,
                        )
                    }
                })
                .await,
            3 /* terrane-site: src/main.trn:14:9-14:69 */,
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
                            4 /* terrane-site: src/main.trn:15:28-15:71 */,
                        )
                    }
                })
                .await,
            4 /* terrane-site: src/main.trn:15:28-15:71 */,
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
                            5 /* terrane-site: src/main.trn:17:19-17:62 */,
                        )
                    }
                })
                .await,
            5 /* terrane-site: src/main.trn:17:19-17:62 */,
        );
        body = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| <terrane_query_witness::Row as terrane_query_contract::RowAccess>::try_get(
                    &row,
                    String::from("body").as_str(),
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
                            "dependency",
                            "< terrane_query_witness :: Row as terrane_query_contract :: RowAccess > :: try_get",
                        ),
                    )
                }
            },
            6 /* terrane-site: src/main.trn:18:16-18:41 */,
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::byte_at(&body,
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            7 /* terrane-site: src/main.trn:19:10-19:17 */)), 7 /* terrane-site: src/main.trn:19:10-19:17 */))
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
                            8 /* terrane-site: src/main.trn:20:26-20:70 */,
                        )
                    }
                })
                .await,
            8 /* terrane-site: src/main.trn:20:26-20:70 */,
        );
        second_body = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| <terrane_query_witness::Row as terrane_query_contract::RowAccess>::try_get(
                    &second_row,
                    String::from("body").as_str(),
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
                            "dependency",
                            "< terrane_query_witness :: Row as terrane_query_contract :: RowAccess > :: try_get",
                        ),
                    )
                }
            },
            9 /* terrane-site: src/main.trn:21:23-21:55 */,
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::byte_at(&second_body,
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            10 /* terrane-site: src/main.trn:22:10-22:24 */)), 10 /* terrane-site: src/main.trn:22:10-22:24 */))
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
                    11 /* terrane-site: src/main.trn:24:11-24:54 */) } }). await,
                    11 /* terrane-site: src/main.trn:24:11-24:54 */
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
                            12 /* terrane-site: src/main.trn:27:9-27:26 */,
                        )
                    }
                })
                .await,
            12 /* terrane-site: src/main.trn:27:9-27:26 */,
        );
        println!("{}", String::from("complete"));
    });
}
