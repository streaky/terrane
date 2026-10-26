// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    __terrane_run(async move {
        let options: ConnectionOptions;
        let mut connection: Connection;
        let selector: String;
        let row: Row;
        let body: Vec<u8>;
        let mut missing: Option<Vec<u8>> = None;
        let mut failure: Option<TerraneError> = None;
        let mut incompatible: Option<Vec<u8>> = None;
        let mut failure_2: Option<TerraneError> = None;
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
        selector = String::from("first");
        row = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = terrane_query_witness::query(&selector)
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
                            2 /* terrane-site: src/main.trn:10:19-10:58 */,
                        )
                    }
                })
                .await,
            2 /* terrane-site: src/main.trn:10:19-10:58 */,
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
                            "terrane_query_witness",
                            "terrane_query_witness::Row::try_get",
                        ),
                    )
                }
            },
            3 /* terrane-site: src/main.trn:11:16-11:41 */,
        );
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&(body.len() as i128)),
            terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::byte_at(&body,
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            4 /* terrane-site: src/main.trn:12:23-12:30 */)), 4 /* terrane-site: src/main.trn:12:23-12:30 */)),
            terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::byte_at(&body,
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            5 /* terrane-site: src/main.trn:12:32-12:39 */)), 5 /* terrane-site: src/main.trn:12:32-12:39 */))
        );
        let __terrane_completion_0: TerraneCompletion<()> = (|| {
            let __terrane_try_0: TerraneCompletion<()> = (|| {
                let _ = missing
                    .insert(
                        __terrane_raised_completion!(
                            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|
                            | < terrane_query_witness::Row as
                            terrane_query_contract::RowAccess > ::try_get(&row,
                            String::from("missing").as_str()))) { Ok(Ok(value)) =>
                            Ok(value), Ok(Err(error)) => Err(crate
                            ::TerraneForeignError(crate
                            ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                            .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                            Err(crate ::__terrane_dependency_panic(payload,
                            "terrane_query_witness",
                            "terrane_query_witness::Row::try_get")) },
                            6 /* terrane-site: src/main.trn:14:21-14:49 */
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
                            | < terrane_query_witness::Row as
                            terrane_query_contract::RowAccess > ::try_get(&row,
                            String::from("count").as_str()))) { Ok(Ok(value)) =>
                            Ok(value), Ok(Err(error)) => Err(crate
                            ::TerraneForeignError(crate
                            ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                            .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                            Err(crate ::__terrane_dependency_panic(payload,
                            "terrane_query_witness",
                            "terrane_query_witness::Row::try_get")) },
                            7 /* terrane-site: src/main.trn:19:26-19:52 */
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
                            8 /* terrane-site: src/main.trn:23:9-23:26 */,
                        )
                    }
                })
                .await,
            8 /* terrane-site: src/main.trn:23:9-23:26 */,
        );
    });
}
