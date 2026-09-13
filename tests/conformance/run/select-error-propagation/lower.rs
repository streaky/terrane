// Generated deterministically by Terrane <version>.
// Runtime support: async_native.rs, executor_parallel.rs, channels.rs, platform_capability_types.rs, platform_result_type.rs, platform_int_conversion.rs, platform_capability_base.rs, platform_concurrency.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support, terrane-platform-support
// Source: case.trn
// Namespace: select-error-propagation
async fn fail() -> Result<terrane_int_support::Int, TerraneError> {
    return Err(
        TerraneError::raised(
            TerraneErrorKind::CoercionError,
            0 /* terrane-site: case.trn:6:3-6:23 */,
        ),
    );
}
async fn ready() -> String {
    return String::from("winner");
}
async fn cleanup_coercion(
    receiver: TerraneChannelReceiver<terrane_int_support::Int>,
) -> Result<(), TerraneError> {
    let mut __terrane_finally_guard_0 = __terrane_finally_guard();
    let __terrane_maybe_completion_0: Option<TerraneCompletion<()>> = __terrane_cancel_operation(
            &__terrane_finally_guard_0,
            async {
                let __terrane_try_0: TerraneCompletion<()> = async {
                    let received: TerraneChannelReceiveOutcome<
                        terrane_int_support::Int,
                    > = __terrane_await(Box::pin(receiver.receive())).await;
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&received.available)
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
        println!(
            "{}", terrane_scalar_support::scalar_text(&String::from("cleanup-coercion"))
        );
        return TerraneCompletion::Error(
            TerraneError::raised(
                TerraneErrorKind::CoercionError,
                1 /* terrane-site: case.trn:16:5-16:25 */,
            ),
        );
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
        TerraneCompletion::Normal => {
            __terrane_generated_defect("non-fallthrough try completed normally")
        }
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
async fn cleanup_missing(
    receiver: TerraneChannelReceiver<terrane_int_support::Int>,
) -> Result<(), TerraneError> {
    let mut __terrane_finally_guard_1 = __terrane_finally_guard();
    let __terrane_maybe_completion_1: Option<TerraneCompletion<()>> = __terrane_cancel_operation(
            &__terrane_finally_guard_1,
            async {
                let __terrane_try_1: TerraneCompletion<()> = async {
                    let received: TerraneChannelReceiveOutcome<
                        terrane_int_support::Int,
                    > = __terrane_await(Box::pin(receiver.receive())).await;
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&received.available)
                    );
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
                            return TerraneCompletion::Error(__terrane_error_1);
                        }
                    }
                }
                TerraneCompletion::Normal
            },
        )
        .await;
    let __terrane_cancelled_1 = __terrane_maybe_completion_1.is_none();
    let mut __terrane_completion_1 = __terrane_maybe_completion_1
        .unwrap_or(TerraneCompletion::Normal);
    let __terrane_finally_1: TerraneCompletion<()> = (|| {
        println!(
            "{}", terrane_scalar_support::scalar_text(&String::from("cleanup-missing"))
        );
        return TerraneCompletion::Error(
            TerraneError::raised(
                TerraneErrorKind::MissingKey,
                2 /* terrane-site: case.trn:24:5-24:22 */,
            ),
        );
    })();
    match __terrane_finally_1 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_1 = replacement,
    }
    if __terrane_cancelled_1
        && matches!(&__terrane_completion_1, TerraneCompletion::Normal)
    {
        __terrane_finish_cancelled_finally(__terrane_finally_guard_1).await;
    }
    __terrane_finally_guard_1.finish();
    match __terrane_completion_1 {
        TerraneCompletion::Normal => {
            __terrane_generated_defect("non-fallthrough try completed normally")
        }
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
async fn accept(value: terrane_int_support::Int) {
    println!("{}", terrane_scalar_support::scalar_text(&value));
}
fn construction_failure() -> Result<terrane_int_support::Int, TerraneError> {
    return Err(
        TerraneError::raised(
            TerraneErrorKind::MissingKey,
            3 /* terrane-site: case.trn:31:3-31:20 */,
        ),
    );
}
fn late_construction() -> terrane_int_support::Int {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&String::from("unexpected late construction"))
    );
    return terrane_int_support::Int::from(1_i128);
}
async fn blocked(receiver: TerraneChannelReceiver<terrane_int_support::Int>) {
    let mut __terrane_finally_guard_2 = __terrane_finally_guard();
    let __terrane_maybe_completion_2: Option<TerraneCompletion<()>> = __terrane_cancel_operation(
            &__terrane_finally_guard_2,
            async {
                let __terrane_try_2: TerraneCompletion<()> = async {
                    let received: TerraneChannelReceiveOutcome<
                        terrane_int_support::Int,
                    > = __terrane_await(Box::pin(receiver.receive())).await;
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&received.available)
                    );
                    TerraneCompletion::Normal
                }
                    .await;
                match __terrane_try_2 {
                    TerraneCompletion::Return(value) => {
                        return TerraneCompletion::Return(value);
                    }
                    TerraneCompletion::Break => return TerraneCompletion::Break,
                    TerraneCompletion::Continue => return TerraneCompletion::Continue,
                    TerraneCompletion::Normal => {}
                    TerraneCompletion::Error(__terrane_error_2) => {
                        let mut __terrane_handled_2 = false;
                        if !__terrane_handled_2 {
                            return TerraneCompletion::Error(__terrane_error_2);
                        }
                    }
                }
                TerraneCompletion::Normal
            },
        )
        .await;
    let __terrane_cancelled_2 = __terrane_maybe_completion_2.is_none();
    let mut __terrane_completion_2 = __terrane_maybe_completion_2
        .unwrap_or(TerraneCompletion::Normal);
    let __terrane_finally_2: TerraneCompletion<()> = (|| {
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&String::from("cleanup-before-error"))
        );
        TerraneCompletion::Normal
    })();
    match __terrane_finally_2 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_2 = replacement,
    }
    if __terrane_cancelled_2
        && matches!(&__terrane_completion_2, TerraneCompletion::Normal)
    {
        __terrane_finish_cancelled_finally(__terrane_finally_guard_2).await;
    }
    __terrane_finally_guard_2.finish();
    match __terrane_completion_2 {
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
        let mut __terrane_select_cursor_1164 = 0usize;
        let mut __terrane_select_cursor_1450 = 0usize;
        let mut __terrane_select_cursor_1828 = 0usize;
        let pair: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let __terrane_completion_3: TerraneCompletion<()> = async {
            let __terrane_try_3: TerraneCompletion<()> = async {
                {
                    let mut __terrane_select_guard_1164 = __terrane_finally_guard();
                    let mut __terrane_select_cleanup_error_1164: Option<TerraneError> = None;
                    let __terrane_select_control_1164_0 = __terrane_select_control();
                    let mut __terrane_select_future_1164_0 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_1164_0
                        .clone(), blocked(pair.receiver))
                    );
                    let mut __terrane_select_result_1164_0 = None;
                    let __terrane_select_control_1164_1 = __terrane_select_control();
                    let mut __terrane_select_future_1164_1 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_1164_1
                        .clone(), fail())
                    );
                    let mut __terrane_select_result_1164_1 = None;
                    let __terrane_select_winner_1164 = std::future::poll_fn(|
                            __terrane_select_context|
                        {
                            if __terrane_cancellation_is_requested() {
                                return std::task::Poll::Ready(usize::MAX);
                            }
                            for __terrane_select_offset in 0..2usize {
                                let __terrane_select_candidate = (__terrane_select_cursor_1164
                                    + __terrane_select_offset) % 2usize;
                                match __terrane_select_candidate {
                                    0 => {
                                        match Future::poll(
                                            __terrane_select_future_1164_0.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_1164_0 = Some(
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
                                            __terrane_select_future_1164_1.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_1164_1 = Some(
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
                    if __terrane_select_winner_1164 == usize::MAX {
                        __terrane_select_control_1164_1.request_cancel();
                        __terrane_select_control_1164_0.request_cancel();
                        if let Some(Err(__terrane_select_error)) = __terrane_select_future_1164_1
                            .as_mut()
                            .await
                        {
                            __terrane_select_cleanup_error_1164 = Some(
                                __terrane_trace_error(
                                    __terrane_select_error,
                                    5 /* terrane-site: case.trn:50:30-50:37 */,
                                ),
                            );
                        }
                        let _ = __terrane_select_future_1164_0.as_mut().await;
                        __terrane_wait_projected_cleanups().await;
                        __terrane_select_guard_1164.finish();
                        if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1164
                            .take()
                        {
                            return TerraneCompletion::Error(
                                __terrane_select_cleanup_error,
                            );
                        }
                        __terrane_finish_cancelled_select(__terrane_select_guard_1164)
                            .await;
                    }
                    __terrane_select_cursor_1164 = (__terrane_select_winner_1164
                        + 1usize) % 2usize;
                    match __terrane_select_winner_1164 {
                        0 => {
                            __terrane_select_control_1164_1.request_cancel();
                            if let Some(Err(__terrane_select_error)) = __terrane_select_future_1164_1
                                .as_mut()
                                .await
                            {
                                __terrane_select_cleanup_error_1164 = Some(
                                    __terrane_trace_error(
                                        __terrane_select_error,
                                        5 /* terrane-site: case.trn:50:30-50:37 */,
                                    ),
                                );
                            }
                        }
                        1 => {
                            __terrane_select_control_1164_0.request_cancel();
                            let _ = __terrane_select_future_1164_0.as_mut().await;
                        }
                        _ => unreachable!("selected winner is within the case count"),
                    }
                    __terrane_wait_projected_cleanups().await;
                    __terrane_select_guard_1164.finish();
                    if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1164
                        .take()
                    {
                        return TerraneCompletion::Error(__terrane_select_cleanup_error);
                    }
                    match __terrane_select_winner_1164 {
                        0 => {
                            let _ = __terrane_select_result_1164_0
                                .take()
                                .expect("selected case owns its ready result");
                            println!(
                                "{}",
                                terrane_scalar_support::scalar_text(&String::from("unexpected"))
                            );
                        }
                        1 => {
                            let value: terrane_int_support::Int = __terrane_traced_completion!(
                                __terrane_select_result_1164_1.take()
                                .expect("selected case owns its ready result"),
                                5 /* terrane-site: case.trn:50:30-50:37 */
                            );
                            println!("{}", terrane_scalar_support::scalar_text(&value));
                        }
                        _ => unreachable!("selected winner is within the case count"),
                    }
                }
                TerraneCompletion::Normal
            }
                .await;
            match __terrane_try_3 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_3) => {
                    let mut __terrane_handled_3 = false;
                    if !__terrane_handled_3
                        && __terrane_error_3.kind == TerraneErrorKind::CoercionError
                    {
                        __terrane_handled_3 = true;
                        println!(
                            "{}",
                            terrane_scalar_support::scalar_text(&String::from("caught"))
                        );
                    }
                    if !__terrane_handled_3 {
                        return TerraneCompletion::Error(__terrane_error_3);
                    }
                }
            }
            TerraneCompletion::Normal
        }
            .await;
        match __terrane_completion_3 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        let coercion_pair: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let missing_pair: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let __terrane_completion_4: TerraneCompletion<()> = async {
            let __terrane_try_4: TerraneCompletion<()> = async {
                {
                    let mut __terrane_select_guard_1450 = __terrane_finally_guard();
                    let mut __terrane_select_cleanup_error_1450: Option<TerraneError> = None;
                    let __terrane_select_control_1450_0 = __terrane_select_control();
                    let mut __terrane_select_future_1450_0 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_1450_0
                        .clone(), cleanup_coercion(coercion_pair.receiver))
                    );
                    let mut __terrane_select_result_1450_0 = None;
                    let __terrane_select_control_1450_1 = __terrane_select_control();
                    let mut __terrane_select_future_1450_1 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_1450_1
                        .clone(), cleanup_missing(missing_pair.receiver))
                    );
                    let mut __terrane_select_result_1450_1 = None;
                    let __terrane_select_control_1450_2 = __terrane_select_control();
                    let mut __terrane_select_future_1450_2 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_1450_2
                        .clone(), ready())
                    );
                    let mut __terrane_select_result_1450_2 = None;
                    let __terrane_select_winner_1450 = std::future::poll_fn(|
                            __terrane_select_context|
                        {
                            if __terrane_cancellation_is_requested() {
                                return std::task::Poll::Ready(usize::MAX);
                            }
                            for __terrane_select_offset in 0..3usize {
                                let __terrane_select_candidate = (__terrane_select_cursor_1450
                                    + __terrane_select_offset) % 3usize;
                                match __terrane_select_candidate {
                                    0 => {
                                        match Future::poll(
                                            __terrane_select_future_1450_0.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_1450_0 = Some(
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
                                            __terrane_select_future_1450_1.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_1450_1 = Some(
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
                                            __terrane_select_future_1450_2.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_1450_2 = Some(
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
                    if __terrane_select_winner_1450 == usize::MAX {
                        __terrane_select_control_1450_2.request_cancel();
                        __terrane_select_control_1450_1.request_cancel();
                        __terrane_select_control_1450_0.request_cancel();
                        let _ = __terrane_select_future_1450_2.as_mut().await;
                        if let Some(Err(__terrane_select_error)) = __terrane_select_future_1450_1
                            .as_mut()
                            .await
                        {
                            __terrane_select_cleanup_error_1450 = Some(
                                __terrane_trace_error(
                                    __terrane_select_error,
                                    8 /* terrane-site: case.trn:61:18-61:58 */,
                                ),
                            );
                        }
                        if let Some(Err(__terrane_select_error)) = __terrane_select_future_1450_0
                            .as_mut()
                            .await
                        {
                            __terrane_select_cleanup_error_1450 = Some(
                                __terrane_trace_error(
                                    __terrane_select_error,
                                    9 /* terrane-site: case.trn:59:18-59:60 */,
                                ),
                            );
                        }
                        __terrane_wait_projected_cleanups().await;
                        __terrane_select_guard_1450.finish();
                        if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1450
                            .take()
                        {
                            return TerraneCompletion::Error(
                                __terrane_select_cleanup_error,
                            );
                        }
                        __terrane_finish_cancelled_select(__terrane_select_guard_1450)
                            .await;
                    }
                    __terrane_select_cursor_1450 = (__terrane_select_winner_1450
                        + 1usize) % 3usize;
                    match __terrane_select_winner_1450 {
                        0 => {
                            __terrane_select_control_1450_2.request_cancel();
                            __terrane_select_control_1450_1.request_cancel();
                            let _ = __terrane_select_future_1450_2.as_mut().await;
                            if let Some(Err(__terrane_select_error)) = __terrane_select_future_1450_1
                                .as_mut()
                                .await
                            {
                                __terrane_select_cleanup_error_1450 = Some(
                                    __terrane_trace_error(
                                        __terrane_select_error,
                                        8 /* terrane-site: case.trn:61:18-61:58 */,
                                    ),
                                );
                            }
                        }
                        1 => {
                            __terrane_select_control_1450_2.request_cancel();
                            __terrane_select_control_1450_0.request_cancel();
                            let _ = __terrane_select_future_1450_2.as_mut().await;
                            if let Some(Err(__terrane_select_error)) = __terrane_select_future_1450_0
                                .as_mut()
                                .await
                            {
                                __terrane_select_cleanup_error_1450 = Some(
                                    __terrane_trace_error(
                                        __terrane_select_error,
                                        9 /* terrane-site: case.trn:59:18-59:60 */,
                                    ),
                                );
                            }
                        }
                        2 => {
                            __terrane_select_control_1450_1.request_cancel();
                            __terrane_select_control_1450_0.request_cancel();
                            if let Some(Err(__terrane_select_error)) = __terrane_select_future_1450_1
                                .as_mut()
                                .await
                            {
                                __terrane_select_cleanup_error_1450 = Some(
                                    __terrane_trace_error(
                                        __terrane_select_error,
                                        8 /* terrane-site: case.trn:61:18-61:58 */,
                                    ),
                                );
                            }
                            if let Some(Err(__terrane_select_error)) = __terrane_select_future_1450_0
                                .as_mut()
                                .await
                            {
                                __terrane_select_cleanup_error_1450 = Some(
                                    __terrane_trace_error(
                                        __terrane_select_error,
                                        9 /* terrane-site: case.trn:59:18-59:60 */,
                                    ),
                                );
                            }
                        }
                        _ => unreachable!("selected winner is within the case count"),
                    }
                    __terrane_wait_projected_cleanups().await;
                    __terrane_select_guard_1450.finish();
                    if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1450
                        .take()
                    {
                        return TerraneCompletion::Error(__terrane_select_cleanup_error);
                    }
                    match __terrane_select_winner_1450 {
                        0 => {
                            let _ = __terrane_traced_completion!(
                                __terrane_select_result_1450_0.take()
                                .expect("selected case owns its ready result"),
                                9 /* terrane-site: case.trn:59:18-59:60 */
                            );
                            println!(
                                "{}",
                                terrane_scalar_support::scalar_text(&String::from("unexpected coercion"))
                            );
                        }
                        1 => {
                            let _ = __terrane_traced_completion!(
                                __terrane_select_result_1450_1.take()
                                .expect("selected case owns its ready result"),
                                8 /* terrane-site: case.trn:61:18-61:58 */
                            );
                            println!(
                                "{}",
                                terrane_scalar_support::scalar_text(&String::from("unexpected missing"))
                            );
                        }
                        2 => {
                            let winner: String = __terrane_select_result_1450_2
                                .take()
                                .expect("selected case owns its ready result");
                            println!("{}", terrane_scalar_support::scalar_text(&winner));
                        }
                        _ => unreachable!("selected winner is within the case count"),
                    }
                }
                TerraneCompletion::Normal
            }
                .await;
            match __terrane_try_4 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_4) => {
                    let mut __terrane_handled_4 = false;
                    if !__terrane_handled_4
                        && __terrane_error_4.kind == TerraneErrorKind::CoercionError
                    {
                        __terrane_handled_4 = true;
                        println!(
                            "{}",
                            terrane_scalar_support::scalar_text(&String::from("cleanup-error-replaced"))
                        );
                    }
                    if !__terrane_handled_4 {
                        return TerraneCompletion::Error(__terrane_error_4);
                    }
                }
            }
            TerraneCompletion::Normal
        }
            .await;
        match __terrane_completion_4 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        let construction_pair: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let __terrane_completion_5: TerraneCompletion<()> = async {
            let __terrane_try_5: TerraneCompletion<()> = async {
                {
                    let mut __terrane_select_guard_1828 = __terrane_finally_guard();
                    let mut __terrane_select_cleanup_error_1828: Option<TerraneError> = None;
                    let __terrane_select_control_1828_0 = __terrane_select_control();
                    let mut __terrane_select_future_1828_0 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_1828_0
                        .clone(), cleanup_coercion(construction_pair.receiver))
                    );
                    let mut __terrane_select_result_1828_0 = None;
                    let __terrane_select_control_1828_1 = __terrane_select_control();
                    let __terrane_select_unpinned_1828_1 = match || -> Result<
                        _,
                        TerraneError,
                    > {
                        Ok(
                            accept(
                                __terrane_traced_err(
                                    construction_failure(),
                                    11 /* terrane-site: case.trn:73:28-73:49 */,
                                )?,
                            ),
                        )
                    }() {
                        __terrane_select_constructed => {
                            match __terrane_select_constructed {
                                Ok(__terrane_select_future) => __terrane_select_future,
                                Err(__terrane_select_error) => {
                                    __terrane_select_cleanup_error_1828 = Some(
                                        __terrane_select_error,
                                    );
                                    __terrane_select_control_1828_0.request_cancel();
                                    if let Some(Err(__terrane_select_error)) = __terrane_select_future_1828_0
                                        .as_mut()
                                        .await
                                    {
                                        __terrane_select_cleanup_error_1828 = Some(
                                            __terrane_trace_error(
                                                __terrane_select_error,
                                                12 /* terrane-site: case.trn:71:18-71:64 */,
                                            ),
                                        );
                                    }
                                    __terrane_wait_projected_cleanups().await;
                                    __terrane_select_guard_1828.finish();
                                    if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1828
                                        .take()
                                    {
                                        return TerraneCompletion::Error(
                                            __terrane_select_cleanup_error,
                                        );
                                    }
                                    unreachable!("select construction failure propagates");
                                }
                            }
                        }
                    };
                    let mut __terrane_select_future_1828_1 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_1828_1
                        .clone(), __terrane_select_unpinned_1828_1)
                    );
                    let mut __terrane_select_result_1828_1 = None;
                    let __terrane_select_control_1828_2 = __terrane_select_control();
                    let mut __terrane_select_future_1828_2 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_1828_2
                        .clone(), accept(late_construction()))
                    );
                    let mut __terrane_select_result_1828_2 = None;
                    let __terrane_select_winner_1828 = std::future::poll_fn(|
                            __terrane_select_context|
                        {
                            if __terrane_cancellation_is_requested() {
                                return std::task::Poll::Ready(usize::MAX);
                            }
                            for __terrane_select_offset in 0..3usize {
                                let __terrane_select_candidate = (__terrane_select_cursor_1828
                                    + __terrane_select_offset) % 3usize;
                                match __terrane_select_candidate {
                                    0 => {
                                        match Future::poll(
                                            __terrane_select_future_1828_0.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_1828_0 = Some(
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
                                            __terrane_select_future_1828_1.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_1828_1 = Some(
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
                                            __terrane_select_future_1828_2.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_1828_2 = Some(
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
                    if __terrane_select_winner_1828 == usize::MAX {
                        __terrane_select_control_1828_2.request_cancel();
                        __terrane_select_control_1828_1.request_cancel();
                        __terrane_select_control_1828_0.request_cancel();
                        let _ = __terrane_select_future_1828_2.as_mut().await;
                        let _ = __terrane_select_future_1828_1.as_mut().await;
                        if let Some(Err(__terrane_select_error)) = __terrane_select_future_1828_0
                            .as_mut()
                            .await
                        {
                            __terrane_select_cleanup_error_1828 = Some(
                                __terrane_trace_error(
                                    __terrane_select_error,
                                    12 /* terrane-site: case.trn:71:18-71:64 */,
                                ),
                            );
                        }
                        __terrane_wait_projected_cleanups().await;
                        __terrane_select_guard_1828.finish();
                        if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1828
                            .take()
                        {
                            return TerraneCompletion::Error(
                                __terrane_select_cleanup_error,
                            );
                        }
                        __terrane_finish_cancelled_select(__terrane_select_guard_1828)
                            .await;
                    }
                    __terrane_select_cursor_1828 = (__terrane_select_winner_1828
                        + 1usize) % 3usize;
                    match __terrane_select_winner_1828 {
                        0 => {
                            __terrane_select_control_1828_2.request_cancel();
                            __terrane_select_control_1828_1.request_cancel();
                            let _ = __terrane_select_future_1828_2.as_mut().await;
                            let _ = __terrane_select_future_1828_1.as_mut().await;
                        }
                        1 => {
                            __terrane_select_control_1828_2.request_cancel();
                            __terrane_select_control_1828_0.request_cancel();
                            let _ = __terrane_select_future_1828_2.as_mut().await;
                            if let Some(Err(__terrane_select_error)) = __terrane_select_future_1828_0
                                .as_mut()
                                .await
                            {
                                __terrane_select_cleanup_error_1828 = Some(
                                    __terrane_trace_error(
                                        __terrane_select_error,
                                        12 /* terrane-site: case.trn:71:18-71:64 */,
                                    ),
                                );
                            }
                        }
                        2 => {
                            __terrane_select_control_1828_1.request_cancel();
                            __terrane_select_control_1828_0.request_cancel();
                            let _ = __terrane_select_future_1828_1.as_mut().await;
                            if let Some(Err(__terrane_select_error)) = __terrane_select_future_1828_0
                                .as_mut()
                                .await
                            {
                                __terrane_select_cleanup_error_1828 = Some(
                                    __terrane_trace_error(
                                        __terrane_select_error,
                                        12 /* terrane-site: case.trn:71:18-71:64 */,
                                    ),
                                );
                            }
                        }
                        _ => unreachable!("selected winner is within the case count"),
                    }
                    __terrane_wait_projected_cleanups().await;
                    __terrane_select_guard_1828.finish();
                    if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1828
                        .take()
                    {
                        return TerraneCompletion::Error(__terrane_select_cleanup_error);
                    }
                    match __terrane_select_winner_1828 {
                        0 => {
                            let _ = __terrane_traced_completion!(
                                __terrane_select_result_1828_0.take()
                                .expect("selected case owns its ready result"),
                                12 /* terrane-site: case.trn:71:18-71:64 */
                            );
                            println!(
                                "{}",
                                terrane_scalar_support::scalar_text(&String::from("unexpected cleanup winner"))
                            );
                        }
                        1 => {
                            let _ = __terrane_select_result_1828_1
                                .take()
                                .expect("selected case owns its ready result");
                            println!(
                                "{}",
                                terrane_scalar_support::scalar_text(&String::from("unexpected construction winner"))
                            );
                        }
                        2 => {
                            let _ = __terrane_select_result_1828_2
                                .take()
                                .expect("selected case owns its ready result");
                            println!(
                                "{}",
                                terrane_scalar_support::scalar_text(&String::from("unexpected late winner"))
                            );
                        }
                        _ => unreachable!("selected winner is within the case count"),
                    }
                }
                TerraneCompletion::Normal
            }
                .await;
            match __terrane_try_5 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_5) => {
                    let mut __terrane_handled_5 = false;
                    if !__terrane_handled_5
                        && __terrane_error_5.kind == TerraneErrorKind::CoercionError
                    {
                        __terrane_handled_5 = true;
                        println!(
                            "{}",
                            terrane_scalar_support::scalar_text(&String::from("construction-cleanup-replaced"))
                        );
                    }
                    if !__terrane_handled_5 {
                        return TerraneCompletion::Error(__terrane_error_5);
                    }
                }
            }
            TerraneCompletion::Normal
        }
            .await;
        match __terrane_completion_5 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
    });
}
// Source: core/concurrency.trn
// Namespace: core/concurrency
#[derive(Clone)]
pub struct ConcurrencyOperationResult {
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub message: String,
}
impl ConcurrencyOperationResult {
    pub fn terrane_construct(
        did_fail: bool,
        exceeded_deadline: bool,
        detail: String,
    ) -> Self {
        let mut value = Self {
            failed: false,
            deadline_exceeded: false,
            message: String::from(""),
        };
        value.construct(did_fail, exceeded_deadline, detail);
        value
    }
    pub fn construct(
        &mut self,
        did_fail: bool,
        exceeded_deadline: bool,
        detail: String,
    ) {
        self.failed = did_fail;
        self.deadline_exceeded = exceeded_deadline;
        self.message = detail;
    }
}
#[derive(Clone)]
pub struct ConcurrencyIntResult {
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub available: bool,
    pub message: String,
    pub value: terrane_int_support::Int,
}
impl ConcurrencyIntResult {
    pub fn terrane_construct(
        did_fail: bool,
        exceeded_deadline: bool,
        has_value: bool,
        detail: String,
        result_value: terrane_int_support::Int,
    ) -> Self {
        let mut value = Self {
            failed: false,
            deadline_exceeded: false,
            available: false,
            message: String::from(""),
            value: terrane_int_support::Int::from(0_i128),
        };
        value.construct(did_fail, exceeded_deadline, has_value, detail, result_value);
        value
    }
    pub fn construct(
        &mut self,
        did_fail: bool,
        exceeded_deadline: bool,
        has_value: bool,
        detail: String,
        result_value: terrane_int_support::Int,
    ) {
        self.failed = did_fail;
        self.deadline_exceeded = exceeded_deadline;
        self.available = has_value;
        self.message = detail;
        self.value = result_value.clone();
    }
}
#[derive(Clone)]
pub struct IntMutex {
    pub failed: bool,
    pub message: String,
    pub handle: TerranePlatformCapability,
}
impl IntMutex {
    pub fn terrane_construct(initial: terrane_int_support::Int) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            handle: terrane_platform_no_resource(),
        };
        value.construct(initial);
        value
    }
    pub fn construct(&mut self, initial: terrane_int_support::Int) {
        let raw: TerranePlatformResult = terrane_platform_int_mutex(initial);
        self.failed = terrane_platform_result_failed(&raw);
        self.message = terrane_platform_result_message(&raw);
        self.handle = terrane_platform_result_capability(&raw);
    }
    pub fn load(&self) -> ConcurrencyIntResult {
        let raw: TerranePlatformResult = terrane_platform_int_mutex_load(&self.handle);
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
    pub fn store(&self, value: terrane_int_support::Int) -> ConcurrencyOperationResult {
        let raw: TerranePlatformResult = terrane_platform_int_mutex_store(
            &self.handle,
            value,
        );
        return ConcurrencyOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn increase(
        &mut self,
        amount: terrane_int_support::Int,
    ) -> ConcurrencyIntResult {
        let raw: TerranePlatformResult = terrane_platform_int_mutex_add(
            &self.handle,
            amount,
        );
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
}
#[derive(Clone)]
pub struct IntReadWriteLock {
    pub failed: bool,
    pub message: String,
    pub handle: TerranePlatformCapability,
}
impl IntReadWriteLock {
    pub fn terrane_construct(initial: terrane_int_support::Int) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            handle: terrane_platform_no_resource(),
        };
        value.construct(initial);
        value
    }
    pub fn construct(&mut self, initial: terrane_int_support::Int) {
        let raw: TerranePlatformResult = terrane_platform_int_rw_lock(initial);
        self.failed = terrane_platform_result_failed(&raw);
        self.message = terrane_platform_result_message(&raw);
        self.handle = terrane_platform_result_capability(&raw);
    }
    pub fn read(&self) -> ConcurrencyIntResult {
        let raw: TerranePlatformResult = terrane_platform_int_rw_lock_read(&self.handle);
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
    pub fn write(&self, value: terrane_int_support::Int) -> ConcurrencyOperationResult {
        let raw: TerranePlatformResult = terrane_platform_int_rw_lock_write(
            &self.handle,
            value,
        );
        return ConcurrencyOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
}
#[derive(Clone)]
pub struct MemoryOrder {
    pub name: String,
}
impl MemoryOrder {
    pub fn terrane_construct(ordering_name: String) -> Self {
        let mut value = Self {
            name: String::from("sequentially-consistent"),
        };
        value.construct(ordering_name);
        value
    }
    pub fn construct(&mut self, ordering_name: String) {
        self.name = ordering_name;
    }
}
pub fn relaxed_order() -> MemoryOrder {
    return MemoryOrder::terrane_construct(String::from("relaxed"));
}
pub fn acquire_order() -> MemoryOrder {
    return MemoryOrder::terrane_construct(String::from("acquire"));
}
pub fn release_order() -> MemoryOrder {
    return MemoryOrder::terrane_construct(String::from("release"));
}
pub fn acquire_release_order() -> MemoryOrder {
    return MemoryOrder::terrane_construct(String::from("acquire-release"));
}
pub fn sequentially_consistent_order() -> MemoryOrder {
    return MemoryOrder::terrane_construct(String::from("sequentially-consistent"));
}
#[derive(Clone)]
pub struct AtomicInt64 {
    pub failed: bool,
    pub message: String,
    pub handle: TerranePlatformCapability,
}
impl AtomicInt64 {
    pub fn terrane_construct(initial: i64) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            handle: terrane_platform_no_resource(),
        };
        value.construct(initial);
        value
    }
    pub fn construct(&mut self, initial: i64) {
        let raw: TerranePlatformResult = terrane_platform_atomic_int64(initial);
        self.failed = terrane_platform_result_failed(&raw);
        self.message = terrane_platform_result_message(&raw);
        self.handle = terrane_platform_result_capability(&raw);
    }
    pub fn load(&self, ordering: MemoryOrder) -> ConcurrencyIntResult {
        let raw: TerranePlatformResult = terrane_platform_atomic_int64_load(
            &self.handle,
            ordering.name,
        );
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
    pub fn store(
        &self,
        value: i64,
        ordering: MemoryOrder,
    ) -> ConcurrencyOperationResult {
        let raw: TerranePlatformResult = terrane_platform_atomic_int64_store(
            &self.handle,
            value,
            ordering.name,
        );
        return ConcurrencyOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn increase(
        &mut self,
        amount: i64,
        ordering: MemoryOrder,
    ) -> ConcurrencyIntResult {
        let raw: TerranePlatformResult = terrane_platform_atomic_int64_add(
            &self.handle,
            amount,
            ordering.name,
        );
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
}
#[derive(Clone)]
pub struct ThreadLocalInt {
    pub failed: bool,
    pub message: String,
    pub handle: TerranePlatformCapability,
}
impl ThreadLocalInt {
    pub fn terrane_construct(initial: terrane_int_support::Int) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            handle: terrane_platform_no_resource(),
        };
        value.construct(initial);
        value
    }
    pub fn construct(&mut self, initial: terrane_int_support::Int) {
        let raw: TerranePlatformResult = terrane_platform_thread_local_int(initial);
        self.failed = terrane_platform_result_failed(&raw);
        self.message = terrane_platform_result_message(&raw);
        self.handle = terrane_platform_result_capability(&raw);
    }
    pub fn get(&self) -> ConcurrencyIntResult {
        let raw: TerranePlatformResult = terrane_platform_thread_local_int_get(
            &self.handle,
        );
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
    pub fn write(&self, value: terrane_int_support::Int) -> ConcurrencyOperationResult {
        let raw: TerranePlatformResult = terrane_platform_thread_local_int_set(
            &self.handle,
            value,
        );
        return ConcurrencyOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
}
