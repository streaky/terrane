// Generated deterministically by Terrane <version>.
// Runtime support: async.rs, async_select.rs, executor_parallel.rs
// Vendored support crates: terrane-int-support, terrane-scalar-support
// Source: case.trn
// Namespace: select-definite-assignment
async fn one() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(1_i128);
}
async fn two() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(2_i128);
}
fn main() {
    __terrane_run(async move {
        let mut __terrane_select_cursor_148 = 0usize;
        let selected: terrane_int_support::Int;
        {
            let mut __terrane_select_guard_148 = __terrane_finally_guard();
            let __terrane_select_control_148_0 = __terrane_select_control();
            let mut __terrane_select_future_148_0 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_148_0.clone(), one())
            );
            let mut __terrane_select_result_148_0 = None;
            let __terrane_select_control_148_1 = __terrane_select_control();
            let mut __terrane_select_future_148_1 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_148_1.clone(), two())
            );
            let mut __terrane_select_result_148_1 = None;
            let __terrane_select_winner_148 = std::future::poll_fn(|
                    __terrane_select_context|
                {
                    if __terrane_cancellation_is_requested() {
                        return std::task::Poll::Ready(usize::MAX);
                    }
                    for __terrane_select_offset in 0..2usize {
                        let __terrane_select_candidate = (__terrane_select_cursor_148
                            + __terrane_select_offset) % 2usize;
                        match __terrane_select_candidate {
                            0 => {
                                match Future::poll(
                                    __terrane_select_future_148_0.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_148_0 = Some(
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
                                    __terrane_select_future_148_1.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_148_1 = Some(
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
            if __terrane_select_winner_148 == usize::MAX {
                __terrane_select_control_148_1.request_cancel();
                __terrane_select_control_148_0.request_cancel();
                let _ = __terrane_select_future_148_1.as_mut().await;
                let _ = __terrane_select_future_148_0.as_mut().await;
                __terrane_wait_projected_cleanups().await;
                __terrane_select_guard_148.finish();
                __terrane_finish_cancelled_select(__terrane_select_guard_148).await;
            }
            __terrane_select_cursor_148 = (__terrane_select_winner_148 + 1usize)
                % 2usize;
            match __terrane_select_winner_148 {
                0 => {
                    __terrane_select_control_148_1.request_cancel();
                    let _ = __terrane_select_future_148_1.as_mut().await;
                }
                1 => {
                    __terrane_select_control_148_0.request_cancel();
                    let _ = __terrane_select_future_148_0.as_mut().await;
                }
                _ => unreachable!("selected winner is within the case count"),
            }
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_148.finish();
            match __terrane_select_winner_148 {
                0 => {
                    let value: terrane_int_support::Int = __terrane_select_result_148_0
                        .take()
                        .expect("selected case owns its ready result");
                    selected = value.clone();
                }
                1 => {
                    let value: terrane_int_support::Int = __terrane_select_result_148_1
                        .take()
                        .expect("selected case owns its ready result");
                    selected = value.clone();
                }
                _ => unreachable!("selected winner is within the case count"),
            }
        }
        println!("{}", terrane_scalar_support::scalar_text(&selected));
    });
}
