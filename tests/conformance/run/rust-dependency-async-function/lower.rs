// Generated deterministically by Terrane <version>.
// Runtime support: async_native.rs, executor_local.rs, async_dependency.rs, channels.rs, tasks_native_local.rs, platform_capability_types.rs, platform_result_type.rs, platform_int_conversion.rs, platform_capability_base.rs, platform_concurrency.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support, terrane-platform-support
// Source: src/main.trn
// Namespace: app
async fn source_ready() -> String {
    return String::from("source-ready");
}
fn main() {
    __terrane_run(async move {
        let mut __terrane_select_cursor_1085 = 0usize;
        let mut __terrane_select_cursor_1258 = 0usize;
        let mut __terrane_select_cursor_1396 = 0usize;
        let mut __terrane_select_cursor_1583 = 0usize;
        let echoed: String = __terrane_traced(
            __terrane_await({
                    let __terrane_future = echo_after_yield(
                        String::from("projected async success"),
                    );
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            0 /* terrane-site: src/main.trn:11:25-11:67 */,
                        )
                    }
                })
                .await,
            0 /* terrane-site: src/main.trn:11:25-11:67 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&echoed));
        let polls: terrane_int_support::Int = __terrane_traced(
            __terrane_await({
                    let __terrane_future = timer_poll_count();
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            1 /* terrane-site: src/main.trn:13:21-13:38 */,
                        )
                    }
                })
                .await,
            1 /* terrane-site: src/main.trn:13:21-13:38 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&polls));
        let message: String = __terrane_traced(
            __terrane_await({
                    let __terrane_future = socket_round_trip();
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            2 /* terrane-site: src/main.trn:15:26-15:44 */,
                        )
                    }
                })
                .await,
            2 /* terrane-site: src/main.trn:15:26-15:44 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&message));
        let scope: TerraneTaskScope = TerraneTaskScope::new(None);
        let waiting: TerraneScopedTask<String> = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        std::sync::Arc::new(move || -> std::pin::Pin<
                            Box<dyn Future<Output = _>>,
                        > { Box::pin(wait_for_sibling()) })(),
                        __terrane_cancel,
                        __terrane_deadline,
                    )
                    .await
                {
                    Some(Ok(value)) => TerraneTaskResult::Completed(value),
                    Some(Err(error)) => {
                        TerraneTaskResult::Failed(
                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                        )
                    }
                    None => TerraneTaskResult::Cancelled,
                }
            })
        };
        let signalling: TerraneScopedTask<String> = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        std::sync::Arc::new(move || -> std::pin::Pin<
                            Box<dyn Future<Output = _>>,
                        > { Box::pin(signal_sibling()) })(),
                        __terrane_cancel,
                        __terrane_deadline,
                    )
                    .await
                {
                    Some(Ok(value)) => TerraneTaskResult::Completed(value),
                    Some(Err(error)) => {
                        TerraneTaskResult::Failed(
                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                        )
                    }
                    None => TerraneTaskResult::Cancelled,
                }
            })
        };
        let waited: TerraneTaskOutcome<String> = __terrane_await(scope.join(waiting))
            .await;
        let signalled: TerraneTaskOutcome<String> = __terrane_await(
                scope.join(signalling),
            )
            .await;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&waited.completed),
            terrane_scalar_support::scalar_text(&signalled.completed)
        );
        let __terrane_completion_0: TerraneCompletion<()> = async {
            let __terrane_try_0: TerraneCompletion<()> = async {
                let rejected: String = __terrane_traced_completion!(
                    __terrane_await({ let __terrane_future =
                    checked_echo(String::from("reject")); async move {
                    __terrane_raised_err(__terrane_future. await, 3 /* terrane-site: src/main.trn:24:29-24:50 */) } }). await, 3 /* terrane-site: src/main.trn:24:29-24:50 */
                );
                println!("{}", terrane_scalar_support::scalar_text(&rejected));
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
                        && __terrane_error_0.kind
                            == TerraneErrorKind::Custom(DescriptorId(0))
                    {
                        __terrane_handled_0 = true;
                        println!(
                            "{}",
                            terrane_scalar_support::scalar_text(&String::from("caught projected async failure"))
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
        let selection_pair: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let selection_rx: TerraneChannelReceiver<terrane_int_support::Int> = selection_pair
            .receiver;
        {
            let mut __terrane_select_guard_1085 = __terrane_finally_guard();
            let mut __terrane_select_cleanup_error_1085: Option<TerraneError> = None;
            let __terrane_select_control_1085_0 = __terrane_select_control();
            let mut __terrane_select_future_1085_0 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_1085_0.clone(), { let
                __terrane_future = echo_after_yield(String::from("selected-projected"));
                async move { __terrane_raised_err(__terrane_future. await,
                4 /* terrane-site: src/main.trn:31:34-31:71 */) } })
            );
            let mut __terrane_select_result_1085_0 = None;
            let __terrane_select_control_1085_1 = __terrane_select_control();
            let mut __terrane_select_future_1085_1 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_1085_1.clone(),
                Box::pin(selection_rx.receive()))
            );
            let mut __terrane_select_result_1085_1 = None;
            let __terrane_select_winner_1085 = std::future::poll_fn(|
                    __terrane_select_context|
                {
                    if __terrane_cancellation_is_requested() {
                        return std::task::Poll::Ready(usize::MAX);
                    }
                    for __terrane_select_offset in 0..2usize {
                        let __terrane_select_candidate = (__terrane_select_cursor_1085
                            + __terrane_select_offset) % 2usize;
                        match __terrane_select_candidate {
                            0 => {
                                match Future::poll(
                                    __terrane_select_future_1085_0.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_1085_0 = Some(
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
                                    __terrane_select_future_1085_1.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_1085_1 = Some(
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
            if __terrane_select_winner_1085 == usize::MAX {
                __terrane_select_control_1085_1.request_cancel();
                __terrane_select_control_1085_0.request_cancel();
                let _ = __terrane_select_future_1085_1.as_mut().await;
                if let Some(Err(__terrane_select_error)) = __terrane_select_future_1085_0
                    .as_mut()
                    .await
                {
                    __terrane_select_cleanup_error_1085 = Some(
                        __terrane_trace_error(
                            __terrane_select_error,
                            4 /* terrane-site: src/main.trn:31:34-31:71 */,
                        ),
                    );
                }
                __terrane_wait_projected_cleanups().await;
                __terrane_select_guard_1085.finish();
                if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1085
                    .take()
                {
                    __terrane_uncaught(__terrane_select_cleanup_error);
                }
                __terrane_finish_cancelled_select(__terrane_select_guard_1085).await;
            }
            __terrane_select_cursor_1085 = (__terrane_select_winner_1085 + 1usize)
                % 2usize;
            match __terrane_select_winner_1085 {
                0 => {
                    __terrane_select_control_1085_1.request_cancel();
                    let _ = __terrane_select_future_1085_1.as_mut().await;
                }
                1 => {
                    __terrane_select_control_1085_0.request_cancel();
                    if let Some(Err(__terrane_select_error)) = __terrane_select_future_1085_0
                        .as_mut()
                        .await
                    {
                        __terrane_select_cleanup_error_1085 = Some(
                            __terrane_trace_error(
                                __terrane_select_error,
                                4 /* terrane-site: src/main.trn:31:34-31:71 */,
                            ),
                        );
                    }
                }
                _ => unreachable!("selected winner is within the case count"),
            }
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_1085.finish();
            if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1085
                .take()
            {
                __terrane_uncaught(__terrane_select_cleanup_error);
            }
            match __terrane_select_winner_1085 {
                0 => {
                    let selected: String = __terrane_traced(
                        __terrane_select_result_1085_0
                            .take()
                            .expect("selected case owns its ready result"),
                        4 /* terrane-site: src/main.trn:31:34-31:71 */,
                    );
                    println!("{}", terrane_scalar_support::scalar_text(&selected));
                }
                1 => {
                    let _ = __terrane_select_result_1085_1
                        .take()
                        .expect("selected case owns its ready result");
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("unexpected receive"))
                    );
                }
                _ => unreachable!("selected winner is within the case count"),
            }
        }
        {
            let mut __terrane_select_guard_1258 = __terrane_finally_guard();
            let mut __terrane_select_cleanup_error_1258: Option<TerraneError> = None;
            let __terrane_select_control_1258_0 = __terrane_select_control();
            let mut __terrane_select_future_1258_0 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_1258_0.clone(), { let
                __terrane_future = socket_round_trip(); async move {
                __terrane_raised_err(__terrane_future. await, 5 /* terrane-site: src/main.trn:36:28-36:46 */) } })
            );
            let mut __terrane_select_result_1258_0 = None;
            let __terrane_select_control_1258_1 = __terrane_select_control();
            let mut __terrane_select_future_1258_1 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_1258_1.clone(),
                source_ready())
            );
            let mut __terrane_select_result_1258_1 = None;
            let __terrane_select_winner_1258 = std::future::poll_fn(|
                    __terrane_select_context|
                {
                    if __terrane_cancellation_is_requested() {
                        return std::task::Poll::Ready(usize::MAX);
                    }
                    for __terrane_select_offset in 0..2usize {
                        let __terrane_select_candidate = (__terrane_select_cursor_1258
                            + __terrane_select_offset) % 2usize;
                        match __terrane_select_candidate {
                            0 => {
                                match Future::poll(
                                    __terrane_select_future_1258_0.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_1258_0 = Some(
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
                                    __terrane_select_future_1258_1.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_1258_1 = Some(
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
            if __terrane_select_winner_1258 == usize::MAX {
                __terrane_select_control_1258_1.request_cancel();
                __terrane_select_control_1258_0.request_cancel();
                let _ = __terrane_select_future_1258_1.as_mut().await;
                if let Some(Err(__terrane_select_error)) = __terrane_select_future_1258_0
                    .as_mut()
                    .await
                {
                    __terrane_select_cleanup_error_1258 = Some(
                        __terrane_trace_error(
                            __terrane_select_error,
                            5 /* terrane-site: src/main.trn:36:28-36:46 */,
                        ),
                    );
                }
                __terrane_wait_projected_cleanups().await;
                __terrane_select_guard_1258.finish();
                if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1258
                    .take()
                {
                    __terrane_uncaught(__terrane_select_cleanup_error);
                }
                __terrane_finish_cancelled_select(__terrane_select_guard_1258).await;
            }
            __terrane_select_cursor_1258 = (__terrane_select_winner_1258 + 1usize)
                % 2usize;
            match __terrane_select_winner_1258 {
                0 => {
                    __terrane_select_control_1258_1.request_cancel();
                    let _ = __terrane_select_future_1258_1.as_mut().await;
                }
                1 => {
                    __terrane_select_control_1258_0.request_cancel();
                    if let Some(Err(__terrane_select_error)) = __terrane_select_future_1258_0
                        .as_mut()
                        .await
                    {
                        __terrane_select_cleanup_error_1258 = Some(
                            __terrane_trace_error(
                                __terrane_select_error,
                                5 /* terrane-site: src/main.trn:36:28-36:46 */,
                            ),
                        );
                    }
                }
                _ => unreachable!("selected winner is within the case count"),
            }
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_1258.finish();
            if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1258
                .take()
            {
                __terrane_uncaught(__terrane_select_cleanup_error);
            }
            match __terrane_select_winner_1258 {
                0 => {
                    let projected: String = __terrane_traced(
                        __terrane_select_result_1258_0
                            .take()
                            .expect("selected case owns its ready result"),
                        5 /* terrane-site: src/main.trn:36:28-36:46 */,
                    );
                    println!("{}", terrane_scalar_support::scalar_text(&projected));
                }
                1 => {
                    let source: String = __terrane_select_result_1258_1
                        .take()
                        .expect("selected case owns its ready result");
                    println!("{}", terrane_scalar_support::scalar_text(&source));
                }
                _ => unreachable!("selected winner is within the case count"),
            }
        }
        {
            let mut __terrane_select_guard_1396 = __terrane_finally_guard();
            let mut __terrane_select_cleanup_error_1396: Option<TerraneError> = None;
            let __terrane_select_control_1396_0 = __terrane_select_control();
            let mut __terrane_select_future_1396_0 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_1396_0.clone(), { let
                __terrane_future = echo_after_yield(String::from("pending-first")); async
                move { __terrane_raised_err(__terrane_future. await,
                6 /* terrane-site: src/main.trn:41:24-41:56 */) } })
            );
            let mut __terrane_select_result_1396_0 = None;
            let __terrane_select_control_1396_1 = __terrane_select_control();
            let mut __terrane_select_future_1396_1 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_1396_1.clone(), { let
                __terrane_future = echo_after_yield(String::from("pending-second"));
                async move { __terrane_raised_err(__terrane_future. await,
                7 /* terrane-site: src/main.trn:43:25-43:58 */) } })
            );
            let mut __terrane_select_result_1396_1 = None;
            let __terrane_select_winner_1396 = std::future::poll_fn(|
                    __terrane_select_context|
                {
                    if __terrane_cancellation_is_requested() {
                        return std::task::Poll::Ready(usize::MAX);
                    }
                    for __terrane_select_offset in 0..2usize {
                        let __terrane_select_candidate = (__terrane_select_cursor_1396
                            + __terrane_select_offset) % 2usize;
                        match __terrane_select_candidate {
                            0 => {
                                match Future::poll(
                                    __terrane_select_future_1396_0.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_1396_0 = Some(
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
                                    __terrane_select_future_1396_1.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_1396_1 = Some(
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
            if __terrane_select_winner_1396 == usize::MAX {
                __terrane_select_control_1396_1.request_cancel();
                __terrane_select_control_1396_0.request_cancel();
                if let Some(Err(__terrane_select_error)) = __terrane_select_future_1396_1
                    .as_mut()
                    .await
                {
                    __terrane_select_cleanup_error_1396 = Some(
                        __terrane_trace_error(
                            __terrane_select_error,
                            7 /* terrane-site: src/main.trn:43:25-43:58 */,
                        ),
                    );
                }
                if let Some(Err(__terrane_select_error)) = __terrane_select_future_1396_0
                    .as_mut()
                    .await
                {
                    __terrane_select_cleanup_error_1396 = Some(
                        __terrane_trace_error(
                            __terrane_select_error,
                            6 /* terrane-site: src/main.trn:41:24-41:56 */,
                        ),
                    );
                }
                __terrane_wait_projected_cleanups().await;
                __terrane_select_guard_1396.finish();
                if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1396
                    .take()
                {
                    __terrane_uncaught(__terrane_select_cleanup_error);
                }
                __terrane_finish_cancelled_select(__terrane_select_guard_1396).await;
            }
            __terrane_select_cursor_1396 = (__terrane_select_winner_1396 + 1usize)
                % 2usize;
            match __terrane_select_winner_1396 {
                0 => {
                    __terrane_select_control_1396_1.request_cancel();
                    if let Some(Err(__terrane_select_error)) = __terrane_select_future_1396_1
                        .as_mut()
                        .await
                    {
                        __terrane_select_cleanup_error_1396 = Some(
                            __terrane_trace_error(
                                __terrane_select_error,
                                7 /* terrane-site: src/main.trn:43:25-43:58 */,
                            ),
                        );
                    }
                }
                1 => {
                    __terrane_select_control_1396_0.request_cancel();
                    if let Some(Err(__terrane_select_error)) = __terrane_select_future_1396_0
                        .as_mut()
                        .await
                    {
                        __terrane_select_cleanup_error_1396 = Some(
                            __terrane_trace_error(
                                __terrane_select_error,
                                6 /* terrane-site: src/main.trn:41:24-41:56 */,
                            ),
                        );
                    }
                }
                _ => unreachable!("selected winner is within the case count"),
            }
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_1396.finish();
            if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1396
                .take()
            {
                __terrane_uncaught(__terrane_select_cleanup_error);
            }
            match __terrane_select_winner_1396 {
                0 => {
                    let first: String = __terrane_traced(
                        __terrane_select_result_1396_0
                            .take()
                            .expect("selected case owns its ready result"),
                        6 /* terrane-site: src/main.trn:41:24-41:56 */,
                    );
                    println!("{}", terrane_scalar_support::scalar_text(&first));
                }
                1 => {
                    let second: String = __terrane_traced(
                        __terrane_select_result_1396_1
                            .take()
                            .expect("selected case owns its ready result"),
                        7 /* terrane-site: src/main.trn:43:25-43:58 */,
                    );
                    println!("{}", terrane_scalar_support::scalar_text(&second));
                }
                _ => unreachable!("selected winner is within the case count"),
            }
        }
        __terrane_raised(
            reset_operation_state(),
            8 /* terrane-site: src/main.trn:45:3-45:25 */,
        );
        {
            let mut __terrane_select_guard_1583 = __terrane_finally_guard();
            let mut __terrane_select_cleanup_error_1583: Option<TerraneError> = None;
            let __terrane_select_control_1583_0 = __terrane_select_control();
            let mut __terrane_select_future_1583_0 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_1583_0.clone(), { let
                __terrane_future = wait_forever(); async move {
                __terrane_raised_err(__terrane_future. await, 9 /* terrane-site: src/main.trn:47:16-47:29 */) } })
            );
            let mut __terrane_select_result_1583_0 = None;
            let __terrane_select_control_1583_1 = __terrane_select_control();
            let mut __terrane_select_future_1583_1 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_1583_1.clone(), { let
                __terrane_future = timer_poll_count(); async move {
                __terrane_raised_err(__terrane_future. await, 10 /* terrane-site: src/main.trn:49:38-49:55 */) } })
            );
            let mut __terrane_select_result_1583_1 = None;
            let __terrane_select_winner_1583 = std::future::poll_fn(|
                    __terrane_select_context|
                {
                    if __terrane_cancellation_is_requested() {
                        return std::task::Poll::Ready(usize::MAX);
                    }
                    for __terrane_select_offset in 0..2usize {
                        let __terrane_select_candidate = (__terrane_select_cursor_1583
                            + __terrane_select_offset) % 2usize;
                        match __terrane_select_candidate {
                            0 => {
                                match Future::poll(
                                    __terrane_select_future_1583_0.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_1583_0 = Some(
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
                                    __terrane_select_future_1583_1.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_1583_1 = Some(
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
            if __terrane_select_winner_1583 == usize::MAX {
                __terrane_select_control_1583_1.request_cancel();
                __terrane_select_control_1583_0.request_cancel();
                if let Some(Err(__terrane_select_error)) = __terrane_select_future_1583_1
                    .as_mut()
                    .await
                {
                    __terrane_select_cleanup_error_1583 = Some(
                        __terrane_trace_error(
                            __terrane_select_error,
                            10 /* terrane-site: src/main.trn:49:38-49:55 */,
                        ),
                    );
                }
                if let Some(Err(__terrane_select_error)) = __terrane_select_future_1583_0
                    .as_mut()
                    .await
                {
                    __terrane_select_cleanup_error_1583 = Some(
                        __terrane_trace_error(
                            __terrane_select_error,
                            9 /* terrane-site: src/main.trn:47:16-47:29 */,
                        ),
                    );
                }
                __terrane_wait_projected_cleanups().await;
                __terrane_select_guard_1583.finish();
                if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1583
                    .take()
                {
                    __terrane_uncaught(__terrane_select_cleanup_error);
                }
                __terrane_finish_cancelled_select(__terrane_select_guard_1583).await;
            }
            __terrane_select_cursor_1583 = (__terrane_select_winner_1583 + 1usize)
                % 2usize;
            match __terrane_select_winner_1583 {
                0 => {
                    __terrane_select_control_1583_1.request_cancel();
                    if let Some(Err(__terrane_select_error)) = __terrane_select_future_1583_1
                        .as_mut()
                        .await
                    {
                        __terrane_select_cleanup_error_1583 = Some(
                            __terrane_trace_error(
                                __terrane_select_error,
                                10 /* terrane-site: src/main.trn:49:38-49:55 */,
                            ),
                        );
                    }
                }
                1 => {
                    __terrane_select_control_1583_0.request_cancel();
                    if let Some(Err(__terrane_select_error)) = __terrane_select_future_1583_0
                        .as_mut()
                        .await
                    {
                        __terrane_select_cleanup_error_1583 = Some(
                            __terrane_trace_error(
                                __terrane_select_error,
                                9 /* terrane-site: src/main.trn:47:16-47:29 */,
                            ),
                        );
                    }
                }
                _ => unreachable!("selected winner is within the case count"),
            }
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_1583.finish();
            if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1583
                .take()
            {
                __terrane_uncaught(__terrane_select_cleanup_error);
            }
            match __terrane_select_winner_1583 {
                0 => {
                    let _ = __terrane_traced(
                        __terrane_select_result_1583_0
                            .take()
                            .expect("selected case owns its ready result"),
                        9 /* terrane-site: src/main.trn:47:16-47:29 */,
                    );
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("unexpected projected winner"))
                    );
                }
                1 => {
                    let selection_polls: terrane_int_support::Int = __terrane_traced(
                        __terrane_select_result_1583_1
                            .take()
                            .expect("selected case owns its ready result"),
                        10 /* terrane-site: src/main.trn:49:38-49:55 */,
                    );
                    println!(
                        "{}{}",
                        terrane_scalar_support::scalar_text(&String::from("selection-polls")),
                        terrane_scalar_support::scalar_text(&selection_polls)
                    );
                }
                _ => unreachable!("selected winner is within the case count"),
            }
        }
        let started: bool = __terrane_traced(
            __terrane_await({
                    let __terrane_future = wait_until_operation_started();
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            11 /* terrane-site: src/main.trn:51:24-51:53 */,
                        )
                    }
                })
                .await,
            11 /* terrane-site: src/main.trn:51:24-51:53 */,
        );
        let drops: terrane_int_support::Int = __terrane_raised(
            operation_drop_count(),
            12 /* terrane-site: src/main.trn:52:15-52:36 */,
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&started),
            terrane_scalar_support::scalar_text(&drops)
        );
    });
}
// Source: <terrane>/projected/deps/async-witness.trn
// Namespace: deps/async-witness
pub async fn checked_echo(value: String) -> Result<String, crate::TerraneForeignError> {
    let value = value;
    match crate::__terrane_dependency_await_unwind(async_witness::checked_echo(value))
        .await
    {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(error)) => {
            Err(
                crate::TerraneForeignError(
                    crate::TerraneError::custom_raised(
                        crate::TERRANE_DEPENDENCY_ERROR,
                        format!(
                            "Rust dependency `async-witness` member `async_witness::checked_echo` failed: {error}"
                        ),
                        crate::TERRANE_NO_SITE,
                    ),
                ),
            )
        }
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "async-witness",
                    "async_witness::checked_echo",
                ),
            )
        }
    }
}
pub async fn echo_after_yield(
    value: String,
) -> Result<String, crate::TerraneForeignError> {
    let value = value;
    match crate::__terrane_dependency_await_unwind(
            async_witness::echo_after_yield(value),
        )
        .await
    {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "async-witness",
                    "async_witness::echo_after_yield",
                ),
            )
        }
    }
}
pub fn operation_drop_count() -> Result<
    terrane_int_support::Int,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| async_witness::operation_drop_count()),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from_u128(value as u128)),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "async-witness",
                    "async_witness::operation_drop_count",
                ),
            )
        }
    }
}
pub fn reset_operation_state() -> Result<(), crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| async_witness::reset_operation_state()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "async-witness",
                    "async_witness::reset_operation_state",
                ),
            )
        }
    }
}
pub async fn signal_sibling() -> Result<String, crate::TerraneForeignError> {
    match crate::__terrane_dependency_await_unwind(async_witness::signal_sibling()).await
    {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "async-witness",
                    "async_witness::signal_sibling",
                ),
            )
        }
    }
}
pub async fn socket_round_trip() -> Result<String, crate::TerraneForeignError> {
    match crate::__terrane_dependency_await_unwind(async_witness::socket_round_trip())
        .await
    {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "async-witness",
                    "async_witness::socket_round_trip",
                ),
            )
        }
    }
}
pub async fn timer_poll_count() -> Result<
    terrane_int_support::Int,
    crate::TerraneForeignError,
> {
    match crate::__terrane_dependency_await_unwind(async_witness::timer_poll_count())
        .await
    {
        Ok(value) => Ok(terrane_int_support::Int::from_u128(value as u128)),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "async-witness",
                    "async_witness::timer_poll_count",
                ),
            )
        }
    }
}
pub async fn wait_for_sibling() -> Result<String, crate::TerraneForeignError> {
    match crate::__terrane_dependency_await_unwind(async_witness::wait_for_sibling())
        .await
    {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "async-witness",
                    "async_witness::wait_for_sibling",
                ),
            )
        }
    }
}
pub async fn wait_forever() -> Result<String, crate::TerraneForeignError> {
    match crate::__terrane_dependency_await_unwind(async_witness::wait_forever()).await {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "async-witness",
                    "async_witness::wait_forever",
                ),
            )
        }
    }
}
pub async fn wait_until_operation_started() -> Result<bool, crate::TerraneForeignError> {
    match crate::__terrane_dependency_await_unwind(
            async_witness::wait_until_operation_started(),
        )
        .await
    {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "async-witness",
                    "async_witness::wait_until_operation_started",
                ),
            )
        }
    }
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
