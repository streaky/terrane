// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: cancelled-stream-operation
async fn read_one() -> ReadResult {
    let input: ByteReader;
    let pending;
    let result: ReadResult;
    input = stdin();
    pending = (&input).read_async(terrane_int_support::Int::from(2_i128), None);
    result = __terrane_await(pending).await;
    input.close();
    return result;
}
fn main() {
    __terrane_run(async move {
        let scope: TerraneTaskScope;
        let child: TerraneScopedTask<ReadResult>;
        let outcome: TerraneTaskOutcome<ReadResult>;
        scope = TerraneTaskScope::new(None);
        child = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        std::sync::Arc::new(move || -> std::pin::Pin<
                            Box<dyn Future<Output = _> + Send>,
                        > { Box::pin(read_one()) })(),
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
