// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: closure-semantic-captures
fn apply(
    callback: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int,
    >,
    value: terrane_int_support::Int,
) -> terrane_int_support::Int {
    return callback(value.clone());
}
fn main() {
    let outer: i64 = 10;
    let callback: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int,
    > = {
        std::sync::Arc::new(move |
            outer: terrane_int_support::Int,
        | -> terrane_int_support::Int {
            let local: terrane_int_support::Int = outer.clone()
                + terrane_int_support::Int::from(1_i128);
            return local.clone();
        })
    };
    let result: terrane_int_support::Int = apply(
        callback.clone(),
        terrane_int_support::Int::from(2_i128),
    );
    println!("{}", terrane_scalar_support::scalar_text(& (result)));
    println!("{}", terrane_scalar_support::scalar_text(& (outer)));
}
