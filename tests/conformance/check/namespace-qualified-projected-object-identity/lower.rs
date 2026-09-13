// Generated deterministically by Terrane <version>.
// Runtime support: async.rs, async_dependency.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-string-support
// Source: src/main.trn
// Namespace: app
fn cross_async(value: TerraneNs4Deps7ReqwestResponse) {
    let _ = &value;
    return ();
}
fn cross_blocking(value: TerraneNs4Deps7Reqwest8BlockingResponse) {
    let _ = &value;
    return ();
}
fn keep_crossings(
    async_crossing: std::sync::Arc<
        dyn Fn(TerraneNs4Deps7ReqwestResponse) -> () + Send + Sync,
    >,
    blocking_crossing: std::sync::Arc<
        dyn Fn(TerraneNs4Deps7Reqwest8BlockingResponse) -> () + Send + Sync,
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
// Source: <terrane>/projected/deps/reqwest.trn
// Namespace: deps/reqwest
pub use reqwest::Upgraded;
pub use reqwest::Response as TerraneNs4Deps7ReqwestResponse;
// Source: <terrane>/projected/deps/reqwest/blocking.trn
// Namespace: deps/reqwest/blocking
pub use reqwest::blocking::Response as TerraneNs4Deps7Reqwest8BlockingResponse;
