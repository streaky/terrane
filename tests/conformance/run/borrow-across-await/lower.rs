// Generated deterministically by Terrane <version>.
// Runtime support: async.rs, executor_parallel.rs
// Vendored support crates: terrane-int-support, terrane-scalar-support
// Source: case.trn
// Namespace: borrow-across-await
async fn answer() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(42_i128);
}
async fn inspect() -> terrane_int_support::Int {
    let value_terrane_f0_s101: terrane_int_support::Int;
    let observed_terrane_f0_s117: &terrane_int_support::Int;
    let result_terrane_f0_s148: terrane_int_support::Int;
    value_terrane_f0_s101 = terrane_int_support::Int::from(7_i128);
    observed_terrane_f0_s117 = &value_terrane_f0_s101;
    result_terrane_f0_s148 = __terrane_await(answer()).await;
    println!(
        "{}", terrane_scalar_support::scalar_text(&observed_terrane_f0_s117.clone())
    );
    return result_terrane_f0_s148.clone();
}
fn main() {
    __terrane_run(async move {
        let result_terrane_f0_s229: terrane_int_support::Int;
        result_terrane_f0_s229 = __terrane_await(inspect()).await;
        println!("{}", terrane_scalar_support::scalar_text(&result_terrane_f0_s229));
    });
}
