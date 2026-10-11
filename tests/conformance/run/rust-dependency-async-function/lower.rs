// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
async fn source_ready() -> String {
    return String::from("source-ready");
}
fn main() {
    __terrane_run(async move {
        let echoed: String;
        let polls: terrane_int_support::Int;
        let message: String;
        let scope: TerraneTaskScope;
        let waiting: TerraneScopedTask<String>;
        let signalling: TerraneScopedTask<String>;
        let waited: TerraneTaskOutcome<String>;
        let signalled: TerraneTaskOutcome<String>;
        let mut rejected: Option<String> = None;
        let selection_pair: TerraneChannelPair<terrane_int_support::Int>;
        let selection_rx: TerraneChannelReceiver<terrane_int_support::Int>;
        let selected: String;
        let projected: String;
        let source: String;
        let first: String;
        let second: String;
        let selection_polls: terrane_int_support::Int;
        let started: bool;
        let drops: terrane_int_support::Int;
        let mut __terrane_select_cursor_1059 = 0usize;
        let mut __terrane_select_cursor_1232 = 0usize;
        let mut __terrane_select_cursor_1370 = 0usize;
        let mut __terrane_select_cursor_1557 = 0usize;
        echoed = __terrane_traced(
            __terrane_await({
                    let __terrane_future = echo_after_yield(
                        String::from("projected async success"),
                    );
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            0 /* terrane-site: src/main.trn:10:25-10:67 */,
                        )
                    }
                })
                .await,
            0 /* terrane-site: src/main.trn:10:25-10:67 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&echoed));
        polls = __terrane_traced(
            __terrane_await({
                    let __terrane_future = timer_poll_count();
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            1 /* terrane-site: src/main.trn:12:21-12:38 */,
                        )
                    }
                })
                .await,
            1 /* terrane-site: src/main.trn:12:21-12:38 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&polls));
        message = __terrane_traced(
            __terrane_await({
                    let __terrane_future = socket_round_trip();
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            2 /* terrane-site: src/main.trn:14:26-14:44 */,
                        )
                    }
                })
                .await,
            2 /* terrane-site: src/main.trn:14:26-14:44 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&message));
        scope = TerraneTaskScope::new(None);
        waiting = {
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
        signalling = {
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
        waited = __terrane_await(scope.join(waiting)).await;
        signalled = __terrane_await(scope.join(signalling)).await;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&waited.completed),
            terrane_scalar_support::scalar_text(&signalled.completed)
        );
        let __terrane_completion_0: TerraneCompletion<()> = async {
            let __terrane_try_0: TerraneCompletion<()> = async {
                let _ = rejected
                    .insert(
                        __terrane_traced_completion!(
                            __terrane_await({ let __terrane_future =
                            checked_echo(String::from("reject")); async move {
                            __terrane_raised_err(__terrane_future. await,
                            3 /* terrane-site: src/main.trn:23:29-23:50 */) } }).
                            await, 3 /* terrane-site: src/main.trn:23:29-23:50 */
                        ),
                    );
                println!(
                    "{}", terrane_scalar_support::scalar_text(&rejected.as_ref()
                    .expect("flow-proven available binding").clone())
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
                    if !__terrane_handled_0
                        && __terrane_error_0.kind
                            == TerraneErrorKind::Custom(DescriptorId(2))
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
        selection_pair = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        selection_rx = selection_pair.receiver;
        {
            let mut __terrane_select_guard_1059 = __terrane_finally_guard();
            let mut __terrane_select_cleanup_error_1059: Option<TerraneError> = None;
            let __terrane_select_control_1059_0 = __terrane_select_control();
            let mut __terrane_select_future_1059_0 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_1059_0.clone(), { let
                __terrane_future = echo_after_yield(String::from("selected-projected"));
                async move { __terrane_raised_err(__terrane_future. await,
                4 /* terrane-site: src/main.trn:30:34-30:71 */) } })
            );
            let mut __terrane_select_result_1059_0 = None;
            let __terrane_select_control_1059_1 = __terrane_select_control();
            let mut __terrane_select_future_1059_1 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_1059_1.clone(),
                Box::pin(selection_rx.receive()))
            );
            let mut __terrane_select_result_1059_1 = None;
            let __terrane_select_winner_1059 = std::future::poll_fn(|
                    __terrane_select_context|
                {
                    if __terrane_cancellation_is_requested() {
                        return std::task::Poll::Ready(usize::MAX);
                    }
                    for __terrane_select_offset in 0..2usize {
                        let __terrane_select_candidate = (__terrane_select_cursor_1059
                            + __terrane_select_offset) % 2usize;
                        match __terrane_select_candidate {
                            0 => {
                                match Future::poll(
                                    __terrane_select_future_1059_0.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_1059_0 = Some(
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
                                    __terrane_select_future_1059_1.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_1059_1 = Some(
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
            if __terrane_select_winner_1059 == usize::MAX {
                __terrane_select_control_1059_1.request_cancel();
                __terrane_select_control_1059_0.request_cancel();
                let _ = __terrane_select_future_1059_1.as_mut().await;
                if let Some(Err(__terrane_select_error)) = __terrane_select_future_1059_0
                    .as_mut()
                    .await
                {
                    __terrane_select_cleanup_error_1059 = Some(
                        __terrane_trace_error(
                            __terrane_select_error,
                            4 /* terrane-site: src/main.trn:30:34-30:71 */,
                        ),
                    );
                }
                __terrane_wait_projected_cleanups().await;
                __terrane_select_guard_1059.finish();
                if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1059
                    .take()
                {
                    __terrane_uncaught(__terrane_select_cleanup_error);
                }
                __terrane_finish_cancelled_select(__terrane_select_guard_1059).await;
            }
            __terrane_select_cursor_1059 = (__terrane_select_winner_1059 + 1usize)
                % 2usize;
            match __terrane_select_winner_1059 {
                0 => {
                    __terrane_select_control_1059_1.request_cancel();
                    let _ = __terrane_select_future_1059_1.as_mut().await;
                }
                1 => {
                    __terrane_select_control_1059_0.request_cancel();
                    if let Some(Err(__terrane_select_error)) = __terrane_select_future_1059_0
                        .as_mut()
                        .await
                    {
                        __terrane_select_cleanup_error_1059 = Some(
                            __terrane_trace_error(
                                __terrane_select_error,
                                4 /* terrane-site: src/main.trn:30:34-30:71 */,
                            ),
                        );
                    }
                }
                _ => unreachable!("selected winner is within the case count"),
            }
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_1059.finish();
            if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1059
                .take()
            {
                __terrane_uncaught(__terrane_select_cleanup_error);
            }
            match __terrane_select_winner_1059 {
                0 => {
                    selected = __terrane_traced(
                        __terrane_select_result_1059_0
                            .take()
                            .expect("selected case owns its ready result"),
                        4 /* terrane-site: src/main.trn:30:34-30:71 */,
                    );
                    println!("{}", terrane_scalar_support::scalar_text(&selected));
                }
                1 => {
                    let _ = __terrane_select_result_1059_1
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
            let mut __terrane_select_guard_1232 = __terrane_finally_guard();
            let mut __terrane_select_cleanup_error_1232: Option<TerraneError> = None;
            let __terrane_select_control_1232_0 = __terrane_select_control();
            let mut __terrane_select_future_1232_0 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_1232_0.clone(), { let
                __terrane_future = socket_round_trip(); async move {
                __terrane_raised_err(__terrane_future. await, 5 /* terrane-site: src/main.trn:35:28-35:46 */) } })
            );
            let mut __terrane_select_result_1232_0 = None;
            let __terrane_select_control_1232_1 = __terrane_select_control();
            let mut __terrane_select_future_1232_1 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_1232_1.clone(),
                source_ready())
            );
            let mut __terrane_select_result_1232_1 = None;
            let __terrane_select_winner_1232 = std::future::poll_fn(|
                    __terrane_select_context|
                {
                    if __terrane_cancellation_is_requested() {
                        return std::task::Poll::Ready(usize::MAX);
                    }
                    for __terrane_select_offset in 0..2usize {
                        let __terrane_select_candidate = (__terrane_select_cursor_1232
                            + __terrane_select_offset) % 2usize;
                        match __terrane_select_candidate {
                            0 => {
                                match Future::poll(
                                    __terrane_select_future_1232_0.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_1232_0 = Some(
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
                                    __terrane_select_future_1232_1.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_1232_1 = Some(
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
            if __terrane_select_winner_1232 == usize::MAX {
                __terrane_select_control_1232_1.request_cancel();
                __terrane_select_control_1232_0.request_cancel();
                let _ = __terrane_select_future_1232_1.as_mut().await;
                if let Some(Err(__terrane_select_error)) = __terrane_select_future_1232_0
                    .as_mut()
                    .await
                {
                    __terrane_select_cleanup_error_1232 = Some(
                        __terrane_trace_error(
                            __terrane_select_error,
                            5 /* terrane-site: src/main.trn:35:28-35:46 */,
                        ),
                    );
                }
                __terrane_wait_projected_cleanups().await;
                __terrane_select_guard_1232.finish();
                if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1232
                    .take()
                {
                    __terrane_uncaught(__terrane_select_cleanup_error);
                }
                __terrane_finish_cancelled_select(__terrane_select_guard_1232).await;
            }
            __terrane_select_cursor_1232 = (__terrane_select_winner_1232 + 1usize)
                % 2usize;
            match __terrane_select_winner_1232 {
                0 => {
                    __terrane_select_control_1232_1.request_cancel();
                    let _ = __terrane_select_future_1232_1.as_mut().await;
                }
                1 => {
                    __terrane_select_control_1232_0.request_cancel();
                    if let Some(Err(__terrane_select_error)) = __terrane_select_future_1232_0
                        .as_mut()
                        .await
                    {
                        __terrane_select_cleanup_error_1232 = Some(
                            __terrane_trace_error(
                                __terrane_select_error,
                                5 /* terrane-site: src/main.trn:35:28-35:46 */,
                            ),
                        );
                    }
                }
                _ => unreachable!("selected winner is within the case count"),
            }
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_1232.finish();
            if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1232
                .take()
            {
                __terrane_uncaught(__terrane_select_cleanup_error);
            }
            match __terrane_select_winner_1232 {
                0 => {
                    projected = __terrane_traced(
                        __terrane_select_result_1232_0
                            .take()
                            .expect("selected case owns its ready result"),
                        5 /* terrane-site: src/main.trn:35:28-35:46 */,
                    );
                    println!("{}", terrane_scalar_support::scalar_text(&projected));
                }
                1 => {
                    source = __terrane_select_result_1232_1
                        .take()
                        .expect("selected case owns its ready result");
                    println!("{}", terrane_scalar_support::scalar_text(&source));
                }
                _ => unreachable!("selected winner is within the case count"),
            }
        }
        {
            let mut __terrane_select_guard_1370 = __terrane_finally_guard();
            let mut __terrane_select_cleanup_error_1370: Option<TerraneError> = None;
            let __terrane_select_control_1370_0 = __terrane_select_control();
            let mut __terrane_select_future_1370_0 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_1370_0.clone(), { let
                __terrane_future = echo_after_yield(String::from("pending-first")); async
                move { __terrane_raised_err(__terrane_future. await,
                6 /* terrane-site: src/main.trn:40:24-40:56 */) } })
            );
            let mut __terrane_select_result_1370_0 = None;
            let __terrane_select_control_1370_1 = __terrane_select_control();
            let mut __terrane_select_future_1370_1 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_1370_1.clone(), { let
                __terrane_future = echo_after_yield(String::from("pending-second"));
                async move { __terrane_raised_err(__terrane_future. await,
                7 /* terrane-site: src/main.trn:42:25-42:58 */) } })
            );
            let mut __terrane_select_result_1370_1 = None;
            let __terrane_select_winner_1370 = std::future::poll_fn(|
                    __terrane_select_context|
                {
                    if __terrane_cancellation_is_requested() {
                        return std::task::Poll::Ready(usize::MAX);
                    }
                    for __terrane_select_offset in 0..2usize {
                        let __terrane_select_candidate = (__terrane_select_cursor_1370
                            + __terrane_select_offset) % 2usize;
                        match __terrane_select_candidate {
                            0 => {
                                match Future::poll(
                                    __terrane_select_future_1370_0.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_1370_0 = Some(
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
                                    __terrane_select_future_1370_1.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_1370_1 = Some(
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
            if __terrane_select_winner_1370 == usize::MAX {
                __terrane_select_control_1370_1.request_cancel();
                __terrane_select_control_1370_0.request_cancel();
                if let Some(Err(__terrane_select_error)) = __terrane_select_future_1370_1
                    .as_mut()
                    .await
                {
                    __terrane_select_cleanup_error_1370 = Some(
                        __terrane_trace_error(
                            __terrane_select_error,
                            7 /* terrane-site: src/main.trn:42:25-42:58 */,
                        ),
                    );
                }
                if let Some(Err(__terrane_select_error)) = __terrane_select_future_1370_0
                    .as_mut()
                    .await
                {
                    __terrane_select_cleanup_error_1370 = Some(
                        __terrane_trace_error(
                            __terrane_select_error,
                            6 /* terrane-site: src/main.trn:40:24-40:56 */,
                        ),
                    );
                }
                __terrane_wait_projected_cleanups().await;
                __terrane_select_guard_1370.finish();
                if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1370
                    .take()
                {
                    __terrane_uncaught(__terrane_select_cleanup_error);
                }
                __terrane_finish_cancelled_select(__terrane_select_guard_1370).await;
            }
            __terrane_select_cursor_1370 = (__terrane_select_winner_1370 + 1usize)
                % 2usize;
            match __terrane_select_winner_1370 {
                0 => {
                    __terrane_select_control_1370_1.request_cancel();
                    if let Some(Err(__terrane_select_error)) = __terrane_select_future_1370_1
                        .as_mut()
                        .await
                    {
                        __terrane_select_cleanup_error_1370 = Some(
                            __terrane_trace_error(
                                __terrane_select_error,
                                7 /* terrane-site: src/main.trn:42:25-42:58 */,
                            ),
                        );
                    }
                }
                1 => {
                    __terrane_select_control_1370_0.request_cancel();
                    if let Some(Err(__terrane_select_error)) = __terrane_select_future_1370_0
                        .as_mut()
                        .await
                    {
                        __terrane_select_cleanup_error_1370 = Some(
                            __terrane_trace_error(
                                __terrane_select_error,
                                6 /* terrane-site: src/main.trn:40:24-40:56 */,
                            ),
                        );
                    }
                }
                _ => unreachable!("selected winner is within the case count"),
            }
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_1370.finish();
            if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1370
                .take()
            {
                __terrane_uncaught(__terrane_select_cleanup_error);
            }
            match __terrane_select_winner_1370 {
                0 => {
                    first = __terrane_traced(
                        __terrane_select_result_1370_0
                            .take()
                            .expect("selected case owns its ready result"),
                        6 /* terrane-site: src/main.trn:40:24-40:56 */,
                    );
                    println!("{}", terrane_scalar_support::scalar_text(&first));
                }
                1 => {
                    second = __terrane_traced(
                        __terrane_select_result_1370_1
                            .take()
                            .expect("selected case owns its ready result"),
                        7 /* terrane-site: src/main.trn:42:25-42:58 */,
                    );
                    println!("{}", terrane_scalar_support::scalar_text(&second));
                }
                _ => unreachable!("selected winner is within the case count"),
            }
        }
        __terrane_raised(
            reset_operation_state(),
            8 /* terrane-site: src/main.trn:44:3-44:25 */,
        );
        {
            let mut __terrane_select_guard_1557 = __terrane_finally_guard();
            let mut __terrane_select_cleanup_error_1557: Option<TerraneError> = None;
            let __terrane_select_control_1557_0 = __terrane_select_control();
            let mut __terrane_select_future_1557_0 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_1557_0.clone(), { let
                __terrane_future = wait_forever(); async move {
                __terrane_raised_err(__terrane_future. await, 9 /* terrane-site: src/main.trn:46:16-46:29 */) } })
            );
            let mut __terrane_select_result_1557_0 = None;
            let __terrane_select_control_1557_1 = __terrane_select_control();
            let mut __terrane_select_future_1557_1 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_1557_1.clone(), { let
                __terrane_future = timer_poll_count(); async move {
                __terrane_raised_err(__terrane_future. await, 10 /* terrane-site: src/main.trn:48:38-48:55 */) } })
            );
            let mut __terrane_select_result_1557_1 = None;
            let __terrane_select_winner_1557 = std::future::poll_fn(|
                    __terrane_select_context|
                {
                    if __terrane_cancellation_is_requested() {
                        return std::task::Poll::Ready(usize::MAX);
                    }
                    for __terrane_select_offset in 0..2usize {
                        let __terrane_select_candidate = (__terrane_select_cursor_1557
                            + __terrane_select_offset) % 2usize;
                        match __terrane_select_candidate {
                            0 => {
                                match Future::poll(
                                    __terrane_select_future_1557_0.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_1557_0 = Some(
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
                                    __terrane_select_future_1557_1.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_1557_1 = Some(
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
            if __terrane_select_winner_1557 == usize::MAX {
                __terrane_select_control_1557_1.request_cancel();
                __terrane_select_control_1557_0.request_cancel();
                if let Some(Err(__terrane_select_error)) = __terrane_select_future_1557_1
                    .as_mut()
                    .await
                {
                    __terrane_select_cleanup_error_1557 = Some(
                        __terrane_trace_error(
                            __terrane_select_error,
                            10 /* terrane-site: src/main.trn:48:38-48:55 */,
                        ),
                    );
                }
                if let Some(Err(__terrane_select_error)) = __terrane_select_future_1557_0
                    .as_mut()
                    .await
                {
                    __terrane_select_cleanup_error_1557 = Some(
                        __terrane_trace_error(
                            __terrane_select_error,
                            9 /* terrane-site: src/main.trn:46:16-46:29 */,
                        ),
                    );
                }
                __terrane_wait_projected_cleanups().await;
                __terrane_select_guard_1557.finish();
                if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1557
                    .take()
                {
                    __terrane_uncaught(__terrane_select_cleanup_error);
                }
                __terrane_finish_cancelled_select(__terrane_select_guard_1557).await;
            }
            __terrane_select_cursor_1557 = (__terrane_select_winner_1557 + 1usize)
                % 2usize;
            match __terrane_select_winner_1557 {
                0 => {
                    __terrane_select_control_1557_1.request_cancel();
                    if let Some(Err(__terrane_select_error)) = __terrane_select_future_1557_1
                        .as_mut()
                        .await
                    {
                        __terrane_select_cleanup_error_1557 = Some(
                            __terrane_trace_error(
                                __terrane_select_error,
                                10 /* terrane-site: src/main.trn:48:38-48:55 */,
                            ),
                        );
                    }
                }
                1 => {
                    __terrane_select_control_1557_0.request_cancel();
                    if let Some(Err(__terrane_select_error)) = __terrane_select_future_1557_0
                        .as_mut()
                        .await
                    {
                        __terrane_select_cleanup_error_1557 = Some(
                            __terrane_trace_error(
                                __terrane_select_error,
                                9 /* terrane-site: src/main.trn:46:16-46:29 */,
                            ),
                        );
                    }
                }
                _ => unreachable!("selected winner is within the case count"),
            }
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_1557.finish();
            if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1557
                .take()
            {
                __terrane_uncaught(__terrane_select_cleanup_error);
            }
            match __terrane_select_winner_1557 {
                0 => {
                    let _ = __terrane_traced(
                        __terrane_select_result_1557_0
                            .take()
                            .expect("selected case owns its ready result"),
                        9 /* terrane-site: src/main.trn:46:16-46:29 */,
                    );
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("unexpected projected winner"))
                    );
                }
                1 => {
                    selection_polls = __terrane_traced(
                        __terrane_select_result_1557_1
                            .take()
                            .expect("selected case owns its ready result"),
                        10 /* terrane-site: src/main.trn:48:38-48:55 */,
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
        started = __terrane_traced(
            __terrane_await({
                    let __terrane_future = wait_until_operation_started();
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            11 /* terrane-site: src/main.trn:50:24-50:53 */,
                        )
                    }
                })
                .await,
            11 /* terrane-site: src/main.trn:50:24-50:53 */,
        );
        drops = __terrane_raised(
            operation_drop_count(),
            12 /* terrane-site: src/main.trn:51:15-51:36 */,
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&started),
            terrane_scalar_support::scalar_text(&drops)
        );
    });
}
