async fn __terrane_cancellable<F: Future>(
    future: F,
    cancelled: impl Fn() -> bool,
) -> Option<F::Output> {
    let mut future = std::pin::pin!(future);
    loop {
        if cancelled() {
            return None;
        }
        tokio::select! {
            output = &mut future => return Some(output),
            () = tokio::time::sleep(std::time::Duration::from_millis(1)) => {}
        }
    }
}
