// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: select-pending-wakeup
async fn choose(
    started: TerraneChannelSender<terrane_int_support::Int>,
    left: TerraneChannelReceiver<terrane_int_support::Int>,
    right: TerraneChannelReceiver<terrane_int_support::Int>,
) {
    let signalled: TerraneChannelSendOutcome<terrane_int_support::Int>;
    let received: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
    let value: Option<terrane_int_support::Int>;
    let mut __terrane_select_cursor_244 = 0usize;
    signalled = __terrane_await(
            Box::pin(started.send(terrane_int_support::Int::from(1_i128))),
        )
        .await;
    let _ = &signalled;
    {
        let mut __terrane_select_guard_244 = __terrane_finally_guard();
        let __terrane_select_control_244_0 = __terrane_select_control();
        let mut __terrane_select_future_244_0 = std::pin::pin!(
            __terrane_select_operation(__terrane_select_control_244_0.clone(),
            Box::pin(left.receive()))
        );
        let mut __terrane_select_result_244_0 = None;
        let __terrane_select_control_244_1 = __terrane_select_control();
        let mut __terrane_select_future_244_1 = std::pin::pin!(
            __terrane_select_operation(__terrane_select_control_244_1.clone(),
            Box::pin(right.receive()))
        );
        let mut __terrane_select_result_244_1 = None;
        let __terrane_select_winner_244 = std::future::poll_fn(|
                __terrane_select_context|
            {
                if __terrane_cancellation_is_requested() {
                    return std::task::Poll::Ready(usize::MAX);
                }
                for __terrane_select_offset in 0..2usize {
                    let __terrane_select_candidate = (__terrane_select_cursor_244
                        + __terrane_select_offset) % 2usize;
                    match __terrane_select_candidate {
                        0 => {
                            match Future::poll(
                                __terrane_select_future_244_0.as_mut(),
                                __terrane_select_context,
                            ) {
                                std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                    __terrane_select_result_244_0 = Some(
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
                                __terrane_select_future_244_1.as_mut(),
                                __terrane_select_context,
                            ) {
                                std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                    __terrane_select_result_244_1 = Some(
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
        if __terrane_select_winner_244 == usize::MAX {
            __terrane_select_control_244_1.request_cancel();
            __terrane_select_control_244_0.request_cancel();
            let _ = __terrane_select_future_244_1.as_mut().await;
            let _ = __terrane_select_future_244_0.as_mut().await;
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_244.finish();
            __terrane_finish_cancelled_select(__terrane_select_guard_244).await;
        }
        __terrane_select_cursor_244 = (__terrane_select_winner_244 + 1usize) % 2usize;
        match __terrane_select_winner_244 {
            0 => {
                __terrane_select_control_244_1.request_cancel();
                let _ = __terrane_select_future_244_1.as_mut().await;
            }
            1 => {
                __terrane_select_control_244_0.request_cancel();
                let _ = __terrane_select_future_244_0.as_mut().await;
            }
            _ => unreachable!("selected winner is within the case count"),
        }
        __terrane_wait_projected_cleanups().await;
        __terrane_select_guard_244.finish();
        match __terrane_select_winner_244 {
            0 => {
                received = __terrane_select_result_244_0
                    .take()
                    .expect("selected case owns its ready result");
                value = received.value;
                if (&value).is_some() {
                    println!(
                        "{}{}", String::from("left"),
                        terrane_scalar_support::scalar_text(&match &value { Some(value)
                        => value, _ => unreachable!("flow-proven storage refinement") })
                    );
                }
            }
            1 => {
                received = __terrane_select_result_244_1
                    .take()
                    .expect("selected case owns its ready result");
                value = received.value;
                if (&value).is_some() {
                    println!(
                        "{}{}", String::from("right"),
                        terrane_scalar_support::scalar_text(&match &value { Some(value)
                        => value, _ => unreachable!("flow-proven storage refinement") })
                    );
                }
            }
            _ => unreachable!("selected winner is within the case count"),
        }
    }
}
fn main() {
    __terrane_run(async move {
        let started: TerraneChannelPair<terrane_int_support::Int>;
        let left: TerraneChannelPair<terrane_int_support::Int>;
        let right: TerraneChannelPair<terrane_int_support::Int>;
        let scope: TerraneTaskScope;
        let child: TerraneScopedTask<()>;
        let observed: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
        let sent: TerraneChannelSendOutcome<terrane_int_support::Int>;
        let outcome: TerraneTaskOutcome<()>;
        started = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        left = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        right = TerraneChannelPair::new(
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
            let __terrane_spawned_task = choose(
                started.sender,
                left.receiver,
                right.receiver,
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
        observed = __terrane_await(started.receiver.receive()).await;
        sent = __terrane_await(right.sender.send(terrane_int_support::Int::from(9_i128)))
            .await;
        outcome = __terrane_await(scope.join(child)).await;
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&observed.available),
            terrane_scalar_support::scalar_text(&sent.accepted),
            terrane_scalar_support::scalar_text(&outcome.completed)
        );
    });
}
