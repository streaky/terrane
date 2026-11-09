// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: optional-mutable-local-support
fn main() {
    let callback: Option<TerraneMutableCallable<(), terrane_int_support::Int>>;
    callback = None;
    if (&callback).is_none() {
        println!("{}", terrane_scalar_support::scalar_text(&0));
    }
}
