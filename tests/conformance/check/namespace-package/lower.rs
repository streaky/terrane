// Generated deterministically by Terrane <version>.
// Source: app/main.trn
// Namespace: app
fn main() {
    println!("{}", String::from(" namespace package"));
}
// Source: app/support/support.trn
// Namespace: app/support
static __TERRANE_F1_VALUE: std::sync::LazyLock<terrane_int_support::Int> = std::sync::LazyLock::new(||
terrane_int_support::Int::from(1_i128));
