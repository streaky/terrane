// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-scalar-support
// Source: case.trn
// Namespace: name-aliasing
static __TERRANE_F0_ORIGINAL: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| String::from(
    "namespace",
));
static __TERRANE_F0_ALIAS: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    (*__TERRANE_F0_ORIGINAL).clone()
});
fn main() {
    let local_terrane_f0_s99: String;
    let copy_terrane_f0_s120: String;
    local_terrane_f0_s99 = String::from("function");
    copy_terrane_f0_s120 = local_terrane_f0_s99;
    println!("{}", terrane_scalar_support::scalar_text(&&* __TERRANE_F0_ALIAS));
    println!("{}", terrane_scalar_support::scalar_text(&copy_terrane_f0_s120));
}
