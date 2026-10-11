// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: throwing-scoped-task
async fn fail() -> Result<terrane_int_support::Int, TerraneError> {
    return Err(
        TerraneError::raised(
            TerraneErrorKind::CoercionError,
            0 /* terrane-site: case.trn:5:3-5:23 */,
        ),
    );
}
fn main() {
    __terrane_run(async move {
        let scope: TerraneTaskScope;
        let child: TerraneScopedTask<terrane_int_support::Int>;
        let outcome: TerraneTaskOutcome<terrane_int_support::Int>;
        scope = TerraneTaskScope::new(None);
        child = {
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
        outcome = __terrane_await(scope.join(child)).await;
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&outcome.completed),
            terrane_scalar_support::scalar_text(&outcome.cancelled),
            terrane_scalar_support::scalar_text(&outcome.value.clone().is_none())
        );
        println!("{}", terrane_scalar_support::scalar_text(&outcome.error.is_some()));
    });
}
