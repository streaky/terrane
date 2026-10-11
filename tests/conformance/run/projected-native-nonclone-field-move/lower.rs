// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let value: Payload;
    let items: terrane_collection_support::List<NonClone>;
    value = __terrane_raised(
        make_value(),
        0 /* terrane-site: src/main.trn:6:13-6:24 */,
    );
    items = terrane_collection_support::List::new(value.0);
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(items
        .length()))
    );
}
