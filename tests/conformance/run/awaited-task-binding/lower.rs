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
        let child_terrane_f0_s95;
        let value_terrane_f0_s115: terrane_int_support::Int;
        child_terrane_f0_s95 = answer();
        value_terrane_f0_s115 = __terrane_await(child_terrane_f0_s95).await;
        println!("{}", terrane_scalar_support::scalar_text(&value_terrane_f0_s115));
    });
}
