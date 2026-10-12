// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: select-cleanup-shield
async fn loser(
    started: TerraneChannelSender<terrane_int_support::Int>,
    cleanup_started: TerraneChannelSender<terrane_int_support::Int>,
    cleanup_release: TerraneChannelReceiver<terrane_int_support::Int>,
    work: TerraneChannelReceiver<terrane_int_support::Int>,
) {
    let mut signalled: Option<TerraneChannelSendOutcome<terrane_int_support::Int>> = None;
    let mut received: Option<TerraneChannelReceiveOutcome<terrane_int_support::Int>> = None;
    let mut cleaning: Option<TerraneChannelSendOutcome<terrane_int_support::Int>> = None;
    let mut released: Option<TerraneChannelReceiveOutcome<terrane_int_support::Int>> = None;
    let mut __terrane_finally_guard_0 = __terrane_finally_guard();
    let __terrane_maybe_completion_0: Option<TerraneCompletion<()>> = __terrane_cancel_operation(
            &__terrane_finally_guard_0,
            async {
                let __terrane_try_0: TerraneCompletion<()> = async {
                    let _ = signalled
                        .insert(
                            __terrane_await(
                                    Box::pin(
                                        started.send(terrane_int_support::Int::from(1_i128)),
                                    ),
                                )
                                .await,
                        );
                    let _ = received
                        .insert(__terrane_await(Box::pin(work.receive())).await);
                    println!(
                        "{}{}", terrane_scalar_support::scalar_text(&signalled.as_ref()
                        .expect("flow-proven available binding").clone().accepted),
                        terrane_scalar_support::scalar_text(&received.as_ref()
                        .expect("flow-proven available binding").clone().available)
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
                            return TerraneCompletion::Error(__terrane_error_0);
                        }
                    }
                }
                TerraneCompletion::Normal
            },
        )
        .await;
    let __terrane_cancelled_0 = __terrane_maybe_completion_0.is_none();
    let mut __terrane_completion_0 = __terrane_maybe_completion_0
        .unwrap_or(TerraneCompletion::Normal);
    let __terrane_finally_0: TerraneCompletion<()> = async {
        let _ = cleaning
            .insert(
                __terrane_await(
                        Box::pin(
                            cleanup_started.send(terrane_int_support::Int::from(1_i128)),
                        ),
                    )
                    .await,
            );
        let _ = released
            .insert(__terrane_await(Box::pin(cleanup_release.receive())).await);
        println!(
            "{}{}{}", String::from("cleanup-finished"),
            terrane_scalar_support::scalar_text(&cleaning.as_ref()
            .expect("flow-proven available binding").clone().accepted),
            terrane_scalar_support::scalar_text(&released.as_ref()
            .expect("flow-proven available binding").clone().available)
        );
        TerraneCompletion::Normal
    }
        .await;
    match __terrane_finally_0 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_0 = replacement,
    }
    if __terrane_cancelled_0
        && matches!(&__terrane_completion_0, TerraneCompletion::Normal)
    {
        __terrane_finish_cancelled_finally(__terrane_finally_guard_0).await;
    }
    __terrane_finally_guard_0.finish();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
