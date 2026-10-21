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
                                                "Rust dependency `{}` member `{}` failed: {error}",
                                                "terrane_borrowed_future_witness",
                                                "terrane_borrowed_future_witness::Builder::serve"
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
                                                "Rust dependency `{}` member `{}` failed: {error}",
                                                "terrane_borrowed_future_witness",
                                                "terrane_borrowed_future_witness::Builder::serve"
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
fn present(value: &Option<Builder>) -> bool {
    return (&value).is_some();
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
    pub fn available(&self) -> bool {
        return present(&self.value);
    }
    pub async fn receive(&mut self) -> Result<(), TerraneError> {
        let value: Option<Builder>;
        println!("{}", terrane_scalar_support::scalar_text(&self.available()));
        value = std::mem::take(&mut self.value);
        println!("{}", terrane_scalar_support::scalar_text(&(&self.value).is_some()));
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
                                4 /* terrane-site: src/main.trn:27:19-27:32 */,
                            )
                        }
                    })
                    .await,
                4 /* terrane-site: src/main.trn:27:19-27:32 */,
            )?;
        }
        return Ok(());
    }
    pub async fn __terrane_shared_async_receive(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
    ) -> Result<(), TerraneError> {
        let value: Option<Builder>;
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_call_result =
            __terrane_shared_handle.lock().expect("shared async receiver lock poisoned")
            .available(); __terrane_call_result })
        );
        value = std::mem::take(
            &mut __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .value,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&(&__terrane_shared_handle.lock()
            .expect("shared async receiver lock poisoned").value).is_some())
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
                                4 /* terrane-site: src/main.trn:27:19-27:32 */,
                            )
                        }
                    })
                    .await,
                4 /* terrane-site: src/main.trn:27:19-27:32 */,
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
        5 /* terrane-site: src/main.trn:32:11-32:27 */,
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
                        6 /* terrane-site: src/main.trn:36:11-36:23 */,
                    )
                }
            })
            .await,
        6 /* terrane-site: src/main.trn:36:11-36:23 */,
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
                            7 /* terrane-site: src/main.trn:40:21-40:34 */,
                        )?,
                    ),
                ),
            )
            .await,
        8 /* terrane-site: src/main.trn:40:11-40:35 */,
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
                let mut discarded: Builder;
                let advanced: terrane_int_support::Int;
                let mut local: Option<Builder> = None;
                let mut local_result: Option<terrane_int_support::Int> = None;
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
                    10 /* terrane-site: src/main.trn:50:32-50:61 */,
                );
                println!(
                    "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | wrapped
                    .into_value())) { Ok(value) => Ok(value), Err(payload) => Err(crate
                    ::__terrane_dependency_panic(payload,
                    "terrane_borrowed_future_witness",
                    "terrane_borrowed_future_witness::Envelope::into_value")) },
                    11 /* terrane-site: src/main.trn:51:13-51:32 */)),
                    terrane_scalar_support::scalar_text(&__terrane_raised(outcomes
                    .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
                    12 /* terrane-site: src/main.trn:51:35-51:46 */)),
                    12 /* terrane-site: src/main.trn:51:35-51:46 */).completed)
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
                            13 /* terrane-site: src/main.trn:54:15-54:30 */
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
                            }))). await, 14 /* terrane-site: src/main.trn:59:15-59:36 */
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
                discarded = __terrane_raised(
                    terrane_static_trn_4275696c646572_new(),
                    15 /* terrane-site: src/main.trn:62:17-62:30 */,
                );
                {
                    let _: terrane_int_support::Int = __terrane_traced(
                        __terrane_await({
                                let __terrane_future = {
                                    let __terrane_call = (&mut discarded).advance();
                                    Box::pin(async move {
                                        match crate::__terrane_dependency_await_unwind(
                                                __terrane_call,
                                            )
                                            .await
                                        {
                                            Ok(value) => {
                                                Ok(terrane_int_support::Int::from_u128(value as u128))
                                            }
                                            Err(payload) => {
                                                Err(
                                                    crate::__terrane_dependency_panic(
                                                        payload,
                                                        "terrane_borrowed_future_witness",
                                                        "terrane_borrowed_future_witness::Builder::advance",
                                                    ),
                                                )
                                            }
                                        }
                                    })
                                };
                                async move {
                                    __terrane_raised_err(
                                        __terrane_future.await,
                                        16 /* terrane-site: src/main.trn:63:15-63:33 */,
                                    )
                                }
                            })
                            .await,
                        16 /* terrane-site: src/main.trn:63:15-63:33 */,
                    );
                }
                advanced = __terrane_traced(
                    __terrane_await({
                            let __terrane_future = {
                                let __terrane_call = (&discarded).serve();
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
                                                            "Rust dependency `{}` member `{}` failed: {error}",
                                                            "terrane_borrowed_future_witness",
                                                            "terrane_borrowed_future_witness::Builder::serve"
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
                                    17 /* terrane-site: src/main.trn:64:22-64:38 */,
                                )
                            }
                        })
                        .await,
                    17 /* terrane-site: src/main.trn:64:22-64:38 */,
                );
                println!("{}", terrane_scalar_support::scalar_text(&advanced));
                let __terrane_completion_2: TerraneCompletion<()> = async {
                    let __terrane_try_2: TerraneCompletion<()> = async {
                        {
                            let _: terrane_int_support::Int = __terrane_traced_completion!(
                                __terrane_await({ let __terrane_future = { let
                                __terrane_call = (&discarded).reject_value(); Box::pin(async
                                move { match crate
                                ::__terrane_dependency_await_unwind(__terrane_call). await {
                                Ok(Ok(value)) =>
                                Ok(terrane_int_support::Int::from_u128(value as u128)),
                                Ok(Err(error)) => Err(crate ::TerraneForeignError(crate
                                ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                                .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                                Err(crate ::__terrane_dependency_panic(payload,
                                "terrane_borrowed_future_witness",
                                "terrane_borrowed_future_witness::Builder::reject_value")) }
                                }) }; async move { __terrane_raised_err(__terrane_future.
                                await, 18 /* terrane-site: src/main.trn:67:19-67:42 */) }
                                }). await, 18 /* terrane-site: src/main.trn:67:19-67:42 */
                            );
                        }
                        TerraneCompletion::Normal
                    }
                        .await;
                    match __terrane_try_2 {
                        TerraneCompletion::Return(value) => {
                            return TerraneCompletion::Return(value);
                        }
                        TerraneCompletion::Break => return TerraneCompletion::Break,
                        TerraneCompletion::Continue => return TerraneCompletion::Continue,
                        TerraneCompletion::Normal => {}
                        TerraneCompletion::Error(__terrane_error_2) => {
                            let mut __terrane_handled_2 = false;
                            if !__terrane_handled_2
                                && __terrane_error_2.kind
                                    == TerraneErrorKind::Custom(DescriptorId(2))
                            {
                                __terrane_handled_2 = true;
                                println!("{}", String::from("discard rejected"));
                            }
                            if !__terrane_handled_2 {
                                return TerraneCompletion::Error(__terrane_error_2);
                            }
                        }
                    }
                    TerraneCompletion::Normal
                }
                    .await;
                match __terrane_completion_2 {
                    TerraneCompletion::Normal => {}
                    TerraneCompletion::Return(value) => return value,
                    TerraneCompletion::Error(error) => __terrane_uncaught(error),
                    TerraneCompletion::Break | TerraneCompletion::Continue => {
                        __terrane_generated_defect("loop control escaped a non-loop try")
                    }
                }
                let __terrane_completion_3: TerraneCompletion<()> = async {
                    let __terrane_try_3: TerraneCompletion<()> = async {
                        let _ = local
                            .insert(
                                __terrane_raised_completion!(
                                    terrane_static_trn_4275696c646572_new(),
                                    19 /* terrane-site: src/main.trn:71:17-71:30 */
                                ),
                            );
                        let _ = local_result
                            .insert(
                                __terrane_traced_completion!(
                                    __terrane_await({ let __terrane_future = { let
                                    __terrane_call = (&local.as_ref()
                                    .expect("flow-proven available binding")).serve();
                                    Box::pin(async move { match crate
                                    ::__terrane_dependency_await_unwind(std::future::IntoFuture::into_future(__terrane_call))
                                    . await { Ok(Ok(value)) =>
                                    Ok(terrane_int_support::Int::from_u128(value as u128)),
                                    Ok(Err(error)) => Err(crate ::TerraneForeignError(crate
                                    ::TerraneError::custom_raised(crate
                                    ::TERRANE_DEPENDENCY_ERROR,
                                    format!("Rust dependency `{}` member `{}` failed: {error}",
                                    "terrane_borrowed_future_witness",
                                    "terrane_borrowed_future_witness::Builder::serve"), crate
                                    ::TERRANE_NO_SITE))), Err(payload) => Err(crate
                                    ::__terrane_dependency_panic(payload,
                                    "terrane_borrowed_future_witness",
                                    "terrane_borrowed_future_witness::Builder::serve")) } }) };
                                    async move { __terrane_raised_err(__terrane_future. await,
                                    20 /* terrane-site: src/main.trn:72:30-72:42 */) } }).
                                    await, 20 /* terrane-site: src/main.trn:72:30-72:42 */
                                ),
                            );
                        println!(
                            "{}", terrane_scalar_support::scalar_text(&local_result
                            .as_ref().expect("flow-proven available binding"))
                        );
                        TerraneCompletion::Normal
                    }
                        .await;
                    match __terrane_try_3 {
                        TerraneCompletion::Return(value) => {
                            return TerraneCompletion::Return(value);
                        }
                        TerraneCompletion::Break => return TerraneCompletion::Break,
                        TerraneCompletion::Continue => return TerraneCompletion::Continue,
                        TerraneCompletion::Normal => {}
                        TerraneCompletion::Error(__terrane_error_3) => {
                            let mut __terrane_handled_3 = false;
                            if !__terrane_handled_3 {
                                __terrane_handled_3 = true;
                                println!("{}", String::from("unexpected local failure"));
                            }
                            if !__terrane_handled_3 {
                                return TerraneCompletion::Error(__terrane_error_3);
                            }
                        }
                    }
                    TerraneCompletion::Normal
                }
                    .await;
                match __terrane_completion_3 {
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
