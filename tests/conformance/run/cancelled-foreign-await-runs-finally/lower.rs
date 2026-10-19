// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
async fn blocked() -> String {
    let cleanup: String;
    let mut observed: Option<String> = None;
    cleanup = String::from("cleanup");
    let mut __terrane_finally_guard_0 = __terrane_finally_guard();
    let __terrane_maybe_completion_0: Option<TerraneCompletion<String>> = __terrane_cancel_operation(
            &__terrane_finally_guard_0,
            async {
                let __terrane_try_0: TerraneCompletion<String> = async {
                    return TerraneCompletion::Return(
                        __terrane_traced_completion!(
                            __terrane_await({ let __terrane_future = wait_forever();
                            async move { __terrane_raised_err(__terrane_future. await,
                            0 /* terrane-site: src/main.trn:8:22-8:35 */) } }). await,
                            0 /* terrane-site: src/main.trn:8:22-8:35 */
                        ),
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
    let __terrane_finally_0: TerraneCompletion<String> = async {
        let _ = observed
            .insert(
                __terrane_traced_completion!(
                    __terrane_await({ let __terrane_future = echo_after_yield(cleanup);
                    async move { __terrane_raised_err(__terrane_future. await,
                    1 /* terrane-site: src/main.trn:10:33-10:58 */) } }). await,
                    1 /* terrane-site: src/main.trn:10:33-10:58 */
                ),
            );
        println!(
            "{}", terrane_scalar_support::scalar_text(&observed.as_ref()
            .expect("flow-proven available binding"))
        );
        TerraneCompletion::Normal
    }
        .await;
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
async fn after_cancellation() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(9_i128);
}
fn main() {
    __terrane_run(async move {
        let scope: TerraneTaskScope;
        let child: TerraneScopedTask<String>;
        let started: bool;
        let outcome: TerraneTaskOutcome<String>;
        let next_scope: TerraneTaskScope;
        let next_child: TerraneScopedTask<terrane_int_support::Int>;
        let next_outcome: TerraneTaskOutcome<terrane_int_support::Int>;
        let next_value: Option<terrane_int_support::Int>;
        let drops: terrane_int_support::Int;
        __terrane_raised(
            reset_operation_state(),
            2 /* terrane-site: src/main.trn:17:5-17:27 */,
        );
        scope = TerraneTaskScope::new(None);
        child = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        std::sync::Arc::new(move || -> std::pin::Pin<
                            Box<dyn Future<Output = _> + Send>,
                        > { Box::pin(blocked()) })(),
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
        started = __terrane_traced(
            __terrane_await({
                    let __terrane_future = wait_until_operation_started();
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            3 /* terrane-site: src/main.trn:20:26-20:55 */,
                        )
                    }
                })
                .await,
            3 /* terrane-site: src/main.trn:20:26-20:55 */,
        );
        scope.cancel();
        outcome = __terrane_await(scope.join(child)).await;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&started),
            terrane_scalar_support::scalar_text(&outcome.cancelled)
        );
        next_scope = TerraneTaskScope::new(None);
        next_child = {
            let __terrane_scope = next_scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        std::sync::Arc::new(move || -> std::pin::Pin<
                            Box<dyn Future<Output = _> + Send>,
                        > { Box::pin(after_cancellation()) })(),
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
        next_outcome = __terrane_await(next_scope.join(next_child)).await;
        next_value = next_outcome.value.clone();
        if (&next_value).is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&match &next_value {
                Some(value) => value, _ => unreachable!("flow-proven storage refinement")
                })
            );
        }
        drops = __terrane_raised(
            operation_drop_count(),
            4 /* terrane-site: src/main.trn:30:17-30:38 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&drops));
    });
}