async fn winner(started: TerraneChannelReceiver<terrane_int_support::Int>) {
    let observed: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
    observed = __terrane_await(Box::pin(started.receive())).await;
    println!(
        "{}{}", String::from("winner-ready"),
        terrane_scalar_support::scalar_text(&observed.available)
    );
}
async fn selecting(
    started_tx: TerraneChannelSender<terrane_int_support::Int>,
    started_rx: TerraneChannelReceiver<terrane_int_support::Int>,
    cleanup_started: TerraneChannelSender<terrane_int_support::Int>,
    cleanup_release: TerraneChannelReceiver<terrane_int_support::Int>,
    work: TerraneChannelReceiver<terrane_int_support::Int>,
) {
    let mut __terrane_select_cursor_909 = 0usize;
    {
        let mut __terrane_select_guard_909 = __terrane_finally_guard();
        let __terrane_select_control_909_0 = __terrane_select_control();
        let mut __terrane_select_future_909_0 = std::pin::pin!(
            __terrane_select_operation(__terrane_select_control_909_0.clone(),
            loser(started_tx, cleanup_started, cleanup_release, work))
        );
        let mut __terrane_select_result_909_0 = None;
        let __terrane_select_control_909_1 = __terrane_select_control();
        let mut __terrane_select_future_909_1 = std::pin::pin!(
            __terrane_select_operation(__terrane_select_control_909_1.clone(),
            winner(started_rx))
        );
        let mut __terrane_select_result_909_1 = None;
        let __terrane_select_winner_909 = std::future::poll_fn(|
                __terrane_select_context|
            {
                if __terrane_cancellation_is_requested() {
                    return std::task::Poll::Ready(usize::MAX);
                }
                for __terrane_select_offset in 0..2usize {
                    let __terrane_select_candidate = (__terrane_select_cursor_909
                        + __terrane_select_offset) % 2usize;
                    match __terrane_select_candidate {
                        0 => {
                            match Future::poll(
                                __terrane_select_future_909_0.as_mut(),
                                __terrane_select_context,
                            ) {
                                std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                    __terrane_select_result_909_0 = Some(
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
                                __terrane_select_future_909_1.as_mut(),
                                __terrane_select_context,
                            ) {
                                std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                    __terrane_select_result_909_1 = Some(
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
        if __terrane_select_winner_909 == usize::MAX {
            __terrane_select_control_909_1.request_cancel();
            __terrane_select_control_909_0.request_cancel();
            let _ = __terrane_select_future_909_1.as_mut().await;
            let _ = __terrane_select_future_909_0.as_mut().await;
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_909.finish();
            __terrane_finish_cancelled_select(__terrane_select_guard_909).await;
        }
        __terrane_select_cursor_909 = (__terrane_select_winner_909 + 1usize) % 2usize;
        match __terrane_select_winner_909 {
            0 => {
                __terrane_select_control_909_1.request_cancel();
                let _ = __terrane_select_future_909_1.as_mut().await;
            }
            1 => {
                __terrane_select_control_909_0.request_cancel();
                let _ = __terrane_select_future_909_0.as_mut().await;
            }
            _ => unreachable!("selected winner is within the case count"),
        }
        __terrane_wait_projected_cleanups().await;
        __terrane_select_guard_909.finish();
        match __terrane_select_winner_909 {
            0 => {
                let _ = __terrane_select_result_909_0
                    .take()
                    .expect("selected case owns its ready result");
                println!("{}", String::from("unexpected loser"));
            }
            1 => {
                let _ = __terrane_select_result_909_1
                    .take()
                    .expect("selected case owns its ready result");
                println!("{}", String::from("winner-body"));
            }
            _ => unreachable!("selected winner is within the case count"),
        }
    }
}
fn main() {
    __terrane_run(async move {
        let started: TerraneChannelPair<terrane_int_support::Int>;
        let cleanup_started: TerraneChannelPair<terrane_int_support::Int>;
        let cleanup_release: TerraneChannelPair<terrane_int_support::Int>;
        let work: TerraneChannelPair<terrane_int_support::Int>;
        let scope: TerraneTaskScope;
        let child: TerraneScopedTask<()>;
        let cleaning: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
        let release: TerraneChannelSendOutcome<terrane_int_support::Int>;
        let outcome: TerraneTaskOutcome<()>;
        started = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        cleanup_started = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        cleanup_release = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        work = TerraneChannelPair::new(
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
            let __terrane_spawned_task = selecting(
                started.sender,
                started.receiver,
                cleanup_started.sender,
                cleanup_release.receiver,
                work.receiver,
            );
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
        cleaning = __terrane_await(cleanup_started.receiver.receive()).await;
        scope.cancel();
        release = __terrane_await(
                cleanup_release.sender.send(terrane_int_support::Int::from(1_i128)),
            )
            .await;
        outcome = __terrane_await(scope.join(child)).await;
        println!(
            "{}{}{}{}", terrane_scalar_support::scalar_text(&cleaning.available),
            terrane_scalar_support::scalar_text(&release.accepted),
            terrane_scalar_support::scalar_text(&outcome.completed),
            terrane_scalar_support::scalar_text(&outcome.cancelled)
        );
    });
}
