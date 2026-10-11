// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: global-initialization-function-read
fn add_one() -> terrane_int_support::Int {
    return __TERRANE_GLOBAL_SEED
        .lock()
        .expect("program-global lock poisoned")
        .clone()
        .unwrap_or_else(|| __terrane_uninitialized_binding("seed", "case.trn", 5, 10))
        .clone() + terrane_int_support::Int::from(1_i128);
}
fn main() {
    println!(
        "{}", terrane_scalar_support::scalar_text(&__TERRANE_GLOBAL_RESULT.lock()
        .expect("program-global lock poisoned").clone().unwrap_or_else(| |
        __terrane_uninitialized_binding("result", "case.trn", 7, 10)))
    );
}
