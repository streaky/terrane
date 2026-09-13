// Generated deterministically by Terrane <version>.
// Runtime support: async_native.rs, executor_local.rs, async_dependency.rs, tasks_native_local.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: src/main.trn
// Namespace: app
async fn drain_network(mut sequence: TcpSequence) -> bool {
    let first: terrane_collection_support::AsyncIterationStep<String> = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    async move {
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
                                            crate::TERRANE_DEPENDENCY_ERROR,
                                            format!(
                                                "Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::TcpSequence::next` failed: {error}"
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::TcpSequence::next",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        0 /* terrane-site: src/main.trn:7:17-7:31 */,
                    )
                }
            })
            .await,
        0 /* terrane-site: src/main.trn:7:17-7:31 */,
    );
    let first_value: Option<String> = first.value;
    if first_value.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* first_value.as_ref()
            .expect("semantic optional narrowing"))
        );
    }
    let second: terrane_collection_support::AsyncIterationStep<String> = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    async move {
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
                                            crate::TERRANE_DEPENDENCY_ERROR,
                                            format!(
                                                "Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::TcpSequence::next` failed: {error}"
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::TcpSequence::next",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        1 /* terrane-site: src/main.trn:11:18-11:32 */,
                    )
                }
            })
            .await,
        1 /* terrane-site: src/main.trn:11:18-11:32 */,
    );
    let second_value: Option<String> = second.value;
    if second_value.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* second_value.as_ref()
            .expect("semantic optional narrowing"))
        );
    }
    let ended: terrane_collection_support::AsyncIterationStep<String> = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    async move {
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
                                            crate::TERRANE_DEPENDENCY_ERROR,
                                            format!(
                                                "Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::TcpSequence::next` failed: {error}"
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::TcpSequence::next",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        2 /* terrane-site: src/main.trn:15:17-15:31 */,
                    )
                }
            })
            .await,
        2 /* terrane-site: src/main.trn:15:17-15:31 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&ended.end));
    return __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = sequence.close();
                    async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                        {
                            Ok(Ok(value)) => Ok(value),
                            Ok(Err(error)) => {
                                Err(
                                    crate::TerraneForeignError(
                                        crate::TerraneError::custom_raised(
                                            crate::TERRANE_DEPENDENCY_ERROR,
                                            format!(
                                                "Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::TcpSequence::close` failed: {error}"
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::TcpSequence::close",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        3 /* terrane-site: src/main.trn:17:16-17:31 */,
                    )
                }
            })
            .await,
        3 /* terrane-site: src/main.trn:17:16-17:31 */,
    );
}
async fn drain_tokio(mut sequence: TokioSequence) -> bool {
    let first: terrane_collection_support::AsyncIterationStep<
        terrane_int_support::Int,
    > = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    async move {
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
                                            crate::TERRANE_DEPENDENCY_ERROR,
                                            format!(
                                                "Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::TokioSequence::next` failed: {error}"
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::TokioSequence::next",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        4 /* terrane-site: src/main.trn:20:17-20:31 */,
                    )
                }
            })
            .await,
        4 /* terrane-site: src/main.trn:20:17-20:31 */,
    );
    let first_value: Option<terrane_int_support::Int> = first.value;
    if first_value.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* first_value.as_ref()
            .expect("semantic optional narrowing"))
        );
    }
    let second: terrane_collection_support::AsyncIterationStep<
        terrane_int_support::Int,
    > = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    async move {
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
                                            crate::TERRANE_DEPENDENCY_ERROR,
                                            format!(
                                                "Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::TokioSequence::next` failed: {error}"
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::TokioSequence::next",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        5 /* terrane-site: src/main.trn:24:18-24:32 */,
                    )
                }
            })
            .await,
        5 /* terrane-site: src/main.trn:24:18-24:32 */,
    );
    let second_value: Option<terrane_int_support::Int> = second.value;
    if second_value.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* second_value.as_ref()
            .expect("semantic optional narrowing"))
        );
    }
    let ended: terrane_collection_support::AsyncIterationStep<
        terrane_int_support::Int,
    > = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    async move {
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
                                            crate::TERRANE_DEPENDENCY_ERROR,
                                            format!(
                                                "Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::TokioSequence::next` failed: {error}"
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::TokioSequence::next",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        6 /* terrane-site: src/main.trn:28:17-28:31 */,
                    )
                }
            })
            .await,
        6 /* terrane-site: src/main.trn:28:17-28:31 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&ended.end));
    return __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = sequence.close();
                    async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                        {
                            Ok(Ok(value)) => Ok(value),
                            Ok(Err(error)) => {
                                Err(
                                    crate::TerraneForeignError(
                                        crate::TerraneError::custom_raised(
                                            crate::TERRANE_DEPENDENCY_ERROR,
                                            format!(
                                                "Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::TokioSequence::close` failed: {error}"
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::TokioSequence::close",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        7 /* terrane-site: src/main.trn:30:16-30:31 */,
                    )
                }
            })
            .await,
        7 /* terrane-site: src/main.trn:30:16-30:31 */,
    );
}
async fn drain_queue(mut sequence: QueueSequence) -> bool {
    let first: terrane_collection_support::AsyncIterationStep<String> = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    async move {
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
                                            crate::TERRANE_DEPENDENCY_ERROR,
                                            format!(
                                                "Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::QueueSequence::next` failed: {error}"
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::QueueSequence::next",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        8 /* terrane-site: src/main.trn:33:17-33:31 */,
                    )
                }
            })
            .await,
        8 /* terrane-site: src/main.trn:33:17-33:31 */,
    );
    let first_value: Option<String> = first.value;
    if first_value.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* first_value.as_ref()
            .expect("semantic optional narrowing"))
        );
    }
    let second: terrane_collection_support::AsyncIterationStep<String> = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    async move {
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
                                            crate::TERRANE_DEPENDENCY_ERROR,
                                            format!(
                                                "Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::QueueSequence::next` failed: {error}"
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::QueueSequence::next",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        9 /* terrane-site: src/main.trn:37:18-37:32 */,
                    )
                }
            })
            .await,
        9 /* terrane-site: src/main.trn:37:18-37:32 */,
    );
    let second_value: Option<String> = second.value;
    if second_value.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* second_value.as_ref()
            .expect("semantic optional narrowing"))
        );
    }
    let ended: terrane_collection_support::AsyncIterationStep<String> = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    async move {
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
                                            crate::TERRANE_DEPENDENCY_ERROR,
                                            format!(
                                                "Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::QueueSequence::next` failed: {error}"
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::QueueSequence::next",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        10 /* terrane-site: src/main.trn:41:17-41:31 */,
                    )
                }
            })
            .await,
        10 /* terrane-site: src/main.trn:41:17-41:31 */,
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
                            crate::TERRANE_DEPENDENCY_ERROR,
                            format!(
                                "Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::QueueSequence::close` failed: {error}"
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
                        "terrane_sequence_witness",
                        "terrane_sequence_witness::QueueSequence::close",
                    ),
                )
            }
        },
        11 /* terrane-site: src/main.trn:43:10-43:25 */,
    );
}
async fn wait_pending_tokio(mut sequence: TokioSequence) {
    let waiting: terrane_collection_support::AsyncIterationStep<
        terrane_int_support::Int,
    > = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    async move {
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
                                            crate::TERRANE_DEPENDENCY_ERROR,
                                            format!(
                                                "Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::TokioSequence::next` failed: {error}"
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::TokioSequence::next",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        12 /* terrane-site: src/main.trn:46:19-46:33 */,
                    )
                }
            })
            .await,
        12 /* terrane-site: src/main.trn:46:19-46:33 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&waiting.end));
}
async fn wait_pending_queue(mut sequence: QueueSequence) {
    let waiting: terrane_collection_support::AsyncIterationStep<String> = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    async move {
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
                                            crate::TERRANE_DEPENDENCY_ERROR,
                                            format!(
                                                "Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::QueueSequence::next` failed: {error}"
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::QueueSequence::next",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        13 /* terrane-site: src/main.trn:50:19-50:33 */,
                    )
                }
            })
            .await,
        13 /* terrane-site: src/main.trn:50:19-50:33 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&waiting.end));
}
fn main() {
    __terrane_run(async move {
        let network_drained: bool = __terrane_await(
                drain_network(
                    __terrane_traced(
                        __terrane_await({
                                let __terrane_future = make_tcp_sequence();
                                async move {
                                    __terrane_raised_err(
                                        __terrane_future.await,
                                        14 /* terrane-site: src/main.trn:54:49-54:67 */,
                                    )
                                }
                            })
                            .await,
                        14 /* terrane-site: src/main.trn:54:49-54:67 */,
                    ),
                ),
            )
            .await;
        println!("{}", terrane_scalar_support::scalar_text(&network_drained));
        let tokio_drained: bool = __terrane_await(
                drain_tokio(
                    __terrane_raised(
                        make_tokio_sequence(),
                        15 /* terrane-site: src/main.trn:56:39-56:59 */,
                    ),
                ),
            )
            .await;
        println!("{}", terrane_scalar_support::scalar_text(&tokio_drained));
        let queue_drained: bool = __terrane_await(
                drain_queue(
                    __terrane_raised(
                        make_queue_sequence(),
                        16 /* terrane-site: src/main.trn:58:39-58:59 */,
                    ),
                ),
            )
            .await;
        println!("{}", terrane_scalar_support::scalar_text(&queue_drained));
        let __terrane_completion_0: TerraneCompletion<()> = async {
            let __terrane_try_0: TerraneCompletion<()> = async {
                let mut failing: TokioSequence = __terrane_raised_completion!(
                    make_failing_tokio_sequence(), 17 /* terrane-site: src/main.trn:61:15-61:43 */
                );
                let failed_step: terrane_collection_support::AsyncIterationStep<
                    terrane_int_support::Int,
                > = __terrane_traced_completion!(
                    __terrane_await({ let __terrane_future = { let __terrane_call = (&mut
                    failing).next(); async move { match crate
                    ::__terrane_dependency_await_unwind(__terrane_call). await {
                    Ok(Ok(value)) => Ok(match value { Some(item) =>
                    terrane_collection_support::AsyncIterationStep::item(terrane_int_support::Int::from(i128::from(item))),
                    None => terrane_collection_support::AsyncIterationStep::end() }),
                    Ok(Err(error)) => Err(crate ::TerraneForeignError(crate
                    ::TerraneError::custom_raised(crate ::TERRANE_DEPENDENCY_ERROR,
                    format!("Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::TokioSequence::next` failed: {error}"),
                    crate ::TERRANE_NO_SITE))), Err(payload) => Err(crate
                    ::__terrane_dependency_panic(payload, "terrane_sequence_witness",
                    "terrane_sequence_witness::TokioSequence::next")) } } }; async move {
                    __terrane_raised_err(__terrane_future. await, 18 /* terrane-site: src/main.trn:62:25-62:38 */) } }). await, 18 /* terrane-site: src/main.trn:62:25-62:38 */
                );
                println!("{}", terrane_scalar_support::scalar_text(&failed_step.end));
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
                        let error = __terrane_error_0.clone();
                        println!(
                            "{}", terrane_scalar_support::scalar_text(&error.message()
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
                let mut failing_queue: QueueSequence = __terrane_raised_completion!(
                    make_failing_queue_sequence(), 19 /* terrane-site: src/main.trn:67:21-67:49 */
                );
                let failed_queue_step: terrane_collection_support::AsyncIterationStep<
                    String,
                > = __terrane_traced_completion!(
                    __terrane_await({ let __terrane_future = { let __terrane_call = (&mut
                    failing_queue).next(); async move { match crate
                    ::__terrane_dependency_await_unwind(__terrane_call). await {
                    Ok(Ok(value)) => Ok(match value { Some(item) =>
                    terrane_collection_support::AsyncIterationStep::item(item), None =>
                    terrane_collection_support::AsyncIterationStep::end() }),
                    Ok(Err(error)) => Err(crate ::TerraneForeignError(crate
                    ::TerraneError::custom_raised(crate ::TERRANE_DEPENDENCY_ERROR,
                    format!("Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::QueueSequence::next` failed: {error}"),
                    crate ::TERRANE_NO_SITE))), Err(payload) => Err(crate
                    ::__terrane_dependency_panic(payload, "terrane_sequence_witness",
                    "terrane_sequence_witness::QueueSequence::next")) } } }; async move {
                    __terrane_raised_err(__terrane_future. await, 20 /* terrane-site: src/main.trn:68:31-68:50 */) } }). await, 20 /* terrane-site: src/main.trn:68:31-68:50 */
                );
                println!(
                    "{}", terrane_scalar_support::scalar_text(&failed_queue_step.end)
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
                            == TerraneErrorKind::Custom(DescriptorId(0))
                    {
                        __terrane_handled_1 = true;
                        let error = __terrane_error_1.clone();
                        println!(
                            "{}", terrane_scalar_support::scalar_text(&error.message()
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
        let tokio_scope: TerraneTaskScope = TerraneTaskScope::new(None);
        let pending: TokioSequence = __terrane_raised(
            make_pending_tokio_sequence(),
            21 /* terrane-site: src/main.trn:73:13-73:41 */,
        );
        let tokio_child: TerraneScopedTask<()> = {
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
        let tokio_outcome: TerraneTaskOutcome<()> = __terrane_await(
                tokio_scope.join(tokio_child),
            )
            .await;
        println!("{}", terrane_scalar_support::scalar_text(&tokio_outcome.cancelled));
        let queue_scope: TerraneTaskScope = TerraneTaskScope::new(None);
        let pending_queue: QueueSequence = __terrane_raised(
            make_pending_queue_sequence(),
            22 /* terrane-site: src/main.trn:79:19-79:47 */,
        );
        let queue_child: TerraneScopedTask<()> = {
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
        let queue_outcome: TerraneTaskOutcome<()> = __terrane_await(
                queue_scope.join(queue_child),
            )
            .await;
        println!("{}", terrane_scalar_support::scalar_text(&queue_outcome.cancelled));
    });
}
// Source: <terrane>/projected/deps/terrane-sequence-witness.trn
// Namespace: deps/terrane-sequence-witness
pub use terrane_sequence_witness::QueueSequence;
pub use terrane_sequence_witness::TcpSequence;
pub use terrane_sequence_witness::TokioSequence;
pub fn make_failing_queue_sequence() -> Result<
    QueueSequence,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_sequence_witness::make_failing_queue_sequence()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-sequence-witness",
                    "terrane_sequence_witness::make_failing_queue_sequence",
                ),
            )
        }
    }
}
pub fn make_failing_tokio_sequence() -> Result<
    TokioSequence,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_sequence_witness::make_failing_tokio_sequence()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-sequence-witness",
                    "terrane_sequence_witness::make_failing_tokio_sequence",
                ),
            )
        }
    }
}
pub fn make_pending_queue_sequence() -> Result<
    QueueSequence,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_sequence_witness::make_pending_queue_sequence()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-sequence-witness",
                    "terrane_sequence_witness::make_pending_queue_sequence",
                ),
            )
        }
    }
}
pub fn make_pending_tokio_sequence() -> Result<
    TokioSequence,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_sequence_witness::make_pending_tokio_sequence()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-sequence-witness",
                    "terrane_sequence_witness::make_pending_tokio_sequence",
                ),
            )
        }
    }
}
pub fn make_queue_sequence() -> Result<QueueSequence, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_sequence_witness::make_queue_sequence()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-sequence-witness",
                    "terrane_sequence_witness::make_queue_sequence",
                ),
            )
        }
    }
}
pub async fn make_tcp_sequence() -> Result<TcpSequence, crate::TerraneForeignError> {
    match crate::__terrane_dependency_await_unwind(
            terrane_sequence_witness::make_tcp_sequence(),
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
                            "Rust dependency `terrane-sequence-witness` member `terrane_sequence_witness::make_tcp_sequence` failed: {error}"
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
                    "terrane-sequence-witness",
                    "terrane_sequence_witness::make_tcp_sequence",
                ),
            )
        }
    }
}
pub fn make_tokio_sequence() -> Result<TokioSequence, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_sequence_witness::make_tokio_sequence()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-sequence-witness",
                    "terrane_sequence_witness::make_tokio_sequence",
                ),
            )
        }
    }
}
