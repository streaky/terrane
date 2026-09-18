// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support
// Source: src/main.trn
// Namespace: app
fn main() {}
// Source: dependencies/example/library/src/library.trn
// Namespace: example/library
static __TERRANE_F1_ANSWER: std::sync::LazyLock<terrane_int_support::Int> = std::sync::LazyLock::new(||
terrane_int_support::Int::from(42_i128));
