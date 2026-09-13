// Generated deterministically by Terrane <version>.
// Runtime support: async_native.rs, executor_parallel.rs, tasks_native_parallel.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: failing-task-cancels-sibling
async fn fail() -> Result<terrane_int_support::Int, TerraneError> {
    return Err(
        TerraneError::raised(
            TerraneErrorKind::CoercionError,
            0 /* terrane-site: case.trn:5:3-5:23 */,
        ),
    );
}
async fn survive() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(7_i128);
}
fn main() {
    __terrane_run(async move {
        let scope: TerraneTaskScope = TerraneTaskScope::new(None);
        let failing: TerraneScopedTask<terrane_int_support::Int> = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        std::sync::Arc::new(move || -> std::pin::Pin<
                            Box<dyn Future<Output = _> + Send>,
                        > { Box::pin(fail()) })(),
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
        let sibling: TerraneScopedTask<terrane_int_support::Int> = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        std::sync::Arc::new(move || -> std::pin::Pin<
                            Box<dyn Future<Output = _> + Send>,
                        > { Box::pin(survive()) })(),
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
        let failed: TerraneTaskOutcome<terrane_int_support::Int> = __terrane_await(
                scope.join(failing),
            )
            .await;
        let survived: TerraneTaskOutcome<terrane_int_support::Int> = __terrane_await(
                scope.join(sibling),
            )
            .await;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&failed.completed),
            terrane_scalar_support::scalar_text(&failed.cancelled)
        );
        println!("{}", terrane_scalar_support::scalar_text(&survived.cancelled));
    });
}
