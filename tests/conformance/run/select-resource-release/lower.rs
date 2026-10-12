// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: select-resource-release
#[derive(Clone)]
pub struct Marker {
    __terrane_lifetime: std::sync::Arc<()>,
    pub name: String,
}
impl Marker {
    pub fn terrane_construct() -> Self {
        Self {
            name: String::from("selected"),
            __terrane_lifetime: std::sync::Arc::new(()),
        }
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn destruct(&mut self) {
        println!("{}", String::from("resource-released"));
    }
}
impl Drop for Marker {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) != 1 {
            return;
        }
        self.destruct();
    }
}
async fn winner() -> Marker {
    return Marker::terrane_construct();
}
async fn failing_loser(
    receiver: TerraneChannelReceiver<terrane_int_support::Int>,
) -> Result<(), TerraneError> {
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
        println!("{}", String::from("loser-cleanup"));
        return TerraneCompletion::Error(
            TerraneError::raised(
                TerraneErrorKind::CoercionError,
                0 /* terrane-site: case.trn:19:5-19:25 */,
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
fn main() {
    __terrane_run(async move {
        let pair: TerraneChannelPair<terrane_int_support::Int>;
        let mut selected: Option<Marker> = None;
        let mut __terrane_select_cursor_588 = 0usize;
        pair = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let __terrane_completion_1: TerraneCompletion<()> = async {
            let __terrane_try_1: TerraneCompletion<()> = async {
                {
                    let mut __terrane_select_guard_588 = __terrane_finally_guard();
                    let mut __terrane_select_cleanup_error_588: Option<TerraneError> = None;
                    let __terrane_select_control_588_0 = __terrane_select_control();
                    let mut __terrane_select_future_588_0 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_588_0
                        .clone(), failing_loser(pair.receiver))
                    );
                    let mut __terrane_select_result_588_0 = None;
                    let __terrane_select_control_588_1 = __terrane_select_control();
                    let mut __terrane_select_future_588_1 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_588_1
                        .clone(), winner())
                    );
                    let mut __terrane_select_result_588_1 = None;
                    let __terrane_select_winner_588 = std::future::poll_fn(|
                            __terrane_select_context|
                        {
                            if __terrane_cancellation_is_requested() {
                                return std::task::Poll::Ready(usize::MAX);
                            }
                            for __terrane_select_offset in 0..2usize {
                                let __terrane_select_candidate = (__terrane_select_cursor_588
                                    + __terrane_select_offset) % 2usize;
                                match __terrane_select_candidate {
                                    0 => {
                                        match Future::poll(
                                            __terrane_select_future_588_0.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_588_0 = Some(
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
                                            __terrane_select_future_588_1.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_588_1 = Some(
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
                    if __terrane_select_winner_588 == usize::MAX {
                        __terrane_select_control_588_1.request_cancel();
                        __terrane_select_control_588_0.request_cancel();
                        let _ = __terrane_select_future_588_1.as_mut().await;
                        if let Some(Err(__terrane_select_error)) = __terrane_select_future_588_0
                            .as_mut()
                            .await
                        {
                            __terrane_select_cleanup_error_588 = Some(
                                __terrane_trace_error(
                                    __terrane_select_error,
                                    2 /* terrane-site: case.trn:25:18-25:48 */,
                                ),
                            );
                        }
                        __terrane_wait_projected_cleanups().await;
                        __terrane_select_guard_588.finish();
                        if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_588
                            .take()
                        {
                            return TerraneCompletion::Error(
                                __terrane_select_cleanup_error,
                            );
                        }
                        __terrane_finish_cancelled_select(__terrane_select_guard_588)
                            .await;
                    }
                    __terrane_select_cursor_588 = (__terrane_select_winner_588 + 1usize)
                        % 2usize;
                    match __terrane_select_winner_588 {
                        0 => {
                            __terrane_select_control_588_1.request_cancel();
                            let _ = __terrane_select_future_588_1.as_mut().await;
                        }
                        1 => {
                            __terrane_select_control_588_0.request_cancel();
                            if let Some(Err(__terrane_select_error)) = __terrane_select_future_588_0
                                .as_mut()
                                .await
                            {
                                __terrane_select_cleanup_error_588 = Some(
                                    __terrane_trace_error(
                                        __terrane_select_error,
                                        2 /* terrane-site: case.trn:25:18-25:48 */,
                                    ),
                                );
                            }
                        }
                        _ => unreachable!("selected winner is within the case count"),
                    }
                    __terrane_wait_projected_cleanups().await;
                    __terrane_select_guard_588.finish();
                    if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_588
                        .take()
                    {
                        return TerraneCompletion::Error(__terrane_select_cleanup_error);
                    }
                    match __terrane_select_winner_588 {
                        0 => {
                            let _ = __terrane_traced_completion!(
                                __terrane_select_result_588_0.take()
                                .expect("selected case owns its ready result"),
                                2 /* terrane-site: case.trn:25:18-25:48 */
                            );
                            println!("{}", String::from("unexpected loser"));
                        }
                        1 => {
                            let _ = selected
                                .insert(
                                    __terrane_select_result_588_1
                                        .take()
                                        .expect("selected case owns its ready result"),
                                );
                            println!(
                                "{}", terrane_scalar_support::scalar_text(&selected.as_ref()
                                .expect("flow-proven available binding").clone().name)
                            );
                        }
                        _ => unreachable!("selected winner is within the case count"),
                    }
                }
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
                    if !__terrane_handled_1
                        && __terrane_error_1.kind == TerraneErrorKind::CoercionError
                    {
                        __terrane_handled_1 = true;
                        println!("{}", String::from("caught-cleanup-error"));
                    }
                    if !__terrane_handled_1 {
                        return TerraneCompletion::Error(__terrane_error_1);
                    }
                }
            }
            TerraneCompletion::Normal
        }
            .await;
        match __terrane_completion_1 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
    });
}
