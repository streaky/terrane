// Generated deterministically by Terrane <version>.
// Runtime support: async_native.rs, executor_local.rs, async_dependency.rs, tasks_native_local.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: src/main.trn
// Namespace: app
async fn blocked() -> String {
    let cleanup: String = String::from("cleanup");
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
        let observed: String = __terrane_traced_completion!(
            __terrane_await({ let __terrane_future = echo_after_yield(cleanup); async
            move { __terrane_raised_err(__terrane_future. await, 1 /* terrane-site: src/main.trn:10:33-10:58 */) } }). await, 1 /* terrane-site: src/main.trn:10:33-10:58 */
        );
        println!("{}", terrane_scalar_support::scalar_text(&observed));
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
        __terrane_raised(
            reset_operation_state(),
            2 /* terrane-site: src/main.trn:17:5-17:27 */,
        );
        let scope: TerraneTaskScope = TerraneTaskScope::new(None);
        let child: TerraneScopedTask<String> = {
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
        let started: bool = __terrane_traced(
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
        let outcome: TerraneTaskOutcome<String> = __terrane_await(scope.join(child))
            .await;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&started),
            terrane_scalar_support::scalar_text(&outcome.cancelled)
        );
        let next_scope: TerraneTaskScope = TerraneTaskScope::new(None);
        let next_child: TerraneScopedTask<terrane_int_support::Int> = {
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
        let next_outcome: TerraneTaskOutcome<terrane_int_support::Int> = __terrane_await(
                next_scope.join(next_child),
            )
            .await;
        let next_value: Option<terrane_int_support::Int> = next_outcome.value.clone();
        if next_value.is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&* next_value.as_ref()
                .expect("semantic optional narrowing"))
            );
        }
        let drops: terrane_int_support::Int = __terrane_raised(
            operation_drop_count(),
            4 /* terrane-site: src/main.trn:30:17-30:38 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&drops));
    });
}
// Source: <terrane>/projected/deps/async-witness.trn
// Namespace: deps/async-witness
pub async fn echo_after_yield(
    value: String,
) -> Result<String, crate::TerraneForeignError> {
    let value = value;
    match crate::__terrane_dependency_await_unwind(
            async_witness::echo_after_yield(value),
        )
        .await
    {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "async-witness",
                    "async_witness::echo_after_yield",
                ),
            )
        }
    }
}
pub fn operation_drop_count() -> Result<
    terrane_int_support::Int,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| async_witness::operation_drop_count()),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from_u128(value as u128)),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "async-witness",
                    "async_witness::operation_drop_count",
                ),
            )
        }
    }
}
pub fn reset_operation_state() -> Result<(), crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| async_witness::reset_operation_state()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "async-witness",
                    "async_witness::reset_operation_state",
                ),
            )
        }
    }
}
pub async fn wait_forever() -> Result<String, crate::TerraneForeignError> {
    match crate::__terrane_dependency_await_unwind(async_witness::wait_forever()).await {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "async-witness",
                    "async_witness::wait_forever",
                ),
            )
        }
    }
}
pub async fn wait_until_operation_started() -> Result<bool, crate::TerraneForeignError> {
    match crate::__terrane_dependency_await_unwind(
            async_witness::wait_until_operation_started(),
        )
        .await
    {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "async-witness",
                    "async_witness::wait_until_operation_started",
                ),
            )
        }
    }
}
