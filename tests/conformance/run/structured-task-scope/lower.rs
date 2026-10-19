// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: structured-task-scope
async fn work() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(42_i128);
}
fn main() {
    __terrane_run(async move {
        let scope: TerraneTaskScope;
        let child: TerraneScopedTask<terrane_int_support::Int>;
        let outcome: TerraneTaskOutcome<terrane_int_support::Int>;
        let value: Option<terrane_int_support::Int>;
        scope = TerraneTaskScope::new(None);
        child = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        std::sync::Arc::new(move || -> std::pin::Pin<
                            Box<dyn Future<Output = _> + Send>,
                        > { Box::pin(work()) })(),
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
        outcome = __terrane_await(scope.join(child)).await;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&outcome.completed),
            terrane_scalar_support::scalar_text(&outcome.cancelled)
        );
        value = outcome.value.clone();
        if (&value).is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&match &value { Some(value) =>
                value, _ => unreachable!("flow-proven storage refinement") })
            );
        }
    });
}
