// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn cross_async(
    value: TerraneNsDepsReqwestResponseIdentity2F646570732F7265717765737400526573706F6E7365,
) {
    let _ = &value;
    return ();
}
fn cross_blocking(
    value: TerraneNsDepsReqwestBlockingResponseIdentity2F646570732F726571776573742F626C6F636B696E6700526573706F6E7365,
) {
    let _ = &value;
    return ();
}
fn keep_crossings(
    async_crossing: std::sync::Arc<
        dyn Fn(
            TerraneNsDepsReqwestResponseIdentity2F646570732F7265717765737400526573706F6E7365,
        ) -> () + Send + Sync,
    >,
    blocking_crossing: std::sync::Arc<
        dyn Fn(
            TerraneNsDepsReqwestBlockingResponseIdentity2F646570732F726571776573742F626C6F636B696E6700526573706F6E7365,
        ) -> () + Send + Sync,
    >,
) {
    let _ = (&async_crossing, &blocking_crossing);
    return ();
}
fn main() {
    keep_crossings(
        std::sync::Arc::new(cross_async),
        std::sync::Arc::new(cross_blocking),
    );
}
