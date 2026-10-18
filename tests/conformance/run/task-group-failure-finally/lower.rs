// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: task-group-failure-finally
#[derive(Clone)]
pub struct State {
    pub cleaned: bool,
}
impl State {
    pub fn terrane_construct() -> Self {
        Self { cleaned: false }
    }
}
#[derive(Clone)]
pub struct WorkerError {
    pub message: String,
}
impl WorkerError {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("worker failed"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
async fn wait(
    observer: std::sync::Arc<TerraneSharedCell<State>>,
) -> terrane_int_support::Int {
    let mut pause: Option<Duration> = None;
    let mut __terrane_finally_guard_0 = __terrane_finally_guard();
    let __terrane_maybe_completion_0: Option<
        TerraneCompletion<terrane_int_support::Int>,
    > = __terrane_cancel_operation(
            &__terrane_finally_guard_0,
            async {
                let __terrane_try_0: TerraneCompletion<terrane_int_support::Int> = async {
                    let _ = pause
                        .insert(
                            __terrane_traced_completion!(
                                Duration::terrane_static_seconds(terrane_int_support::Int::from(30_i128)),
                                0 /* terrane-site: case.trn:15:17-15:38 */
                            ),
                        );
                    {
                        let _: () = __terrane_await(
                                terrane_time_sleep_after(
                                    pause
                                        .as_ref()
                                        .expect("flow-proven available binding")
                                        .clone()
                                        .total_nanoseconds
                                        .clone(),
                                ),
                            )
                            .await;
                    }
                    return TerraneCompletion::Return(
                        terrane_int_support::Int::from(1_i128),
                    );
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
            let __terrane_owner = observer.clone();
            let __terrane_value = true;
            __terrane_owner.lock().expect("shared reference lock poisoned").cleaned = __terrane_value;
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
        TerraneCompletion::Normal => {
            __terrane_generated_defect("non-fallthrough try completed normally")
        }
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
async fn fail() -> Result<terrane_int_support::Int, TerraneError> {
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
            1 /* terrane-site: case.trn:22:5-22:33 */,
        )
    });
}
fn main() {
    __terrane_run(async move {
        __terrane_group_frame(async move {
                let observer: std::sync::Arc<TerraneSharedCell<State>>;
                let group: TerraneTaskGroup<terrane_int_support::Int>;
                let outcomes: terrane_collection_support::List<
                    TerraneTaskOutcome<terrane_int_support::Int>,
                >;
                let first: TerraneTaskOutcome<terrane_int_support::Int>;
                let second: TerraneTaskOutcome<terrane_int_support::Int>;
                observer = std::sync::Arc::new(
                    TerraneSharedCell::new(State::terrane_construct()),
                );
                group = TerraneTaskGroup::<terrane_int_support::Int>::new();
                {
                    let __terrane_child = {
                        let __terrane_scope = group.scope();
                        let __terrane_cancel = __terrane_scope.cancellation();
                        let __terrane_deadline = __terrane_scope.deadline.clone();
                        let __terrane_spawned_task = wait(observer.clone());
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
                {
                    let __terrane_child = {
                        let __terrane_scope = group.scope();
                        let __terrane_cancel = __terrane_scope.cancellation();
                        let __terrane_deadline = __terrane_scope.deadline.clone();
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
                first = __terrane_raised(
                    outcomes
                        .get_or_error(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(
                                    &terrane_int_support::Int::from(0_i128),
                                ),
                                3 /* terrane-site: case.trn:30:13-30:24 */,
                            ),
                        ),
                    3 /* terrane-site: case.trn:30:13-30:24 */,
                );
                second = __terrane_raised(
                    outcomes
                        .get_or_error(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(
                                    &terrane_int_support::Int::from(1_i128),
                                ),
                                4 /* terrane-site: case.trn:31:14-31:25 */,
                            ),
                        ),
                    4 /* terrane-site: case.trn:31:14-31:25 */,
                );
                println!(
                    "{}", terrane_scalar_support::scalar_text(&(first.cancelled &&! first
                    .completed))
                );
                println!(
                    "{}", terrane_scalar_support::scalar_text(&second.error.is_some())
                );
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot =
                    observer.lock().expect("shared reference lock poisoned").cleaned;
                    __terrane_snapshot })
                );
            })
            .await
    });
}
