// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: select-loser-cleanup
async fn ready() -> String {
    return String::from("winner");
}
async fn blocked(
    marker: String,
    receiver: TerraneChannelReceiver<terrane_int_support::Int>,
) {
    let mut received: Option<TerraneChannelReceiveOutcome<terrane_int_support::Int>> = None;
    let mut __terrane_finally_guard_0 = __terrane_finally_guard();
    let __terrane_maybe_completion_0: Option<TerraneCompletion<()>> = __terrane_cancel_operation(
            &__terrane_finally_guard_0,
            async {
                let __terrane_try_0: TerraneCompletion<()> = async {
                    let _ = received
                        .insert(__terrane_await(Box::pin(receiver.receive())).await);
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&received.as_ref()
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
fn main() {
    __terrane_run(async move {
        let cleanup_pair_a: TerraneChannelPair<terrane_int_support::Int>;
        let cleanup_pair_b: TerraneChannelPair<terrane_int_support::Int>;
        let mut value: Option<()> = None;
        let mut winner: String;
        let waiter_pair: TerraneChannelPair<terrane_int_support::Int>;
        let waiter_tx: TerraneChannelSender<terrane_int_support::Int>;
        let waiter_rx: TerraneChannelReceiver<terrane_int_support::Int>;
        let mut received: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
        let sent: TerraneChannelSendOutcome<terrane_int_support::Int>;
        let value_2: Option<terrane_int_support::Int>;
        let mut __terrane_select_cursor_432 = 0usize;
        let mut __terrane_select_cursor_790 = 0usize;
        cleanup_pair_a = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        cleanup_pair_b = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        {
            let mut __terrane_select_guard_432 = __terrane_finally_guard();
            let __terrane_select_control_432_0 = __terrane_select_control();
            let mut __terrane_select_future_432_0 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_432_0.clone(),
                blocked(String::from("cleanup-a"), cleanup_pair_a.receiver))
            );
            let mut __terrane_select_result_432_0 = None;
            let __terrane_select_control_432_1 = __terrane_select_control();
            let mut __terrane_select_future_432_1 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_432_1.clone(),
                blocked(String::from("cleanup-b"), cleanup_pair_b.receiver))
            );
            let mut __terrane_select_result_432_1 = None;
            let __terrane_select_control_432_2 = __terrane_select_control();
            let mut __terrane_select_future_432_2 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_432_2.clone(),
                ready())
            );
            let mut __terrane_select_result_432_2 = None;
            let __terrane_select_winner_432 = std::future::poll_fn(|
                    __terrane_select_context|
                {
                    if __terrane_cancellation_is_requested() {
                        return std::task::Poll::Ready(usize::MAX);
                    }
                    for __terrane_select_offset in 0..3usize {
                        let __terrane_select_candidate = (__terrane_select_cursor_432
                            + __terrane_select_offset) % 3usize;
                        match __terrane_select_candidate {
                            0 => {
                                match Future::poll(
                                    __terrane_select_future_432_0.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_432_0 = Some(
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
                                    __terrane_select_future_432_1.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_432_1 = Some(
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
                            2 => {
                                match Future::poll(
                                    __terrane_select_future_432_2.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_432_2 = Some(
                                            __terrane_select_value,
                                        );
                                        return std::task::Poll::Ready(2usize);
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
            if __terrane_select_winner_432 == usize::MAX {
                __terrane_select_control_432_2.request_cancel();
                __terrane_select_control_432_1.request_cancel();
                __terrane_select_control_432_0.request_cancel();
                let _ = __terrane_select_future_432_2.as_mut().await;
                let _ = __terrane_select_future_432_1.as_mut().await;
                let _ = __terrane_select_future_432_0.as_mut().await;
                __terrane_wait_projected_cleanups().await;
                __terrane_select_guard_432.finish();
                __terrane_finish_cancelled_select(__terrane_select_guard_432).await;
            }
            __terrane_select_cursor_432 = (__terrane_select_winner_432 + 1usize)
                % 3usize;
            match __terrane_select_winner_432 {
                0 => {
                    __terrane_select_control_432_2.request_cancel();
                    __terrane_select_control_432_1.request_cancel();
                    let _ = __terrane_select_future_432_2.as_mut().await;
                    let _ = __terrane_select_future_432_1.as_mut().await;
                }
                1 => {
                    __terrane_select_control_432_2.request_cancel();
                    __terrane_select_control_432_0.request_cancel();
                    let _ = __terrane_select_future_432_2.as_mut().await;
                    let _ = __terrane_select_future_432_0.as_mut().await;
                }
                2 => {
                    __terrane_select_control_432_1.request_cancel();
                    __terrane_select_control_432_0.request_cancel();
                    let _ = __terrane_select_future_432_1.as_mut().await;
                    let _ = __terrane_select_future_432_0.as_mut().await;
                }
                _ => unreachable!("selected winner is within the case count"),
            }
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_432.finish();
            match __terrane_select_winner_432 {
                0 => {
                    let _ = value
                        .insert(
                            __terrane_select_result_432_0
                                .take()
                                .expect("selected case owns its ready result"),
                        );
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&* value.as_ref()
                        .expect("flow-proven available binding"))
                    );
                }
                1 => {
                    let _ = value
                        .insert(
                            __terrane_select_result_432_1
                                .take()
                                .expect("selected case owns its ready result"),
                        );
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&* value.as_ref()
                        .expect("flow-proven available binding"))
                    );
                }
                2 => {
                    winner = __terrane_select_result_432_2
                        .take()
                        .expect("selected case owns its ready result");
                    println!("{}", terrane_scalar_support::scalar_text(&winner));
                }
                _ => unreachable!("selected winner is within the case count"),
            }
        }
        waiter_pair = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        waiter_tx = waiter_pair.sender;
        waiter_rx = waiter_pair.receiver;
        {
            let mut __terrane_select_guard_790 = __terrane_finally_guard();
            let __terrane_select_control_790_0 = __terrane_select_control();
            let mut __terrane_select_future_790_0 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_790_0.clone(),
                Box::pin(waiter_rx.receive()))
            );
            let mut __terrane_select_result_790_0 = None;
            let __terrane_select_control_790_1 = __terrane_select_control();
            let mut __terrane_select_future_790_1 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_790_1.clone(),
                ready())
            );
            let mut __terrane_select_result_790_1 = None;
            let __terrane_select_winner_790 = std::future::poll_fn(|
                    __terrane_select_context|
                {
                    if __terrane_cancellation_is_requested() {
                        return std::task::Poll::Ready(usize::MAX);
                    }
                    for __terrane_select_offset in 0..2usize {
                        let __terrane_select_candidate = (__terrane_select_cursor_790
                            + __terrane_select_offset) % 2usize;
                        match __terrane_select_candidate {
                            0 => {
                                match Future::poll(
                                    __terrane_select_future_790_0.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_790_0 = Some(
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
                                    __terrane_select_future_790_1.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_790_1 = Some(
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
            if __terrane_select_winner_790 == usize::MAX {
                __terrane_select_control_790_1.request_cancel();
                __terrane_select_control_790_0.request_cancel();
                let _ = __terrane_select_future_790_1.as_mut().await;
                let _ = __terrane_select_future_790_0.as_mut().await;
                __terrane_wait_projected_cleanups().await;
                __terrane_select_guard_790.finish();
                __terrane_finish_cancelled_select(__terrane_select_guard_790).await;
            }
            __terrane_select_cursor_790 = (__terrane_select_winner_790 + 1usize)
                % 2usize;
            match __terrane_select_winner_790 {
                0 => {
                    __terrane_select_control_790_1.request_cancel();
                    let _ = __terrane_select_future_790_1.as_mut().await;
                }
                1 => {
                    __terrane_select_control_790_0.request_cancel();
                    let _ = __terrane_select_future_790_0.as_mut().await;
                }
                _ => unreachable!("selected winner is within the case count"),
            }
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_790.finish();
            match __terrane_select_winner_790 {
                0 => {
                    received = __terrane_select_result_790_0
                        .take()
                        .expect("selected case owns its ready result");
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&received.available)
                    );
                }
                1 => {
                    winner = __terrane_select_result_790_1
                        .take()
                        .expect("selected case owns its ready result");
                    println!("{}", terrane_scalar_support::scalar_text(&winner));
                }
                _ => unreachable!("selected winner is within the case count"),
            }
        }
        sent = __terrane_await(
                Box::pin(waiter_tx.send(terrane_int_support::Int::from(9_i128))),
            )
            .await;
        received = __terrane_await(Box::pin(waiter_rx.receive())).await;
        value_2 = received.value;
        if value_2.is_some() {
            println!(
                "{}{}", terrane_scalar_support::scalar_text(&sent.accepted),
                terrane_scalar_support::scalar_text(&match &value_2 { Some(value) =>
                value, _ => unreachable!("flow-proven storage refinement") })
            );
        }
    });
}
