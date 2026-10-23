// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let values: terrane_collection_support::List<Widget>;
    values = terrane_collection_support::List::<
        Widget,
    >::new(
        vec![
            __terrane_raised(widget(), 0 /* terrane-site: src/main.trn:5:21-5:28 */),
            __terrane_raised(widget(), 1 /* terrane-site: src/main.trn:5:32-5:39 */)
        ],
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        terrane_render_witness::render_many(values.into_vec()))) { Ok(value) =>
        Ok(terrane_int_support::Int::from_u128(value as u128)), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "dependency",
        "dependency::ForeignType::render_many")) }, 2 /* terrane-site: src/main.trn:6:13-6:32 */))
    );
}
