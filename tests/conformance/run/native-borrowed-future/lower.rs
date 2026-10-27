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
                let named: String;
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
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await({
                    let __terrane_future =
                    read_literal_later(String::from("borrowed\nλ")); async move {
                    __terrane_raised_err(__terrane_future. await, 9 /* terrane-site: src/main.trn:47:19-47:52 */) } }). await, 9 /* terrane-site: src/main.trn:47:19-47:52 */))
                );
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let
                    __terrane_native_input_0 =
                    __terrane_raised(terrane_static_trn_4f7074696f6e73_new(),
                    10 /* terrane-site: src/main.trn:48:14-48:27 */); let
                    __terrane_native_input_1 = "generic\nλ"; __terrane_raised(match
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                    __terrane_native_input_0.with_label(__terrane_native_input_1))) {
                    Ok(value) => Ok(value), Err(payload) => Err(crate
                    ::__terrane_dependency_panic(payload,
                    "terrane_borrowed_future_witness",
                    "terrane_borrowed_future_witness::Options::with_label")) },
                    11 /* terrane-site: src/main.trn:48:13-48:53 */) })
                );
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let
                    __terrane_native_input_0 =
                    __terrane_raised(terrane_static_trn_4f7074696f6e73_new(),
                    12 /* terrane-site: src/main.trn:49:14-49:27 */); let
                    __terrane_native_input_1 = "direct escaped\tvalue";
                    __terrane_raised(match
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                    __terrane_native_input_0
                    .with_borrowed_label(__terrane_native_input_1))) { Ok(value) =>
                    Ok(value), Err(payload) => Err(crate
                    ::__terrane_dependency_panic(payload,
                    "terrane_borrowed_future_witness",
                    "terrane_borrowed_future_witness::Options::with_borrowed_label")) },
                    13 /* terrane-site: src/main.trn:49:13-49:73 */) })
                );
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let
                    __terrane_native_input_0 =
                    __terrane_raised(terrane_static_trn_4f7074696f6e73_new(),
                    14 /* terrane-site: src/main.trn:50:14-50:27 */); let
                    __terrane_native_input_1 = String::from("owned parameter");
                    __terrane_raised(match
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                    __terrane_native_input_0.with_owned_label(__terrane_native_input_1)))
                    { Ok(value) => Ok(value), Err(payload) => Err(crate
                    ::__terrane_dependency_panic(payload,
                    "terrane_borrowed_future_witness",
                    "terrane_borrowed_future_witness::Options::with_owned_label")) },
                    15 /* terrane-site: src/main.trn:50:13-50:64 */) })
                );
                named = String::from("named value survives");
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let
                    __terrane_native_input_0 =
                    __terrane_raised(terrane_static_trn_4f7074696f6e73_new(),
                    16 /* terrane-site: src/main.trn:52:14-52:27 */); let
                    __terrane_native_input_1 = named.as_str(); __terrane_raised(match
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                    __terrane_native_input_0.with_label(__terrane_native_input_1))) {
                    Ok(value) => Ok(value), Err(payload) => Err(crate
                    ::__terrane_dependency_panic(payload,
                    "terrane_borrowed_future_witness",
                    "terrane_borrowed_future_witness::Options::with_label")) },
                    17 /* terrane-site: src/main.trn:52:13-52:46 */) })
                );
                println!("{}", terrane_scalar_support::scalar_text(&named));
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await({
                    let __terrane_future =
                    read_owned_later(String::from("owned async value")); async move {
                    __terrane_raised_err(__terrane_future. await, 18 /* terrane-site: src/main.trn:54:19-54:56 */) } }). await, 18 /* terrane-site: src/main.trn:54:19-54:56 */))
                );
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await({
                    let __terrane_future =
                    read_alias_later(String::from("grouped alias λ")); async move {
                    __terrane_raised_err(__terrane_future. await, 19 /* terrane-site: src/main.trn:55:19-55:56 */) } }). await, 19 /* terrane-site: src/main.trn:55:19-55:56 */))
                );
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let
                    __terrane_native_input_0 =
                    __terrane_raised(terrane_static_trn_4f7074696f6e73_new(),
                    20 /* terrane-site: src/main.trn:56:14-56:27 */); let
                    __terrane_native_input_1 = &String::from("owned String borrow");
                    __terrane_raised(match
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                    __terrane_native_input_0.with_string_ref(__terrane_native_input_1)))
                    { Ok(value) => Ok(value), Err(payload) => Err(crate
                    ::__terrane_dependency_panic(payload,
                    "terrane_borrowed_future_witness",
                    "terrane_borrowed_future_witness::Options::with_string_ref")) },
                    21 /* terrane-site: src/main.trn:56:13-56:67 */) })
                );
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await({
                    let __terrane_future = { let __terrane_call =
                    terrane_borrowed_future_witness::literal_query("query graph λ")
                    .fetch(); Box::pin(async move { match crate
                    ::__terrane_dependency_await_unwind(__terrane_call). await {
                    Ok(value) => Ok(value), Err(payload) => Err(crate
                    ::__terrane_dependency_panic(payload,
                    "terrane_borrowed_future_witness",
                    "terrane_borrowed_future_witness::TextQuery::fetch")) } }) }; async
                    move { __terrane_raised_err(__terrane_future. await,
                    22 /* terrane-site: src/main.trn:57:19-57:60 */) } }). await,
                    22 /* terrane-site: src/main.trn:57:19-57:60 */))
                );
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
                    24 /* terrane-site: src/main.trn:61:32-61:61 */,
                );
                println!(
                    "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | wrapped
                    .into_value())) { Ok(value) => Ok(value), Err(payload) => Err(crate
                    ::__terrane_dependency_panic(payload,
                    "terrane_borrowed_future_witness",
                    "terrane_borrowed_future_witness::Envelope::into_value")) },
                    25 /* terrane-site: src/main.trn:62:13-62:32 */)),
                    terrane_scalar_support::scalar_text(&__terrane_raised(outcomes
                    .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
                    26 /* terrane-site: src/main.trn:62:35-62:46 */)),
                    26 /* terrane-site: src/main.trn:62:35-62:46 */).completed)
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
                            27 /* terrane-site: src/main.trn:65:15-65:30 */
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
                            }))). await, 28 /* terrane-site: src/main.trn:70:15-70:36 */
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
                    29 /* terrane-site: src/main.trn:73:17-73:30 */,
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
                                        30 /* terrane-site: src/main.trn:74:15-74:33 */,
                                    )
                                }
                            })
                            .await,
                        30 /* terrane-site: src/main.trn:74:15-74:33 */,
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
                                    31 /* terrane-site: src/main.trn:75:22-75:38 */,
                                )
                            }
                        })
                        .await,
                    31 /* terrane-site: src/main.trn:75:22-75:38 */,
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
                                await, 32 /* terrane-site: src/main.trn:78:19-78:42 */) }
                                }). await, 32 /* terrane-site: src/main.trn:78:19-78:42 */
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
                                    33 /* terrane-site: src/main.trn:82:17-82:30 */
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
                                    34 /* terrane-site: src/main.trn:83:30-83:42 */) } }).
                                    await, 34 /* terrane-site: src/main.trn:83:30-83:42 */
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
