// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn identity(value: terrane_int_support::Int) -> terrane_int_support::Int {
    return value.clone();
}
async fn identity_async(value: terrane_int_support::Int) -> terrane_int_support::Int {
    return value.clone();
}
fn main() {
    __terrane_run(async move {
        let values: terrane_collection_support::List<terrane_int_support::Int>;
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
            terrane_callback_witness::open_callback(match | | -> Result < _, crate
            ::TerraneForeignError > { Ok(terrane_int_support::coerce:: < i64 >
            (&terrane_int_support::Int::from(1_i128)).map_err(| error | crate
            ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
            ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
            std::panic::panic_any(error) }, match | | -> Result < _, crate
            ::TerraneForeignError > { Ok({ let callback = std::sync::Arc::new(identity)
            .clone(); move | callback_argument_0 : i64 | { match | | -> Result < _, crate
            ::TerraneForeignError > { let callback_value =
            callback(terrane_int_support::Int::from(i128::from(callback_argument_0)));
            Ok(terrane_int_support::coerce:: < i64 > (&callback_value).map_err(| error |
            crate ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
            ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
            std::panic::panic_any(error.0) } } }) } () { Ok(value) => value, Err(error)
            => std::panic::panic_any(error) }))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "terrane-callback-witness",
            "terrane_callback_witness::open_callback")) }, 0 /* terrane-site: src/main.trn:9:13-9:39 */))
        );
        values = terrane_collection_support::List::<
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_int_support::Int::from(2_i128),
                terrane_int_support::Int::from(3_i128)
            ],
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
            terrane_callback_witness::render_many(match | | -> Result < _, crate
            ::TerraneForeignError > { Ok(values.into_vec().into_iter().map(| item | ->
            Result < _, crate ::TerraneForeignError > { Ok(terrane_int_support::coerce::
            < i64 > (&item).map_err(| error | crate ::TerraneForeignError(crate
            ::TerraneRaised::raised(error, crate ::TERRANE_NO_SITE))) ?) }).collect:: <
            Result < std::vec::Vec < i64 >, _ > > () ?) } () { Ok(value) => value,
            Err(error) => std::panic::panic_any(error) }))) { Ok(value) => Ok(value),
            Err(payload) => Err(crate ::__terrane_dependency_panic(payload,
            "terrane-callback-witness", "terrane_callback_witness::render_many")) },
            1 /* terrane-site: src/main.trn:11:13-11:32 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await({
            let __terrane_future = { let __terrane_call =
            terrane_callback_witness::open_async(match | | -> Result < _, crate
            ::TerraneForeignError > { Ok(terrane_int_support::coerce:: < i64 >
            (&terrane_int_support::Int::from(4_i128)).map_err(| error | crate
            ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
            ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
            std::panic::panic_any(error) }, match | | -> Result < _, crate
            ::TerraneForeignError > { Ok({ let callback = std::sync::Arc::new(move |
            argument_0 : terrane_int_support::Int | -> std::pin::Pin < Box < dyn Future <
            Output = _ > + Send > > { Box::pin(identity_async(argument_0)) }).clone();
            move | callback_argument_0 : i64 | { let callback = callback.clone(); let
            callback_future =
            callback(terrane_int_support::Int::from(i128::from(callback_argument_0)));
            Box::pin(async move { match async { let callback_value = callback_future.
            await; Ok:: < _, crate ::TerraneForeignError > (terrane_int_support::coerce::
            < i64 > (&callback_value).map_err(| error | crate ::TerraneForeignError(crate
            ::TerraneRaised::raised(error, crate ::TERRANE_NO_SITE))) ?) } . await {
            Ok(value) => value, Err(error) => std::panic::panic_any(error.0) } }) } }) }
            () { Ok(value) => value, Err(error) => std::panic::panic_any(error) }); async
            move { match crate ::__terrane_dependency_await_unwind(__terrane_call). await
            { Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
            Err(payload) => Err(crate ::__terrane_dependency_panic(payload,
            "terrane-callback-witness", "terrane_callback_witness::open_async")) } } };
            async move { __terrane_raised_err(__terrane_future. await,
            2 /* terrane-site: src/main.trn:13:19-13:48 */) } }). await,
            3 /* terrane-site: src/main.trn:13:18-13:49 */))
        );
    });
}
