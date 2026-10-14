// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn identity(value: terrane_int_support::Int) -> terrane_int_support::Int {
    return value.clone();
}
fn main() {
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        (&terrane_opaque_result_witness::make(match | | -> Result < _, crate
        ::TerraneForeignError > { Ok(terrane_int_support::coerce:: < u32 >
        (&terrane_int_support::Int::from(42_i128)).map_err(| error | crate
        ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error) })).value())) { Ok(value) =>
        Ok(terrane_int_support::Int::from_u128(value as u128)), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "terrane_opaque_result_witness",
        "terrane_opaque_result_witness::Wrapper<_>::value")) }, 0 /* terrane-site: src/main.trn:8:13-8:30 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        terrane_opaque_result_witness::make(match | | -> Result < _, crate
        ::TerraneForeignError > { Ok(terrane_int_support::coerce:: < u32 >
        (&terrane_int_support::Int::from(43_i128)).map_err(| error | crate
        ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error) }); __terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        terrane_opaque_result_witness::consume(__terrane_native_input_0))) { Ok(value) =>
        Ok(terrane_int_support::Int::from_u128(value as u128)), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "terrane-opaque-result-witness",
        "terrane_opaque_result_witness::consume")) }, 1 /* terrane-site: src/main.trn:9:13-9:32 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        terrane_opaque_result_witness::choose(match | | -> Result < _, crate
        ::TerraneForeignError > { Ok({ let callback = std::sync::Arc::new(identity)
        .clone(); move | callback_argument_0 : i64 | { match | | -> Result < _, crate
        ::TerraneForeignError > { let callback_value =
        callback(terrane_int_support::Int::from(i128::from(callback_argument_0)));
        Ok(terrane_int_support::coerce:: < i64 > (&callback_value).map_err(| error |
        crate ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error.0) } } }) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error) }))) { Ok(value) => Ok(value), Err(payload) =>
        Err(crate ::__terrane_dependency_panic(payload, "terrane-opaque-result-witness",
        "terrane_opaque_result_witness::choose")) }, 2 /* terrane-site: src/main.trn:10:21-10:37 */); __terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        terrane_opaque_result_witness::serve(__terrane_native_input_0))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "terrane-opaque-result-witness",
        "terrane_opaque_result_witness::serve")) }, 3 /* terrane-site: src/main.trn:10:13-10:38 */) })
    );
}
