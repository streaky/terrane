pub fn emit_log_event() {
    log::warn!(target: "dependency.log", item = 3_u64; "cache miss");
}

pub fn emit_tracing_event() {
    let span = tracing::info_span!("load_batch", batch = 11_u64);
    let _guard = span.enter();
    tracing::info!(target: "dependency.trace", item = 7_u64, "loaded");
}
