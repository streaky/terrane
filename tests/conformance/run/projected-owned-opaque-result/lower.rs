// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn identity(value: terrane_int_support::Int) -> terrane_int_support::Int {
    return value.clone();
}
fn main() {
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > {
        Ok((&terrane_opaque_result_witness::make(terrane_int_support::coerce:: < u32 >
        (&terrane_int_support::Int::from(42_i128)).map_err(| error | crate
        ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ?)).value()) })) { Ok(result) => result ?, Err(payload) =>
        return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_opaque_result_witness",
        "terrane_opaque_result_witness::Wrapper::value")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 0 /* terrane-site: src/main.trn:8:13-8:30 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > {
        Ok(terrane_opaque_result_witness::consume(terrane_opaque_result_witness::make(terrane_int_support::coerce::
        < u32 > (&terrane_int_support::Int::from(43_i128)).map_err(| error | crate
        ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ?))) })) { Ok(result) => result ?, Err(payload) => return
        Err(crate ::__terrane_dependency_panic(payload, "terrane-opaque-result-witness",
        "terrane_opaque_result_witness::consume")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 1 /* terrane-site: src/main.trn:9:13-9:32 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 = {
        let __terrane_conversion_input_0 = std::sync::Arc::new(identity);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(terrane_opaque_result_witness::choose({ let
        callback = __terrane_conversion_input_0; move | callback_argument_0 : i64 | {
        match | | -> Result < _, crate ::TerraneForeignError > { let callback_value =
        callback(terrane_int_support::Int::from(i128::from(callback_argument_0)));
        Ok(terrane_int_support::coerce:: < i64 > (&callback_value).map_err(| error |
        crate ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error.0) } } })) })) { Ok(result) => result ?, Err(payload)
        => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-opaque-result-witness", "terrane_opaque_result_witness::choose")) };
        Ok:: < _, crate ::TerraneForeignError > (value) } } (), 2 /* terrane-site: src/main.trn:10:21-10:37 */) }; __terrane_raised(| | -> Result < _, crate
        ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > {
        Ok(terrane_opaque_result_witness::serve(__terrane_conversion_input_0)) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane-opaque-result-witness",
        "terrane_opaque_result_witness::serve")) }; Ok:: < _, crate ::TerraneForeignError
        > (terrane_int_support::Int::from(i128::from(value))) } } (),
        3 /* terrane-site: src/main.trn:10:13-10:38 */) })
    );
}
