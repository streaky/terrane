// Generated deterministically by Terrane <version>.
// Runtime support: async.rs, async_select.rs, executor_parallel.rs
// Vendored support crates: terrane-int-support, terrane-scalar-support
// Source: case.trn
// Namespace: heterogeneous-select
fn mark(value: String) -> String {
    println!("{}", terrane_scalar_support::scalar_text(&value));
    return value;
}
async fn number_task(marker: String) -> terrane_int_support::Int {
    let _ = &marker;
    return terrane_int_support::Int::from(7_i128);
}
async fn text_task(marker: String) -> String {
    let _ = &marker;
    return String::from("text");
}
fn main() {
    __terrane_run(async move {
        let mut __terrane_select_cursor_275 = 0usize;
        let mut count: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
        while count.clone() < terrane_int_support::Int::from(4_i128) {
            {
                let mut __terrane_select_guard_275 = __terrane_finally_guard();
                let __terrane_select_control_275_0 = __terrane_select_control();
                let mut __terrane_select_future_275_0 = std::pin::pin!(
                    __terrane_select_operation(__terrane_select_control_275_0.clone(),
                    number_task(mark(String::from("construct-a"))))
                );
                let mut __terrane_select_result_275_0 = None;
                let __terrane_select_control_275_1 = __terrane_select_control();
                let mut __terrane_select_future_275_1 = std::pin::pin!(
                    __terrane_select_operation(__terrane_select_control_275_1.clone(),
                    text_task(mark(String::from("construct-b"))))
                );
                let mut __terrane_select_result_275_1 = None;
                let __terrane_select_winner_275 = std::future::poll_fn(|
                        __terrane_select_context|
                    {
                        if __terrane_cancellation_is_requested() {
                            return std::task::Poll::Ready(usize::MAX);
                        }
                        for __terrane_select_offset in 0..2usize {
                            let __terrane_select_candidate = (__terrane_select_cursor_275
                                + __terrane_select_offset) % 2usize;
                            match __terrane_select_candidate {
                                0 => {
                                    match Future::poll(
                                        __terrane_select_future_275_0.as_mut(),
                                        __terrane_select_context,
                                    ) {
                                        std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                            __terrane_select_result_275_0 = Some(
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
                                        __terrane_select_future_275_1.as_mut(),
                                        __terrane_select_context,
                                    ) {
                                        std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                            __terrane_select_result_275_1 = Some(
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
                if __terrane_select_winner_275 == usize::MAX {
                    __terrane_select_control_275_1.request_cancel();
                    __terrane_select_control_275_0.request_cancel();
                    let _ = __terrane_select_future_275_1.as_mut().await;
                    let _ = __terrane_select_future_275_0.as_mut().await;
                    __terrane_wait_projected_cleanups().await;
                    __terrane_select_guard_275.finish();
                    __terrane_finish_cancelled_select(__terrane_select_guard_275).await;
                }
                __terrane_select_cursor_275 = (__terrane_select_winner_275 + 1usize)
                    % 2usize;
                match __terrane_select_winner_275 {
                    0 => {
                        __terrane_select_control_275_1.request_cancel();
                        let _ = __terrane_select_future_275_1.as_mut().await;
                    }
                    1 => {
                        __terrane_select_control_275_0.request_cancel();
                        let _ = __terrane_select_future_275_0.as_mut().await;
                    }
                    _ => unreachable!("selected winner is within the case count"),
                }
                __terrane_wait_projected_cleanups().await;
                __terrane_select_guard_275.finish();
                match __terrane_select_winner_275 {
                    0 => {
                        let number: terrane_int_support::Int = __terrane_select_result_275_0
                            .take()
                            .expect("selected case owns its ready result");
                        println!("{}", terrane_scalar_support::scalar_text(&number));
                    }
                    1 => {
                        let text: String = __terrane_select_result_275_1
                            .take()
                            .expect("selected case owns its ready result");
                        println!("{}", terrane_scalar_support::scalar_text(&text));
                    }
                    _ => unreachable!("selected winner is within the case count"),
                }
            }
            count = count.clone() + terrane_int_support::Int::from(1_i128);
        }
    });
}
