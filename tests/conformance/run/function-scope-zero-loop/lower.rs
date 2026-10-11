// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: function-scope-zero-loop
fn value(mut choose: bool) -> terrane_int_support::Int {
    let mut x: Option<i64> = None;
    while choose {
        let _ = x.insert(3);
        choose = false;
    }
    return terrane_int_support::Int::from(
        x
            .as_ref()
            .unwrap_or_else(|| __terrane_uninitialized_binding("x", "case.trn", 6, 12))
            .clone() as i128,
    );
}
fn main() {
    println!("{}", terrane_scalar_support::scalar_text(&value(true)));
}
