// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: closure-semantic-captures
fn apply(
    callback: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
    >,
    value: terrane_int_support::Int,
) -> terrane_int_support::Int {
    return callback(value.clone());
}
fn main() {
    let outer: i64;
    let callback: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
    >;
    let result: terrane_int_support::Int;
    outer = 10;
    callback = {
        std::sync::Arc::new(move |
            outer_2: terrane_int_support::Int,
        | -> terrane_int_support::Int {
            let local: terrane_int_support::Int;
            local = outer_2.clone() + terrane_int_support::Int::from(1_i128);
            return local.clone();
        })
    };
    result = apply(callback.clone(), terrane_int_support::Int::from(2_i128));
    println!("{}", terrane_scalar_support::scalar_text(&result));
    println!("{}", terrane_scalar_support::scalar_text(&outer));
}
