// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let value: witness::Opaque<String>;
    value = __terrane_raised(
        make_text(),
        0 /* terrane-site: src/main.trn:6:13-6:23 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(consume_text(value
        .clone()), 1 /* terrane-site: src/main.trn:7:13-7:32 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(consume_text(value),
        2 /* terrane-site: src/main.trn:8:13-8:32 */))
    );
}
