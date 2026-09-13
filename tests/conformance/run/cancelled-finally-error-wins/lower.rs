// Generated deterministically by Terrane <version>.
// Runtime support: async_native.rs, executor_local.rs, async_dependency.rs, tasks_native_local.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: src/main.trn
// Namespace: app
async fn blocked() -> Result<String, TerraneError> {
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
    let __terrane_finally_0: TerraneCompletion<String> = (|| {
        return TerraneCompletion::Error(
            TerraneError::raised(
                TerraneErrorKind::CoercionError,
                1 /* terrane-site: src/main.trn:10:9-10:29 */,
            ),
        );
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
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
fn main() {
    __terrane_run(async move {
        __terrane_raised(
            reset_operation_state(),
            2 /* terrane-site: src/main.trn:13:5-13:27 */,
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
                    Some(Ok(value)) => TerraneTaskResult::Completed(value),
                    Some(Err(error)) => TerraneTaskResult::Failed(error),
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
                            3 /* terrane-site: src/main.trn:16:26-16:55 */,
                        )
                    }
                })
                .await,
            3 /* terrane-site: src/main.trn:16:26-16:55 */,
        );
        scope.cancel();
        let outcome: TerraneTaskOutcome<String> = __terrane_await(scope.join(child))
            .await;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&started),
            terrane_scalar_support::scalar_text(&outcome.cancelled)
        );
        println!("{}", terrane_scalar_support::scalar_text(&outcome.error.is_some()));
    });
}
// Source: <terrane>/projected/deps/async-witness.trn
// Namespace: deps/async-witness
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
