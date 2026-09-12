fn __terrane_run<F: Future>(future: F) -> F::Output {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Terrane async runtime must initialize");
    tokio::task::LocalSet::new().block_on(&runtime, async move {
        let output = future.await;
        __terrane_wait_projected_cleanups().await;
        output
    })
}
