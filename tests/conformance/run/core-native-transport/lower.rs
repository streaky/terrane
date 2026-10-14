// Generated deterministically by Terrane <version>.
#[path = "lower.authored/rust/native.rs"]
mod native;
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: core-native-transport
async fn serve_one(listener: NativeTcpListener) {
    return __terrane_await(serve_borrowed(&listener)).await;
}
async fn serve_borrowed(listener: &NativeTcpListener) {
    let _ = &listener;
    return crate::native::serve_one(listener).await;
}
async fn exchange(stream: &NativeTcpStream) -> String {
    let _ = &stream;
    return crate::native::exchange(stream).await;
}
fn main() {
    __terrane_run(async move {
        __terrane_group_frame(async move {
                let options: TcpListenerOptions;
                let listener: TcpListener;
                let address: SocketResult;
                let exported_listener: NativeTcpListener;
                let servers: TerraneTaskGroup<()>;
                let mut completed: bool;
                let mut connected: Option<StreamResult> = None;
                let mut stream: Option<TcpStream> = None;
                let mut exported: Option<NativeTcpStream> = None;
                let mut response: Option<String> = None;
                let mut outcomes: Option<
                    terrane_collection_support::List<TerraneTaskOutcome<()>>,
                > = None;
                let mut outcome: Option<TerraneTaskOutcome<()>> = None;
                options = TcpListenerOptions::terrane_construct(
                    terrane_int_support::Int::from(-1_i128),
                    false,
                    true,
                    terrane_int_support::Int::from(23_i128),
                );
                listener = __terrane_traced(
                    __terrane_await(
                            listen_tcp(String::from("localhost:0"), Some(options)),
                        )
                        .await,
                    0 /* terrane-site: src/main.trn:20:22-20:56 */,
                );
                address = socket_address_from_string(listener.local_address.clone());
                if address.failed {
                    exit(make_exit_status(terrane_int_support::Int::from(1_i128)));
                }
                exported_listener = __terrane_traced(
                    listener.into_native(),
                    1 /* terrane-site: src/main.trn:24:25-24:46 */,
                );
                servers = TerraneTaskGroup::<()>::new();
                {
                    let __terrane_child = {
                        let __terrane_scope = servers.scope();
                        let __terrane_cancel = __terrane_scope.cancellation();
                        let __terrane_deadline = __terrane_scope.deadline.clone();
                        let __terrane_spawned_task = serve_one(exported_listener);
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
                    servers.spawn(__terrane_child);
                };
                completed = false;
                let mut __terrane_finally_guard_0 = __terrane_finally_guard();
                let __terrane_maybe_completion_0: Option<TerraneCompletion<()>> = __terrane_cancel_operation(
                        &__terrane_finally_guard_0,
                        async {
                            let __terrane_try_0: TerraneCompletion<()> = async {
                                let _ = connected
                                    .insert(
                                        __terrane_await(connect_tcp(address.value, None, None))
                                            .await,
                                    );
                                if connected
                                    .as_ref()
                                    .expect("flow-proven available binding")
                                    .failed
                                {
                                    exit(
                                        make_exit_status(terrane_int_support::Int::from(2_i128)),
                                    );
                                }
                                let _ = stream
                                    .insert(
                                        connected.take().expect("flow-proven availability").value,
                                    );
                                let _ = exported
                                    .insert(
                                        __terrane_traced_completion!(
                                            stream.take().expect("flow-proven availability")
                                            .into_native(), 2 /* terrane-site: src/main.trn:33:20-33:39 */
                                        ),
                                    );
                                let _ = response
                                    .insert(
                                        __terrane_await(
                                                exchange(
                                                    exported.as_ref().expect("flow-proven available binding"),
                                                ),
                                            )
                                            .await,
                                    );
                                println!(
                                    "{}", terrane_scalar_support::scalar_text(&response.as_ref()
                                    .expect("flow-proven available binding"))
                                );
                                completed = true;
                                TerraneCompletion::Normal
                            }
                                .await;
                            match __terrane_try_0 {
                                TerraneCompletion::Return(value) => {
                                    return TerraneCompletion::Return(value);
                                }
                                TerraneCompletion::Break => return TerraneCompletion::Break,
                                TerraneCompletion::Continue => {
                                    return TerraneCompletion::Continue;
                                }
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
                let __terrane_finally_0: TerraneCompletion<()> = async {
                    if !completed {
                        servers.cancel();
                    }
                    let _ = outcomes
                        .insert(
                            __terrane_await(
                                    std::boxed::Box::pin(servers.prepare_drain()),
                                )
                                .await,
                        );
                    let __terrane_iterable_0 = outcomes
                        .as_ref()
                        .expect("flow-proven available binding")
                        .clone();
                    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
                        &__terrane_iterable_0,
                    );
                    loop {
                        let _ = outcome
                            .insert(
                                match __terrane_iterator_0.next() {
                                    terrane_collection_support::IterationStep::Item(item) => {
                                        item
                                    }
                                    terrane_collection_support::IterationStep::End => break,
                                },
                            );
                        if outcome
                            .as_ref()
                            .expect("flow-proven available binding")
                            .clone()
                            .error
                            .is_some()
                        {
                            exit(
                                make_exit_status(terrane_int_support::Int::from(3_i128)),
                            );
                        }
                    }
                    TerraneCompletion::Normal
                }
                    .await;
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
                let __terrane_completion_1: TerraneCompletion<()> = async {
                    let __terrane_try_1: TerraneCompletion<()> = async {
                        {
                            let _: TcpListener = __terrane_traced_completion!(
                                __terrane_await(listen_tcp(String::from("missing-port"),
                                None)). await, 3 /* terrane-site: src/main.trn:45:19-45:45 */
                            );
                        }
                        TerraneCompletion::Normal
                    }
                        .await;
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
                                    == TerraneErrorKind::Custom(DescriptorId(0))
                            {
                                __terrane_handled_1 = true;
                                println!("{}", String::from("invalid-address"));
                            }
                            if !__terrane_handled_1 {
                                return TerraneCompletion::Error(__terrane_error_1);
                            }
                        }
                    }
                    TerraneCompletion::Normal
                }
                    .await;
                match __terrane_completion_1 {
                    TerraneCompletion::Normal => {}
                    TerraneCompletion::Return(value) => return value,
                    TerraneCompletion::Error(error) => __terrane_uncaught(error),
                    TerraneCompletion::Break | TerraneCompletion::Continue => {
                        __terrane_generated_defect("loop control escaped a non-loop try")
                    }
                }
                return ();
            })
            .await
    });
}
