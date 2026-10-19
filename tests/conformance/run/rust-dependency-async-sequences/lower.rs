// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
async fn drain_network(mut sequence: TcpSequence) -> bool {
    let first: terrane_collection_support::AsyncIterationStep<String>;
    let first_value: Option<String>;
    let second: terrane_collection_support::AsyncIterationStep<String>;
    let second_value: Option<String>;
    let ended: terrane_collection_support::AsyncIterationStep<String>;
    first = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    Box::pin(async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                        {
                            Ok(Ok(value)) => {
                                Ok(
                                    match value {
                                        Some(item) => {
                                            terrane_collection_support::AsyncIterationStep::item(item)
                                        }
                                        None => {
                                            terrane_collection_support::AsyncIterationStep::end()
                                        }
                                    },
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::TcpSequence::next",
                                    ),
                                )
                            }
                        }
                    })
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        0 /* terrane-site: src/main.trn:6:17-6:31 */,
                    )
                }
            })
            .await,
        0 /* terrane-site: src/main.trn:6:17-6:31 */,
    );
    first_value = first.value;
    if (&first_value).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &first_value { Some(value)
            => value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    second = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    Box::pin(async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                        {
                            Ok(Ok(value)) => {
                                Ok(
                                    match value {
                                        Some(item) => {
                                            terrane_collection_support::AsyncIterationStep::item(item)
                                        }
                                        None => {
                                            terrane_collection_support::AsyncIterationStep::end()
                                        }
                                    },
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::TcpSequence::next",
                                    ),
                                )
                            }
                        }
                    })
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        1 /* terrane-site: src/main.trn:10:18-10:32 */,
                    )
                }
            })
            .await,
        1 /* terrane-site: src/main.trn:10:18-10:32 */,
    );
    second_value = second.value;
    if (&second_value).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &second_value { Some(value)
            => value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    ended = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    Box::pin(async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                        {
                            Ok(Ok(value)) => {
                                Ok(
                                    match value {
                                        Some(item) => {
                                            terrane_collection_support::AsyncIterationStep::item(item)
                                        }
                                        None => {
                                            terrane_collection_support::AsyncIterationStep::end()
                                        }
                                    },
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::TcpSequence::next",
                                    ),
                                )
                            }
                        }
                    })
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        2 /* terrane-site: src/main.trn:14:17-14:31 */,
                    )
                }
            })
            .await,
        2 /* terrane-site: src/main.trn:14:17-14:31 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&ended.end));
    return __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = sequence.close();
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::TcpSequence::close",
                                    ),
                                )
                            }
                        }
                    })
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        3 /* terrane-site: src/main.trn:16:16-16:31 */,
                    )
                }
            })
            .await,
        3 /* terrane-site: src/main.trn:16:16-16:31 */,
    );
}
async fn drain_tokio(mut sequence: TokioSequence) -> bool {
    let first: terrane_collection_support::AsyncIterationStep<terrane_int_support::Int>;
    let first_value: Option<terrane_int_support::Int>;
    let second: terrane_collection_support::AsyncIterationStep<terrane_int_support::Int>;
    let second_value: Option<terrane_int_support::Int>;
    let ended: terrane_collection_support::AsyncIterationStep<terrane_int_support::Int>;
    first = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    Box::pin(async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                        {
                            Ok(Ok(value)) => {
                                Ok(
                                    match value {
                                        Some(item) => {
                                            terrane_collection_support::AsyncIterationStep::item(
                                                terrane_int_support::Int::from(i128::from(item)),
                                            )
                                        }
                                        None => {
                                            terrane_collection_support::AsyncIterationStep::end()
                                        }
                                    },
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::TokioSequence::next",
                                    ),
                                )
                            }
                        }
                    })
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        4 /* terrane-site: src/main.trn:19:17-19:31 */,
                    )
                }
            })
            .await,
        4 /* terrane-site: src/main.trn:19:17-19:31 */,
    );
    first_value = first.value;
    if (&first_value).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &first_value { Some(value)
            => value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    second = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    Box::pin(async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                        {
                            Ok(Ok(value)) => {
                                Ok(
                                    match value {
                                        Some(item) => {
                                            terrane_collection_support::AsyncIterationStep::item(
                                                terrane_int_support::Int::from(i128::from(item)),
                                            )
                                        }
                                        None => {
                                            terrane_collection_support::AsyncIterationStep::end()
                                        }
                                    },
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::TokioSequence::next",
                                    ),
                                )
                            }
                        }
                    })
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        5 /* terrane-site: src/main.trn:23:18-23:32 */,
                    )
                }
            })
            .await,
        5 /* terrane-site: src/main.trn:23:18-23:32 */,
    );
    second_value = second.value;
    if (&second_value).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &second_value { Some(value)
            => value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    ended = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    Box::pin(async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                        {
                            Ok(Ok(value)) => {
                                Ok(
                                    match value {
                                        Some(item) => {
                                            terrane_collection_support::AsyncIterationStep::item(
                                                terrane_int_support::Int::from(i128::from(item)),
                                            )
                                        }
                                        None => {
                                            terrane_collection_support::AsyncIterationStep::end()
                                        }
                                    },
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::TokioSequence::next",
                                    ),
                                )
                            }
                        }
                    })
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        6 /* terrane-site: src/main.trn:27:17-27:31 */,
                    )
                }
            })
            .await,
        6 /* terrane-site: src/main.trn:27:17-27:31 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&ended.end));
    return __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = sequence.close();
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::TokioSequence::close",
                                    ),
                                )
                            }
                        }
                    })
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        7 /* terrane-site: src/main.trn:29:16-29:31 */,
                    )
                }
            })
            .await,
        7 /* terrane-site: src/main.trn:29:16-29:31 */,
    );
}
async fn drain_queue(mut sequence: QueueSequence) -> bool {
    let first: terrane_collection_support::AsyncIterationStep<String>;
    let first_value: Option<String>;
    let second: terrane_collection_support::AsyncIterationStep<String>;
    let second_value: Option<String>;
    let ended: terrane_collection_support::AsyncIterationStep<String>;
    first = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    Box::pin(async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                        {
                            Ok(Ok(value)) => {
                                Ok(
                                    match value {
                                        Some(item) => {
                                            terrane_collection_support::AsyncIterationStep::item(item)
                                        }
                                        None => {
                                            terrane_collection_support::AsyncIterationStep::end()
                                        }
                                    },
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::QueueSequence::next",
                                    ),
                                )
                            }
                        }
                    })
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        8 /* terrane-site: src/main.trn:32:17-32:31 */,
                    )
                }
            })
            .await,
        8 /* terrane-site: src/main.trn:32:17-32:31 */,
    );
    first_value = first.value;
    if (&first_value).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &first_value { Some(value)
            => value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    second = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    Box::pin(async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                        {
                            Ok(Ok(value)) => {
                                Ok(
                                    match value {
                                        Some(item) => {
                                            terrane_collection_support::AsyncIterationStep::item(item)
                                        }
                                        None => {
                                            terrane_collection_support::AsyncIterationStep::end()
                                        }
                                    },
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::QueueSequence::next",
                                    ),
                                )
                            }
                        }
                    })
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        9 /* terrane-site: src/main.trn:36:18-36:32 */,
                    )
                }
            })
            .await,
        9 /* terrane-site: src/main.trn:36:18-36:32 */,
    );
    second_value = second.value;
    if (&second_value).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &second_value { Some(value)
            => value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    ended = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    Box::pin(async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                        {
                            Ok(Ok(value)) => {
                                Ok(
                                    match value {
                                        Some(item) => {
                                            terrane_collection_support::AsyncIterationStep::item(item)
                                        }
                                        None => {
                                            terrane_collection_support::AsyncIterationStep::end()
                                        }
                                    },
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::QueueSequence::next",
                                    ),
                                )
                            }
                        }
                    })
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        10 /* terrane-site: src/main.trn:40:17-40:31 */,
                    )
                }
            })
            .await,
        10 /* terrane-site: src/main.trn:40:17-40:31 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&ended.end));
    return __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| sequence.close()),
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
                        "terrane_sequence_witness",
                        "terrane_sequence_witness::QueueSequence::close",
                    ),
                )
            }
        },
        11 /* terrane-site: src/main.trn:42:10-42:25 */,
    );
}
async fn wait_pending_tokio(mut sequence: TokioSequence) {
    let waiting: terrane_collection_support::AsyncIterationStep<
        terrane_int_support::Int,
    >;
    waiting = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    Box::pin(async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                        {
                            Ok(Ok(value)) => {
                                Ok(
                                    match value {
                                        Some(item) => {
                                            terrane_collection_support::AsyncIterationStep::item(
                                                terrane_int_support::Int::from(i128::from(item)),
                                            )
                                        }
                                        None => {
                                            terrane_collection_support::AsyncIterationStep::end()
                                        }
                                    },
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::TokioSequence::next",
                                    ),
                                )
                            }
                        }
                    })
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        12 /* terrane-site: src/main.trn:45:19-45:33 */,
                    )
                }
            })
            .await,
        12 /* terrane-site: src/main.trn:45:19-45:33 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&waiting.end));
}
async fn wait_pending_queue(mut sequence: QueueSequence) {
    let waiting: terrane_collection_support::AsyncIterationStep<String>;
    waiting = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    Box::pin(async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                        {
                            Ok(Ok(value)) => {
                                Ok(
                                    match value {
                                        Some(item) => {
                                            terrane_collection_support::AsyncIterationStep::item(item)
                                        }
                                        None => {
                                            terrane_collection_support::AsyncIterationStep::end()
                                        }
                                    },
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::QueueSequence::next",
                                    ),
                                )
                            }
                        }
                    })
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        13 /* terrane-site: src/main.trn:49:19-49:33 */,
                    )
                }
            })
            .await,
        13 /* terrane-site: src/main.trn:49:19-49:33 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&waiting.end));
}
fn main() {
    __terrane_run(async move {
        let network_drained: bool;
        let tokio_drained: bool;
        let queue_drained: bool;
        let mut failing: Option<TokioSequence> = None;
        let mut failed_step: Option<
            terrane_collection_support::AsyncIterationStep<terrane_int_support::Int>,
        > = None;
        let mut error: Option<TerraneError> = None;
        let mut failing_queue: Option<QueueSequence> = None;
        let mut failed_queue_step: Option<
            terrane_collection_support::AsyncIterationStep<String>,
        > = None;
        let mut error_2: Option<TerraneError> = None;
        let tokio_scope: TerraneTaskScope;
        let pending: TokioSequence;
        let tokio_child: TerraneScopedTask<()>;
        let tokio_outcome: TerraneTaskOutcome<()>;
        let queue_scope: TerraneTaskScope;
        let pending_queue: QueueSequence;
        let queue_child: TerraneScopedTask<()>;
        let queue_outcome: TerraneTaskOutcome<()>;
        network_drained = __terrane_await(
                drain_network(
                    __terrane_traced(
                        __terrane_await({
                                let __terrane_future = make_tcp_sequence();
                                async move {
                                    __terrane_raised_err(
                                        __terrane_future.await,
                                        14 /* terrane-site: src/main.trn:53:49-53:67 */,
                                    )
                                }
                            })
                            .await,
                        14 /* terrane-site: src/main.trn:53:49-53:67 */,
                    ),
                ),
            )
            .await;
        println!("{}", terrane_scalar_support::scalar_text(&network_drained));
        tokio_drained = __terrane_await(
                drain_tokio(
                    __terrane_raised(
                        make_tokio_sequence(),
                        15 /* terrane-site: src/main.trn:55:39-55:59 */,
                    ),
                ),
            )
            .await;
        println!("{}", terrane_scalar_support::scalar_text(&tokio_drained));
        queue_drained = __terrane_await(
                drain_queue(
                    __terrane_raised(
                        make_queue_sequence(),
                        16 /* terrane-site: src/main.trn:57:39-57:59 */,
                    ),
                ),
            )
            .await;
        println!("{}", terrane_scalar_support::scalar_text(&queue_drained));
        let __terrane_completion_0: TerraneCompletion<()> = async {
            let __terrane_try_0: TerraneCompletion<()> = async {
                let _ = failing
                    .insert(
                        __terrane_raised_completion!(
                            make_failing_tokio_sequence(), 17 /* terrane-site: src/main.trn:60:15-60:43 */
                        ),
                    );
                let _ = failed_step
                    .insert(
                        __terrane_traced_completion!(
                            __terrane_await({ let __terrane_future = { let __terrane_call
                            = failing.as_mut().expect("flow-proven available binding")
                            .next(); Box::pin(async move { match crate
                            ::__terrane_dependency_await_unwind(__terrane_call). await {
                            Ok(Ok(value)) => Ok(match value { Some(item) =>
                            terrane_collection_support::AsyncIterationStep::item(terrane_int_support::Int::from(i128::from(item))),
                            None => terrane_collection_support::AsyncIterationStep::end()
                            }), Ok(Err(error)) => Err(crate ::TerraneForeignError(crate
                            ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                            .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                            Err(crate ::__terrane_dependency_panic(payload,
                            "terrane_sequence_witness",
                            "terrane_sequence_witness::TokioSequence::next")) } }) };
                            async move { __terrane_raised_err(__terrane_future. await,
                            18 /* terrane-site: src/main.trn:61:25-61:38 */) } }).
                            await, 18 /* terrane-site: src/main.trn:61:25-61:38 */
                        ),
                    );
                println!(
                    "{}", terrane_scalar_support::scalar_text(&failed_step.as_ref()
                    .expect("flow-proven available binding").clone().end)
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
                        let _ = error.insert(__terrane_error_0.clone());
                        println!(
                            "{}", terrane_scalar_support::scalar_text(&error.as_ref()
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
        let __terrane_completion_1: TerraneCompletion<()> = async {
            let __terrane_try_1: TerraneCompletion<()> = async {
                let _ = failing_queue
                    .insert(
                        __terrane_raised_completion!(
                            make_failing_queue_sequence(), 19 /* terrane-site: src/main.trn:66:21-66:49 */
                        ),
                    );
                let _ = failed_queue_step
                    .insert(
                        __terrane_traced_completion!(
                            __terrane_await({ let __terrane_future = { let __terrane_call
                            = failing_queue.as_mut()
                            .expect("flow-proven available binding").next();
                            Box::pin(async move { match crate
                            ::__terrane_dependency_await_unwind(__terrane_call). await {
                            Ok(Ok(value)) => Ok(match value { Some(item) =>
                            terrane_collection_support::AsyncIterationStep::item(item),
                            None => terrane_collection_support::AsyncIterationStep::end()
                            }), Ok(Err(error)) => Err(crate ::TerraneForeignError(crate
                            ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                            .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                            Err(crate ::__terrane_dependency_panic(payload,
                            "terrane_sequence_witness",
                            "terrane_sequence_witness::QueueSequence::next")) } }) };
                            async move { __terrane_raised_err(__terrane_future. await,
                            20 /* terrane-site: src/main.trn:67:31-67:50 */) } }).
                            await, 20 /* terrane-site: src/main.trn:67:31-67:50 */
                        ),
                    );
                println!(
                    "{}", terrane_scalar_support::scalar_text(&failed_queue_step.as_ref()
                    .expect("flow-proven available binding").clone().end)
                );
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
                            == TerraneErrorKind::Custom(DescriptorId(2))
                    {
                        __terrane_handled_1 = true;
                        let _ = error_2.insert(__terrane_error_1.clone());
                        println!(
                            "{}", terrane_scalar_support::scalar_text(&error_2.as_ref()
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
        tokio_scope = TerraneTaskScope::new(None);
        pending = __terrane_raised(
            make_pending_tokio_sequence(),
            21 /* terrane-site: src/main.trn:72:13-72:41 */,
        );
        tokio_child = {
            let __terrane_scope = tokio_scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = wait_pending_tokio(pending);
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
        tokio_scope.cancel();
        tokio_outcome = __terrane_await(tokio_scope.join(tokio_child)).await;
        println!("{}", terrane_scalar_support::scalar_text(&tokio_outcome.cancelled));
        queue_scope = TerraneTaskScope::new(None);
        pending_queue = __terrane_raised(
            make_pending_queue_sequence(),
            22 /* terrane-site: src/main.trn:78:19-78:47 */,
        );
        queue_child = {
            let __terrane_scope = queue_scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = wait_pending_queue(pending_queue);
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
        queue_scope.cancel();
        queue_outcome = __terrane_await(queue_scope.join(queue_child)).await;
        println!("{}", terrane_scalar_support::scalar_text(&queue_outcome.cancelled));
    });
}
