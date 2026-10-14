// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: task-group-deferred-call
fn risky() -> Result<terrane_int_support::Int, TerraneError> {
    return Err(
        TerraneError::raised(
            TerraneErrorKind::CoercionError,
            0 /* terrane-site: case.trn:5:5-5:25 */,
        ),
    );
}
async fn child() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(1_i128);
}
fn main() {
    __terrane_run(async move {
        __terrane_group_frame(async move {
                let group: TerraneTaskGroup<terrane_int_support::Int>;
                let later: std::sync::Arc<
                    dyn Fn(
                    ) -> Result<terrane_int_support::Int, TerraneError> + Send + Sync,
                >;
                group = TerraneTaskGroup::<terrane_int_support::Int>::new();
                {
                    let __terrane_child = {
                        let __terrane_scope = group.scope();
                        let __terrane_cancel = __terrane_scope.cancellation();
                        let __terrane_deadline = __terrane_scope.deadline.clone();
                        let __terrane_spawned_task = child();
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
                    group.spawn(__terrane_child);
                };
                later = {
                    std::sync::Arc::new(move || -> Result<
                        terrane_int_support::Int,
                        TerraneError,
                    > {
                        return Ok(
                            __terrane_traced_err(
                                risky(),
                                1 /* terrane-site: case.trn:14:17-14:23 */,
                            )?,
                        );
                    })
                };
                {
                    let _: terrane_collection_support::List<
                        TerraneTaskOutcome<terrane_int_support::Int>,
                    > = __terrane_await(std::boxed::Box::pin(group.prepare_drain()))
                        .await;
                }
                let __terrane_completion_0: TerraneCompletion<()> = (|| {
                    let __terrane_try_0: TerraneCompletion<()> = (|| {
                        println!(
                            "{}",
                            terrane_scalar_support::scalar_text(&__terrane_traced_completion!(later(),
                            2 /* terrane-site: case.trn:17:17-17:23 */))
                        );
                        TerraneCompletion::Normal
                    })();
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
                                __terrane_handled_0 = true;
                                println!("{}", String::from("handled"));
                            }
                            if !__terrane_handled_0 {
                                return TerraneCompletion::Error(__terrane_error_0);
                            }
                        }
                    }
                    TerraneCompletion::Normal
                })();
                match __terrane_completion_0 {
                    TerraneCompletion::Normal => {}
                    TerraneCompletion::Return(value) => return value,
                    TerraneCompletion::Error(error) => __terrane_uncaught(error),
                    TerraneCompletion::Break | TerraneCompletion::Continue => {
                        __terrane_generated_defect("loop control escaped a non-loop try")
                    }
                }
            })
            .await
    });
}
