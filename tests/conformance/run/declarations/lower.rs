// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: declarations
fn main() {
    let local_value: i8;
    let mut block_value: Option<i64> = None;
    let else_value: i64;
    local_value = 16;
    println!(
        "{}", terrane_scalar_support::scalar_text(&__TERRANE_GLOBAL_NAMESPACE_VALUE
        .lock().expect("program-global lock poisoned").clone().unwrap_or_else(| |
        __terrane_uninitialized_binding("namespace-value", "case.trn", 6, 10)))
    );
    println!("{}", terrane_scalar_support::scalar_text(&local_value));
    if local_value > 10 {
        let _ = block_value.insert(300);
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&block_value.as_ref().unwrap_or_else(|
        | __terrane_uninitialized_binding("block-value", "case.trn", 10, 10)))
    );
    if local_value < 0 {
        else_value = 100;
    } else {
        else_value = 400;
    }
    println!("{}", terrane_scalar_support::scalar_text(&else_value));
}
