// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    __terrane_run(async move {
        let address: String;
        let chosen: String;
        let listener: TcpListener;
        let client: TcpStream;
        let mut occupied: Option<TcpListener> = None;
        address = __terrane_raised(
            available_loopback_address(),
            0 /* terrane-site: src/main.trn:9:13-9:40 */,
        );
        chosen = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| terrane_static_witness::GenericFactory::open(
                    address,
                )),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "dependency",
                            "terrane_static_witness :: GenericFactory :: open",
                        ),
                    )
                }
            },
            1 /* terrane-site: src/main.trn:10:12-10:41 */,
        );
        listener = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = tokio::net::TcpListener::bind(
                            chosen.clone(),
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
                                                crate::TERRANE_DEPENDENCY_ERROR,
                                                format!(
                                                    "Rust dependency `dependency` member `tokio :: net :: TcpListener :: bind` failed: {error}"
                                                ),
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
                                            "tokio :: net :: TcpListener :: bind",
                                        ),
                                    )
                                }
                            }
                        }
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
                                                crate::TERRANE_DEPENDENCY_ERROR,
                                                format!(
                                                    "Rust dependency `dependency` member `tokio :: net :: TcpStream :: connect` failed: {error}"
                                                ),
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
                                            "tokio :: net :: TcpStream :: connect",
                                        ),
                                    )
                                }
                            }
                        }
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
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&client)
            .nodelay())) { Ok(Ok(value)) => Ok(value), Ok(Err(error)) => Err(crate
            ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
            ::TERRANE_DEPENDENCY_ERROR,
            format!("Rust dependency `tokio` member `tokio::net::TcpStream::nodelay` failed: {error}"),
            crate ::TERRANE_NO_SITE))), Err(payload) => Err(crate
            ::__terrane_dependency_panic(payload, "tokio",
            "tokio::net::TcpStream::nodelay")) }, 4 /* terrane-site: src/main.trn:13:11-13:26 */))
        );
        let __terrane_completion_0: TerraneCompletion<()> = async {
            let __terrane_try_0: TerraneCompletion<()> = async {
                let _ = occupied
                    .insert(
                        __terrane_traced_completion!(
                            __terrane_await({ let __terrane_future = { let __terrane_call
                            = tokio::net::TcpListener::bind(chosen); async move { match
                            crate ::__terrane_dependency_await_unwind(__terrane_call).
                            await { Ok(Ok(value)) => Ok(value), Ok(Err(error)) =>
                            Err(crate ::TerraneForeignError(crate
                            ::TerraneError::custom_raised(crate
                            ::TERRANE_DEPENDENCY_ERROR,
                            format!("Rust dependency `dependency` member `tokio :: net :: TcpListener :: bind` failed: {error}"),
                            crate ::TERRANE_NO_SITE))), Err(payload) => Err(crate
                            ::__terrane_dependency_panic(payload, "dependency",
                            "tokio :: net :: TcpListener :: bind")) } } }; async move {
                            __terrane_raised_err(__terrane_future. await,
                            5 /* terrane-site: src/main.trn:15:22-15:47 */) } }).
                            await, 5 /* terrane-site: src/main.trn:15:22-15:47 */
                        ),
                    );
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&__terrane_raised_completion!(match
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&occupied
                    .as_ref().expect("flow-proven available binding")).ttl())) {
                    Ok(Ok(value)) => Ok(terrane_int_support::Int::from_u128(value as
                    u128)), Ok(Err(error)) => Err(crate ::TerraneForeignError(crate
                    ::TerraneError::custom_raised(crate ::TERRANE_DEPENDENCY_ERROR,
                    format!("Rust dependency `tokio` member `tokio::net::TcpListener::ttl` failed: {error}"),
                    crate ::TERRANE_NO_SITE))), Err(payload) => Err(crate
                    ::__terrane_dependency_panic(payload, "tokio",
                    "tokio::net::TcpListener::ttl")) }, 6 /* terrane-site: src/main.trn:16:13-16:26 */))
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
                            == TerraneErrorKind::Custom(DescriptorId(0))
                    {
                        __terrane_handled_0 = true;
                        println!(
                            "{}",
                            terrane_scalar_support::scalar_text(&String::from("occupied"))
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
    });
}
