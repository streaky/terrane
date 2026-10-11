// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    __terrane_run(async move {
        let path: String;
        let mut missing_options: SqliteConnectOptions;
        let mut missing_connection: Option<SqliteConnection> = None;
        let mut create_options: SqliteConnectOptions;
        let connection: SqliteConnection;
        path = __terrane_raised(
            prepare_missing_path(),
            0 /* terrane-site: src/main.trn:8:10-8:31 */,
        );
        missing_options = __terrane_raised(
            terrane_static_trn_53716c697465436f6e6e6563744f7074696f6e73_new(),
            1 /* terrane-site: src/main.trn:9:21-9:47 */,
        );
        missing_options = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| missing_options.filename(path.as_str())),
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
            2 /* terrane-site: src/main.trn:10:21-10:51 */,
        );
        missing_options = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| missing_options.create_if_missing(false)),
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
            3 /* terrane-site: src/main.trn:11:21-11:61 */,
        );
        let __terrane_completion_0: TerraneCompletion<()> = async {
            let __terrane_try_0: TerraneCompletion<()> = async {
                let _ = missing_connection
                    .insert(
                        __terrane_traced_completion!(
                            __terrane_await({ let __terrane_future = { let __terrane_call
                            = < sqlx_sqlite::SqliteConnectOptions as
                            sqlx_core::connection::ConnectOptions
                            >::connect(&missing_options); async move { match crate
                            ::__terrane_dependency_await_unwind(__terrane_call). await {
                            Ok(Ok(value)) => Ok(value), Ok(Err(error)) => Err(crate
                            ::TerraneForeignError(crate
                            ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                            .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                            Err(crate ::__terrane_dependency_panic(payload,
                            "sqlx_sqlite", "sqlx_sqlite::SqliteConnectOptions::connect"))
                            } } }; async move { __terrane_raised_err(__terrane_future.
                            await, 4 /* terrane-site: src/main.trn:13:32-13:56 */) }
                            }). await, 4 /* terrane-site: src/main.trn:13:32-13:56 */
                        ),
                    );
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&String::from("unexpected-open"))
                );
                __terrane_traced_completion!(
                    __terrane_await({ let __terrane_future = { let __terrane_call = <
                    sqlx_sqlite::SqliteConnection as sqlx_core::connection::Connection
                    >::close(missing_connection.take()
                    .expect("flow-proven availability")); async move { match crate
                    ::__terrane_dependency_await_unwind(__terrane_call). await {
                    Ok(Ok(value)) => Ok(value), Ok(Err(error)) => Err(crate
                    ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
                    ::DescriptorId(2), error.to_string(), crate ::TERRANE_NO_SITE))),
                    Err(payload) => Err(crate ::__terrane_dependency_panic(payload,
                    "sqlx_sqlite", "sqlx_sqlite::SqliteConnection::close")) } } }; async
                    move { __terrane_raised_err(__terrane_future. await,
                    5 /* terrane-site: src/main.trn:15:11-15:36 */) } }). await,
                    5 /* terrane-site: src/main.trn:15:11-15:36 */
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
                        println!(
                            "{}",
                            terrane_scalar_support::scalar_text(&String::from("missing"))
                        );
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
        create_options = __terrane_raised(
            terrane_static_trn_53716c697465436f6e6e6563744f7074696f6e73_new(),
            6 /* terrane-site: src/main.trn:18:20-18:46 */,
        );
        create_options = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| create_options.filename(path.as_str())),
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
            7 /* terrane-site: src/main.trn:19:20-19:49 */,
        );
        create_options = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| create_options.create_if_missing(true)),
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
            8 /* terrane-site: src/main.trn:20:20-20:58 */,
        );
        connection = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = <sqlx_sqlite::SqliteConnectOptions as sqlx_core::connection::ConnectOptions>::connect(
                            &create_options,
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
                            9 /* terrane-site: src/main.trn:21:22-21:45 */,
                        )
                    }
                })
                .await,
            9 /* terrane-site: src/main.trn:21:22-21:45 */,
        );
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
                            10 /* terrane-site: src/main.trn:22:9-22:26 */,
                        )
                    }
                })
                .await,
            10 /* terrane-site: src/main.trn:22:9-22:26 */,
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&String::from("created-and-closed"))
        );
    });
}
