// Generated deterministically by Terrane <version>.
// Runtime support: async.rs, executor_parallel.rs
// Vendored support crates: terrane-scalar-support
// Source: case.trn
// Namespace: sample
async fn quiet() {
    let observed: i64 = 42;
    let _ = &observed;
    return ();
}
fn main() {
    __terrane_run(async move {
        let ignored: () = __terrane_await(quiet()).await;
        let _ = &ignored;
        println!("{}", terrane_scalar_support::scalar_text(&String::from("completed")));
    });
}
