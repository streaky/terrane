// Generated deterministically by Terrane <version>.
// Runtime support: async.rs, executor_parallel.rs
// Vendored support crates: terrane-int-support, terrane-scalar-support
// Source: case.trn
// Namespace: async-await
async fn answer() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(42_i128);
}
fn main() {
    __terrane_run(async move {
        let value_terrane_f0_s86: terrane_int_support::Int;
        value_terrane_f0_s86 = __terrane_await(answer()).await;
        println!("{}", terrane_scalar_support::scalar_text(&value_terrane_f0_s86));
    });
}
