// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: global-closure-read
fn main() {
    let f: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    f = {
        std::sync::Arc::new(move || -> terrane_int_support::Int {
            return __TERRANE_GLOBAL_CALLS
                .lock()
                .expect("program-global lock poisoned")
                .clone()
                .unwrap_or_else(|| __terrane_uninitialized_binding(
                    "calls",
                    "case.trn",
                    5,
                    16,
                ))
                .clone();
        })
    };
    println!("{}", terrane_scalar_support::scalar_text(&f()));
}
