// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: global-closure-after-write
fn main() {
    let f: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    {
        let value = terrane_int_support::Int::from(5_i128);
        *__TERRANE_GLOBAL_CALLS.lock().expect("program-global lock poisoned") = Some(
            value,
        );
    }
    f = {
        std::sync::Arc::new(move || -> terrane_int_support::Int {
            return __TERRANE_GLOBAL_CALLS
                .lock()
                .expect("program-global lock poisoned")
                .clone()
                .unwrap_or_else(|| __terrane_uninitialized_binding(
                    "calls",
                    "case.trn",
                    6,
                    16,
                ))
                .clone();
        })
    };
    println!("{}", terrane_scalar_support::scalar_text(&f()));
}
