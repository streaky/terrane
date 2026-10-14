// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: heterogeneous-task-reassignment
async fn first() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(11_i128);
}
async fn second() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(22_i128);
}
async fn risky() -> Result<terrane_int_support::Int, TerraneError> {
    return Err(
        TerraneError::raised(
            TerraneErrorKind::CoercionError,
            0 /* terrane-site: case.trn:11:5-11:25 */,
        ),
    );
}
fn main() {
    __terrane_run(async move {
        let ordinary;
        let mut pending: std::pin::Pin<
            Box<dyn Future<Output = terrane_int_support::Int> + Send + '_>,
        >;
        let choose_first: bool;
        let mut selected: std::pin::Pin<
            Box<
                dyn Future<
                    Output = Result<terrane_int_support::Int, TerraneError>,
                > + Send + '_,
            >,
        >;
        let alias: std::pin::Pin<
            Box<
                dyn Future<
                    Output = Result<terrane_int_support::Int, TerraneError>,
                > + Send + '_,
            >,
        >;
        let alias_once: std::pin::Pin<
            Box<dyn Future<Output = terrane_int_support::Int> + Send + '_>,
        >;
        ordinary = first();
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(ordinary). await)
        );
        pending = Box::pin(first());
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(pending). await)
        );
        pending = Box::pin(second());
        let _ = &mut pending;
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(pending). await)
        );
        choose_first = true;
        selected = {
            let __terrane_task_future = first();
            Box::pin(async move { Ok(__terrane_task_future.await) })
        };
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await(selected)
            . await, 1 /* terrane-site: case.trn:23:18-23:26 */))
        );
        if choose_first {
            selected = {
                let __terrane_task_future = second();
                Box::pin(async move { Ok(__terrane_task_future.await) })
            };
            let _ = &mut selected;
        } else {
            selected = {
                let __terrane_task_future = first();
                Box::pin(async move { Ok(__terrane_task_future.await) })
            };
            let _ = &mut selected;
        }
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await(selected)
            . await, 2 /* terrane-site: case.trn:28:18-28:26 */))
        );
        selected = Box::pin(risky());
        let _ = &mut selected;
        alias = selected;
        let __terrane_completion_0: TerraneCompletion<()> = async {
            let __terrane_try_0: TerraneCompletion<()> = async {
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&__terrane_traced_completion!(__terrane_await(alias)
                    . await, 4 /* terrane-site: case.trn:33:22-33:27 */))
                );
                TerraneCompletion::Normal
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
                        __terrane_handled_0 = true;
                        println!("{}", String::from("caught"));
                    }
                    if !__terrane_handled_0 {
                        return TerraneCompletion::Error(__terrane_error_0);
                    }
                }
            }
            TerraneCompletion::Normal
        }
            .await;
        match __terrane_completion_0 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        pending = Box::pin(first());
        let _ = &mut pending;
        alias_once = pending;
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(alias_once).
            await)
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(branch(true)).
            await)
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(branch(false)).
            await)
        );
        {
            let _: () = __terrane_traced(
                __terrane_await(safe_group()).await,
                6 /* terrane-site: case.trn:42:15-42:28 */,
            );
        }
    });
}
async fn branch(choose: bool) -> terrane_int_support::Int {
    let mut pending: std::pin::Pin<
        Box<dyn Future<Output = terrane_int_support::Int> + Send + '_>,
    >;
    pending = Box::pin(first());
    {
        let _: terrane_int_support::Int = __terrane_await(pending).await;
    }
    if choose {
        pending = Box::pin(first());
        let _ = &mut pending;
    } else {
        pending = Box::pin(second());
        let _ = &mut pending;
    }
    return __terrane_await(pending).await;
}
async fn safe_group() -> Result<(), TerraneError> {
    __terrane_group_frame(async move {
            let mut pending: std::pin::Pin<
                Box<
                    dyn Future<
                        Output = Result<terrane_int_support::Int, TerraneError>,
                    > + Send + '_,
                >,
            >;
            let group: TerraneTaskGroup<terrane_int_support::Int>;
            let other: std::pin::Pin<
                Box<
                    dyn Future<
                        Output = Result<terrane_int_support::Int, TerraneError>,
                    > + Send + '_,
                >,
            >;
            pending = {
                let __terrane_task_future = first();
                Box::pin(async move { Ok(__terrane_task_future.await) })
            };
            group = TerraneTaskGroup::<terrane_int_support::Int>::new();
            {
                let __terrane_child = {
                    let __terrane_scope = group.scope();
                    let __terrane_cancel = __terrane_scope.cancellation();
                    let __terrane_deadline = __terrane_scope.deadline.clone();
                    let __terrane_spawned_task = first();
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
            other = pending;
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_traced_err(__terrane_await(other)
                . await, 7 /* terrane-site: case.trn:58:18-58:23 */) ?)
            );
            {
                let _: terrane_collection_support::List<
                    TerraneTaskOutcome<terrane_int_support::Int>,
                > = __terrane_await(std::boxed::Box::pin(group.prepare_drain())).await;
            }
            pending = Box::pin(risky());
            let _ = &mut pending;
            let __terrane_completion_1: TerraneCompletion<()> = async {
                let __terrane_try_1: TerraneCompletion<()> = async {
                    {
                        let _: terrane_int_support::Int = __terrane_traced_completion!(
                            __terrane_await(pending). await, 9 /* terrane-site: case.trn:62:19-62:26 */
                        );
                    }
                    TerraneCompletion::Normal
                }
                    .await;
                match __terrane_try_1 {
                    TerraneCompletion::Return(value) => {
                        return TerraneCompletion::Return(value);
                    }
                    TerraneCompletion::Break => return TerraneCompletion::Break,
                    TerraneCompletion::Continue => return TerraneCompletion::Continue,
                    TerraneCompletion::Normal => {}
                    TerraneCompletion::Error(__terrane_error_1) => {
                        let mut __terrane_handled_1 = false;
                        if !__terrane_handled_1 {
                            __terrane_handled_1 = true;
                            println!("{}", String::from("safe-before-risky"));
                        }
                        if !__terrane_handled_1 {
                            return TerraneCompletion::Error(__terrane_error_1);
                        }
                    }
                }
                TerraneCompletion::Normal
            }
                .await;
            match __terrane_completion_1 {
                TerraneCompletion::Normal => {}
                TerraneCompletion::Return(value) => return Ok(value),
                TerraneCompletion::Error(error) => return Err(error),
                TerraneCompletion::Break | TerraneCompletion::Continue => {
                    __terrane_generated_defect("loop control escaped a non-loop try")
                }
            }
            return Ok(());
        })
        .await
}
