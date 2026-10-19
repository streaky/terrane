// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
async fn socket_session(mut socket: WebSocket) {
    let mut open: Option<bool> = None;
    let mut message: Option<Option<Message>> = None;
    let mut kind: Option<String> = None;
    let mut value: Option<String> = None;
    let mut payload: Option<Vec<u8>> = None;
    let __terrane_completion_0: TerraneCompletion<()> = async {
        let __terrane_try_0: TerraneCompletion<()> = async {
            let _ = open.insert(true);
            while *open.as_ref().expect("flow-proven available binding") {
                let _ = message
                    .insert(
                        __terrane_traced_completion!(
                            __terrane_await({ let __terrane_future = { let __terrane_call
                            = (&mut socket).recv(); Box::pin(async move { match crate
                            ::__terrane_dependency_await_unwind(__terrane_call). await {
                            Ok(Ok(value)) => Ok(value), Ok(Err(error)) => Err(crate
                            ::TerraneForeignError(crate
                            ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                            .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                            Err(crate ::__terrane_dependency_panic(payload,
                            "terrane_websocket_peer_witness",
                            "terrane_websocket_peer_witness::WebSocket::recv")) } }) };
                            async move { __terrane_raised_err(__terrane_future. await,
                            0 /* terrane-site: src/main.trn:9:36-9:48 */) } }). await,
                            0 /* terrane-site: src/main.trn:9:36-9:48 */
                        ),
                    );
                if message.as_ref().expect("flow-proven available binding").is_some() {
                    let _ = kind
                        .insert(
                            __terrane_raised_completion!(
                                match
                                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                                match &match message.as_ref()
                                .expect("flow-proven available binding") { Some(value) =>
                                value, _ => unreachable!("flow-proven storage refinement") }
                                { terrane_websocket_peer_witness::Message::Text { .. } =>
                                "Text".to_owned(),
                                terrane_websocket_peer_witness::Message::Binary { .. } =>
                                "Binary".to_owned(),
                                terrane_websocket_peer_witness::Message::Ping { .. } =>
                                "Ping".to_owned(),
                                terrane_websocket_peer_witness::Message::Pong { .. } =>
                                "Pong".to_owned(),
                                terrane_websocket_peer_witness::Message::Close { .. } =>
                                "Close".to_owned() })) { Ok(value) => Ok(value),
                                Err(payload) => Err(crate
                                ::__terrane_dependency_panic(payload,
                                "terrane_websocket_peer_witness",
                                "terrane_websocket_peer_witness::Message::variant-name")) },
                                1 /* terrane-site: src/main.trn:11:23-11:44 */
                            ),
                        );
                    if kind
                        .as_ref()
                        .expect("flow-proven available binding")
                        .clone()
                        .as_str() == "Text"
                    {
                        let _ = value
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
                                    Err(crate ::__terrane_dependency_panic(payload,
                                    "terrane_websocket_peer_witness",
                                    "terrane_websocket_peer_witness::Message::into_text")) },
                                    2 /* terrane-site: src/main.trn:13:26-13:51 */
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
                                terrane_websocket_peer_witness::Message::Close(None))) {
                                Ok(value) => Ok(value), Err(payload) => Err(crate
                                ::__terrane_dependency_panic(payload,
                                "terrane_websocket_peer_witness",
                                "terrane_websocket_peer_witness::Message::Close")) },
                                3 /* terrane-site: src/main.trn:16:34-16:54 */));
                                Box::pin(async move { match crate
                                ::__terrane_dependency_await_unwind(__terrane_call). await {
                                Ok(Ok(value)) => Ok(value), Ok(Err(error)) => Err(crate
                                ::TerraneForeignError(crate
                                ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                                .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                                Err(crate ::__terrane_dependency_panic(payload,
                                "terrane_websocket_peer_witness",
                                "terrane_websocket_peer_witness::WebSocket::send")) } }) };
                                async move { __terrane_raised_err(__terrane_future. await,
                                4 /* terrane-site: src/main.trn:16:20-16:55 */) } }).
                                await, 5 /* terrane-site: src/main.trn:16:19-16:56 */
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
                            = (&mut socket)
                            .send(__terrane_raised_completion!(terrane_static_trn_4d657373616765_text(value
                            .as_ref().expect("flow-proven available binding").clone()),
                            6 /* terrane-site: src/main.trn:19:32-19:52 */));
                            Box::pin(async move { match crate
                            ::__terrane_dependency_await_unwind(__terrane_call). await {
                            Ok(Ok(value)) => Ok(value), Ok(Err(error)) => Err(crate
                            ::TerraneForeignError(crate
                            ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                            .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                            Err(crate ::__terrane_dependency_panic(payload,
                            "terrane_websocket_peer_witness",
                            "terrane_websocket_peer_witness::WebSocket::send")) } }) };
                            async move { __terrane_raised_err(__terrane_future. await,
                            7 /* terrane-site: src/main.trn:19:18-19:53 */) } }).
                            await, 8 /* terrane-site: src/main.trn:19:17-19:54 */
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
                                    ::__terrane_dependency_panic(payload,
                                    "terrane_websocket_peer_witness",
                                    "terrane_websocket_peer_witness::Message::into_data")) },
                                    9 /* terrane-site: src/main.trn:21:21-21:46 */
                                ),
                            );
                        println!(
                            "{}{}", String::from("binary"),
                            terrane_scalar_support::scalar_text(&(payload.as_ref()
                            .expect("flow-proven available binding").clone().len() as
                            i128))
                        );
                        __terrane_traced_completion!(
                            __terrane_await({ let __terrane_future = { let __terrane_call
                            = (&mut socket).send(__terrane_raised_completion!(match
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                            terrane_websocket_peer_witness::Message::Binary(payload
                            .take().expect("flow-proven availability")))) { Ok(value) =>
                            Ok(value), Err(payload) => Err(crate
                            ::__terrane_dependency_panic(payload,
                            "terrane_websocket_peer_witness",
                            "terrane_websocket_peer_witness::Message::Binary")) },
                            10 /* terrane-site: src/main.trn:23:32-23:61 */));
                            Box::pin(async move { match crate
                            ::__terrane_dependency_await_unwind(__terrane_call). await {
                            Ok(Ok(value)) => Ok(value), Ok(Err(error)) => Err(crate
                            ::TerraneForeignError(crate
                            ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                            .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                            Err(crate ::__terrane_dependency_panic(payload,
                            "terrane_websocket_peer_witness",
                            "terrane_websocket_peer_witness::WebSocket::send")) } }) };
                            async move { __terrane_raised_err(__terrane_future. await,
                            11 /* terrane-site: src/main.trn:23:18-23:62 */) } }).
                            await, 12 /* terrane-site: src/main.trn:23:17-23:63 */
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
                                    ::__terrane_dependency_panic(payload,
                                    "terrane_websocket_peer_witness",
                                    "terrane_websocket_peer_witness::Message::into_data")) },
                                    13 /* terrane-site: src/main.trn:25:21-25:46 */
                                ),
                            );
                        println!(
                            "{}{}", String::from("ping"),
                            terrane_scalar_support::scalar_text(&(payload.as_ref()
                            .expect("flow-proven available binding").clone().len() as
                            i128))
                        );
                        __terrane_traced_completion!(
                            __terrane_await({ let __terrane_future = { let __terrane_call
                            = (&mut socket).send(__terrane_raised_completion!(match
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                            terrane_websocket_peer_witness::Message::Pong(payload.take()
                            .expect("flow-proven availability")))) { Ok(value) =>
                            Ok(value), Err(payload) => Err(crate
                            ::__terrane_dependency_panic(payload,
                            "terrane_websocket_peer_witness",
                            "terrane_websocket_peer_witness::Message::Pong")) },
                            14 /* terrane-site: src/main.trn:27:32-27:59 */));
                            Box::pin(async move { match crate
                            ::__terrane_dependency_await_unwind(__terrane_call). await {
                            Ok(Ok(value)) => Ok(value), Ok(Err(error)) => Err(crate
                            ::TerraneForeignError(crate
                            ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                            .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                            Err(crate ::__terrane_dependency_panic(payload,
                            "terrane_websocket_peer_witness",
                            "terrane_websocket_peer_witness::WebSocket::send")) } }) };
                            async move { __terrane_raised_err(__terrane_future. await,
                            15 /* terrane-site: src/main.trn:27:18-27:60 */) } }).
                            await, 16 /* terrane-site: src/main.trn:27:17-27:61 */
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
                                    ::__terrane_dependency_panic(payload,
                                    "terrane_websocket_peer_witness",
                                    "terrane_websocket_peer_witness::Message::into_data")) },
                                    17 /* terrane-site: src/main.trn:29:21-29:46 */
                                ),
                            );
                        println!(
                            "{}{}", String::from("pong"),
                            terrane_scalar_support::scalar_text(&(payload.as_ref()
                            .expect("flow-proven available binding").clone().len() as
                            i128))
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
fn main() {
    __terrane_run(async move {
        __terrane_traced(
            __terrane_await({
                    let __terrane_future = exercise(
                        std::sync::Arc::new(move |
                            argument_0: WebSocket,
                        | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
                            Box::pin(socket_session(argument_0))
                        }),
                    );
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            18 /* terrane-site: src/main.trn:40:10-40:34 */,
                        )
                    }
                })
                .await,
            19 /* terrane-site: src/main.trn:40:9-40:35 */,
        );
    });
}
