pub fn emit_log_event() {
    log::warn!(target: "dependency.log", item = 3_u64; "cache miss");
}

pub fn emit_tracing_event() {
    tracing::info!(target: "dependency.trace", item = 7_u64, "loaded");
}
