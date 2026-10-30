// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: task-group-stored-alias-finally
#[derive(Clone)]
pub struct State {
    pub child_started: bool,
    pub child_finished_normally: bool,
    pub child_cleaned: bool,
}
impl State {
    pub fn terrane_construct() -> Self {
        Self {
            child_started: false,
            child_finished_normally: false,
            child_cleaned: false,
        }
    }
}
#[derive(Clone)]
pub struct WorkerError {
    pub message: String,
}
impl WorkerError {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("failed"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
async fn risky() -> Result<(), TerraneError> {
    return Err({
        let value = WorkerError::terrane_construct();
        let rendered = value.render();
        let original = &value.message;
        let (message, rendered) = if &rendered == original {
            (rendered, None)
        } else {
            (original.clone(), Some(rendered))
        };
        TerraneError::raised_with_rendered_message(
            TerraneErrorKind::Custom(DescriptorId(0)),
            message,
            rendered,
            0 /* terrane-site: case.trn:16:5-16:33 */,
        )
    });
}
async fn child(
    observer: std::sync::Arc<TerraneSharedCell<State>>,
) -> terrane_int_support::Int {
    {
        let __terrane_owner_0 = observer.clone();
        let __terrane_value = true;
        __terrane_owner_0
            .lock()
            .expect("shared reference lock poisoned")
            .child_started = __terrane_value;
    }
    let mut __terrane_finally_guard_0 = __terrane_finally_guard();
    let __terrane_maybe_completion_0: Option<
        TerraneCompletion<terrane_int_support::Int>,
    > = __terrane_cancel_operation(
            &__terrane_finally_guard_0,
            async {
                let __terrane_try_0: TerraneCompletion<terrane_int_support::Int> = async {
                    {
                        let _: () = __terrane_await(
                                terrane_time_sleep_after(
                                    __terrane_traced_completion!(
                                        Duration::terrane_static_seconds(terrane_int_support::Int::from(30_i128)),
                                        1 /* terrane-site: case.trn:21:34-21:55 */
                                    )
                                        .total_nanoseconds
                                        .clone(),
                                ),
                            )
                            .await;
                    }
                    {
                        let __terrane_owner_1 = observer.clone();
                        let __terrane_value = true;
                        __terrane_owner_1
                            .lock()
                            .expect("shared reference lock poisoned")
                            .child_finished_normally = __terrane_value;
                    }
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
    let __terrane_finally_0: TerraneCompletion<terrane_int_support::Int> = (|| {
        {
            let __terrane_owner_2 = observer.clone();
            let __terrane_value = true;
            __terrane_owner_2
                .lock()
                .expect("shared reference lock poisoned")
                .child_cleaned = __terrane_value;
        }
        TerraneCompletion::Normal
    })();
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
    return terrane_int_support::Int::from(1_i128);
}
async fn worker(
    observer: std::sync::Arc<TerraneSharedCell<State>>,
) -> Result<(), TerraneError> {
    __terrane_group_frame(async move {
            let group: TerraneTaskGroup<terrane_int_support::Int>;
            let mut pending = None;
            let mut other = None;
            let mut alias = None;
            group = TerraneTaskGroup::<terrane_int_support::Int>::new();
            {
                let __terrane_child = {
                    let __terrane_scope = group.scope();
                    let __terrane_cancel = __terrane_scope.cancellation();
                    let __terrane_deadline = __terrane_scope.deadline.clone();
                    let __terrane_spawned_task = child(observer.clone());
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
            let mut __terrane_finally_guard_1 = __terrane_finally_guard();
            let __terrane_maybe_completion_1: Option<TerraneCompletion<()>> = __terrane_cancel_operation(
                    &__terrane_finally_guard_1,
                    async {
                        let __terrane_try_1: TerraneCompletion<()> = async {
                            while !{
                                let __terrane_snapshot = observer
                                    .lock()
                                    .expect("shared reference lock poisoned")
                                    .child_started;
                                __terrane_snapshot
                            } {
                                {
                                    let _: () = __terrane_await(
                                            terrane_time_sleep_after(
                                                __terrane_traced_completion!(
                                                    Duration::terrane_static_nanoseconds(terrane_int_support::Int::from(1000000_i128)),
                                                    2 /* terrane-site: case.trn:32:38-32:68 */
                                                )
                                                    .total_nanoseconds
                                                    .clone(),
                                            ),
                                        )
                                        .await;
                                }
                            }
                            let _ = pending.insert(risky());
                            let _ = other
                                .insert(pending.take().expect("flow-proven availability"));
                            let _ = alias
                                .insert(other.take().expect("flow-proven availability"));
                            {
                                let _: () = __terrane_traced_completion!(
                                    __terrane_await(alias.take()
                                    .expect("flow-proven availability")). await,
                                    4 /* terrane-site: case.trn:36:19-36:24 */
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
                            TerraneCompletion::Continue => {
                                return TerraneCompletion::Continue;
                            }
                            TerraneCompletion::Normal => {}
                            TerraneCompletion::Error(__terrane_error_1) => {
                                let mut __terrane_handled_1 = false;
                                if !__terrane_handled_1 {
                                    return TerraneCompletion::Error(__terrane_error_1);
                                }
                            }
                        }
                        TerraneCompletion::Normal
                    },
                )
                .await;
            let __terrane_cancelled_1 = __terrane_maybe_completion_1.is_none();
            let mut __terrane_completion_1 = __terrane_maybe_completion_1
                .unwrap_or(TerraneCompletion::Normal);
            let __terrane_finally_1: TerraneCompletion<()> = async {
                group.cancel();
                {
                    let _: terrane_collection_support::List<
                        TerraneTaskOutcome<terrane_int_support::Int>,
                    > = __terrane_await(std::boxed::Box::pin(group.prepare_drain()))
                        .await;
                }
                TerraneCompletion::Normal
            }
                .await;
            match __terrane_finally_1 {
                TerraneCompletion::Normal => {}
                replacement => __terrane_completion_1 = replacement,
            }
            if __terrane_cancelled_1
                && matches!(&__terrane_completion_1, TerraneCompletion::Normal)
            {
                __terrane_finish_cancelled_finally(__terrane_finally_guard_1).await;
            }
            __terrane_finally_guard_1.finish();
            match __terrane_completion_1 {
                TerraneCompletion::Normal => {}
                TerraneCompletion::Return(value) => return Ok(value),
                TerraneCompletion::Error(error) => return Err(error),
                TerraneCompletion::Break | TerraneCompletion::Continue => {
                    __terrane_generated_defect("loop control escaped a non-loop try")
                }
            }
            return Ok(());
        })
        .await
}
fn main() {
    __terrane_run(async move {
        let observer: std::sync::Arc<TerraneSharedCell<State>>;
        observer = std::sync::Arc::new(
            TerraneSharedCell::new(State::terrane_construct()),
        );
        let __terrane_completion_2: TerraneCompletion<()> = async {
            let __terrane_try_2: TerraneCompletion<()> = async {
                {
                    let _: () = __terrane_traced_completion!(
                        __terrane_await(worker(observer.clone())). await,
                        6 /* terrane-site: case.trn:45:19-45:37 */
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
                    if !__terrane_handled_2 {
                        __terrane_handled_2 = true;
                        println!("{}", String::from("caught"));
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
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot =
            observer.lock().expect("shared reference lock poisoned").child_started;
            __terrane_snapshot })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot =
            observer.lock().expect("shared reference lock poisoned")
            .child_finished_normally; __terrane_snapshot })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot =
            observer.lock().expect("shared reference lock poisoned").child_cleaned;
            __terrane_snapshot })
        );
        {
            let _: () = __terrane_await(caught_call()).await;
        }
    });
}
async fn caught_call() {
    __terrane_group_frame(async move {
            let group: TerraneTaskGroup<terrane_int_support::Int>;
            group = TerraneTaskGroup::<terrane_int_support::Int>::new();
            let __terrane_completion_3: TerraneCompletion<()> = async {
                let __terrane_try_3: TerraneCompletion<()> = async {
                    {
                        let _: () = __terrane_traced_completion!(
                            __terrane_await(risky()). await, 8 /* terrane-site: case.trn:56:19-56:27 */
                        );
                    }
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
                            println!("{}", String::from("handled"));
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
            {
                let _: terrane_collection_support::List<
                    TerraneTaskOutcome<terrane_int_support::Int>,
                > = __terrane_await(std::boxed::Box::pin(group.prepare_drain())).await;
            }
            return ();
        })
        .await
}
