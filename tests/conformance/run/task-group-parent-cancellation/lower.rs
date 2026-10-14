// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: task-group-parent-cancellation
#[derive(Clone)]
pub struct State {
    pub child_cleaned: bool,
    pub parent_cleaned: bool,
}
impl State {
    pub fn terrane_construct() -> Self {
        Self {
            child_cleaned: false,
            parent_cleaned: false,
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
async fn child(observer: std::sync::Arc<std::sync::Mutex<State>>) {
    let mut __terrane_finally_guard_0 = __terrane_finally_guard();
    let __terrane_maybe_completion_0: Option<TerraneCompletion<()>> = __terrane_cancel_operation(
            &__terrane_finally_guard_0,
            async {
                let __terrane_try_0: TerraneCompletion<()> = async {
                    {
                        let _: () = __terrane_await(
                                terrane_time_sleep_after(
                                    __terrane_traced_completion!(
                                        Duration::terrane_static_seconds(terrane_int_support::Int::from(30_i128)),
                                        0 /* terrane-site: case.trn:16:34-16:55 */
                                    )
                                        .total_nanoseconds
                                        .clone(),
                                ),
                            )
                            .await;
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
    let __terrane_finally_0: TerraneCompletion<()> = (|| {
        observer.lock().expect("shared reference lock poisoned").child_cleaned = true;
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
    return ();
}
async fn parent(observer: std::sync::Arc<std::sync::Mutex<State>>) {
    __terrane_group_frame(async move {
            let group: TerraneTaskGroup<()>;
            group = TerraneTaskGroup::<()>::new();
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
                            {
                                let _: () = __terrane_await(
                                        terrane_time_sleep_after(
                                            __terrane_traced_completion!(
                                                Duration::terrane_static_seconds(terrane_int_support::Int::from(30_i128)),
                                                1 /* terrane-site: case.trn:25:34-25:55 */
                                            )
                                                .total_nanoseconds
                                                .clone(),
                                        ),
                                    )
                                    .await;
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
                {
                    let _: terrane_collection_support::List<TerraneTaskOutcome<()>> = __terrane_await(
                            std::boxed::Box::pin(group.prepare_drain()),
                        )
                        .await;
                }
                observer
                    .lock()
                    .expect("shared reference lock poisoned")
                    .parent_cleaned = true;
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
                TerraneCompletion::Return(value) => return value,
                TerraneCompletion::Error(error) => __terrane_uncaught(error),
                TerraneCompletion::Break | TerraneCompletion::Continue => {
                    __terrane_generated_defect("loop control escaped a non-loop try")
                }
            }
            return ();
        })
        .await
}
async fn fail() -> Result<(), TerraneError> {
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
            2 /* terrane-site: case.trn:32:5-32:33 */,
        )
    });
}
fn main() {
    __terrane_run(async move {
        let observer: std::sync::Arc<std::sync::Mutex<State>>;
        let scope: TerraneTaskScope;
        let pending: TerraneScopedTask<()>;
        let failing: TerraneScopedTask<()>;
        let failed: TerraneTaskOutcome<()>;
        let joined: TerraneTaskOutcome<()>;
        observer = std::sync::Arc::new(
            std::sync::Mutex::new(State::terrane_construct()),
        );
        scope = TerraneTaskScope::new(None);
        pending = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = parent(observer.clone());
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
        failing = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = fail();
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
        failed = __terrane_await(scope.join(failing)).await;
        joined = __terrane_await(scope.join(pending)).await;
        println!("{}", terrane_scalar_support::scalar_text(&failed.error.is_some()));
        println!("{}", terrane_scalar_support::scalar_text(&joined.cancelled));
        println!(
            "{}", terrane_scalar_support::scalar_text(&(observer.lock()
            .expect("shared reference lock poisoned").child_cleaned.clone() &&observer
            .lock().expect("shared reference lock poisoned").parent_cleaned.clone()))
        );
    });
}
