// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    __terrane_run(async move {
        let mut options: SqliteConnectOptions;
        let mut connection: SqliteConnection;
        let select_sql: String;
        let row: SqliteRow;
        let body: Vec<u8>;
        let mut missing: Option<Vec<u8>> = None;
        let mut failure: Option<TerraneError> = None;
        let mut incompatible: Option<Vec<u8>> = None;
        let mut failure_2: Option<TerraneError> = None;
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
        select_sql = String::from("select cast(char(1,2) as blob) as body, 42 as count");
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
                            4 /* terrane-site: src/main.trn:14:25-14:70 */,
                        )
                    }
                })
                .await,
            4 /* terrane-site: src/main.trn:14:25-14:70 */,
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
            5 /* terrane-site: src/main.trn:15:16-15:47 */,
        );
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&(body.len() as i128)),
            terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::byte_at(&body,
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            6 /* terrane-site: src/main.trn:16:23-16:30 */)), 6 /* terrane-site: src/main.trn:16:23-16:30 */)),
            terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::byte_at(&body,
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            7 /* terrane-site: src/main.trn:16:32-16:39 */)), 7 /* terrane-site: src/main.trn:16:32-16:39 */))
        );
        let __terrane_completion_0: TerraneCompletion<()> = (|| {
            let __terrane_try_0: TerraneCompletion<()> = (|| {
                let _ = missing
                    .insert(
                        __terrane_raised_completion!(
                            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|
                            | < sqlx_sqlite::SqliteRow as sqlx_core::row::Row >
                            ::try_get(&row, String::from("missing").as_str()))) {
                            Ok(Ok(value)) => Ok(value), Ok(Err(error)) => Err(crate
                            ::TerraneForeignError(crate
                            ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                            .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                            Err(crate ::__terrane_dependency_panic(payload, "dependency",
                            "< sqlx_sqlite :: SqliteRow as sqlx_core :: row :: Row > :: try_get"))
                            }, 8 /* terrane-site: src/main.trn:18:21-18:55 */
                        ),
                    );
                println!(
                    "{}", terrane_scalar_support::scalar_text(&(missing.as_ref()
                    .expect("flow-proven available binding").clone().len() as i128))
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
                            == TerraneErrorKind::Custom(DescriptorId(2))
                    {
                        __terrane_handled_0 = true;
                        let _ = failure.insert(__terrane_error_0.clone());
                        println!(
                            "{}", terrane_scalar_support::scalar_text(&failure.as_ref()
                            .expect("flow-proven available binding").clone().message()
                            .to_owned())
                        );
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
        let __terrane_completion_1: TerraneCompletion<()> = (|| {
            let __terrane_try_1: TerraneCompletion<()> = (|| {
                let _ = incompatible
                    .insert(
                        __terrane_raised_completion!(
                            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|
                            | < sqlx_sqlite::SqliteRow as sqlx_core::row::Row >
                            ::try_get(&row, String::from("count").as_str()))) {
                            Ok(Ok(value)) => Ok(value), Ok(Err(error)) => Err(crate
                            ::TerraneForeignError(crate
                            ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                            .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                            Err(crate ::__terrane_dependency_panic(payload, "dependency",
                            "< sqlx_sqlite :: SqliteRow as sqlx_core :: row :: Row > :: try_get"))
                            }, 9 /* terrane-site: src/main.trn:23:26-23:58 */
                        ),
                    );
                println!(
                    "{}", terrane_scalar_support::scalar_text(&(incompatible.as_ref()
                    .expect("flow-proven available binding").clone().len() as i128))
                );
                TerraneCompletion::Normal
            })();
            match __terrane_try_1 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_1) => {
                    let mut __terrane_handled_1 = false;
                    if !__terrane_handled_1
                        && __terrane_error_1.kind
                            == TerraneErrorKind::Custom(DescriptorId(2))
                    {
                        __terrane_handled_1 = true;
                        let _ = failure_2.insert(__terrane_error_1.clone());
                        println!(
                            "{}", terrane_scalar_support::scalar_text(&failure_2.as_ref()
                            .expect("flow-proven available binding").clone().message()
                            .to_owned())
                        );
                    }
                    if !__terrane_handled_1 {
                        return TerraneCompletion::Error(__terrane_error_1);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        match __terrane_completion_1 {
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
                            10 /* terrane-site: src/main.trn:27:9-27:26 */,
                        )
                    }
                })
                .await,
            10 /* terrane-site: src/main.trn:27:9-27:26 */,
        );
    });
}
