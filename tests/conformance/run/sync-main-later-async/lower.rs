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
    reason = "executor shutdown uses one hook for both simple and cancellation-aware runtimes"
)]
async fn __terrane_wait_projected_cleanups() {}
// Source: src/main.trn
// Namespace: app
#[allow(dead_code)]
async fn later() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(2_i128);
}
fn main() {
    println!("{}", terrane_scalar_support::scalar_text(&1));
}
