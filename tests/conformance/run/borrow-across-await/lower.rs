// Generated deterministically by Terrane <version>.
// Runtime support: async.rs, executor_parallel.rs
// Vendored support crates: terrane-int-support, terrane-scalar-support
// Source: case.trn
// Namespace: borrow-across-await
async fn answer() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(42_i128);
}
async fn inspect() -> terrane_int_support::Int {
    let value: terrane_int_support::Int = terrane_int_support::Int::from(7_i128);
    let observed: &terrane_int_support::Int = &value;
    let result: terrane_int_support::Int = __terrane_await(answer()).await;
    println!("{}", terrane_scalar_support::scalar_text(&observed.clone()));
    return result.clone();
}
fn main() {
    __terrane_run(async move {
        let result: terrane_int_support::Int = __terrane_await(inspect()).await;
        println!("{}", terrane_scalar_support::scalar_text(&result));
    });
}
