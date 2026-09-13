// Generated deterministically by Terrane <version>.
// Runtime support: async.rs, executor_parallel.rs
// Vendored support crates: terrane-int-support, terrane-scalar-support
// Source: case.trn
// Namespace: awaited-task-binding
async fn answer() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(42_i128);
}
fn main() {
    __terrane_run(async move {
        let child = answer();
        let value: terrane_int_support::Int = __terrane_await(child).await;
        println!("{}", terrane_scalar_support::scalar_text(&value));
    });
}
