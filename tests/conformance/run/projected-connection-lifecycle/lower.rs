// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    __terrane_run(async move {
        let path: String;
        let mut missing_options: ConnectionOptions;
        let mut missing_connection: Option<Connection> = None;
        let mut create_options: ConnectionOptions;
        let connection: Connection;
        path = String::from("fixture.db");
        missing_options = __terrane_raised(
            terrane_static_trn_436f6e6e656374696f6e4f7074696f6e73_new(),
            0 /* terrane-site: src/main.trn:7:21-7:44 */,
        );
        missing_options = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| missing_options.filename(path.clone())),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "terrane_lifecycle_witness",
                            "terrane_lifecycle_witness::ConnectionOptions::filename",
                        ),
                    )
                }
            },
            1 /* terrane-site: src/main.trn:8:21-8:51 */,
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
                            "terrane_lifecycle_witness",
                            "terrane_lifecycle_witness::ConnectionOptions::create_if_missing",
                        ),
                    )
                }
            },
            2 /* terrane-site: src/main.trn:9:21-9:61 */,
        );
        let __terrane_completion_0: TerraneCompletion<()> = async {
            let __terrane_try_0: TerraneCompletion<()> = async {
                let _ = missing_connection
                    .insert(
                        __terrane_traced_completion!(
                            __terrane_await({ let __terrane_future = { let __terrane_call
                            = (&missing_options).connect(); Box::pin(async move { match
                            crate ::__terrane_dependency_await_unwind(__terrane_call).
                            await { Ok(Ok(value)) => Ok(value), Ok(Err(error)) =>
                            Err(crate ::TerraneForeignError(crate
                            ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                            .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                            Err(crate ::__terrane_dependency_panic(payload,
                            "terrane_lifecycle_witness",
                            "terrane_lifecycle_witness::ConnectionOptions::connect")) }
                            }) }; async move { __terrane_raised_err(__terrane_future.
                            await, 3 /* terrane-site: src/main.trn:11:32-11:56 */) }
                            }). await, 3 /* terrane-site: src/main.trn:11:32-11:56 */
                        ),
                    );
                println!("{}", String::from("unexpected-open"));
                __terrane_traced_completion!(
                    __terrane_await({ let __terrane_future = { let __terrane_call =
                    missing_connection.take().expect("flow-proven availability").close();
                    Box::pin(async move { match crate
                    ::__terrane_dependency_await_unwind(__terrane_call). await {
                    Ok(Ok(value)) => Ok(value), Ok(Err(error)) => Err(crate
                    ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
                    ::DescriptorId(2), error.to_string(), crate ::TERRANE_NO_SITE))),
                    Err(payload) => Err(crate ::__terrane_dependency_panic(payload,
                    "terrane_lifecycle_witness",
                    "terrane_lifecycle_witness::Connection::close")) } }) }; async move {
                    __terrane_raised_err(__terrane_future. await, 4 /* terrane-site: src/main.trn:13:11-13:36 */) } }). await, 4 /* terrane-site: src/main.trn:13:11-13:36 */
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
                        println!("{}", String::from("missing"));
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
            terrane_static_trn_436f6e6e656374696f6e4f7074696f6e73_new(),
            5 /* terrane-site: src/main.trn:16:20-16:43 */,
        );
        create_options = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| create_options.filename(path)),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "terrane_lifecycle_witness",
                            "terrane_lifecycle_witness::ConnectionOptions::filename",
                        ),
                    )
                }
            },
            6 /* terrane-site: src/main.trn:17:20-17:49 */,
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
                            "terrane_lifecycle_witness",
                            "terrane_lifecycle_witness::ConnectionOptions::create_if_missing",
                        ),
                    )
                }
            },
            7 /* terrane-site: src/main.trn:18:20-18:58 */,
        );
        connection = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = (&create_options).connect();
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
                                            "terrane_lifecycle_witness",
                                            "terrane_lifecycle_witness::ConnectionOptions::connect",
                                        ),
                                    )
                                }
                            }
                        })
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            8 /* terrane-site: src/main.trn:19:22-19:45 */,
                        )
                    }
                })
                .await,
            8 /* terrane-site: src/main.trn:19:22-19:45 */,
        );
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
                                            "terrane_lifecycle_witness",
                                            "terrane_lifecycle_witness::Connection::close",
                                        ),
                                    )
                                }
                            }
                        })
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            9 /* terrane-site: src/main.trn:20:9-20:26 */,
                        )
                    }
                })
                .await,
            9 /* terrane-site: src/main.trn:20:9-20:26 */,
        );
        println!("{}", String::from("created-and-closed"));
    });
}
