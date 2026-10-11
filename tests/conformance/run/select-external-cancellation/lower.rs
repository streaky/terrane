// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: select-external-cancellation
async fn contender(
    marker: String,
    started: TerraneChannelSender<terrane_int_support::Int>,
    gate: TerraneChannelReceiver<terrane_int_support::Int>,
) {
    let mut sent: Option<TerraneChannelSendOutcome<terrane_int_support::Int>> = None;
    let mut received: Option<TerraneChannelReceiveOutcome<terrane_int_support::Int>> = None;
    let mut __terrane_finally_guard_0 = __terrane_finally_guard();
    let __terrane_maybe_completion_0: Option<TerraneCompletion<()>> = __terrane_cancel_operation(
            &__terrane_finally_guard_0,
            async {
                let __terrane_try_0: TerraneCompletion<()> = async {
                    let _ = sent
                        .insert(
                            __terrane_await(
                                    Box::pin(
                                        started.send(terrane_int_support::Int::from(1_i128)),
                                    ),
                                )
                                .await,
                        );
                    let _ = received
                        .insert(__terrane_await(Box::pin(gate.receive())).await);
                    println!(
                        "{}{}", terrane_scalar_support::scalar_text(&sent.as_ref()
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
    let __terrane_finally_0: TerraneCompletion<()> = (|| {
        println!("{}", terrane_scalar_support::scalar_text(&marker));
        TerraneCompletion::Normal
    })();
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
async fn wait_for_either(
    started_a: TerraneChannelSender<terrane_int_support::Int>,
    gate_a: TerraneChannelReceiver<terrane_int_support::Int>,
    started_b: TerraneChannelSender<terrane_int_support::Int>,
    gate_b: TerraneChannelReceiver<terrane_int_support::Int>,
) {
    let mut __terrane_select_cursor_517 = 0usize;
    {
        let mut __terrane_select_guard_517 = __terrane_finally_guard();
        let __terrane_select_control_517_0 = __terrane_select_control();
        let mut __terrane_select_future_517_0 = std::pin::pin!(
            __terrane_select_operation(__terrane_select_control_517_0.clone(),
            contender(String::from("cleanup-a"), started_a, gate_a))
        );
        let mut __terrane_select_result_517_0 = None;
        let __terrane_select_control_517_1 = __terrane_select_control();
        let mut __terrane_select_future_517_1 = std::pin::pin!(
            __terrane_select_operation(__terrane_select_control_517_1.clone(),
            contender(String::from("cleanup-b"), started_b, gate_b))
        );
        let mut __terrane_select_result_517_1 = None;
        let __terrane_select_winner_517 = std::future::poll_fn(|
                __terrane_select_context|
            {
                if __terrane_cancellation_is_requested() {
                    return std::task::Poll::Ready(usize::MAX);
                }
                for __terrane_select_offset in 0..2usize {
                    let __terrane_select_candidate = (__terrane_select_cursor_517
                        + __terrane_select_offset) % 2usize;
                    match __terrane_select_candidate {
                        0 => {
                            match Future::poll(
                                __terrane_select_future_517_0.as_mut(),
                                __terrane_select_context,
                            ) {
                                std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                    __terrane_select_result_517_0 = Some(
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
                                __terrane_select_future_517_1.as_mut(),
                                __terrane_select_context,
                            ) {
                                std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                    __terrane_select_result_517_1 = Some(
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
        if __terrane_select_winner_517 == usize::MAX {
            __terrane_select_control_517_1.request_cancel();
            __terrane_select_control_517_0.request_cancel();
            let _ = __terrane_select_future_517_1.as_mut().await;
            let _ = __terrane_select_future_517_0.as_mut().await;
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_517.finish();
            __terrane_finish_cancelled_select(__terrane_select_guard_517).await;
        }
        __terrane_select_cursor_517 = (__terrane_select_winner_517 + 1usize) % 2usize;
        match __terrane_select_winner_517 {
            0 => {
                __terrane_select_control_517_1.request_cancel();
                let _ = __terrane_select_future_517_1.as_mut().await;
            }
            1 => {
                __terrane_select_control_517_0.request_cancel();
                let _ = __terrane_select_future_517_0.as_mut().await;
            }
            _ => unreachable!("selected winner is within the case count"),
        }
        __terrane_wait_projected_cleanups().await;
        __terrane_select_guard_517.finish();
        match __terrane_select_winner_517 {
            0 => {
                let _ = __terrane_select_result_517_0
                    .take()
                    .expect("selected case owns its ready result");
                println!(
                    "{}", terrane_scalar_support::scalar_text(&String::from("a-won"))
                );
            }
            1 => {
                let _ = __terrane_select_result_517_1
                    .take()
                    .expect("selected case owns its ready result");
                println!(
                    "{}", terrane_scalar_support::scalar_text(&String::from("b-won"))
                );
            }
            _ => unreachable!("selected winner is within the case count"),
        }
    }
}
fn main() {
    __terrane_run(async move {
        let started_a: TerraneChannelPair<terrane_int_support::Int>;
        let started_b: TerraneChannelPair<terrane_int_support::Int>;
        let gate_a: TerraneChannelPair<terrane_int_support::Int>;
        let gate_b: TerraneChannelPair<terrane_int_support::Int>;
        let scope: TerraneTaskScope;
        let child: TerraneScopedTask<()>;
        let observed_a: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
        let observed_b: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
        let outcome: TerraneTaskOutcome<()>;
        started_a = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        started_b = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        gate_a = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        gate_b = TerraneChannelPair::new(
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
            let __terrane_spawned_task = wait_for_either(
                started_a.sender,
                gate_a.receiver,
                started_b.sender,
                gate_b.receiver,
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
        observed_a = __terrane_await(started_a.receiver.receive()).await;
        observed_b = __terrane_await(started_b.receiver.receive()).await;
        scope.cancel();
        outcome = __terrane_await(scope.join(child)).await;
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&observed_a.available),
            terrane_scalar_support::scalar_text(&observed_b.available),
            terrane_scalar_support::scalar_text(&outcome.cancelled)
        );
    });
}
