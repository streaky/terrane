// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(upstream_value(),
        0 /* terrane-site: src/main.trn:6:11-6:26 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(overlay_value(),
        1 /* terrane-site: src/main.trn:7:11-7:25 */))
    );
}
