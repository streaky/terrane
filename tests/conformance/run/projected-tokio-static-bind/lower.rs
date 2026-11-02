// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    __terrane_run(async move {
        let address: String;
        let chosen: String;
        let listener: tokio::net::TcpListener;
        let client: tokio::net::TcpStream;
        let mut occupied: Option<tokio::net::TcpListener> = None;
        address = __terrane_raised(
            available_loopback_address(),
            0 /* terrane-site: src/main.trn:9:13-9:40 */,
        );
        chosen = {
            let __terrane_conversion_input_0 = address;
            __terrane_raised(
                || -> Result<_, crate::TerraneForeignError> {
                    {
                        let value = match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| -> Result<
                                _,
                                crate::TerraneForeignError,
                            > {
                                Ok(
                                    terrane_static_witness::GenericFactory::open(
                                        __terrane_conversion_input_0,
                                    ),
                                )
                            }),
                        ) {
                            Ok(result) => result?,
                            Err(payload) => {
                                return Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "terrane_static_witness",
                                        "terrane_static_witness::GenericFactory::open",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),
                1 /* terrane-site: src/main.trn:10:12-10:41 */,
            )
        };
        listener = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = tokio::net::TcpListener::bind(
                            chosen.clone(),
                        );
                        Box::pin(async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                                .map_err(|payload| crate::__terrane_dependency_panic(
                                    payload,
                                    "tokio",
                                    "tokio::net::TcpListener::bind",
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
                            2 /* terrane-site: src/main.trn:11:20-11:45 */,
                        )
                    }
                })
                .await,
            2 /* terrane-site: src/main.trn:11:20-11:45 */,
        );
        let _ = &listener;
        client = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = tokio::net::TcpStream::connect(
                            chosen.clone(),
                        );
                        Box::pin(async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                                .map_err(|payload| crate::__terrane_dependency_panic(
                                    payload,
                                    "tokio",
                                    "tokio::net::TcpStream::connect",
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
                            3 /* terrane-site: src/main.trn:12:18-12:44 */,
                        )
                    }
                })
                .await,
            3 /* terrane-site: src/main.trn:12:18-12:44 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { match match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&client).nodelay()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "tokio",
            "tokio::net::TcpStream::nodelay")) } { Ok(value) => Ok(value), Err(error) =>
            Err(crate ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
            ::DescriptorId(2), error.to_string(), crate ::TERRANE_NO_SITE))) } } (),
            4 /* terrane-site: src/main.trn:13:11-13:26 */))
        );
        let __terrane_completion_0: TerraneCompletion<()> = async {
            let __terrane_try_0: TerraneCompletion<()> = async {
                let _ = occupied
                    .insert(
                        __terrane_traced_completion!(
                            __terrane_await({ let __terrane_future = { let __terrane_call
                            = tokio::net::TcpListener::bind(chosen); Box::pin(async move
                            { match crate
                            ::__terrane_dependency_await_unwind(__terrane_call). await
                            .map_err(| payload | crate
                            ::__terrane_dependency_panic(payload, "tokio",
                            "tokio::net::TcpListener::bind")) ? { Ok(value) => Ok(value),
                            Err(error) => Err(crate ::TerraneForeignError(crate
                            ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                            .to_string(), crate ::TERRANE_NO_SITE))) } }) }; async move {
                            __terrane_raised_err(__terrane_future. await,
                            5 /* terrane-site: src/main.trn:15:22-15:47 */) } }).
                            await, 5 /* terrane-site: src/main.trn:15:22-15:47 */
                        ),
                    );
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&__terrane_raised_completion!(| |
                    -> Result < _, crate ::TerraneForeignError > { match match
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result <
                    _, crate ::TerraneForeignError > { Ok((&occupied.as_ref()
                    .expect("flow-proven available binding")).ttl()) })) { Ok(result) =>
                    result ?, Err(payload) => return Err(crate
                    ::__terrane_dependency_panic(payload, "tokio",
                    "tokio::net::TcpListener::ttl")) } { Ok(value) =>
                    Ok(terrane_int_support::Int::from_u128(value as u128)), Err(error) =>
                    Err(crate ::TerraneForeignError(crate
                    ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                    .to_string(), crate ::TERRANE_NO_SITE))) } } (),
                    6 /* terrane-site: src/main.trn:16:13-16:26 */))
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
                        println!("{}", String::from("occupied"));
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
    });
}
