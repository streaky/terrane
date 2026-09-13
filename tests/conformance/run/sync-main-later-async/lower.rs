// Generated deterministically by Terrane <version>.
// Runtime support: async.rs
// Vendored support crates: terrane-int-support, terrane-scalar-support
// Source: src/main.trn
// Namespace: app
#[allow(dead_code)]
async fn later() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(2_i128);
}
fn main() {
    println!("{}", terrane_scalar_support::scalar_text(&1));
}
