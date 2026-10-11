// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let value: witness::Outer<witness::CopyOnly<bool>>;
    value = __terrane_raised(nested(), 0 /* terrane-site: src/main.trn:6:41-6:48 */);
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(read(&value),
        1 /* terrane-site: src/main.trn:7:13-7:24 */))
    );
}
