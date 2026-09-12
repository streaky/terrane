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
// Namespace: borrow-across-await
async fn answer() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(42_i128);
}
async fn inspect() -> terrane_int_support::Int {
    let value: terrane_int_support::Int = terrane_int_support::Int::from(7_i128);
    let observed: &terrane_int_support::Int = &value;
    let result: terrane_int_support::Int = __terrane_await(answer()).await;
    println!("{}", terrane_scalar_support::scalar_text(&observed.clone()));
    return result.clone();
}
fn main() {
    __terrane_run(async move {
        let result: terrane_int_support::Int = __terrane_await(inspect()).await;
        println!("{}", terrane_scalar_support::scalar_text(&result));
    });
}
