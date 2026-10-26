// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: select-allowed-nesting
async fn work() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(7_i128);
}
#[derive(Clone)]
pub struct Selector {}
impl Selector {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub async fn choose(&self) -> terrane_int_support::Int {
        let value: terrane_int_support::Int;
        let other: terrane_int_support::Int;
        let mut __terrane_select_cursor_119 = 0usize;
        {
            let mut __terrane_select_guard_119 = __terrane_finally_guard();
            let __terrane_select_control_119_0 = __terrane_select_control();
            let mut __terrane_select_future_119_0 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_119_0.clone(),
                work())
            );
            let mut __terrane_select_result_119_0 = None;
            let __terrane_select_control_119_1 = __terrane_select_control();
            let mut __terrane_select_future_119_1 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_119_1.clone(),
                work())
            );
            let mut __terrane_select_result_119_1 = None;
            let __terrane_select_winner_119 = std::future::poll_fn(|
                    __terrane_select_context|
                {
                    if __terrane_cancellation_is_requested() {
                        return std::task::Poll::Ready(usize::MAX);
                    }
                    for __terrane_select_offset in 0..2usize {
                        let __terrane_select_candidate = (__terrane_select_cursor_119
                            + __terrane_select_offset) % 2usize;
                        match __terrane_select_candidate {
                            0 => {
                                match Future::poll(
                                    __terrane_select_future_119_0.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_119_0 = Some(
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
                                    __terrane_select_future_119_1.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_119_1 = Some(
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
            if __terrane_select_winner_119 == usize::MAX {
                __terrane_select_control_119_1.request_cancel();
                __terrane_select_control_119_0.request_cancel();
                let _ = __terrane_select_future_119_1.as_mut().await;
                let _ = __terrane_select_future_119_0.as_mut().await;
                __terrane_wait_projected_cleanups().await;
                __terrane_select_guard_119.finish();
                __terrane_finish_cancelled_select(__terrane_select_guard_119).await;
            }
            __terrane_select_cursor_119 = (__terrane_select_winner_119 + 1usize)
                % 2usize;
            match __terrane_select_winner_119 {
                0 => {
                    __terrane_select_control_119_1.request_cancel();
                    let _ = __terrane_select_future_119_1.as_mut().await;
                }
                1 => {
                    __terrane_select_control_119_0.request_cancel();
                    let _ = __terrane_select_future_119_0.as_mut().await;
                }
                _ => unreachable!("selected winner is within the case count"),
            }
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_119.finish();
            match __terrane_select_winner_119 {
                0 => {
                    value = __terrane_select_result_119_0
                        .take()
                        .expect("selected case owns its ready result");
                    return value.clone();
                }
                1 => {
                    other = __terrane_select_result_119_1
                        .take()
                        .expect("selected case owns its ready result");
                    return other.clone();
                }
                _ => unreachable!("selected winner is within the case count"),
            }
        }
    }
    pub async fn __terrane_shared_async_choose(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
    ) -> terrane_int_support::Int {
        let value: terrane_int_support::Int;
        let other: terrane_int_support::Int;
        let mut __terrane_select_cursor_119 = 0usize;
        {
            let mut __terrane_select_guard_119 = __terrane_finally_guard();
            let __terrane_select_control_119_0 = __terrane_select_control();
            let mut __terrane_select_future_119_0 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_119_0.clone(),
                work())
            );
            let mut __terrane_select_result_119_0 = None;
            let __terrane_select_control_119_1 = __terrane_select_control();
            let mut __terrane_select_future_119_1 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_119_1.clone(),
                work())
            );
            let mut __terrane_select_result_119_1 = None;
            let __terrane_select_winner_119 = std::future::poll_fn(|
                    __terrane_select_context|
                {
                    if __terrane_cancellation_is_requested() {
                        return std::task::Poll::Ready(usize::MAX);
                    }
                    for __terrane_select_offset in 0..2usize {
                        let __terrane_select_candidate = (__terrane_select_cursor_119
                            + __terrane_select_offset) % 2usize;
                        match __terrane_select_candidate {
                            0 => {
                                match Future::poll(
                                    __terrane_select_future_119_0.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_119_0 = Some(
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
                                    __terrane_select_future_119_1.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_119_1 = Some(
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
            if __terrane_select_winner_119 == usize::MAX {
                __terrane_select_control_119_1.request_cancel();
                __terrane_select_control_119_0.request_cancel();
                let _ = __terrane_select_future_119_1.as_mut().await;
                let _ = __terrane_select_future_119_0.as_mut().await;
                __terrane_wait_projected_cleanups().await;
                __terrane_select_guard_119.finish();
                __terrane_finish_cancelled_select(__terrane_select_guard_119).await;
            }
            __terrane_select_cursor_119 = (__terrane_select_winner_119 + 1usize)
                % 2usize;
            match __terrane_select_winner_119 {
                0 => {
                    __terrane_select_control_119_1.request_cancel();
                    let _ = __terrane_select_future_119_1.as_mut().await;
                }
                1 => {
                    __terrane_select_control_119_0.request_cancel();
                    let _ = __terrane_select_future_119_0.as_mut().await;
                }
                _ => unreachable!("selected winner is within the case count"),
            }
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_119.finish();
            match __terrane_select_winner_119 {
                0 => {
                    value = __terrane_select_result_119_0
                        .take()
                        .expect("selected case owns its ready result");
                    return value.clone();
                }
                1 => {
                    other = __terrane_select_result_119_1
                        .take()
                        .expect("selected case owns its ready result");
                    return other.clone();
                }
                _ => unreachable!("selected winner is within the case count"),
            }
        }
    }
}
fn make_callback() -> std::sync::Arc<
    dyn Fn(
    ) -> std::pin::Pin<
            Box<dyn Future<Output = terrane_int_support::Int> + Send>,
        > + Send + Sync,
> {
    let callback: std::sync::Arc<
        dyn Fn(
        ) -> std::pin::Pin<
                Box<dyn Future<Output = terrane_int_support::Int> + Send>,
            > + Send + Sync,
    >;
    callback = {
        std::sync::Arc::new(move || -> std::pin::Pin<
            Box<dyn Future<Output = terrane_int_support::Int> + Send>,
        > {
            Box::pin(async move {
                let mut __terrane_select_cursor_318 = 0usize;
                let value: terrane_int_support::Int;
                let other: terrane_int_support::Int;
                {
                    let mut __terrane_select_guard_318 = __terrane_finally_guard();
                    let __terrane_select_control_318_0 = __terrane_select_control();
                    let mut __terrane_select_future_318_0 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_318_0
                        .clone(), work())
                    );
                    let mut __terrane_select_result_318_0 = None;
                    let __terrane_select_control_318_1 = __terrane_select_control();
                    let mut __terrane_select_future_318_1 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_318_1
                        .clone(), work())
                    );
                    let mut __terrane_select_result_318_1 = None;
                    let __terrane_select_winner_318 = std::future::poll_fn(|
                            __terrane_select_context|
                        {
                            if __terrane_cancellation_is_requested() {
                                return std::task::Poll::Ready(usize::MAX);
                            }
                            for __terrane_select_offset in 0..2usize {
                                let __terrane_select_candidate = (__terrane_select_cursor_318
                                    + __terrane_select_offset) % 2usize;
                                match __terrane_select_candidate {
                                    0 => {
                                        match Future::poll(
                                            __terrane_select_future_318_0.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_318_0 = Some(
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
                                            __terrane_select_future_318_1.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_318_1 = Some(
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
                    if __terrane_select_winner_318 == usize::MAX {
                        __terrane_select_control_318_1.request_cancel();
                        __terrane_select_control_318_0.request_cancel();
                        let _ = __terrane_select_future_318_1.as_mut().await;
                        let _ = __terrane_select_future_318_0.as_mut().await;
                        __terrane_wait_projected_cleanups().await;
                        __terrane_select_guard_318.finish();
                        __terrane_finish_cancelled_select(__terrane_select_guard_318)
                            .await;
                    }
                    __terrane_select_cursor_318 = (__terrane_select_winner_318 + 1usize)
                        % 2usize;
                    match __terrane_select_winner_318 {
                        0 => {
                            __terrane_select_control_318_1.request_cancel();
                            let _ = __terrane_select_future_318_1.as_mut().await;
                        }
                        1 => {
                            __terrane_select_control_318_0.request_cancel();
                            let _ = __terrane_select_future_318_0.as_mut().await;
                        }
                        _ => unreachable!("selected winner is within the case count"),
                    }
                    __terrane_wait_projected_cleanups().await;
                    __terrane_select_guard_318.finish();
                    match __terrane_select_winner_318 {
                        0 => {
                            value = __terrane_select_result_318_0
                                .take()
                                .expect("selected case owns its ready result");
                            return value.clone();
                        }
                        1 => {
                            other = __terrane_select_result_318_1
                                .take()
                                .expect("selected case owns its ready result");
                            return other.clone();
                        }
                        _ => unreachable!("selected winner is within the case count"),
                    }
                }
            })
        })
    };
    return callback.clone();
}
fn main() {
    __terrane_run(async move {
        let callback: std::sync::Arc<
            dyn Fn(
            ) -> std::pin::Pin<
                    Box<dyn Future<Output = terrane_int_support::Int> + Send>,
                > + Send + Sync,
        >;
        let value: Selector;
        callback = make_callback();
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(callback()).
            await)
        );
        value = Selector::terrane_construct();
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await((&value).choose())
            . await)
        );
    });
}
