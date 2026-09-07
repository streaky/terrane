fn __terrane_block_on_cancellable<F: Future>(
    future: F,
    cancelled: impl Fn() -> bool,
) -> Option<F::Output> {
    #[derive(Default)]
    struct Wake {
        ready: std::sync::Mutex<bool>,
        available: std::sync::Condvar,
    }
    impl std::task::Wake for Wake {
        fn wake(self: std::sync::Arc<Self>) {
            self.wake_by_ref();
        }

        fn wake_by_ref(self: &std::sync::Arc<Self>) {
            let mut ready = self.ready.lock().expect("task wake state must lock");
            *ready = true;
            self.available.notify_one();
        }
    }
    let wake = std::sync::Arc::new(Wake::default());
    let waker = std::task::Waker::from(wake.clone());
    let mut context = std::task::Context::from_waker(&waker);
    let mut future = std::pin::pin!(future);
    loop {
        if cancelled() {
            return None;
        }
        match future.as_mut().poll(&mut context) {
            std::task::Poll::Ready(value) => return Some(value),
            std::task::Poll::Pending => {
                let mut ready = wake.ready.lock().expect("task wake state must lock");
                if !*ready {
                    let (next_ready, _) = wake
                        .available
                        .wait_timeout(ready, std::time::Duration::from_millis(1))
                        .expect("task wake wait must remain valid");
                    ready = next_ready;
                }
                *ready = false;
            }
        }
    }
}
