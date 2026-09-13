// Generated deterministically by Terrane <version>.
// Runtime support: async_native.rs, executor_local.rs, async_dependency.rs, tasks_native_local.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: src/main.trn
// Namespace: app
async fn consume(incoming: Incoming) -> String {
    return __terrane_traced(
        __terrane_await({
                let __terrane_future = drain_slowly(incoming);
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        0 /* terrane-site: src/main.trn:6:16-6:38 */,
                    )
                }
            })
            .await,
        0 /* terrane-site: src/main.trn:6:16-6:38 */,
    );
}
async fn send_blocked(
    mut sink: Outgoing,
) -> terrane_collection_support::AsyncSinkOutcome {
    return __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sink).send(String::from("blocked"));
                    async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                        {
                            Ok(Ok(value)) => {
                                Ok(
                                    terrane_collection_support::AsyncSinkOutcome::from_accepted(
                                        value,
                                    ),
                                )
                            }
                            Ok(Err(error)) => {
                                Err(
                                    crate::TerraneForeignError(
                                        crate::TerraneError::custom_raised(
                                            crate::TERRANE_DEPENDENCY_ERROR,
                                            format!(
                                                "Rust dependency `terrane_sink_witness` member `terrane_sink_witness::Outgoing::send` failed: {error}"
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
                                        "terrane_sink_witness",
                                        "terrane_sink_witness::Outgoing::send",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        1 /* terrane-site: src/main.trn:9:16-9:36 */,
                    )
                }
            })
            .await,
        1 /* terrane-site: src/main.trn:9:16-9:36 */,
    );
}
fn main() {
    __terrane_run(async move {
        let whole: Duplex = __terrane_raised(
            duplex(terrane_int_support::Int::from(1_i128)),
            2 /* terrane-site: src/main.trn:12:11-12:20 */,
        );
        let mut halves: SplitEndpoints = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| whole.split()),
            ) {
                Ok(Ok(value)) => Ok(value),
                Ok(Err(error)) => {
                    Err(
                        crate::TerraneForeignError(
                            crate::TerraneError::custom_raised(
                                crate::TERRANE_DEPENDENCY_ERROR,
                                format!(
                                    "Rust dependency `terrane_sink_witness` member `terrane_sink_witness::Duplex::split` failed: {error}"
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
                            "terrane_sink_witness",
                            "terrane_sink_witness::Duplex::split",
                        ),
                    )
                }
            },
            3 /* terrane-site: src/main.trn:13:12-13:24 */,
        );
        let incoming: Incoming = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| halves.take_incoming()),
            ) {
                Ok(Ok(value)) => Ok(value),
                Ok(Err(error)) => {
                    Err(
                        crate::TerraneForeignError(
                            crate::TerraneError::custom_raised(
                                crate::TERRANE_DEPENDENCY_ERROR,
                                format!(
                                    "Rust dependency `terrane_sink_witness` member `terrane_sink_witness::SplitEndpoints::take_incoming` failed: {error}"
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
                            "terrane_sink_witness",
                            "terrane_sink_witness::SplitEndpoints::take_incoming",
                        ),
                    )
                }
            },
            4 /* terrane-site: src/main.trn:14:14-14:35 */,
        );
        let mut outgoing: Outgoing = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| halves.take_outgoing()),
            ) {
                Ok(Ok(value)) => Ok(value),
                Ok(Err(error)) => {
                    Err(
                        crate::TerraneForeignError(
                            crate::TerraneError::custom_raised(
                                crate::TERRANE_DEPENDENCY_ERROR,
                                format!(
                                    "Rust dependency `terrane_sink_witness` member `terrane_sink_witness::SplitEndpoints::take_outgoing` failed: {error}"
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
                            "terrane_sink_witness",
                            "terrane_sink_witness::SplitEndpoints::take_outgoing",
                        ),
                    )
                }
            },
            5 /* terrane-site: src/main.trn:15:14-15:35 */,
        );
        let scope: TerraneTaskScope = TerraneTaskScope::new(None);
        let consumer: TerraneScopedTask<String> = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = consume(incoming);
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
        let first: terrane_collection_support::AsyncSinkOutcome = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = (&mut outgoing).send(String::from("one"));
                        async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                            {
                                Ok(Ok(value)) => {
                                    Ok(
                                        terrane_collection_support::AsyncSinkOutcome::from_accepted(
                                            value,
                                        ),
                                    )
                                }
                                Ok(Err(error)) => {
                                    Err(
                                        crate::TerraneForeignError(
                                            crate::TerraneError::custom_raised(
                                                crate::TERRANE_DEPENDENCY_ERROR,
                                                format!(
                                                    "Rust dependency `terrane_sink_witness` member `terrane_sink_witness::Outgoing::send` failed: {error}"
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
                                            "terrane_sink_witness",
                                            "terrane_sink_witness::Outgoing::send",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            6 /* terrane-site: src/main.trn:18:17-18:37 */,
                        )
                    }
                })
                .await,
            6 /* terrane-site: src/main.trn:18:17-18:37 */,
        );
        let second: terrane_collection_support::AsyncSinkOutcome = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = (&mut outgoing).send(String::from("two"));
                        async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                            {
                                Ok(Ok(value)) => {
                                    Ok(
                                        terrane_collection_support::AsyncSinkOutcome::from_accepted(
                                            value,
                                        ),
                                    )
                                }
                                Ok(Err(error)) => {
                                    Err(
                                        crate::TerraneForeignError(
                                            crate::TerraneError::custom_raised(
                                                crate::TERRANE_DEPENDENCY_ERROR,
                                                format!(
                                                    "Rust dependency `terrane_sink_witness` member `terrane_sink_witness::Outgoing::send` failed: {error}"
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
                                            "terrane_sink_witness",
                                            "terrane_sink_witness::Outgoing::send",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            7 /* terrane-site: src/main.trn:19:18-19:38 */,
                        )
                    }
                })
                .await,
            7 /* terrane-site: src/main.trn:19:18-19:38 */,
        );
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&first.accepted),
            terrane_scalar_support::scalar_text(&first.closed),
            terrane_scalar_support::scalar_text(&second.accepted)
        );
        let flushed: bool = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = (&mut outgoing).flush();
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
                                                    "Rust dependency `terrane_sink_witness` member `terrane_sink_witness::Outgoing::flush` failed: {error}"
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
                                            "terrane_sink_witness",
                                            "terrane_sink_witness::Outgoing::flush",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            8 /* terrane-site: src/main.trn:21:24-21:39 */,
                        )
                    }
                })
                .await,
            8 /* terrane-site: src/main.trn:21:24-21:39 */,
        );
        let closed: bool = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = outgoing.close();
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
                                                    "Rust dependency `terrane_sink_witness` member `terrane_sink_witness::Outgoing::close` failed: {error}"
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
                                            "terrane_sink_witness",
                                            "terrane_sink_witness::Outgoing::close",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            9 /* terrane-site: src/main.trn:22:23-22:38 */,
                        )
                    }
                })
                .await,
            9 /* terrane-site: src/main.trn:22:23-22:38 */,
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&flushed),
            terrane_scalar_support::scalar_text(&closed)
        );
        let consumed: TerraneTaskOutcome<String> = __terrane_await(scope.join(consumer))
            .await;
        let consumed_value: Option<String> = consumed.value.clone();
        if consumed_value.is_some() {
            println!(
                "{}{}", terrane_scalar_support::scalar_text(&consumed.completed),
                terrane_scalar_support::scalar_text(&* consumed_value.as_ref()
                .expect("semantic optional narrowing"))
            );
        }
        let mut remote: Outgoing = __terrane_raised(
            remotely_closed_sink(),
            10 /* terrane-site: src/main.trn:28:12-28:33 */,
        );
        let rejected: terrane_collection_support::AsyncSinkOutcome = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = (&mut remote).send(String::from("lost"));
                        async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                            {
                                Ok(Ok(value)) => {
                                    Ok(
                                        terrane_collection_support::AsyncSinkOutcome::from_accepted(
                                            value,
                                        ),
                                    )
                                }
                                Ok(Err(error)) => {
                                    Err(
                                        crate::TerraneForeignError(
                                            crate::TerraneError::custom_raised(
                                                crate::TERRANE_DEPENDENCY_ERROR,
                                                format!(
                                                    "Rust dependency `terrane_sink_witness` member `terrane_sink_witness::Outgoing::send` failed: {error}"
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
                                            "terrane_sink_witness",
                                            "terrane_sink_witness::Outgoing::send",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            11 /* terrane-site: src/main.trn:29:20-29:39 */,
                        )
                    }
                })
                .await,
            11 /* terrane-site: src/main.trn:29:20-29:39 */,
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&rejected.accepted),
            terrane_scalar_support::scalar_text(&rejected.closed)
        );
        let remote_closed: bool = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = remote.close();
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
                                                    "Rust dependency `terrane_sink_witness` member `terrane_sink_witness::Outgoing::close` failed: {error}"
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
                                            "terrane_sink_witness",
                                            "terrane_sink_witness::Outgoing::close",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            12 /* terrane-site: src/main.trn:31:30-31:43 */,
                        )
                    }
                })
                .await,
            12 /* terrane-site: src/main.trn:31:30-31:43 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&remote_closed));
        let blocked: Outgoing = __terrane_raised(
            blocked_sink(),
            13 /* terrane-site: src/main.trn:33:13-33:26 */,
        );
        let blocked_scope: TerraneTaskScope = TerraneTaskScope::new(None);
        let blocked_child: TerraneScopedTask<
            terrane_collection_support::AsyncSinkOutcome,
        > = {
            let __terrane_scope = blocked_scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = send_blocked(blocked);
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
        blocked_scope.cancel();
        let cancelled: TerraneTaskOutcome<
            terrane_collection_support::AsyncSinkOutcome,
        > = __terrane_await(blocked_scope.join(blocked_child)).await;
        println!("{}", terrane_scalar_support::scalar_text(&cancelled.cancelled));
        let mut queue: QueueSink = __terrane_raised(
            queue_sink(),
            14 /* terrane-site: src/main.trn:39:11-39:22 */,
        );
        let queued: terrane_collection_support::AsyncSinkOutcome = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = (&mut queue)
                            .send(String::from("different"));
                        async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                            {
                                Ok(Ok(value)) => {
                                    Ok(
                                        terrane_collection_support::AsyncSinkOutcome::from_accepted(
                                            value,
                                        ),
                                    )
                                }
                                Ok(Err(error)) => {
                                    Err(
                                        crate::TerraneForeignError(
                                            crate::TerraneError::custom_raised(
                                                crate::TERRANE_DEPENDENCY_ERROR,
                                                format!(
                                                    "Rust dependency `terrane_sink_witness` member `terrane_sink_witness::QueueSink::send` failed: {error}"
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
                                            "terrane_sink_witness",
                                            "terrane_sink_witness::QueueSink::send",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            15 /* terrane-site: src/main.trn:40:18-40:41 */,
                        )
                    }
                })
                .await,
            15 /* terrane-site: src/main.trn:40:18-40:41 */,
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&queued.accepted),
            terrane_scalar_support::scalar_text(&queued.closed)
        );
        let queue_flushed: terrane_int_support::Int = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| queue.flush()),
            ) {
                Ok(Ok(value)) => Ok(terrane_int_support::Int::from(i128::from(value))),
                Ok(Err(error)) => {
                    Err(
                        crate::TerraneForeignError(
                            crate::TerraneError::custom_raised(
                                crate::TERRANE_DEPENDENCY_ERROR,
                                format!(
                                    "Rust dependency `terrane_sink_witness` member `terrane_sink_witness::QueueSink::flush` failed: {error}"
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
                            "terrane_sink_witness",
                            "terrane_sink_witness::QueueSink::flush",
                        ),
                    )
                }
            },
            16 /* terrane-site: src/main.trn:42:23-42:35 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&queue_flushed));
        let queue_closed: String = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| queue.close()),
            ) {
                Ok(Ok(value)) => Ok(value),
                Ok(Err(error)) => {
                    Err(
                        crate::TerraneForeignError(
                            crate::TerraneError::custom_raised(
                                crate::TERRANE_DEPENDENCY_ERROR,
                                format!(
                                    "Rust dependency `terrane_sink_witness` member `terrane_sink_witness::QueueSink::close` failed: {error}"
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
                            "terrane_sink_witness",
                            "terrane_sink_witness::QueueSink::close",
                        ),
                    )
                }
            },
            17 /* terrane-site: src/main.trn:44:25-44:37 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&queue_closed));
    });
}
// Source: <terrane>/projected/deps/terrane-sink-witness.trn
// Namespace: deps/terrane-sink-witness
pub use terrane_sink_witness::Incoming;
pub use terrane_sink_witness::Outgoing;
pub use terrane_sink_witness::QueueSink;
pub use terrane_sink_witness::Duplex;
pub use terrane_sink_witness::SplitEndpoints;
pub fn blocked_sink() -> Result<Outgoing, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_sink_witness::blocked_sink()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-sink-witness",
                    "terrane_sink_witness::blocked_sink",
                ),
            )
        }
    }
}
pub async fn drain_slowly(
    incoming: Incoming,
) -> Result<String, crate::TerraneForeignError> {
    let incoming = incoming;
    match crate::__terrane_dependency_await_unwind(
            terrane_sink_witness::drain_slowly(incoming),
        )
        .await
    {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-sink-witness",
                    "terrane_sink_witness::drain_slowly",
                ),
            )
        }
    }
}
pub fn duplex(
    capacity: terrane_int_support::Int,
) -> Result<Duplex, crate::TerraneForeignError> {
    let capacity = terrane_int_support::coerce::<i64>(&capacity)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_sink_witness::duplex(capacity)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-sink-witness",
                    "terrane_sink_witness::duplex",
                ),
            )
        }
    }
}
pub fn queue_sink() -> Result<QueueSink, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_sink_witness::queue_sink()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-sink-witness",
                    "terrane_sink_witness::queue_sink",
                ),
            )
        }
    }
}
pub fn remotely_closed_sink() -> Result<Outgoing, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_sink_witness::remotely_closed_sink()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-sink-witness",
                    "terrane_sink_witness::remotely_closed_sink",
                ),
            )
        }
    }
}
