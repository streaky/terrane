// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: function-scope-partial-typed
fn value(choose: bool) -> terrane_int_support::Int {
    let mut x: Option<i64> = None;
    if choose {
        let _ = x.insert(3);
    }
    return terrane_int_support::Int::from(
        x
            .as_ref()
            .unwrap_or_else(|| __terrane_uninitialized_binding("x", "case.trn", 5, 12))
            .clone() as i128,
    );
}
fn main() {
    println!("{}", terrane_scalar_support::scalar_text(&value(true)));
}
