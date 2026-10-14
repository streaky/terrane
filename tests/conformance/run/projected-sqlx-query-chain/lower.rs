// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    __terrane_run(async move {
        let mut options: SqliteConnectOptions;
        let mut connection: SqliteConnection;
        let create_sql: String;
        let insert_sql: String;
        let select_sql: String;
        let second_sql: String;
        let invalid_sql: String;
        let rows: terrane_collection_support::List<SqliteRow>;
        let row: SqliteRow;
        let body: Vec<u8>;
        let second_row: SqliteRow;
        let second_body: Vec<u8>;
        options = __terrane_raised(
            terrane_static_trn_53716c697465436f6e6e6563744f7074696f6e73_new(),
            0 /* terrane-site: src/main.trn:9:13-9:39 */,
        );
        options = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    options.filename(String::from(":memory:").as_str())
                }),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "sqlx_sqlite",
                            "sqlx_sqlite::SqliteConnectOptions::filename",
                        ),
                    )
                }
            },
            1 /* terrane-site: src/main.trn:10:13-10:41 */,
        );
        options = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| options.create_if_missing(true)),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "sqlx_sqlite",
                            "sqlx_sqlite::SqliteConnectOptions::create_if_missing",
                        ),
                    )
                }
            },
            2 /* terrane-site: src/main.trn:11:13-11:44 */,
        );
        connection = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = <sqlx_sqlite::SqliteConnectOptions as sqlx_core::connection::ConnectOptions>::connect(
                            &options,
                        );
                        async move {
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
                                            "sqlx_sqlite",
                                            "sqlx_sqlite::SqliteConnectOptions::connect",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            3 /* terrane-site: src/main.trn:12:22-12:38 */,
                        )
                    }
                })
                .await,
            3 /* terrane-site: src/main.trn:12:22-12:38 */,
        );
        create_sql = String::from("CREATE TABLE records (body BLOB NOT NULL)");
        insert_sql = String::from("INSERT INTO records (body) VALUES (?)");
        select_sql = String::from("SELECT body FROM records ORDER BY body");
        second_sql = String::from(
            "SELECT body FROM records ORDER BY body LIMIT 1 OFFSET 1",
        );
        invalid_sql = String::from("SELECT missing FROM absent");
        __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = sqlx_core::query::query(&create_sql)
                            .execute(&mut connection);
                        async move {
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
                                            "sqlx_core",
                                            "sqlx_core::query::Query::execute",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            4 /* terrane-site: src/main.trn:18:9-18:52 */,
                        )
                    }
                })
                .await,
            4 /* terrane-site: src/main.trn:18:9-18:52 */,
        );
        __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = sqlx_core::query::query(&insert_sql)
                            .bind(Vec::<u8>::from([102, 105, 114, 115, 116]))
                            .execute(&mut connection);
                        async move {
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
                                            "sqlx_core",
                                            "sqlx_core::query::Query::execute",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            5 /* terrane-site: src/main.trn:19:9-19:69 */,
                        )
                    }
                })
                .await,
            5 /* terrane-site: src/main.trn:19:9-19:69 */,
        );
        __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = sqlx_core::query::query(&insert_sql)
                            .bind(Vec::<u8>::from([115, 101, 99, 111, 110, 100]))
                            .execute(&mut connection);
                        async move {
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
                                            "sqlx_core",
                                            "sqlx_core::query::Query::execute",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            6 /* terrane-site: src/main.trn:20:9-20:70 */,
                        )
                    }
                })
                .await,
            6 /* terrane-site: src/main.trn:20:9-20:70 */,
        );
        rows = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = sqlx_core::query::query(&select_sql)
                            .fetch_all(&mut connection);
                        async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                            {
                                Ok(Ok(value)) => {
                                    Ok(
                                        terrane_collection_support::List::new(
                                            value.into_iter().map(|item| item).collect(),
                                        ),
                                    )
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
                                            "sqlx_core",
                                            "sqlx_core::query::Query::fetch_all",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            7 /* terrane-site: src/main.trn:21:34-21:79 */,
                        )
                    }
                })
                .await,
            7 /* terrane-site: src/main.trn:21:34-21:79 */,
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(rows
            .length()))
        );
        row = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = sqlx_core::query::query(&select_sql)
                            .fetch_one(&mut connection);
                        async move {
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
                                            "sqlx_core",
                                            "sqlx_core::query::Query::fetch_one",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            8 /* terrane-site: src/main.trn:23:25-23:70 */,
                        )
                    }
                })
                .await,
            8 /* terrane-site: src/main.trn:23:25-23:70 */,
        );
        body = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| <sqlx_sqlite::SqliteRow as sqlx_core::row::Row>::try_get(
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
                            "< sqlx_sqlite :: SqliteRow as sqlx_core :: row :: Row > :: try_get",
                        ),
                    )
                }
            },
            9 /* terrane-site: src/main.trn:24:16-24:47 */,
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::byte_at(&body,
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            10 /* terrane-site: src/main.trn:25:10-25:17 */)), 10 /* terrane-site: src/main.trn:25:10-25:17 */))
        );
        second_row = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = sqlx_core::query::query(&second_sql)
                            .fetch_one(&mut connection);
                        async move {
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
                                            "sqlx_core",
                                            "sqlx_core::query::Query::fetch_one",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            11 /* terrane-site: src/main.trn:26:32-26:77 */,
                        )
                    }
                })
                .await,
            11 /* terrane-site: src/main.trn:26:32-26:77 */,
        );
        second_body = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| <sqlx_sqlite::SqliteRow as sqlx_core::row::Row>::try_get(
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
                            "< sqlx_sqlite :: SqliteRow as sqlx_core :: row :: Row > :: try_get",
                        ),
                    )
                }
            },
            12 /* terrane-site: src/main.trn:27:23-27:61 */,
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::byte_at(&second_body,
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            13 /* terrane-site: src/main.trn:28:10-28:24 */)), 13 /* terrane-site: src/main.trn:28:10-28:24 */))
        );
        let __terrane_completion_0: TerraneCompletion<()> = async {
            let __terrane_try_0: TerraneCompletion<()> = async {
                __terrane_traced_completion!(
                    __terrane_await({ let __terrane_future = { let __terrane_call =
                    sqlx_core::query::query(&invalid_sql).execute(&mut connection); async
                    move { match crate
                    ::__terrane_dependency_await_unwind(__terrane_call). await {
                    Ok(Ok(value)) => Ok(value), Ok(Err(error)) => Err(crate
                    ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
                    ::DescriptorId(2), error.to_string(), crate ::TERRANE_NO_SITE))),
                    Err(payload) => Err(crate ::__terrane_dependency_panic(payload,
                    "sqlx_core", "sqlx_core::query::Query::execute")) } } }; async move {
                    __terrane_raised_err(__terrane_future. await, 14 /* terrane-site: src/main.trn:30:11-30:55 */) } }). await, 14 /* terrane-site: src/main.trn:30:11-30:55 */
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
                        let __terrane_call = <sqlx_sqlite::SqliteConnection as sqlx_core::connection::Connection>::close(
                            connection,
                        );
                        async move {
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
                                            "sqlx_sqlite",
                                            "sqlx_sqlite::SqliteConnection::close",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            15 /* terrane-site: src/main.trn:33:9-33:26 */,
                        )
                    }
                })
                .await,
            15 /* terrane-site: src/main.trn:33:9-33:26 */,
        );
        println!("{}", String::from("complete"));
    });
}
