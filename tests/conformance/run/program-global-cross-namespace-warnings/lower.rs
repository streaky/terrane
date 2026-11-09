// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: a/main.trn
// Namespace: a
fn bump() {
    {
        let value = __TERRANE_GLOBAL_CALLS
            .lock()
            .expect("program-global lock poisoned")
            .clone()
            .unwrap_or_else(|| __terrane_uninitialized_binding(
                "calls",
                "main.trn",
                5,
                18,
            ))
            .clone() + terrane_int_support::Int::from(1_i128);
        *__TERRANE_GLOBAL_CALLS.lock().expect("program-global lock poisoned") = Some(
            value,
        );
    }
}
// Source: b/main.trn
// Namespace: b
fn main() {
    bump();
    println!(
        "{}", terrane_scalar_support::scalar_text(&__TERRANE_GLOBAL_CALLS.lock()
        .expect("program-global lock poisoned").clone().unwrap_or_else(| |
        __terrane_uninitialized_binding("calls", "main.trn", 5, 10)))
    );
}
#[allow(dead_code)]
fn local_control() {
    let calls: i64;
    calls = 0;
    let _ = &calls;
}
