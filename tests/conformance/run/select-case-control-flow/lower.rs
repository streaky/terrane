// Generated deterministically by Terrane <version>.
// Runtime support: async.rs, async_select.rs, executor_parallel.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: select-case-control-flow
async fn one() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(1_i128);
}
async fn two() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(2_i128);
}
async fn selected_return() -> terrane_int_support::Int {
    let mut __terrane_select_cursor_197 = 0usize;
    {
        let mut __terrane_select_guard_197 = __terrane_finally_guard();
        let __terrane_select_control_197_0 = __terrane_select_control();
        let mut __terrane_select_future_197_0 = std::pin::pin!(
            __terrane_select_operation(__terrane_select_control_197_0.clone(), one())
        );
        let mut __terrane_select_result_197_0 = None;
        let __terrane_select_control_197_1 = __terrane_select_control();
        let mut __terrane_select_future_197_1 = std::pin::pin!(
            __terrane_select_operation(__terrane_select_control_197_1.clone(), two())
        );
        let mut __terrane_select_result_197_1 = None;
        let __terrane_select_winner_197 = std::future::poll_fn(|
                __terrane_select_context|
            {
                if __terrane_cancellation_is_requested() {
                    return std::task::Poll::Ready(usize::MAX);
                }
                for __terrane_select_offset in 0..2usize {
                    let __terrane_select_candidate = (__terrane_select_cursor_197
                        + __terrane_select_offset) % 2usize;
                    match __terrane_select_candidate {
                        0 => {
                            match Future::poll(
                                __terrane_select_future_197_0.as_mut(),
                                __terrane_select_context,
                            ) {
                                std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                    __terrane_select_result_197_0 = Some(
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
                                __terrane_select_future_197_1.as_mut(),
                                __terrane_select_context,
                            ) {
                                std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                    __terrane_select_result_197_1 = Some(
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
        if __terrane_select_winner_197 == usize::MAX {
            __terrane_select_control_197_1.request_cancel();
            __terrane_select_control_197_0.request_cancel();
            let _ = __terrane_select_future_197_1.as_mut().await;
            let _ = __terrane_select_future_197_0.as_mut().await;
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_197.finish();
            __terrane_finish_cancelled_select(__terrane_select_guard_197).await;
        }
        __terrane_select_cursor_197 = (__terrane_select_winner_197 + 1usize) % 2usize;
        match __terrane_select_winner_197 {
            0 => {
                __terrane_select_control_197_1.request_cancel();
                let _ = __terrane_select_future_197_1.as_mut().await;
            }
            1 => {
                __terrane_select_control_197_0.request_cancel();
                let _ = __terrane_select_future_197_0.as_mut().await;
            }
            _ => unreachable!("selected winner is within the case count"),
        }
        __terrane_wait_projected_cleanups().await;
        __terrane_select_guard_197.finish();
        match __terrane_select_winner_197 {
            0 => {
                let value: terrane_int_support::Int = __terrane_select_result_197_0
                    .take()
                    .expect("selected case owns its ready result");
                return value.clone();
            }
            1 => {
                let value: terrane_int_support::Int = __terrane_select_result_197_1
                    .take()
                    .expect("selected case owns its ready result");
                return value.clone();
            }
            _ => unreachable!("selected winner is within the case count"),
        }
    }
}
async fn selected_throw() -> Result<(), TerraneError> {
    let mut __terrane_select_cursor_353 = 0usize;
    {
        let mut __terrane_select_guard_353 = __terrane_finally_guard();
        let __terrane_select_control_353_0 = __terrane_select_control();
        let mut __terrane_select_future_353_0 = std::pin::pin!(
            __terrane_select_operation(__terrane_select_control_353_0.clone(), one())
        );
        let mut __terrane_select_result_353_0 = None;
        let __terrane_select_control_353_1 = __terrane_select_control();
        let mut __terrane_select_future_353_1 = std::pin::pin!(
            __terrane_select_operation(__terrane_select_control_353_1.clone(), two())
        );
        let mut __terrane_select_result_353_1 = None;
        let __terrane_select_winner_353 = std::future::poll_fn(|
                __terrane_select_context|
            {
                if __terrane_cancellation_is_requested() {
                    return std::task::Poll::Ready(usize::MAX);
                }
                for __terrane_select_offset in 0..2usize {
                    let __terrane_select_candidate = (__terrane_select_cursor_353
                        + __terrane_select_offset) % 2usize;
                    match __terrane_select_candidate {
                        0 => {
                            match Future::poll(
                                __terrane_select_future_353_0.as_mut(),
                                __terrane_select_context,
                            ) {
                                std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                    __terrane_select_result_353_0 = Some(
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
                                __terrane_select_future_353_1.as_mut(),
                                __terrane_select_context,
                            ) {
                                std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                    __terrane_select_result_353_1 = Some(
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
        if __terrane_select_winner_353 == usize::MAX {
            __terrane_select_control_353_1.request_cancel();
            __terrane_select_control_353_0.request_cancel();
            let _ = __terrane_select_future_353_1.as_mut().await;
            let _ = __terrane_select_future_353_0.as_mut().await;
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_353.finish();
            __terrane_finish_cancelled_select(__terrane_select_guard_353).await;
        }
        __terrane_select_cursor_353 = (__terrane_select_winner_353 + 1usize) % 2usize;
        match __terrane_select_winner_353 {
            0 => {
                __terrane_select_control_353_1.request_cancel();
                let _ = __terrane_select_future_353_1.as_mut().await;
            }
            1 => {
                __terrane_select_control_353_0.request_cancel();
                let _ = __terrane_select_future_353_0.as_mut().await;
            }
            _ => unreachable!("selected winner is within the case count"),
        }
        __terrane_wait_projected_cleanups().await;
        __terrane_select_guard_353.finish();
        match __terrane_select_winner_353 {
            0 => {
                let _ = __terrane_select_result_353_0
                    .take()
                    .expect("selected case owns its ready result");
                return Err(
                    TerraneError::raised(
                        TerraneErrorKind::CoercionError,
                        0 /* terrane-site: case.trn:20:7-20:27 */,
                    ),
                );
            }
            1 => {
                let _ = __terrane_select_result_353_1
                    .take()
                    .expect("selected case owns its ready result");
                return Ok(());
            }
            _ => unreachable!("selected winner is within the case count"),
        }
    }
}
fn main() {
    __terrane_run(async move {
        let mut __terrane_select_cursor_508 = 0usize;
        let mut iteration: terrane_int_support::Int = terrane_int_support::Int::from(
            0_i128,
        );
        while iteration.clone() < terrane_int_support::Int::from(2_i128) {
            {
                let mut __terrane_select_guard_508 = __terrane_finally_guard();
                let __terrane_select_control_508_0 = __terrane_select_control();
                let mut __terrane_select_future_508_0 = std::pin::pin!(
                    __terrane_select_operation(__terrane_select_control_508_0.clone(),
                    one())
                );
                let mut __terrane_select_result_508_0 = None;
                let __terrane_select_control_508_1 = __terrane_select_control();
                let mut __terrane_select_future_508_1 = std::pin::pin!(
                    __terrane_select_operation(__terrane_select_control_508_1.clone(),
                    two())
                );
                let mut __terrane_select_result_508_1 = None;
                let __terrane_select_winner_508 = std::future::poll_fn(|
                        __terrane_select_context|
                    {
                        if __terrane_cancellation_is_requested() {
                            return std::task::Poll::Ready(usize::MAX);
                        }
                        for __terrane_select_offset in 0..2usize {
                            let __terrane_select_candidate = (__terrane_select_cursor_508
                                + __terrane_select_offset) % 2usize;
                            match __terrane_select_candidate {
                                0 => {
                                    match Future::poll(
                                        __terrane_select_future_508_0.as_mut(),
                                        __terrane_select_context,
                                    ) {
                                        std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                            __terrane_select_result_508_0 = Some(
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
                                        __terrane_select_future_508_1.as_mut(),
                                        __terrane_select_context,
                                    ) {
                                        std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                            __terrane_select_result_508_1 = Some(
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
                if __terrane_select_winner_508 == usize::MAX {
                    __terrane_select_control_508_1.request_cancel();
                    __terrane_select_control_508_0.request_cancel();
                    let _ = __terrane_select_future_508_1.as_mut().await;
                    let _ = __terrane_select_future_508_0.as_mut().await;
                    __terrane_wait_projected_cleanups().await;
                    __terrane_select_guard_508.finish();
                    __terrane_finish_cancelled_select(__terrane_select_guard_508).await;
                }
                __terrane_select_cursor_508 = (__terrane_select_winner_508 + 1usize)
                    % 2usize;
                match __terrane_select_winner_508 {
                    0 => {
                        __terrane_select_control_508_1.request_cancel();
                        let _ = __terrane_select_future_508_1.as_mut().await;
                    }
                    1 => {
                        __terrane_select_control_508_0.request_cancel();
                        let _ = __terrane_select_future_508_0.as_mut().await;
                    }
                    _ => unreachable!("selected winner is within the case count"),
                }
                __terrane_wait_projected_cleanups().await;
                __terrane_select_guard_508.finish();
                match __terrane_select_winner_508 {
                    0 => {
                        let _ = __terrane_select_result_508_0
                            .take()
                            .expect("selected case owns its ready result");
                        iteration = iteration.clone()
                            + terrane_int_support::Int::from(1_i128);
                        continue;
                    }
                    1 => {
                        let _ = __terrane_select_result_508_1
                            .take()
                            .expect("selected case owns its ready result");
                        break;
                    }
                    _ => unreachable!("selected winner is within the case count"),
                }
            }
        }
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&iteration),
            terrane_scalar_support::scalar_text(&__terrane_await(selected_return()).
            await)
        );
        let __terrane_completion_0: TerraneCompletion<()> = async {
            let __terrane_try_0: TerraneCompletion<()> = async {
                let finished: () = __terrane_traced_completion!(
                    __terrane_await(selected_throw()). await, 2 /* terrane-site: case.trn:35:22-35:39 */
                );
                if finished == () {
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("unexpected completion"))
                    );
                }
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
                    if !__terrane_handled_0
                        && __terrane_error_0.kind == TerraneErrorKind::CoercionError
                    {
                        __terrane_handled_0 = true;
                        println!(
                            "{}",
                            terrane_scalar_support::scalar_text(&String::from("caught"))
                        );
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
    });
}
