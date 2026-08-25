// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: function-values-closures
fn apply(
    callback: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int,
    >,
    value: terrane_int_support::Int,
) -> terrane_int_support::Int {
    return callback(value.clone());
}
fn main() {
    let base: i64 = 10;
    let add: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int,
    > = {
        let base = base.clone();
        std::sync::Arc::new(move |
            value: terrane_int_support::Int,
        | -> terrane_int_support::Int {
            return terrane_int_support::Int::from(base as i128) + value.clone();
        })
    };
    let result: terrane_int_support::Int = apply(
        add.clone(),
        terrane_int_support::Int::from(5_i128),
    );
    println!("{}", terrane_scalar_support::scalar_text(&base));
    println!("{}", terrane_scalar_support::scalar_text(&result));
}
