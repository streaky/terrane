// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: optional-mutable-result-support
fn empty() -> Option<TerraneMutableCallable<(), terrane_int_support::Int>> {
    return None;
}
fn main() {
    let callback: Option<TerraneMutableCallable<(), terrane_int_support::Int>>;
    callback = empty();
    if (&callback).is_none() {
        println!("{}", terrane_scalar_support::scalar_text(&0));
    }
}
