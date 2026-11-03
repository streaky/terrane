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
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        values; __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let
        value = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result
        < _, crate ::TerraneForeignError > {
        Ok(terrane_render_witness::render_many(__terrane_conversion_input_0.into_vec()))
        })) { Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane-render-witness",
        "terrane_render_witness::render_many")) }; Ok:: < _, crate ::TerraneForeignError
        > (terrane_int_support::Int::from_u128(value as u128)) } } (),
        2 /* terrane-site: src/main.trn:6:13-6:32 */) })
    );
}
