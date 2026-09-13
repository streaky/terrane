#[derive(Clone)]
struct TerraneSelectControl {
    requested: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl TerraneSelectControl {
    fn request_cancel(&self) {
        self.requested
            .store(true, std::sync::atomic::Ordering::Release);
    }

    fn is_cancelled(&self) -> bool {
        self.requested
            .load(std::sync::atomic::Ordering::Acquire)
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
