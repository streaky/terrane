// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn inspect(value: &Event) -> terrane_int_support::Int {
    return {
        let __terrane_snapshot = terrane_int_support::Int::from(
                i128::from(*&value.value),
            )
            .clone();
        __terrane_snapshot
    } + terrane_int_support::Int::from(2_i128);
}
fn main() {
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        std::sync::Arc::new(inspect);
        __terrane_raised(invoke_good(__terrane_conversion_input_0, &{ let
        __terrane_conversion_input_0 = terrane_int_support::Int::from(5_i128);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > {
        Ok(terrane_owning_review_witness::Event { value : terrane_int_support::coerce:: <
        i64 > (&__terrane_conversion_input_0).map_err(| error | crate
        ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ? }) } (), 0 /* terrane-site: src/main.trn:6:36-6:61 */)
        }), 1 /* terrane-site: src/main.trn:6:13-6:62 */) })
    );
}
