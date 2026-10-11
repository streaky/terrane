// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: task-scope-typed-deadline
async fn work() -> terrane_int_support::Int {
    let ignored: ();
    ignored = __terrane_await(
            terrane_time_sleep_after(
                __terrane_traced(
                        Duration::terrane_static_seconds(
                            terrane_int_support::Int::from(1_i128),
                        ),
                        0 /* terrane-site: case.trn:6:36-6:56 */,
                    )
                    .total_nanoseconds
                    .clone(),
            ),
        )
        .await;
    let _ = &ignored;
    return terrane_int_support::Int::from(42_i128);
}
fn main() {
    __terrane_run(async move {
        let parent: TerraneTaskScope;
        let child: TerraneTaskScope;
        let task: TerraneScopedTask<terrane_int_support::Int>;
        let outcome: TerraneTaskOutcome<terrane_int_support::Int>;
        parent = TerraneTaskScope::new(
            Some(
                Clock::terrane_static_deadline(
                        __terrane_traced(
                            Duration::terrane_static_milliseconds(
                                terrane_int_support::Int::from(2_i128),
                            ),
                            1 /* terrane-site: case.trn:10:45-10:70 */,
                        ),
                    )
                    .expires_at
                    .elapsed_nanoseconds
                    .clone(),
            ),
        );
        child = parent
            .child_scope(
                &Clock::terrane_static_deadline(
                        __terrane_traced(
                            Duration::terrane_static_seconds(
                                terrane_int_support::Int::from(1_i128),
                            ),
                            2 /* terrane-site: case.trn:11:52-11:72 */,
                        ),
                    )
                    .expires_at
                    .elapsed_nanoseconds,
            );
        task = {
            let __terrane_scope = child.clone();
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
        outcome = __terrane_await(child.join(task)).await;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&outcome.completed),
            terrane_scalar_support::scalar_text(&outcome.cancelled)
        );
    });
}
