// Generated deterministically by Terrane <version>.
// Runtime support: async_native.rs, executor_parallel.rs, tasks_native_parallel.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: select-concurrent-activations
async fn a() -> String {
    return String::from("a");
}
async fn b() -> String {
    return String::from("b");
}
async fn x() -> String {
    return String::from("x");
}
async fn y() -> String {
    return String::from("y");
}
fn recursive_choose() -> std::sync::Arc<
    dyn Fn(
        terrane_int_support::Int,
    ) -> std::pin::Pin<Box<dyn Future<Output = String> + Send>> + Send + Sync,
> {
    return std::sync::Arc::new(move |
        argument_0: terrane_int_support::Int,
    | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
        Box::pin(choose(argument_0))
    });
}
async fn choose(depth: terrane_int_support::Int) -> String {
    let mut __terrane_select_cursor_317 = 0usize;
    let mut __terrane_select_cursor_447 = 0usize;
    {
        let mut __terrane_select_guard_317 = __terrane_finally_guard();
        let __terrane_select_control_317_0 = __terrane_select_control();
        let mut __terrane_select_future_317_0 = std::pin::pin!(
            __terrane_select_operation(__terrane_select_control_317_0.clone(), a())
        );
        let mut __terrane_select_result_317_0 = None;
        let __terrane_select_control_317_1 = __terrane_select_control();
        let mut __terrane_select_future_317_1 = std::pin::pin!(
            __terrane_select_operation(__terrane_select_control_317_1.clone(), b())
        );
        let mut __terrane_select_result_317_1 = None;
        let __terrane_select_winner_317 = std::future::poll_fn(|
                __terrane_select_context|
            {
                if __terrane_cancellation_is_requested() {
                    return std::task::Poll::Ready(usize::MAX);
                }
                for __terrane_select_offset in 0..2usize {
                    let __terrane_select_candidate = (__terrane_select_cursor_317
                        + __terrane_select_offset) % 2usize;
                    match __terrane_select_candidate {
                        0 => {
                            match Future::poll(
                                __terrane_select_future_317_0.as_mut(),
                                __terrane_select_context,
                            ) {
                                std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                    __terrane_select_result_317_0 = Some(
                                        __terrane_select_value,
                                    );
                                    return std::task::Poll::Ready(0usize);
                                }
                                std::task::Poll::Ready(None) => {
                                    unreachable!(
                                        "case cancellation starts only after winner selection"
                                    )
                                }
                                std::task::Poll::Pending => {}
                            }
                        }
                        1 => {
                            match Future::poll(
                                __terrane_select_future_317_1.as_mut(),
                                __terrane_select_context,
                            ) {
                                std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                    __terrane_select_result_317_1 = Some(
                                        __terrane_select_value,
                                    );
                                    return std::task::Poll::Ready(1usize);
                                }
                                std::task::Poll::Ready(None) => {
                                    unreachable!(
                                        "case cancellation starts only after winner selection"
                                    )
                                }
                                std::task::Poll::Pending => {}
                            }
                        }
                        _ => unreachable!("select candidate is within the case count"),
                    }
                }
                std::task::Poll::Pending
            })
            .await;
        if __terrane_select_winner_317 == usize::MAX {
            __terrane_select_control_317_1.request_cancel();
            __terrane_select_control_317_0.request_cancel();
            let _ = __terrane_select_future_317_1.as_mut().await;
            let _ = __terrane_select_future_317_0.as_mut().await;
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_317.finish();
            __terrane_finish_cancelled_select(__terrane_select_guard_317).await;
        }
        __terrane_select_cursor_317 = (__terrane_select_winner_317 + 1usize) % 2usize;
        match __terrane_select_winner_317 {
            0 => {
                __terrane_select_control_317_1.request_cancel();
                let _ = __terrane_select_future_317_1.as_mut().await;
            }
            1 => {
                __terrane_select_control_317_0.request_cancel();
                let _ = __terrane_select_future_317_0.as_mut().await;
            }
            _ => unreachable!("selected winner is within the case count"),
        }
        __terrane_wait_projected_cleanups().await;
        __terrane_select_guard_317.finish();
        match __terrane_select_winner_317 {
            0 => {
                let _ = __terrane_select_result_317_0
                    .take()
                    .expect("selected case owns its ready result");
                if depth.clone() > terrane_int_support::Int::from(0_i128) {
                    let recurse: std::sync::Arc<
                        dyn Fn(
                            terrane_int_support::Int,
                        ) -> std::pin::Pin<
                                Box<dyn Future<Output = String> + Send>,
                            > + Send + Sync,
                    > = recursive_choose();
                    return __terrane_await(
                            recurse(
                                depth.clone() - terrane_int_support::Int::from(1_i128),
                            ),
                        )
                        .await;
                }
                {
                    let mut __terrane_select_guard_447 = __terrane_finally_guard();
                    let __terrane_select_control_447_0 = __terrane_select_control();
                    let mut __terrane_select_future_447_0 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_447_0
                        .clone(), x())
                    );
                    let mut __terrane_select_result_447_0 = None;
                    let __terrane_select_control_447_1 = __terrane_select_control();
                    let mut __terrane_select_future_447_1 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_447_1
                        .clone(), y())
                    );
                    let mut __terrane_select_result_447_1 = None;
                    let __terrane_select_winner_447 = std::future::poll_fn(|
                            __terrane_select_context|
                        {
                            if __terrane_cancellation_is_requested() {
                                return std::task::Poll::Ready(usize::MAX);
                            }
                            for __terrane_select_offset in 0..2usize {
                                let __terrane_select_candidate = (__terrane_select_cursor_447
                                    + __terrane_select_offset) % 2usize;
                                match __terrane_select_candidate {
                                    0 => {
                                        match Future::poll(
                                            __terrane_select_future_447_0.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_447_0 = Some(
                                                    __terrane_select_value,
                                                );
                                                return std::task::Poll::Ready(0usize);
                                            }
                                            std::task::Poll::Ready(None) => {
                                                unreachable!(
                                                    "case cancellation starts only after winner selection"
                                                )
                                            }
                                            std::task::Poll::Pending => {}
                                        }
                                    }
                                    1 => {
                                        match Future::poll(
                                            __terrane_select_future_447_1.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_447_1 = Some(
                                                    __terrane_select_value,
                                                );
                                                return std::task::Poll::Ready(1usize);
                                            }
                                            std::task::Poll::Ready(None) => {
                                                unreachable!(
                                                    "case cancellation starts only after winner selection"
                                                )
                                            }
                                            std::task::Poll::Pending => {}
                                        }
                                    }
                                    _ => {
                                        unreachable!("select candidate is within the case count")
                                    }
                                }
                            }
                            std::task::Poll::Pending
                        })
                        .await;
                    if __terrane_select_winner_447 == usize::MAX {
                        __terrane_select_control_447_1.request_cancel();
                        __terrane_select_control_447_0.request_cancel();
                        let _ = __terrane_select_future_447_1.as_mut().await;
                        let _ = __terrane_select_future_447_0.as_mut().await;
                        __terrane_wait_projected_cleanups().await;
                        __terrane_select_guard_447.finish();
                        __terrane_finish_cancelled_select(__terrane_select_guard_447)
                            .await;
                    }
                    __terrane_select_cursor_447 = (__terrane_select_winner_447 + 1usize)
                        % 2usize;
                    match __terrane_select_winner_447 {
                        0 => {
                            __terrane_select_control_447_1.request_cancel();
                            let _ = __terrane_select_future_447_1.as_mut().await;
                        }
                        1 => {
                            __terrane_select_control_447_0.request_cancel();
                            let _ = __terrane_select_future_447_0.as_mut().await;
                        }
                        _ => unreachable!("selected winner is within the case count"),
                    }
                    __terrane_wait_projected_cleanups().await;
                    __terrane_select_guard_447.finish();
                    match __terrane_select_winner_447 {
                        0 => {
                            let _ = __terrane_select_result_447_0
                                .take()
                                .expect("selected case owns its ready result");
                            return String::from("ax");
                        }
                        1 => {
                            let _ = __terrane_select_result_447_1
                                .take()
                                .expect("selected case owns its ready result");
                            return String::from("ay");
                        }
                        _ => unreachable!("selected winner is within the case count"),
                    }
                }
            }
            1 => {
                let _ = __terrane_select_result_317_1
                    .take()
                    .expect("selected case owns its ready result");
                return String::from("b");
            }
            _ => unreachable!("selected winner is within the case count"),
        }
    }
}
fn main() {
    __terrane_run(async move {
        let scope: TerraneTaskScope = TerraneTaskScope::new(None);
        let first: TerraneScopedTask<String> = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = choose(terrane_int_support::Int::from(1_i128));
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
        let second: TerraneScopedTask<String> = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = choose(terrane_int_support::Int::from(1_i128));
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
        let first_outcome: TerraneTaskOutcome<String> = __terrane_await(
                scope.join(first),
            )
            .await;
        let second_outcome: TerraneTaskOutcome<String> = __terrane_await(
                scope.join(second),
            )
            .await;
        let first_value: Option<String> = first_outcome.value.clone();
        let second_value: Option<String> = second_outcome.value.clone();
        if first_value.is_some() {
            if second_value.is_some() {
                println!(
                    "{}{}", terrane_scalar_support::scalar_text(&* first_value.as_ref()
                    .expect("semantic optional narrowing")),
                    terrane_scalar_support::scalar_text(&* second_value.as_ref()
                    .expect("semantic optional narrowing"))
                );
            }
        }
    });
}
