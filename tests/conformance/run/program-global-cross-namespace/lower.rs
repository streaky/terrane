// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: b/main.trn
// Namespace: b
fn main() {
    let value: terrane_int_support::Int;
    let typed: terrane_int_support::Int;
    println!(
        "{}", terrane_scalar_support::scalar_text(&__TERRANE_GLOBAL_CALLS.lock()
        .expect("program-global lock poisoned").clone().unwrap_or_else(| |
        __terrane_uninitialized_binding("calls", "main.trn", 3, 10)))
    );
    value = __TERRANE_GLOBAL_CALLS
        .lock()
        .expect("program-global lock poisoned")
        .clone()
        .unwrap_or_else(|| __terrane_uninitialized_binding("calls", "main.trn", 4, 11))
        .clone();
    println!("{}", terrane_scalar_support::scalar_text(&value));
    typed = __TERRANE_GLOBAL_CALLS
        .lock()
        .expect("program-global lock poisoned")
        .clone()
        .unwrap_or_else(|| __terrane_uninitialized_binding("calls", "main.trn", 6, 15))
        .clone();
    println!("{}", terrane_scalar_support::scalar_text(&typed));
    {
        let value = __TERRANE_GLOBAL_CALLS
            .lock()
            .expect("program-global lock poisoned")
            .clone()
            .unwrap_or_else(|| __terrane_uninitialized_binding(
                "calls",
                "main.trn",
                8,
                18,
            ))
            .clone() + terrane_int_support::Int::from(1_i128);
        *__TERRANE_GLOBAL_CALLS.lock().expect("program-global lock poisoned") = Some(
            value,
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__TERRANE_GLOBAL_CALLS.lock()
        .expect("program-global lock poisoned").clone().unwrap_or_else(| |
        __terrane_uninitialized_binding("calls", "main.trn", 9, 10)))
    );
}
