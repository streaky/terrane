// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn query_text() -> SqlStr {
    return ::sqlx::SqlSafeStr::into_sql_str(::sqlx::AssertSqlSafe("SELECT 77"));
}
fn row_value(record: &SqliteRow) -> i64 {
    return ::sqlx::Row::try_get::<i64, _>(record, 0_usize)
        .expect("SELECT 77 has one integer column");
}
async fn read_borrowed(native: &mut SqliteConnection) -> Result<i64, TerraneError> {
    let arguments: SqliteArguments;
    let sql: SqlStr;
    let record: SqliteRow;
    arguments = __terrane_raised_err(
        terrane_static_trn_53716c697465417267756d656e7473_default(),
        0 /* terrane-site: src/main.trn:16:17-16:43 */,
    )?;
    sql = query_text();
    record = __terrane_traced_err(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = sqlx_core::query::query_with(sql, arguments)
                        .fetch_one(&mut *native);
                    Box::pin(async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
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
                    })
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        1 /* terrane-site: src/main.trn:18:31-18:84 */,
                    )
                }
            })
            .await,
        1 /* terrane-site: src/main.trn:18:31-18:84 */,
    )?;
    return Ok(row_value(&record));
}
fn main() {
    __terrane_run(async move {
        let mut options: SqliteConnectOptions;
        let mut native: Option<SqliteConnection> = None;
        let mut arguments: Option<SqliteArguments> = None;
        let mut sql: Option<SqlStr> = None;
        let mut record: Option<SqliteRow> = None;
        options = __terrane_raised(
            terrane_static_trn_53716c697465436f6e6e6563744f7074696f6e73_new(),
            2 /* terrane-site: src/main.trn:22:15-22:35 */,
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
            3 /* terrane-site: src/main.trn:23:15-23:43 */,
        );
        let mut __terrane_finally_guard_0 = __terrane_finally_guard();
        let __terrane_maybe_completion_0: Option<TerraneCompletion<()>> = __terrane_cancel_operation(
                &__terrane_finally_guard_0,
                async {
                    let __terrane_try_0: TerraneCompletion<()> = async {
                        let _ = native
                            .insert(
                                __terrane_traced_completion!(
                                    __terrane_await({ let __terrane_future = { let
                                    __terrane_call = < sqlx_sqlite::SqliteConnectOptions as
                                    sqlx_core::connection::ConnectOptions > ::connect(&options);
                                    Box::pin(async move { match crate
                                    ::__terrane_dependency_await_unwind(__terrane_call). await {
                                    Ok(Ok(value)) => Ok(value), Ok(Err(error)) => Err(crate
                                    ::TerraneForeignError(crate
                                    ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                                    .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                                    Err(crate ::__terrane_dependency_panic(payload,
                                    "sqlx_sqlite",
                                    "sqlx_sqlite::SqliteConnectOptions::connect")) } }) }; async
                                    move { __terrane_raised_err(__terrane_future. await,
                                    4 /* terrane-site: src/main.trn:25:42-25:58 */) } }).
                                    await, 4 /* terrane-site: src/main.trn:25:42-25:58 */
                                ),
                            );
                        let mut __terrane_finally_guard_1 = __terrane_finally_guard();
                        let __terrane_maybe_completion_1: Option<
                            TerraneCompletion<()>,
                        > = __terrane_cancel_operation(
                                &__terrane_finally_guard_1,
                                async {
                                    let __terrane_try_1: TerraneCompletion<()> = async {
                                        let _ = arguments
                                            .insert(
                                                __terrane_raised_completion!(
                                                    terrane_static_trn_53716c697465417267756d656e7473_default(),
                                                    5 /* terrane-site: src/main.trn:27:25-27:51 */
                                                ),
                                            );
                                        let _ = sql.insert(query_text());
                                        let _ = record
                                            .insert(
                                                __terrane_traced_completion!(
                                                    __terrane_await({ let __terrane_future = { let
                                                    __terrane_call = sqlx_core::query::query_with(sql.as_ref()
                                                    .expect("flow-proven available binding").clone(), arguments
                                                    .take().expect("flow-proven availability")).fetch_one(native
                                                    .as_mut().expect("flow-proven available binding"));
                                                    Box::pin(async move { match crate
                                                    ::__terrane_dependency_await_unwind(__terrane_call). await {
                                                    Ok(Ok(value)) => Ok(value), Ok(Err(error)) => Err(crate
                                                    ::TerraneForeignError(crate
                                                    ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                                                    .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                                                    Err(crate ::__terrane_dependency_panic(payload, "sqlx_core",
                                                    "sqlx_core::query::Query::fetch_one")) } }) }; async move {
                                                    __terrane_raised_err(__terrane_future. await,
                                                    6 /* terrane-site: src/main.trn:29:39-29:92 */) } }).
                                                    await, 6 /* terrane-site: src/main.trn:29:39-29:92 */
                                                ),
                                            );
                                        println!(
                                            "{}", terrane_scalar_support::scalar_text(&row_value(record
                                            .as_ref().expect("flow-proven available binding")))
                                        );
                                        println!(
                                            "{}",
                                            terrane_scalar_support::scalar_text(&__terrane_traced_completion!(__terrane_await(read_borrowed(native
                                            .as_mut().expect("flow-proven available binding"))). await,
                                            7 /* terrane-site: src/main.trn:31:26-31:47 */))
                                        );
                                        TerraneCompletion::Normal
                                    }
                                        .await;
                                    match __terrane_try_1 {
                                        TerraneCompletion::Return(value) => {
                                            return TerraneCompletion::Return(value);
                                        }
                                        TerraneCompletion::Break => return TerraneCompletion::Break,
                                        TerraneCompletion::Continue => {
                                            return TerraneCompletion::Continue;
                                        }
                                        TerraneCompletion::Normal => {}
                                        TerraneCompletion::Error(__terrane_error_1) => {
                                            let mut __terrane_handled_1 = false;
                                            if !__terrane_handled_1 {
                                                return TerraneCompletion::Error(__terrane_error_1);
                                            }
                                        }
                                    }
                                    TerraneCompletion::Normal
                                },
                            )
                            .await;
                        let __terrane_cancelled_1 = __terrane_maybe_completion_1
                            .is_none();
                        let mut __terrane_completion_1 = __terrane_maybe_completion_1
                            .unwrap_or(TerraneCompletion::Normal);
                        let __terrane_finally_1: TerraneCompletion<()> = async {
                            __terrane_traced_completion!(
                                __terrane_await({ let __terrane_future = { let
                                __terrane_call = < sqlx_sqlite::SqliteConnection as
                                sqlx_core::connection::Connection > ::close(native.take()
                                .expect("flow-proven availability")); Box::pin(async move {
                                match crate
                                ::__terrane_dependency_await_unwind(__terrane_call). await {
                                Ok(Ok(value)) => Ok(value), Ok(Err(error)) => Err(crate
                                ::TerraneForeignError(crate
                                ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                                .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                                Err(crate ::__terrane_dependency_panic(payload,
                                "sqlx_sqlite", "sqlx_sqlite::SqliteConnection::close")) } })
                                }; async move { __terrane_raised_err(__terrane_future.
                                await, 8 /* terrane-site: src/main.trn:33:19-33:32 */) }
                                }). await, 8 /* terrane-site: src/main.trn:33:19-33:32 */
                            );
                            TerraneCompletion::Normal
                        }
                            .await;
                        match __terrane_finally_1 {
                            TerraneCompletion::Normal => {}
                            replacement => __terrane_completion_1 = replacement,
                        }
                        if __terrane_cancelled_1
                            && matches!(
                                &__terrane_completion_1, TerraneCompletion::Normal
                            )
                        {
                            __terrane_finish_cancelled_finally(__terrane_finally_guard_1)
                                .await;
                        }
                        __terrane_finally_guard_1.finish();
                        match __terrane_completion_1 {
                            TerraneCompletion::Normal => {}
                            TerraneCompletion::Return(value) => {
                                return TerraneCompletion::Return(value);
                            }
                            TerraneCompletion::Error(error) => {
                                return TerraneCompletion::Error(error);
                            }
                            TerraneCompletion::Break => return TerraneCompletion::Break,
                            TerraneCompletion::Continue => {
                                return TerraneCompletion::Continue;
                            }
                        }
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
                            if !__terrane_handled_0 {
                                return TerraneCompletion::Error(__terrane_error_0);
                            }
                        }
                    }
                    TerraneCompletion::Normal
                },
            )
            .await;
        let __terrane_cancelled_0 = __terrane_maybe_completion_0.is_none();
        let mut __terrane_completion_0 = __terrane_maybe_completion_0
            .unwrap_or(TerraneCompletion::Normal);
        let __terrane_finally_0: TerraneCompletion<()> = (|| {
            println!("{}", String::from("closed"));
            TerraneCompletion::Normal
        })();
        match __terrane_finally_0 {
            TerraneCompletion::Normal => {}
            replacement => __terrane_completion_0 = replacement,
        }
        if __terrane_cancelled_0
            && matches!(&__terrane_completion_0, TerraneCompletion::Normal)
        {
            __terrane_finish_cancelled_finally(__terrane_finally_guard_0).await;
        }
        __terrane_finally_guard_0.finish();
        match __terrane_completion_0 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
    });
}
