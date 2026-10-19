// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
async fn exercise() {
    let mut builder: Builder;
    let mut result: terrane_int_support::Int;
    builder = __terrane_raised(
        terrane_static_trn_4275696c646572_new(),
        0 /* terrane-site: src/main.trn:5:15-5:28 */,
    );
    result = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&builder).serve();
                    Box::pin(async move {
                        match crate::__terrane_dependency_await_unwind(
                                std::future::IntoFuture::into_future(__terrane_call),
                            )
                            .await
                        {
                            Ok(Ok(value)) => {
                                Ok(terrane_int_support::Int::from_u128(value as u128))
                            }
                            Ok(Err(error)) => {
                                Err(
                                    crate::TerraneForeignError(
                                        crate::TerraneError::custom_raised(
                                            crate::TERRANE_DEPENDENCY_ERROR,
                                            format!(
                                                "Rust dependency `terrane_borrowed_future_witness` member `terrane_borrowed_future_witness::Builder::serve` failed: {error}"
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
                                        "terrane_borrowed_future_witness",
                                        "terrane_borrowed_future_witness::Builder::serve",
                                    ),
                                )
                            }
                        }
                    })
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        1 /* terrane-site: src/main.trn:6:20-6:34 */,
                    )
                }
            })
            .await,
        1 /* terrane-site: src/main.trn:6:20-6:34 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&result));
    __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let _ = builder
                    .set(
                        match || -> Result<_, crate::TerraneForeignError> {
                            Ok(
                                terrane_int_support::coerce::<
                                    u8,
                                >(&terrane_int_support::Int::from(7_i128))
                                    .map_err(|error| crate::TerraneForeignError(
                                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                    ))?,
                            )
                        }() {
                            Ok(value) => value,
                            Err(error) => std::panic::panic_any(error),
                        },
                    );
            }),
        ) {
            Ok(()) => Ok(()),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_borrowed_future_witness",
                        "terrane_borrowed_future_witness::Builder::set",
                    ),
                )
            }
        },
        2 /* terrane-site: src/main.trn:8:5-8:19 */,
    );
    result = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&builder).serve();
                    Box::pin(async move {
                        match crate::__terrane_dependency_await_unwind(
                                std::future::IntoFuture::into_future(__terrane_call),
                            )
                            .await
                        {
                            Ok(Ok(value)) => {
                                Ok(terrane_int_support::Int::from_u128(value as u128))
                            }
                            Ok(Err(error)) => {
                                Err(
                                    crate::TerraneForeignError(
                                        crate::TerraneError::custom_raised(
                                            crate::TERRANE_DEPENDENCY_ERROR,
                                            format!(
                                                "Rust dependency `terrane_borrowed_future_witness` member `terrane_borrowed_future_witness::Builder::serve` failed: {error}"
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
                                        "terrane_borrowed_future_witness",
                                        "terrane_borrowed_future_witness::Builder::serve",
                                    ),
                                )
                            }
                        }
                    })
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        3 /* terrane-site: src/main.trn:9:20-9:34 */,
                    )
                }
            })
            .await,
        3 /* terrane-site: src/main.trn:9:20-9:34 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&result));
}
fn main() {
    __terrane_run(async move {
        __terrane_group_frame(async move {
                let group: TerraneTaskGroup<()>;
                let outcomes: terrane_collection_support::List<TerraneTaskOutcome<()>>;
                let wrapped: terrane_borrowed_future_witness::Envelope<bool>;
                group = TerraneTaskGroup::<()>::new();
                {
                    let __terrane_child = {
                        let __terrane_scope = group.scope();
                        let __terrane_cancel = __terrane_scope.cancellation();
                        let __terrane_deadline = __terrane_scope.deadline.clone();
                        let __terrane_spawned_task = exercise();
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
                    group.spawn(__terrane_child);
                };
                outcomes = __terrane_await(std::boxed::Box::pin(group.prepare_drain()))
                    .await;
                wrapped = __terrane_raised(
                    terrane_static_trn_456e76656c6f7065_new::<bool>(true),
                    4 /* terrane-site: src/main.trn:16:32-16:61 */,
                );
                println!(
                    "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | wrapped
                    .into_value())) { Ok(value) => Ok(value), Err(payload) => Err(crate
                    ::__terrane_dependency_panic(payload,
                    "terrane_borrowed_future_witness",
                    "terrane_borrowed_future_witness::Envelope<T>::into_value")) },
                    5 /* terrane-site: src/main.trn:17:13-17:32 */)),
                    terrane_scalar_support::scalar_text(&__terrane_raised(outcomes
                    .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
                    6 /* terrane-site: src/main.trn:17:35-17:46 */)),
                    6 /* terrane-site: src/main.trn:17:35-17:46 */).completed)
                );
            })
            .await
    });
}
