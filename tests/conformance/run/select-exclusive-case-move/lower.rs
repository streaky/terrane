// Generated deterministically by Terrane <version>.
async fn __terrane_await<F: Future>(future: F) -> F::Output {
    struct YieldOnce(bool);
    impl Future for YieldOnce {
        type Output = ();
        fn poll(
            mut self: std::pin::Pin<&mut Self>,
            context: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Self::Output> {
            if self.0 {
                std::task::Poll::Ready(())
            } else {
                self.0 = true;
                context.waker().wake_by_ref();
                std::task::Poll::Pending
            }
        }
    }
    YieldOnce(false).await;
    let output = future.await;
    YieldOnce(false).await;
    output
}
#[allow(
    dead_code,
    clippy::unused_async,
    reason = "executor shutdown uses one hook for both simple and cancellation-aware runtimes"
)]
async fn __terrane_wait_projected_cleanups() {}
#[derive(Clone)]
struct TerraneSelectControl {
    requested: std::sync::Arc<std::sync::atomic::AtomicBool>,
}
impl TerraneSelectControl {
    fn request_cancel(&self) {
        self.requested.store(true, std::sync::atomic::Ordering::Release);
    }
    fn is_cancelled(&self) -> bool {
        self.requested.load(std::sync::atomic::Ordering::Acquire)
    }
}
fn __terrane_select_control() -> TerraneSelectControl {
    TerraneSelectControl {
        requested: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
    }
}
async fn __terrane_select_operation<F: Future>(
    control: TerraneSelectControl,
    future: F,
) -> Option<F::Output> {
    let mut future = std::pin::pin!(future);
    std::future::poll_fn(move |cx| {
            if control.is_cancelled() {
                return std::task::Poll::Ready(None);
            }
            Future::poll(future.as_mut(), cx).map(Some)
        })
        .await
}
struct TerraneFinallyGuard;
impl TerraneFinallyGuard {
    fn finish(&mut self) {}
}
fn __terrane_finally_guard() -> TerraneFinallyGuard {
    TerraneFinallyGuard
}
fn __terrane_cancellation_is_requested() -> bool {
    false
}
async fn __terrane_finish_cancelled_select(_: TerraneFinallyGuard) -> ! {
    std::future::pending().await
}
fn __terrane_run<F: Future>(future: F) -> F::Output {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Terrane async runtime must initialize")
        .block_on(async move {
            let output = future.await;
            __terrane_wait_projected_cleanups().await;
            output
        })
}
// Source: case.trn
// Namespace: select-exclusive-case-move
async fn one() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(1_i128);
}
async fn two() -> String {
    return String::from("two");
}
fn main() {
    __terrane_run(async move {
        let mut __terrane_select_cursor_157 = 0usize;
        let pending = one();
        {
            let mut __terrane_select_guard_157 = __terrane_finally_guard();
            let __terrane_select_control_157_0 = __terrane_select_control();
            let mut __terrane_select_future_157_0 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_157_0.clone(), one())
            );
            let mut __terrane_select_result_157_0 = None;
            let __terrane_select_control_157_1 = __terrane_select_control();
            let mut __terrane_select_future_157_1 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_157_1.clone(), two())
            );
            let mut __terrane_select_result_157_1 = None;
            let __terrane_select_winner_157 = std::future::poll_fn(|
                    __terrane_select_context|
                {
                    if __terrane_cancellation_is_requested() {
                        return std::task::Poll::Ready(usize::MAX);
                    }
                    for __terrane_select_offset in 0..2usize {
                        let __terrane_select_candidate = (__terrane_select_cursor_157
                            + __terrane_select_offset) % 2usize;
                        match __terrane_select_candidate {
                            0 => {
                                match Future::poll(
                                    __terrane_select_future_157_0.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_157_0 = Some(
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
                                    __terrane_select_future_157_1.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_157_1 = Some(
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
            if __terrane_select_winner_157 == usize::MAX {
                __terrane_select_control_157_1.request_cancel();
                __terrane_select_control_157_0.request_cancel();
                let _ = __terrane_select_future_157_1.as_mut().await;
                let _ = __terrane_select_future_157_0.as_mut().await;
                __terrane_wait_projected_cleanups().await;
                __terrane_select_guard_157.finish();
                __terrane_finish_cancelled_select(__terrane_select_guard_157).await;
            }
            __terrane_select_cursor_157 = (__terrane_select_winner_157 + 1usize)
                % 2usize;
            match __terrane_select_winner_157 {
                0 => {
                    __terrane_select_control_157_1.request_cancel();
                    let _ = __terrane_select_future_157_1.as_mut().await;
                }
                1 => {
                    __terrane_select_control_157_0.request_cancel();
                    let _ = __terrane_select_future_157_0.as_mut().await;
                }
                _ => unreachable!("selected winner is within the case count"),
            }
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_157.finish();
            match __terrane_select_winner_157 {
                0 => {
                    let value: terrane_int_support::Int = __terrane_select_result_157_0
                        .take()
                        .expect("selected case owns its ready result");
                    println!(
                        "{}{}", terrane_scalar_support::scalar_text(&value),
                        terrane_scalar_support::scalar_text(&__terrane_await(pending).
                        await)
                    );
                }
                1 => {
                    let text: String = __terrane_select_result_157_1
                        .take()
                        .expect("selected case owns its ready result");
                    println!(
                        "{}{}", terrane_scalar_support::scalar_text(&text),
                        terrane_scalar_support::scalar_text(&__terrane_await(pending).
                        await)
                    );
                }
                _ => unreachable!("selected winner is within the case count"),
            }
        }
    });
}
