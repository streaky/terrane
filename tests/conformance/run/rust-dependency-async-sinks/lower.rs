// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
async fn consume(incoming: Incoming) -> String {
    return __terrane_traced(
        __terrane_await({
                let __terrane_future = drain_slowly(incoming);
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        0 /* terrane-site: src/main.trn:5:16-5:38 */,
                    )
                }
            })
            .await,
        0 /* terrane-site: src/main.trn:5:16-5:38 */,
    );
}
async fn send_blocked(
    mut sink: Outgoing,
) -> Result<terrane_collection_support::AsyncSinkOutcome, TerraneError> {
    return Ok(
        __terrane_traced_err(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = (&mut sink).send(String::from("blocked"));
                        Box::pin(async move {
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
                                            "terrane_sink_witness",
                                            "terrane_sink_witness::Outgoing::send",
                                        ),
                                    )
                                }
                            }
                        })
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            1 /* terrane-site: src/main.trn:8:16-8:36 */,
                        )
                    }
                })
                .await,
            1 /* terrane-site: src/main.trn:8:16-8:36 */,
        )?,
    );
}
fn main() {
    __terrane_run(async move {
        let whole: Duplex;
        let mut halves: SplitEndpoints;
        let incoming: Incoming;
        let mut outgoing: Outgoing;
        let scope: TerraneTaskScope;
        let consumer: TerraneScopedTask<String>;
        let first: terrane_collection_support::AsyncSinkOutcome;
        let second: terrane_collection_support::AsyncSinkOutcome;
        let flushed: bool;
        let closed: bool;
        let consumed: TerraneTaskOutcome<String>;
        let consumed_value: Option<String>;
        let mut remote: Outgoing;
        let rejected: terrane_collection_support::AsyncSinkOutcome;
        let remote_closed: bool;
        let blocked: Outgoing;
        let blocked_scope: TerraneTaskScope;
        let blocked_child: TerraneScopedTask<
            terrane_collection_support::AsyncSinkOutcome,
        >;
        let cancelled: TerraneTaskOutcome<terrane_collection_support::AsyncSinkOutcome>;
        let mut queue: QueueSink;
        let queued: terrane_collection_support::AsyncSinkOutcome;
        let queue_flushed: terrane_int_support::Int;
        let queue_closed: String;
        whole = __terrane_raised(
            duplex(terrane_int_support::Int::from(1_i128)),
            2 /* terrane-site: src/main.trn:11:11-11:20 */,
        );
        halves = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| whole.split()),
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
                            "terrane_sink_witness",
                            "terrane_sink_witness::Duplex::split",
                        ),
                    )
                }
            },
            3 /* terrane-site: src/main.trn:12:12-12:24 */,
        );
        incoming = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| halves.take_incoming()),
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
                            "terrane_sink_witness",
                            "terrane_sink_witness::SplitEndpoints::take_incoming",
                        ),
                    )
                }
            },
            4 /* terrane-site: src/main.trn:13:14-13:35 */,
        );
        outgoing = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| halves.take_outgoing()),
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
                            "terrane_sink_witness",
                            "terrane_sink_witness::SplitEndpoints::take_outgoing",
                        ),
                    )
                }
            },
            5 /* terrane-site: src/main.trn:14:14-14:35 */,
        );
        scope = TerraneTaskScope::new(None);
        consumer = {
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
        first = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = (&mut outgoing).send(String::from("one"));
                        Box::pin(async move {
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
                                            "terrane_sink_witness",
                                            "terrane_sink_witness::Outgoing::send",
                                        ),
                                    )
                                }
                            }
                        })
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            6 /* terrane-site: src/main.trn:17:17-17:37 */,
                        )
                    }
                })
                .await,
            6 /* terrane-site: src/main.trn:17:17-17:37 */,
        );
        second = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = (&mut outgoing).send(String::from("two"));
                        Box::pin(async move {
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
                                            "terrane_sink_witness",
                                            "terrane_sink_witness::Outgoing::send",
                                        ),
                                    )
                                }
                            }
                        })
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            7 /* terrane-site: src/main.trn:18:18-18:38 */,
                        )
                    }
                })
                .await,
            7 /* terrane-site: src/main.trn:18:18-18:38 */,
        );
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&first.accepted),
            terrane_scalar_support::scalar_text(&first.closed),
            terrane_scalar_support::scalar_text(&second.accepted)
        );
        flushed = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = (&mut outgoing).flush();
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
                                            "terrane_sink_witness",
                                            "terrane_sink_witness::Outgoing::flush",
                                        ),
                                    )
                                }
                            }
                        })
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            8 /* terrane-site: src/main.trn:20:24-20:39 */,
                        )
                    }
                })
                .await,
            8 /* terrane-site: src/main.trn:20:24-20:39 */,
        );
        closed = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = outgoing.close();
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
                                            "terrane_sink_witness",
                                            "terrane_sink_witness::Outgoing::close",
                                        ),
                                    )
                                }
                            }
                        })
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            9 /* terrane-site: src/main.trn:21:23-21:38 */,
                        )
                    }
                })
                .await,
            9 /* terrane-site: src/main.trn:21:23-21:38 */,
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&flushed),
            terrane_scalar_support::scalar_text(&closed)
        );
        consumed = __terrane_await(scope.join(consumer)).await;
        consumed_value = consumed.value.clone();
        if (&consumed_value).is_some() {
            println!(
                "{}{}", terrane_scalar_support::scalar_text(&consumed.completed),
                terrane_scalar_support::scalar_text(&match &consumed_value { Some(value)
                => value, _ => unreachable!("flow-proven storage refinement") })
            );
        }
        remote = __terrane_raised(
            remotely_closed_sink(),
            10 /* terrane-site: src/main.trn:27:12-27:33 */,
        );
        rejected = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = (&mut remote).send(String::from("lost"));
                        Box::pin(async move {
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
                                            "terrane_sink_witness",
                                            "terrane_sink_witness::Outgoing::send",
                                        ),
                                    )
                                }
                            }
                        })
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            11 /* terrane-site: src/main.trn:28:20-28:39 */,
                        )
                    }
                })
                .await,
            11 /* terrane-site: src/main.trn:28:20-28:39 */,
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&rejected.accepted),
            terrane_scalar_support::scalar_text(&rejected.closed)
        );
        remote_closed = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = remote.close();
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
                                            "terrane_sink_witness",
                                            "terrane_sink_witness::Outgoing::close",
                                        ),
                                    )
                                }
                            }
                        })
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            12 /* terrane-site: src/main.trn:30:30-30:43 */,
                        )
                    }
                })
                .await,
            12 /* terrane-site: src/main.trn:30:30-30:43 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&remote_closed));
        blocked = __terrane_raised(
            blocked_sink(),
            13 /* terrane-site: src/main.trn:32:13-32:26 */,
        );
        blocked_scope = TerraneTaskScope::new(None);
        blocked_child = {
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
                    Some(Ok(value)) => TerraneTaskResult::Completed(value),
                    Some(Err(error)) => TerraneTaskResult::Failed(error),
                    None => TerraneTaskResult::Cancelled,
                }
            })
        };
        blocked_scope.cancel();
        cancelled = __terrane_await(blocked_scope.join(blocked_child)).await;
        println!("{}", terrane_scalar_support::scalar_text(&cancelled.cancelled));
        queue = __terrane_raised(
            queue_sink(),
            15 /* terrane-site: src/main.trn:38:11-38:22 */,
        );
        queued = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = (&mut queue)
                            .send(String::from("different"));
                        Box::pin(async move {
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
                                            "terrane_sink_witness",
                                            "terrane_sink_witness::QueueSink::send",
                                        ),
                                    )
                                }
                            }
                        })
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            16 /* terrane-site: src/main.trn:39:18-39:41 */,
                        )
                    }
                })
                .await,
            16 /* terrane-site: src/main.trn:39:18-39:41 */,
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&queued.accepted),
            terrane_scalar_support::scalar_text(&queued.closed)
        );
        queue_flushed = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| queue.flush()),
            ) {
                Ok(Ok(value)) => Ok(terrane_int_support::Int::from(i128::from(value))),
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
                            "terrane_sink_witness",
                            "terrane_sink_witness::QueueSink::flush",
                        ),
                    )
                }
            },
            17 /* terrane-site: src/main.trn:41:23-41:35 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&queue_flushed));
        queue_closed = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| queue.close()),
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
                            "terrane_sink_witness",
                            "terrane_sink_witness::QueueSink::close",
                        ),
                    )
                }
            },
            18 /* terrane-site: src/main.trn:43:25-43:37 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&queue_closed));
    });
}
