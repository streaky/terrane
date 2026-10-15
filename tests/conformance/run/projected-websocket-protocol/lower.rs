// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
async fn socket_session(mut socket: WebSocket) {
    let mut open: Option<bool> = None;
    let mut message: Option<Option<Message>> = None;
    let mut kind: Option<String> = None;
    let mut native_text: Option<Utf8Bytes> = None;
    let mut value: Option<String> = None;
    let mut payload: Option<Bytes> = None;
    let __terrane_completion_0: TerraneCompletion<()> = async {
        let __terrane_try_0: TerraneCompletion<()> = async {
            let _ = open.insert(true);
            while *open.as_ref().expect("flow-proven available binding") {
                let _ = message
                    .insert(
                        __terrane_traced_completion!(
                            __terrane_await({ let __terrane_future = { let __terrane_call
                            = (&mut socket).recv(); async move { match crate
                            ::__terrane_dependency_await_unwind(__terrane_call). await {
                            Ok(None) => Ok(None), Ok(Some(Ok(value))) => Ok(Some(value)),
                            Ok(Some(Err(error))) => Err(crate ::TerraneForeignError(crate
                            ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                            .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                            Err(crate ::__terrane_dependency_panic(payload, "axum",
                            "axum::extract::ws::WebSocket::recv")) } } }; async move {
                            __terrane_raised_err(__terrane_future. await,
                            0 /* terrane-site: src/main.trn:17:36-17:48 */) } }).
                            await, 0 /* terrane-site: src/main.trn:17:36-17:48 */
                        ),
                    );
                if message
                    .as_ref()
                    .expect("flow-proven available binding")
                    .clone()
                    .is_some()
                {
                    let _ = kind
                        .insert(
                            __terrane_raised_completion!(
                                match
                                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                                match &match message.as_ref()
                                .expect("flow-proven available binding") { Some(value) =>
                                value, _ => unreachable!("flow-proven storage refinement") }
                                { axum::extract::ws::Message::Text { .. } => "Text"
                                .to_owned(), axum::extract::ws::Message::Binary { .. } =>
                                "Binary".to_owned(), axum::extract::ws::Message::Ping { .. }
                                => "Ping".to_owned(), axum::extract::ws::Message::Pong { ..
                                } => "Pong".to_owned(), axum::extract::ws::Message::Close {
                                .. } => "Close".to_owned() })) { Ok(value) => Ok(value),
                                Err(payload) => Err(crate
                                ::__terrane_dependency_panic(payload, "axum",
                                "axum::extract::ws::Message::variant-name")) },
                                1 /* terrane-site: src/main.trn:19:23-19:44 */
                            ),
                        );
                    if kind
                        .as_ref()
                        .expect("flow-proven available binding")
                        .clone()
                        .as_str() == "Text"
                    {
                        let _ = native_text
                            .insert(
                                __terrane_raised_completion!(
                                    match
                                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                                    message.take().expect("flow-proven availability")
                                    .expect("semantic optional narrowing").into_text())) {
                                    Ok(Ok(value)) => Ok(value), Ok(Err(error)) => Err(crate
                                    ::TerraneForeignError(crate
                                    ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                                    .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                                    Err(crate ::__terrane_dependency_panic(payload, "axum",
                                    "axum::extract::ws::Message::into_text")) },
                                    2 /* terrane-site: src/main.trn:21:25-21:50 */
                                ),
                            );
                        let _ = value
                            .insert(
                                __terrane_raised_completion!(
                                    match
                                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                                    (&native_text.as_ref()
                                    .expect("flow-proven available binding")).as_str())) {
                                    Ok(value) => Ok(value.to_owned()), Err(payload) => Err(crate
                                    ::__terrane_dependency_panic(payload, "axum",
                                    "axum::extract::ws::Utf8Bytes::as_str")) },
                                    3 /* terrane-site: src/main.trn:22:26-22:45 */
                                ),
                            );
                        if value
                            .as_ref()
                            .expect("flow-proven available binding")
                            .clone()
                            .as_str() == "shutdown"
                        {
                            println!("{}", String::from("close-sent"));
                            __terrane_traced_completion!(
                                __terrane_await({ let __terrane_future = { let
                                __terrane_call = (&mut socket)
                                .send(__terrane_raised_completion!(match
                                std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                                axum::extract::ws::Message::Close(None))) { Ok(value) =>
                                Ok(value), Err(payload) => Err(crate
                                ::__terrane_dependency_panic(payload, "axum",
                                "axum::extract::ws::Message::Close")) },
                                4 /* terrane-site: src/main.trn:25:34-25:54 */)); async
                                move { match crate
                                ::__terrane_dependency_await_unwind(__terrane_call). await {
                                Ok(Ok(value)) => Ok(value), Ok(Err(error)) => Err(crate
                                ::TerraneForeignError(crate
                                ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                                .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                                Err(crate ::__terrane_dependency_panic(payload, "axum",
                                "axum::extract::ws::WebSocket::send")) } } }; async move {
                                __terrane_raised_err(__terrane_future. await,
                                5 /* terrane-site: src/main.trn:25:20-25:55 */) } }).
                                await, 6 /* terrane-site: src/main.trn:25:19-25:56 */
                            );
                            return TerraneCompletion::Return(());
                        }
                        println!(
                            "{}{}", String::from("text"),
                            terrane_scalar_support::scalar_text(&value.as_ref()
                            .expect("flow-proven available binding"))
                        );
                        __terrane_traced_completion!(
                            __terrane_await({ let __terrane_future = { let __terrane_call
                            = (&mut socket).send(__terrane_raised_completion!(match
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                            axum::extract::ws::Message::text(value.as_ref()
                            .expect("flow-proven available binding").clone()))) {
                            Ok(value) => Ok(value), Err(payload) => Err(crate
                            ::__terrane_dependency_panic(payload, "dependency",
                            "axum :: extract :: ws :: Message :: text")) },
                            7 /* terrane-site: src/main.trn:28:32-28:52 */)); async
                            move { match crate
                            ::__terrane_dependency_await_unwind(__terrane_call). await {
                            Ok(Ok(value)) => Ok(value), Ok(Err(error)) => Err(crate
                            ::TerraneForeignError(crate
                            ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                            .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                            Err(crate ::__terrane_dependency_panic(payload, "axum",
                            "axum::extract::ws::WebSocket::send")) } } }; async move {
                            __terrane_raised_err(__terrane_future. await,
                            8 /* terrane-site: src/main.trn:28:18-28:53 */) } }).
                            await, 9 /* terrane-site: src/main.trn:28:17-28:54 */
                        );
                    } else if kind
                        .as_ref()
                        .expect("flow-proven available binding")
                        .clone()
                        .as_str() == "Binary"
                    {
                        let _ = payload
                            .insert(
                                __terrane_raised_completion!(
                                    match
                                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                                    message.take().expect("flow-proven availability")
                                    .expect("semantic optional narrowing").into_data())) {
                                    Ok(value) => Ok(value), Err(payload) => Err(crate
                                    ::__terrane_dependency_panic(payload, "axum",
                                    "axum::extract::ws::Message::into_data")) },
                                    10 /* terrane-site: src/main.trn:30:21-30:46 */
                                ),
                            );
                        println!(
                            "{}{}", String::from("binary"),
                            terrane_scalar_support::scalar_text(&__terrane_raised_completion!(match
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                            (&payload.as_ref().expect("flow-proven available binding"))
                            .len())) { Ok(value) =>
                            Ok(terrane_int_support::Int::from_u128(value as u128)),
                            Err(payload) => Err(crate
                            ::__terrane_dependency_panic(payload, "bytes",
                            "bytes::Bytes::len")) }, 11 /* terrane-site: src/main.trn:31:29-31:41 */))
                        );
                        __terrane_traced_completion!(
                            __terrane_await({ let __terrane_future = { let __terrane_call
                            = (&mut socket).send(__terrane_raised_completion!(match
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                            axum::extract::ws::Message::Binary(payload.take()
                            .expect("flow-proven availability").into()))) { Ok(value) =>
                            Ok(value), Err(payload) => Err(crate
                            ::__terrane_dependency_panic(payload, "axum",
                            "axum::extract::ws::Message::Binary")) },
                            12 /* terrane-site: src/main.trn:32:32-32:61 */)); async
                            move { match crate
                            ::__terrane_dependency_await_unwind(__terrane_call). await {
                            Ok(Ok(value)) => Ok(value), Ok(Err(error)) => Err(crate
                            ::TerraneForeignError(crate
                            ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                            .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                            Err(crate ::__terrane_dependency_panic(payload, "axum",
                            "axum::extract::ws::WebSocket::send")) } } }; async move {
                            __terrane_raised_err(__terrane_future. await,
                            13 /* terrane-site: src/main.trn:32:18-32:62 */) } }).
                            await, 14 /* terrane-site: src/main.trn:32:17-32:63 */
                        );
                    } else if kind
                        .as_ref()
                        .expect("flow-proven available binding")
                        .clone()
                        .as_str() == "Ping"
                    {
                        let _ = payload
                            .insert(
                                __terrane_raised_completion!(
                                    match
                                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                                    message.take().expect("flow-proven availability")
                                    .expect("semantic optional narrowing").into_data())) {
                                    Ok(value) => Ok(value), Err(payload) => Err(crate
                                    ::__terrane_dependency_panic(payload, "axum",
                                    "axum::extract::ws::Message::into_data")) },
                                    15 /* terrane-site: src/main.trn:34:21-34:46 */
                                ),
                            );
                        println!(
                            "{}{}", String::from("ping"),
                            terrane_scalar_support::scalar_text(&__terrane_raised_completion!(match
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                            (&payload.as_ref().expect("flow-proven available binding"))
                            .len())) { Ok(value) =>
                            Ok(terrane_int_support::Int::from_u128(value as u128)),
                            Err(payload) => Err(crate
                            ::__terrane_dependency_panic(payload, "bytes",
                            "bytes::Bytes::len")) }, 16 /* terrane-site: src/main.trn:35:27-35:39 */))
                        );
                        __terrane_traced_completion!(
                            __terrane_await({ let __terrane_future = { let __terrane_call
                            = (&mut socket).send(__terrane_raised_completion!(match
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                            axum::extract::ws::Message::Pong(payload.take()
                            .expect("flow-proven availability").into()))) { Ok(value) =>
                            Ok(value), Err(payload) => Err(crate
                            ::__terrane_dependency_panic(payload, "axum",
                            "axum::extract::ws::Message::Pong")) },
                            17 /* terrane-site: src/main.trn:36:32-36:59 */)); async
                            move { match crate
                            ::__terrane_dependency_await_unwind(__terrane_call). await {
                            Ok(Ok(value)) => Ok(value), Ok(Err(error)) => Err(crate
                            ::TerraneForeignError(crate
                            ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                            .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                            Err(crate ::__terrane_dependency_panic(payload, "axum",
                            "axum::extract::ws::WebSocket::send")) } } }; async move {
                            __terrane_raised_err(__terrane_future. await,
                            18 /* terrane-site: src/main.trn:36:18-36:60 */) } }).
                            await, 19 /* terrane-site: src/main.trn:36:17-36:61 */
                        );
                    } else if kind
                        .as_ref()
                        .expect("flow-proven available binding")
                        .clone()
                        .as_str() == "Pong"
                    {
                        let _ = payload
                            .insert(
                                __terrane_raised_completion!(
                                    match
                                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                                    message.take().expect("flow-proven availability")
                                    .expect("semantic optional narrowing").into_data())) {
                                    Ok(value) => Ok(value), Err(payload) => Err(crate
                                    ::__terrane_dependency_panic(payload, "axum",
                                    "axum::extract::ws::Message::into_data")) },
                                    20 /* terrane-site: src/main.trn:38:21-38:46 */
                                ),
                            );
                        println!(
                            "{}{}", String::from("pong"),
                            terrane_scalar_support::scalar_text(&__terrane_raised_completion!(match
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                            (&payload.as_ref().expect("flow-proven available binding"))
                            .len())) { Ok(value) =>
                            Ok(terrane_int_support::Int::from_u128(value as u128)),
                            Err(payload) => Err(crate
                            ::__terrane_dependency_panic(payload, "bytes",
                            "bytes::Bytes::len")) }, 21 /* terrane-site: src/main.trn:39:27-39:39 */))
                        );
                    } else if kind
                        .as_ref()
                        .expect("flow-proven available binding")
                        .clone()
                        .as_str() == "Close"
                    {
                        println!("{}", String::from("close"));
                    }
                } else {
                    println!("{}", String::from("end"));
                    return TerraneCompletion::Return(());
                }
            }
            TerraneCompletion::Normal
        }
            .await;
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
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
                    println!("{}", String::from("failure"));
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
}
async fn health() -> String {
    return String::from("ok");
}
fn websocket_route(
    router: axum::Router<()>,
    path: String,
    handler: std::sync::Arc<
        dyn Fn(
            WebSocket,
        ) -> std::pin::Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync,
    >,
) -> axum::Router<()> {
    let callback: std::sync::Arc<
        dyn Fn(
            axum::extract::WebSocketUpgrade<axum::extract::ws::DefaultOnFailedUpgrade>,
        ) -> std::pin::Pin<
                Box<dyn Future<Output = http::Response<axum_core::body::Body>> + Send>,
            > + Send + Sync,
    >;
    callback = {
        let handler = handler.clone();
        std::sync::Arc::new(move |
            request: axum::extract::WebSocketUpgrade<
                axum::extract::ws::DefaultOnFailedUpgrade,
            >,
        | -> std::pin::Pin<
            Box<dyn Future<Output = http::Response<axum_core::body::Body>> + Send>,
        > {
            let handler = handler.clone();
            Box::pin(async move {
                return __terrane_raised(
                    match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| {
                            request
                                .on_upgrade(
                                    match || -> Result<_, crate::TerraneForeignError> {
                                        Ok({
                                            let callback = {
                                                let callable = handler.clone();
                                                TerraneConsumingCallable::new(move |
                                                    (argument_0,): (WebSocket,),
                                                | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
                                                    Box::pin(callable(argument_0))
                                                })
                                            };
                                            move |callback_argument_0: axum::extract::ws::WebSocket| {
                                                let callback_future = callback.call((callback_argument_0,));
                                                Box::pin(async move {
                                                    match async {
                                                        let callback_value = callback_future.await;
                                                        Ok::<_, crate::TerraneForeignError>(callback_value)
                                                    }
                                                        .await
                                                    {
                                                        Ok(value) => value,
                                                        Err(error) => std::panic::panic_any(error.0),
                                                    }
                                                })
                                            }
                                        })
                                    }() {
                                        Ok(value) => value,
                                        Err(error) => std::panic::panic_any(error),
                                    },
                                )
                        }),
                    ) {
                        Ok(value) => Ok(value),
                        Err(payload) => {
                            Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "axum",
                                    "axum::extract::WebSocketUpgrade<F>::on_upgrade",
                                ),
                            )
                        }
                    },
                    22 /* terrane-site: src/main.trn:52:12-52:39 */,
                );
            })
        })
    };
    return {
        let __terrane_native_input_0 = router;
        let __terrane_native_input_1 = &path;
        let __terrane_native_input_2 = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| axum::routing::get(
                    match || -> Result<_, crate::TerraneForeignError> {
                        Ok({
                            let callback = callback.clone();
                            move |
                                callback_argument_0: axum::extract::WebSocketUpgrade<
                                    axum::extract::ws::DefaultOnFailedUpgrade,
                                >|
                            {
                                let callback = callback.clone();
                                let callback_future = callback(callback_argument_0);
                                Box::pin(async move {
                                    match async {
                                        let callback_value = callback_future.await;
                                        Ok::<_, crate::TerraneForeignError>(callback_value)
                                    }
                                        .await
                                    {
                                        Ok(value) => value,
                                        Err(error) => std::panic::panic_any(error.0),
                                    }
                                })
                            }
                        })
                    }() {
                        Ok(value) => value,
                        Err(error) => std::panic::panic_any(error),
                    },
                )),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "axum",
                            "axum::routing::get",
                        ),
                    )
                }
            },
            23 /* terrane-site: src/main.trn:53:31-53:49 */,
        );
        __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    __terrane_native_input_0
                        .route(__terrane_native_input_1, __terrane_native_input_2)
                }),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "axum",
                            "axum::Router<S>::route",
                        ),
                    )
                }
            },
            24 /* terrane-site: src/main.trn:53:10-53:50 */,
        )
    };
}
async fn run_server(native: NativeTcpListener) {
    let mut router: axum::Router<()>;
    router = __terrane_raised(
        terrane_static_trn_526f75746572_new::<()>(),
        25 /* terrane-site: src/main.trn:56:12-56:24 */,
    );
    router = {
        let __terrane_native_input_0 = router;
        let __terrane_native_input_1 = &String::from("/health");
        let __terrane_native_input_2 = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| axum::routing::get(
                    match || -> Result<_, crate::TerraneForeignError> {
                        Ok({
                            let callback = std::sync::Arc::new(move || -> std::pin::Pin<
                                    Box<dyn Future<Output = _> + Send>,
                                > { Box::pin(health()) })
                                .clone();
                            move || {
                                let callback = callback.clone();
                                let callback_future = callback();
                                Box::pin(async move {
                                    match async {
                                        let callback_value = callback_future.await;
                                        Ok::<_, crate::TerraneForeignError>(callback_value)
                                    }
                                        .await
                                    {
                                        Ok(value) => value,
                                        Err(error) => std::panic::panic_any(error.0),
                                    }
                                })
                            }
                        })
                    }() {
                        Ok(value) => value,
                        Err(error) => std::panic::panic_any(error),
                    },
                )),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "axum",
                            "axum::routing::get",
                        ),
                    )
                }
            },
            26 /* terrane-site: src/main.trn:57:38-57:49 */,
        );
        __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    __terrane_native_input_0
                        .route(__terrane_native_input_1, __terrane_native_input_2)
                }),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "axum",
                            "axum::Router<S>::route",
                        ),
                    )
                }
            },
            27 /* terrane-site: src/main.trn:57:12-57:50 */,
        )
    };
    router = websocket_route(
        router,
        String::from("/ws"),
        std::sync::Arc::new(move |
            argument_0: WebSocket,
        | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
            Box::pin(socket_session(argument_0))
        }),
    );
    __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = axum::serve(native, router);
                    async move {
                        match crate::__terrane_dependency_await_unwind(
                                std::future::IntoFuture::into_future(__terrane_call),
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
                                                "Rust dependency `axum` member `axum::serve` failed: {error}"
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
                                        "axum",
                                        "axum::serve",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        28 /* terrane-site: src/main.trn:59:10-59:41 */,
                    )
                }
            })
            .await,
        29 /* terrane-site: src/main.trn:59:9-59:42 */,
    );
}
fn main() {
    __terrane_run(async move {
        let listener: TcpListener;
        let address: String;
        let native: NativeTcpListener;
        let scope: TerraneTaskScope;
        let server: TerraneScopedTask<()>;
        listener = __terrane_traced(
            __terrane_await(listen_tcp(String::from("127.0.0.1:0"), None)).await,
            30 /* terrane-site: src/main.trn:62:20-62:45 */,
        );
        address = listener.local_address.clone();
        native = __terrane_traced(
            listener.into_native(),
            31 /* terrane-site: src/main.trn:64:12-64:33 */,
        );
        scope = TerraneTaskScope::new(None);
        server = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = run_server(native);
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        __terrane_spawned_task,
                        __terrane_cancel,
                        __terrane_deadline,
                    )
                    .await
                {
                    Some(value) => TerraneTaskResult::Completed(value),
                    None => TerraneTaskResult::Cancelled,
                }
            })
        };
        __terrane_traced(
            __terrane_await({
                    let __terrane_future = drive_peer(address);
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            32 /* terrane-site: src/main.trn:67:10-67:29 */,
                        )
                    }
                })
                .await,
            33 /* terrane-site: src/main.trn:67:9-67:30 */,
        );
        scope.cancel();
        {
            let _: TerraneTaskOutcome<()> = __terrane_await(scope.join(server)).await;
        }
    });
}
