// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: app
fn main() {
    {
        let value = terrane_int_support::Int::from(1_i128);
        *__TERRANE_GLOBAL_CALLS.lock().expect("program-global lock poisoned") = Some(
            value,
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__TERRANE_GLOBAL_CALLS.lock()
        .expect("program-global lock poisoned").clone().unwrap_or_else(| |
        __terrane_uninitialized_binding("calls", "case.trn", 5, 12)))
    );
}
