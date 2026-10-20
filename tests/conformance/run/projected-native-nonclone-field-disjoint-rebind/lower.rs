// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let mut value: Payload;
    let first: terrane_collection_support::List<NonClone>;
    let second: terrane_collection_support::List<NonClone>;
    let again: terrane_collection_support::List<NonClone>;
    value = __terrane_raised(
        make_value(),
        0 /* terrane-site: src/main.trn:5:13-5:24 */,
    );
    first = terrane_collection_support::List::new(value.0);
    second = terrane_collection_support::List::new(value.1);
    value = __terrane_raised(
        make_value(),
        1 /* terrane-site: src/main.trn:8:13-8:24 */,
    );
    again = terrane_collection_support::List::new(value.0);
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(first
        .length())),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(second
        .length())),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(again
        .length()))
    );
}
