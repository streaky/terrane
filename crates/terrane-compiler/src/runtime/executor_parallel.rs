fn __terrane_run<F: Future>(future: F) -> F::Output {
    let live = std::sync::Arc::new(TerraneRuntimeLiveness {
        active: std::sync::atomic::AtomicUsize::new(0),
        drain: std::sync::Mutex::new(()),
        changed: std::sync::Condvar::new(),
    });
    let worker_live = live.clone();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .on_thread_start(move || {
            TERRANE_RUNTIME_LIVE.with(|current| {
                *current.borrow_mut() =
                    Some((tokio::runtime::Handle::current().id(), worker_live.clone()));
            });
        })
        .enable_all()
        .build()
        .expect("Terrane async runtime must initialize");
    // Declared after runtime so both normal return and unwinding mark it dead
    // before Runtime::drop cancels tasks and shuts down its drivers.
    let _liveness = __terrane_runtime_liveness_enter(&runtime, live);
    let future = Box::pin(future);
    runtime.block_on(async move {
        let output = __terrane_async_entry(future).await;
        __terrane_wait_projected_cleanups().await;
        output
    })
}
