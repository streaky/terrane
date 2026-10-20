// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
async fn exercise() -> Result<(), TerraneError> {
    let mut builder: Builder;
    let mut result: terrane_int_support::Int;
    builder = __terrane_raised_err(
        terrane_static_trn_4275696c646572_new(),
        0 /* terrane-site: src/main.trn:6:15-6:28 */,
    )?;
    result = __terrane_traced_err(
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
                        1 /* terrane-site: src/main.trn:7:20-7:34 */,
                    )
                }
            })
            .await,
        1 /* terrane-site: src/main.trn:7:20-7:34 */,
    )?;
    println!("{}", terrane_scalar_support::scalar_text(&result));
    __terrane_raised_err(
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
        2 /* terrane-site: src/main.trn:9:5-9:19 */,
    )?;
    result = __terrane_traced_err(
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
                        3 /* terrane-site: src/main.trn:10:20-10:34 */,
                    )
                }
            })
            .await,
        3 /* terrane-site: src/main.trn:10:20-10:34 */,
    )?;
    println!("{}", terrane_scalar_support::scalar_text(&result));
    Ok(())
}
pub struct Receiver {
    pub value: Option<Builder>,
}
impl Receiver {
    pub fn terrane_construct(value: Option<Builder>) -> Self {
        let mut __terrane_constructed_value = Self { value: None };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: Option<Builder>) {
        {
            let __terrane_value = value;
            self.value = __terrane_value;
        }
    }
    pub async fn receive(&mut self) -> Result<(), TerraneError> {
        let value: Option<Builder>;
        value = std::mem::take(&mut self.value);
        if (&value).is_some() {
            __terrane_traced_err(
                __terrane_await({
                        let __terrane_future = {
                            let __terrane_call = (&match &value {
                                Some(value) => value,
                                _ => unreachable!("flow-proven storage refinement"),
                            })
                                .reject();
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
                                                "terrane_borrowed_future_witness",
                                                "terrane_borrowed_future_witness::Builder::reject",
                                            ),
                                        )
                                    }
                                }
                            })
                        };
                        async move {
                            __terrane_raised_err(
                                __terrane_future.await,
                                4 /* terrane-site: src/main.trn:20:19-20:32 */,
                            )
                        }
                    })
                    .await,
                4 /* terrane-site: src/main.trn:20:19-20:32 */,
            )?;
        }
        return Ok(());
    }
    pub async fn __terrane_shared_async_receive(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
    ) -> Result<(), TerraneError> {
        let value: Option<Builder>;
        value = std::mem::take(
            &mut __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .value,
        );
        if (&value).is_some() {
            __terrane_traced_err(
                __terrane_await({
                        let __terrane_future = {
                            let __terrane_call = (&match &value {
                                Some(value) => value,
                                _ => unreachable!("flow-proven storage refinement"),
                            })
                                .reject();
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
                                                "terrane_borrowed_future_witness",
                                                "terrane_borrowed_future_witness::Builder::reject",
                                            ),
                                        )
                                    }
                                }
                            })
                        };
                        async move {
                            __terrane_raised_err(
                                __terrane_future.await,
                                4 /* terrane-site: src/main.trn:20:19-20:32 */,
                            )
                        }
                    })
                    .await,
                4 /* terrane-site: src/main.trn:20:19-20:32 */,
            )?;
        }
        return Ok(());
    }
}
async fn rejected(value: Option<Builder>) -> Result<(), TerraneError> {
    let mut session: Receiver;
    session = Receiver::terrane_construct(value);
    __terrane_traced_err(
        __terrane_await((&mut session).receive()).await,
        5 /* terrane-site: src/main.trn:25:11-25:27 */,
    )?;
    return Ok(());
}
async fn rejected_free(__trn_5f76616c7565: Option<Builder>) -> Result<(), TerraneError> {
    let _ = &__trn_5f76616c7565;
    __terrane_traced_err(
        __terrane_await({
                let __terrane_future = reject_free();
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        6 /* terrane-site: src/main.trn:29:11-29:23 */,
                    )
                }
            })
            .await,
        6 /* terrane-site: src/main.trn:29:11-29:23 */,
    )?;
    return Ok(());
}
async fn invoke(
    handler: std::sync::Arc<
        dyn Fn(
            Option<Builder>,
        ) -> std::pin::Pin<
                Box<dyn Future<Output = Result<(), TerraneError>> + Send>,
            > + Send + Sync,
    >,
) -> Result<(), TerraneError> {
    __terrane_traced_err(
        __terrane_await(
                handler(
                    Some(
                        __terrane_raised_err(
                            terrane_static_trn_4275696c646572_new(),
                            7 /* terrane-site: src/main.trn:33:21-33:34 */,
                        )?,
                    ),
                ),
            )
            .await,
        8 /* terrane-site: src/main.trn:33:11-33:35 */,
    )?;
    return Ok(());
}
fn describe(value: TerraneError) -> String {
    return value.message().to_owned().clone();
}
fn main() {
    __terrane_run(async move {
        __terrane_group_frame(async move {
                let group: TerraneTaskGroup<()>;
                let outcomes: terrane_collection_support::List<TerraneTaskOutcome<()>>;
                let wrapped: terrane_borrowed_future_witness::Envelope<bool>;
                let handler: std::sync::Arc<
                    dyn Fn(
                        Option<Builder>,
                    ) -> std::pin::Pin<
                            Box<dyn Future<Output = Result<(), TerraneError>> + Send>,
                        > + Send + Sync,
                >;
                let mut error: Option<TerraneError> = None;
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
                                Some(Ok(value)) => TerraneTaskResult::Completed(value),
                                Some(Err(error)) => {
                                    __terrane_scope.cancel();
                                    TerraneTaskResult::Failed(error)
                                }
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
                    10 /* terrane-site: src/main.trn:43:32-43:61 */,
                );
                println!(
                    "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | wrapped
                    .into_value())) { Ok(value) => Ok(value), Err(payload) => Err(crate
                    ::__terrane_dependency_panic(payload,
                    "terrane_borrowed_future_witness",
                    "terrane_borrowed_future_witness::Envelope<T>::into_value")) },
                    11 /* terrane-site: src/main.trn:44:13-44:32 */)),
                    terrane_scalar_support::scalar_text(&__terrane_raised(outcomes
                    .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
                    12 /* terrane-site: src/main.trn:44:35-44:46 */)),
                    12 /* terrane-site: src/main.trn:44:35-44:46 */).completed)
                );
                handler = std::sync::Arc::new(move |
                    argument_0: Option<Builder>,
                | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
                    Box::pin(rejected(argument_0))
                });
                let __terrane_completion_0: TerraneCompletion<()> = async {
                    let __terrane_try_0: TerraneCompletion<()> = async {
                        __terrane_traced_completion!(
                            __terrane_await(invoke(handler.clone())). await,
                            13 /* terrane-site: src/main.trn:47:15-47:30 */
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
                                println!("{}", String::from("caught method error"));
                                println!(
                                    "{}", terrane_scalar_support::scalar_text(&describe(error
                                    .as_ref().expect("flow-proven available binding").clone()))
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
                        __terrane_traced_completion!(
                            __terrane_await(invoke(std::sync::Arc::new(move | argument_0
                            : Option < Builder >| -> std::pin::Pin < Box < dyn Future <
                            Output = _ > + Send >> { Box::pin(rejected_free(argument_0))
                            }))). await, 14 /* terrane-site: src/main.trn:52:15-52:36 */
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
                            if !__terrane_handled_1 {
                                __terrane_handled_1 = true;
                                println!("{}", String::from("caught function error"));
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
            })
            .await
    });
}
