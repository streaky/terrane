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
// Namespace: heterogeneous-select
async fn number_task() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(7_i128);
}
async fn text_task() -> String {
    return String::from("text");
}
fn main() {
    __terrane_run(async move {
        let mut __terrane_select_cursor_181 = 0usize;
        let mut count: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
        while count.clone() < terrane_int_support::Int::from(4_i128) {
            let mut __terrane_select_future_181_0 = std::pin::pin!(number_task());
            let mut __terrane_select_result_181_0 = None;
            let mut __terrane_select_future_181_1 = std::pin::pin!(text_task());
            let mut __terrane_select_result_181_1 = None;
            let __terrane_select_winner_181 = std::future::poll_fn(|
                    __terrane_select_context|
                {
                    for __terrane_select_offset in 0..2usize {
                        let __terrane_select_candidate = (__terrane_select_cursor_181
                            + __terrane_select_offset) % 2usize;
                        match __terrane_select_candidate {
                            0 => {
                                if let std::task::Poll::Ready(__terrane_select_value) = Future::poll(
                                    __terrane_select_future_181_0.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    __terrane_select_result_181_0 = Some(
                                        __terrane_select_value,
                                    );
                                    return std::task::Poll::Ready(0usize);
                                }
                            }
                            1 => {
                                if let std::task::Poll::Ready(__terrane_select_value) = Future::poll(
                                    __terrane_select_future_181_1.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    __terrane_select_result_181_1 = Some(
                                        __terrane_select_value,
                                    );
                                    return std::task::Poll::Ready(1usize);
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
            __terrane_select_cursor_181 = (__terrane_select_winner_181 + 1usize)
                % 2usize;
            drop(__terrane_select_future_181_1);
            drop(__terrane_select_future_181_0);
            match __terrane_select_winner_181 {
                0 => {
                    let number: terrane_int_support::Int = __terrane_select_result_181_0
                        .take()
                        .expect("selected case owns its ready result");
                    println!("{}", terrane_scalar_support::scalar_text(&number));
                }
                1 => {
                    let text: String = __terrane_select_result_181_1
                        .take()
                        .expect("selected case owns its ready result");
                    println!("{}", terrane_scalar_support::scalar_text(&text));
                }
                _ => unreachable!("selected winner is within the case count"),
            }
            count = count.clone() + terrane_int_support::Int::from(1_i128);
        }
    });
}
