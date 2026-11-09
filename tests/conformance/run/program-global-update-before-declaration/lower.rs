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
                4,
                20,
            ))
            .clone() + terrane_int_support::Int::from(1_i128);
        *__TERRANE_GLOBAL_CALLS.lock().expect("program-global lock poisoned") = Some(
            value,
        );
    }
}
fn bump_typed() {
    {
        let value = __TERRANE_GLOBAL_CALLS
            .lock()
            .expect("program-global lock poisoned")
            .clone()
            .unwrap_or_else(|| __terrane_uninitialized_binding(
                "calls",
                "main.trn",
                7,
                24,
            ))
            .clone() + terrane_int_support::Int::from(1_i128);
        *__TERRANE_GLOBAL_CALLS.lock().expect("program-global lock poisoned") = Some(
            value,
        );
    }
}
// Source: z/main.trn
// Namespace: z
fn main() {
    bump();
    bump_typed();
    println!(
        "{}", terrane_scalar_support::scalar_text(&__TERRANE_GLOBAL_CALLS.lock()
        .expect("program-global lock poisoned").clone().unwrap_or_else(| |
        __terrane_uninitialized_binding("calls", "main.trn", 10, 12)))
    );
}
