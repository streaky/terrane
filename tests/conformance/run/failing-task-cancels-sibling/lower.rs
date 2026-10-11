// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
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
        let scope: TerraneTaskScope;
        let failing: TerraneScopedTask<terrane_int_support::Int>;
        let sibling: TerraneScopedTask<terrane_int_support::Int>;
        let failed: TerraneTaskOutcome<terrane_int_support::Int>;
        let survived: TerraneTaskOutcome<terrane_int_support::Int>;
        scope = TerraneTaskScope::new(None);
        failing = {
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
        sibling = {
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
        failed = __terrane_await(scope.join(failing)).await;
        survived = __terrane_await(scope.join(sibling)).await;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&failed.completed),
            terrane_scalar_support::scalar_text(&failed.cancelled)
        );
        println!("{}", terrane_scalar_support::scalar_text(&survived.cancelled));
    });
}
