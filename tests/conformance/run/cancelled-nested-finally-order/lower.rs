// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
async fn blocked() -> String {
    let mut inner: Option<String> = None;
    let mut outer: Option<String> = None;
    let mut __terrane_finally_guard_0 = __terrane_finally_guard();
    let __terrane_maybe_completion_0: Option<TerraneCompletion<String>> = __terrane_cancel_operation(
            &__terrane_finally_guard_0,
            async {
                let __terrane_try_0: TerraneCompletion<String> = async {
                    let mut __terrane_finally_guard_1 = __terrane_finally_guard();
                    let __terrane_maybe_completion_1: Option<
                        TerraneCompletion<String>,
                    > = __terrane_cancel_operation(
                            &__terrane_finally_guard_1,
                            async {
                                let __terrane_try_1: TerraneCompletion<String> = async {
                                    return TerraneCompletion::Return(
                                        __terrane_traced_completion!(
                                            __terrane_await({ let __terrane_future = wait_forever();
                                            async move { __terrane_raised_err(__terrane_future. await,
                                            0 /* terrane-site: src/main.trn:8:20-8:33 */) } }).
                                            await, 0 /* terrane-site: src/main.trn:8:20-8:33 */
                                        ),
                                    );
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
                    let __terrane_finally_1: TerraneCompletion<String> = async {
                        let _ = inner
                            .insert(
                                __terrane_traced_completion!(
                                    __terrane_await({ let __terrane_future =
                                    echo_after_yield(String::from("inner")); async move {
                                    __terrane_raised_err(__terrane_future. await,
                                    1 /* terrane-site: src/main.trn:10:28-10:52 */) } }).
                                    await, 1 /* terrane-site: src/main.trn:10:28-10:52 */
                                ),
                            );
                        println!(
                            "{}", terrane_scalar_support::scalar_text(&inner.as_ref()
                            .expect("flow-proven available binding").clone())
                        );
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
                        __terrane_finish_cancelled_finally(__terrane_finally_guard_1)
                            .await;
                    }
                    __terrane_finally_guard_1.finish();
                    match __terrane_completion_1 {
                        TerraneCompletion::Normal => {
                            __terrane_generated_defect(
                                "non-fallthrough try completed normally",
                            )
                        }
                        TerraneCompletion::Return(value) => {
                            return TerraneCompletion::Return(value);
                        }
                        TerraneCompletion::Error(error) => {
                            return TerraneCompletion::Error(error);
                        }
                        TerraneCompletion::Break | TerraneCompletion::Continue => {
                            __terrane_generated_defect(
                                "loop control escaped a non-loop try",
                            )
                        }
                    }
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
        let _ = outer
            .insert(
                __terrane_traced_completion!(
                    __terrane_await({ let __terrane_future =
                    echo_after_yield(String::from("outer")); async move {
                    __terrane_raised_err(__terrane_future. await, 2 /* terrane-site: src/main.trn:13:26-13:50 */) } }). await, 2 /* terrane-site: src/main.trn:13:26-13:50 */
                ),
            );
        println!(
            "{}", terrane_scalar_support::scalar_text(&outer.as_ref()
            .expect("flow-proven available binding").clone())
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
fn main() {
    __terrane_run(async move {
        let scope: TerraneTaskScope;
        let child: TerraneScopedTask<String>;
        let started: bool;
        let outcome: TerraneTaskOutcome<String>;
        let drops: terrane_int_support::Int;
        __terrane_raised(
            reset_operation_state(),
            3 /* terrane-site: src/main.trn:17:3-17:25 */,
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
                            4 /* terrane-site: src/main.trn:20:24-20:53 */,
                        )
                    }
                })
                .await,
            4 /* terrane-site: src/main.trn:20:24-20:53 */,
        );
        scope.cancel();
        outcome = __terrane_await(scope.join(child)).await;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&started),
            terrane_scalar_support::scalar_text(&outcome.cancelled)
        );
        drops = __terrane_raised(
            operation_drop_count(),
            5 /* terrane-site: src/main.trn:24:15-24:36 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&drops));
    });
}
