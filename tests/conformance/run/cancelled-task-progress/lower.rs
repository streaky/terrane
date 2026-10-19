// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
async fn work() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(7_i128);
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
        scope.cancel();
        outcome = __terrane_await(scope.join(child)).await;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&outcome.cancelled),
            terrane_scalar_support::scalar_text(&(&outcome.value).is_none())
        );
    });
}
