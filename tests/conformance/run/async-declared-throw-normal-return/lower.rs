// Generated deterministically by Terrane <version>.
// Runtime support: async.rs, executor_parallel.rs
// Vendored support crates: terrane-scalar-support
// Source: case.trn
// Namespace: sample
async fn quiet() {
    let observed_terrane_f0_s131: i64;
    observed_terrane_f0_s131 = 42;
    let _ = &observed_terrane_f0_s131;
    return ();
}
fn main() {
    __terrane_run(async move {
        let ignored_terrane_f0_s187: ();
        ignored_terrane_f0_s187 = __terrane_await(quiet()).await;
        let _ = &ignored_terrane_f0_s187;
        println!("{}", terrane_scalar_support::scalar_text(&String::from("completed")));
    });
}
