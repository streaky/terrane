// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-scalar-support
// Source: src/main.trn
// Namespace: app
fn main() {
    let answer: terrane_int_support::Int = { crate::adapters::answer() };
    println!("{}", terrane_scalar_support::scalar_text(&answer));
}
mod adapters {
    pub fn answer() -> terrane_int_support::Int {
        terrane_int_support::Int::from(42_i64)
    }
}
