// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
async fn wait_next(mut sequence: TokioSequence) {
    let step: terrane_collection_support::AsyncIterationStep<terrane_int_support::Int>;
    step = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                        {
                            Ok(Ok(value)) => {
                                Ok(
                                    match value {
                                        Some(item) => {
                                            terrane_collection_support::AsyncIterationStep::item(
                                                terrane_int_support::Int::from(i128::from(item)),
                                            )
                                        }
                                        None => {
                                            terrane_collection_support::AsyncIterationStep::end()
                                        }
                                    },
                                )
                            }
                            Ok(Err(error)) => {
                                Err(
                                    crate::TerraneForeignError(
                                        crate::TerraneError::custom_raised(
                                            crate::DescriptorId(2),
                                            error.to_string(),
                                            crate::TERRANE_NO_SITE,
                                        ),
                                    ),
                                )
                            }
                            Err(payload) => {
                                Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::TokioSequence::next",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        0 /* terrane-site: src/main.trn:4:16-4:30 */,
                    )
                }
            })
            .await,
        0 /* terrane-site: src/main.trn:4:16-4:30 */,
    );
    let _ = &step;
}
fn main() {
    __terrane_run(async move {
        let scope: TerraneTaskScope;
        let sequence: TokioSequence;
        let child: TerraneScopedTask<()>;
        let outcome: TerraneTaskOutcome<()>;
        scope = TerraneTaskScope::new(None);
        sequence = __terrane_raised(
            make_pending_tokio_sequence(),
            1 /* terrane-site: src/main.trn:7:14-7:42 */,
        );
        child = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = wait_next(sequence);
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        __terrane_spawned_task,
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
        let _ = &outcome;
    });
}
