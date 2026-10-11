// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: select-task-outcomes
async fn blocked(receiver: TerraneChannelReceiver<terrane_int_support::Int>) {
    let received: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
    received = __terrane_await(Box::pin(receiver.receive())).await;
    println!("{}", terrane_scalar_support::scalar_text(&received.available));
}
async fn cancelled_winner() {
    let child_pair: TerraneChannelPair<terrane_int_support::Int>;
    let spare_pair: TerraneChannelPair<terrane_int_support::Int>;
    let scope: TerraneTaskScope;
    let child: TerraneScopedTask<()>;
    let outcome: TerraneTaskOutcome<()>;
    let received: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
    let mut __terrane_select_cursor_466 = 0usize;
    child_pair = TerraneChannelPair::new(
        terrane_collection_support::index_from_int(
                &terrane_int_support::Int::from(0_i128),
            )
            .expect("semantic channel capacity"),
        TerraneChannelOverflow::Block,
    );
    spare_pair = TerraneChannelPair::new(
        terrane_collection_support::index_from_int(
                &terrane_int_support::Int::from(0_i128),
            )
            .expect("semantic channel capacity"),
        TerraneChannelOverflow::Block,
    );
    scope = TerraneTaskScope::new(None);
    child = {
        let __terrane_scope = scope.clone();
        let __terrane_cancel = __terrane_scope.cancellation();
        let __terrane_deadline = __terrane_scope.deadline;
        let __terrane_spawned_task = blocked(child_pair.receiver);
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
    scope.cancel();
    {
        let mut __terrane_select_guard_466 = __terrane_finally_guard();
        let __terrane_select_control_466_0 = __terrane_select_control();
        let mut __terrane_select_future_466_0 = std::pin::pin!(
            __terrane_select_operation(__terrane_select_control_466_0.clone(), scope
            .join(child))
        );
        let mut __terrane_select_result_466_0 = None;
        let __terrane_select_control_466_1 = __terrane_select_control();
        let mut __terrane_select_future_466_1 = std::pin::pin!(
            __terrane_select_operation(__terrane_select_control_466_1.clone(), spare_pair
            .receiver.receive())
        );
        let mut __terrane_select_result_466_1 = None;
        let __terrane_select_winner_466 = std::future::poll_fn(|
                __terrane_select_context|
            {
                if __terrane_cancellation_is_requested() {
                    return std::task::Poll::Ready(usize::MAX);
                }
                for __terrane_select_offset in 0..2usize {
                    let __terrane_select_candidate = (__terrane_select_cursor_466
                        + __terrane_select_offset) % 2usize;
                    match __terrane_select_candidate {
                        0 => {
                            match Future::poll(
                                __terrane_select_future_466_0.as_mut(),
                                __terrane_select_context,
                            ) {
                                std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                    __terrane_select_result_466_0 = Some(
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
                                __terrane_select_future_466_1.as_mut(),
                                __terrane_select_context,
                            ) {
                                std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                    __terrane_select_result_466_1 = Some(
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
        if __terrane_select_winner_466 == usize::MAX {
            __terrane_select_control_466_1.request_cancel();
            __terrane_select_control_466_0.request_cancel();
            let _ = __terrane_select_future_466_1.as_mut().await;
            let _ = __terrane_select_future_466_0.as_mut().await;
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_466.finish();
            __terrane_finish_cancelled_select(__terrane_select_guard_466).await;
        }
        __terrane_select_cursor_466 = (__terrane_select_winner_466 + 1usize) % 2usize;
        match __terrane_select_winner_466 {
            0 => {
                __terrane_select_control_466_1.request_cancel();
                let _ = __terrane_select_future_466_1.as_mut().await;
            }
            1 => {
                __terrane_select_control_466_0.request_cancel();
                let _ = __terrane_select_future_466_0.as_mut().await;
            }
            _ => unreachable!("selected winner is within the case count"),
        }
        __terrane_wait_projected_cleanups().await;
        __terrane_select_guard_466.finish();
        match __terrane_select_winner_466 {
            0 => {
                outcome = __terrane_select_result_466_0
                    .take()
                    .expect("selected case owns its ready result");
                println!(
                    "{}{}{}",
                    terrane_scalar_support::scalar_text(&String::from("cancelled")),
                    terrane_scalar_support::scalar_text(&outcome.completed),
                    terrane_scalar_support::scalar_text(&outcome.cancelled)
                );
            }
            1 => {
                received = __terrane_select_result_466_1
                    .take()
                    .expect("selected case owns its ready result");
                println!("{}", terrane_scalar_support::scalar_text(&received.available));
            }
            _ => unreachable!("selected winner is within the case count"),
        }
    }
}
async fn deadline_winner() {
    let child_pair: TerraneChannelPair<terrane_int_support::Int>;
    let spare_pair: TerraneChannelPair<terrane_int_support::Int>;
    let scope: TerraneTaskScope;
    let child: TerraneScopedTask<()>;
    let outcome: TerraneTaskOutcome<()>;
    let received: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
    let mut __terrane_select_cursor_915 = 0usize;
    child_pair = TerraneChannelPair::new(
        terrane_collection_support::index_from_int(
                &terrane_int_support::Int::from(0_i128),
            )
            .expect("semantic channel capacity"),
        TerraneChannelOverflow::Block,
    );
    spare_pair = TerraneChannelPair::new(
        terrane_collection_support::index_from_int(
                &terrane_int_support::Int::from(0_i128),
            )
            .expect("semantic channel capacity"),
        TerraneChannelOverflow::Block,
    );
    scope = TerraneTaskScope::new(
        Some(
            Clock::terrane_static_deadline(
                    __terrane_traced(
                        Duration::terrane_static_nanoseconds(
                            terrane_int_support::Int::from(0_i128),
                        ),
                        0 /* terrane-site: case.trn:24:42-24:66 */,
                    ),
                )
                .expires_at
                .elapsed_nanoseconds
                .clone(),
        ),
    );
    child = {
        let __terrane_scope = scope.clone();
        let __terrane_cancel = __terrane_scope.cancellation();
        let __terrane_deadline = __terrane_scope.deadline;
        let __terrane_spawned_task = blocked(child_pair.receiver);
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
    {
        let mut __terrane_select_guard_915 = __terrane_finally_guard();
        let __terrane_select_control_915_0 = __terrane_select_control();
        let mut __terrane_select_future_915_0 = std::pin::pin!(
            __terrane_select_operation(__terrane_select_control_915_0.clone(), scope
            .join(child))
        );
        let mut __terrane_select_result_915_0 = None;
        let __terrane_select_control_915_1 = __terrane_select_control();
        let mut __terrane_select_future_915_1 = std::pin::pin!(
            __terrane_select_operation(__terrane_select_control_915_1.clone(), spare_pair
            .receiver.receive())
        );
        let mut __terrane_select_result_915_1 = None;
        let __terrane_select_winner_915 = std::future::poll_fn(|
                __terrane_select_context|
            {
                if __terrane_cancellation_is_requested() {
                    return std::task::Poll::Ready(usize::MAX);
                }
                for __terrane_select_offset in 0..2usize {
                    let __terrane_select_candidate = (__terrane_select_cursor_915
                        + __terrane_select_offset) % 2usize;
                    match __terrane_select_candidate {
                        0 => {
                            match Future::poll(
                                __terrane_select_future_915_0.as_mut(),
                                __terrane_select_context,
                            ) {
                                std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                    __terrane_select_result_915_0 = Some(
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
                                __terrane_select_future_915_1.as_mut(),
                                __terrane_select_context,
                            ) {
                                std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                    __terrane_select_result_915_1 = Some(
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
        if __terrane_select_winner_915 == usize::MAX {
            __terrane_select_control_915_1.request_cancel();
            __terrane_select_control_915_0.request_cancel();
            let _ = __terrane_select_future_915_1.as_mut().await;
            let _ = __terrane_select_future_915_0.as_mut().await;
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_915.finish();
            __terrane_finish_cancelled_select(__terrane_select_guard_915).await;
        }
        __terrane_select_cursor_915 = (__terrane_select_winner_915 + 1usize) % 2usize;
        match __terrane_select_winner_915 {
            0 => {
                __terrane_select_control_915_1.request_cancel();
                let _ = __terrane_select_future_915_1.as_mut().await;
            }
            1 => {
                __terrane_select_control_915_0.request_cancel();
                let _ = __terrane_select_future_915_0.as_mut().await;
            }
            _ => unreachable!("selected winner is within the case count"),
        }
        __terrane_wait_projected_cleanups().await;
        __terrane_select_guard_915.finish();
        match __terrane_select_winner_915 {
            0 => {
                outcome = __terrane_select_result_915_0
                    .take()
                    .expect("selected case owns its ready result");
                println!(
                    "{}{}{}",
                    terrane_scalar_support::scalar_text(&String::from("deadline")),
                    terrane_scalar_support::scalar_text(&outcome.completed),
                    terrane_scalar_support::scalar_text(&outcome.cancelled)
                );
            }
            1 => {
                received = __terrane_select_result_915_1
                    .take()
                    .expect("selected case owns its ready result");
                println!("{}", terrane_scalar_support::scalar_text(&received.available));
            }
            _ => unreachable!("selected winner is within the case count"),
        }
    }
}
fn main() {
    __terrane_run(async move {
        let cancelled: ();
        let deadline_result: ();
        cancelled = __terrane_await(cancelled_winner()).await;
        deadline_result = __terrane_await(deadline_winner()).await;
        if cancelled == () && deadline_result == () {}
    });
}
