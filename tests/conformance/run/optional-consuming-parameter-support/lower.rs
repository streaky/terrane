// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: optional-consuming-parameter-support
fn absent(
    callback: Option<TerraneConsumingCallable<(), terrane_int_support::Int>>,
) -> bool {
    return (&callback).is_none();
}
fn main() {
    println!("{}", terrane_scalar_support::scalar_text(&absent(None)));
}
