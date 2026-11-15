// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn produce() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(42_i128);
}
fn length(value: String) -> terrane_int_support::Int {
    return terrane_int_support::Int::from(
        terrane_string_support::length(&value) as i128,
    );
}
async fn produce_async() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(42_i128);
}
async fn produce_async_one_argument(
    value: terrane_int_support::Int,
) -> terrane_int_support::Int {
    return value.clone() + terrane_int_support::Int::from(1_i128);
}
async fn produce_async_output() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(300_i128);
}
fn initial() -> String {
    return String::from("value");
}
fn initial_message() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(7_i128);
}
fn combine(state: String, message: terrane_int_support::Int) -> String {
    let _ = &message;
    return state;
}
fn no_argument() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(0_i128);
}
fn one_argument(value: terrane_int_support::Int) -> terrane_int_support::Int {
    return value.clone();
}
fn view_value(state: &terrane_int_support::Int) -> terrane_int_support::Int {
    return state.clone() + terrane_int_support::Int::from(0_i128);
}
fn main() {
    __terrane_run(async move {
        let mut router: MethodRouter;
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = std::sync::Arc::new(produce); __terrane_raised(| | -> Result < _, crate
            ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(terrane_callable_recipe_witness::invoke({
            let callback = __terrane_conversion_input_0; move | | { match | | -> Result <
            _, crate ::TerraneForeignError > { let callback_value = callback();
            Ok(terrane_int_support::coerce:: < i64 > (&callback_value).map_err(| error |
            crate ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
            ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
            std::panic::panic_any(error.0) } } })) })) { Ok(result) => result ?,
            Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-callable-recipe-witness",
            "terrane_callable_recipe_witness::invoke")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from(i128::from(value))) }
            } (), 0 /* terrane-site: src/main.trn:41:13-41:28 */) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = String::from("one"); let __terrane_conversion_input_1 =
            std::sync::Arc::new(length); __terrane_raised(| | -> Result < _, crate
            ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > {
            Ok(terrane_callable_recipe_witness::transform(__terrane_conversion_input_0, {
            let callback = __terrane_conversion_input_1; move | callback_argument_0 :
            String | { match | | -> Result < _, crate ::TerraneForeignError > { let
            callback_value = callback(callback_argument_0);
            Ok(terrane_int_support::coerce:: < i64 > (&callback_value).map_err(| error |
            crate ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
            ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
            std::panic::panic_any(error.0) } } })) })) { Ok(result) => result ?,
            Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-callable-recipe-witness",
            "terrane_callable_recipe_witness::transform")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from(i128::from(value))) }
            } (), 1 /* terrane-site: src/main.trn:42:13-42:37 */) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = String::from("two"); let __terrane_conversion_input_1 =
            std::sync::Arc::new(length); __terrane_raised(| | -> Result < _, crate
            ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > {
            Ok(terrane_callable_recipe_witness::mutate(__terrane_conversion_input_0, {
            let callback = __terrane_conversion_input_1; move | callback_argument_0 :
            String | { match | | -> Result < _, crate ::TerraneForeignError > { let
            callback_value = callback(callback_argument_0);
            Ok(terrane_int_support::coerce:: < i64 > (&callback_value).map_err(| error |
            crate ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
            ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
            std::panic::panic_any(error.0) } } })) })) { Ok(result) => result ?,
            Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-callable-recipe-witness",
            "terrane_callable_recipe_witness::mutate")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from(i128::from(value))) }
            } (), 2 /* terrane-site: src/main.trn:43:13-43:34 */) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = std::sync::Arc::new(move | | -> std::pin::Pin < Box < dyn Future < Output =
            _ > + Send > > { Box::pin(produce_async()) }); __terrane_raised(| | -> Result
            < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(terrane_callable_recipe_witness::install({
            let callback = __terrane_conversion_input_0; move | | { let callback =
            callback.clone(); let callback_future = callback(); Box::pin(async move {
            match async { let callback_value = callback_future. await; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::coerce:: < i64 >
            (&callback_value).map_err(| error | crate ::TerraneForeignError(crate
            ::TerraneRaised::raised(error, crate ::TERRANE_NO_SITE))) ?) } . await {
            Ok(value) => value, Err(error) => std::panic::panic_any(error.0) } }) } }))
            })) { Ok(result) => result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-callable-recipe-witness",
            "terrane_callable_recipe_witness::install")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 3 /* terrane-site: src/main.trn:44:13-44:35 */) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = std::sync::Arc::new(initial); let __terrane_conversion_input_1 =
            terrane_int_support::Int::from(7_i128); let __terrane_conversion_input_2 =
            std::sync::Arc::new(combine); __terrane_raised(| | -> Result < _, crate
            ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > {
            Ok(terrane_callable_recipe_witness::run_correlated({ let callback =
            __terrane_conversion_input_0; move | | { match | | -> Result < _, crate
            ::TerraneForeignError > { let callback_value = callback(); Ok(callback_value)
            } () { Ok(value) => value, Err(error) => std::panic::panic_any(error.0) } }
            }, terrane_int_support::coerce:: < i64 > (&__terrane_conversion_input_1)
            .map_err(| error | crate ::TerraneForeignError(crate
            ::TerraneRaised::raised(error, crate ::TERRANE_NO_SITE))) ?, { let callback =
            __terrane_conversion_input_2; move | callback_argument_0 : String,
            callback_argument_1 : i64 | { match | | -> Result < _, crate
            ::TerraneForeignError > { let callback_value = callback(callback_argument_0,
            terrane_int_support::Int::from(i128::from(callback_argument_1)));
            Ok(callback_value) } () { Ok(value) => value, Err(error) =>
            std::panic::panic_any(error.0) } } })) })) { Ok(result) => result ?,
            Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-callable-recipe-witness",
            "terrane_callable_recipe_witness::run_correlated")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 4 /* terrane-site: src/main.trn:45:13-45:48 */) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = std::sync::Arc::new(initial); let __terrane_conversion_input_1 =
            std::sync::Arc::new(initial_message); let __terrane_conversion_input_2 =
            std::sync::Arc::new(combine); __terrane_raised(| | -> Result < _, crate
            ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(terrane_callable_recipe_witness::compose({
            let callback = __terrane_conversion_input_0; move | | { match | | -> Result <
            _, crate ::TerraneForeignError > { let callback_value = callback();
            Ok(callback_value) } () { Ok(value) => value, Err(error) =>
            std::panic::panic_any(error.0) } } }, { let callback =
            __terrane_conversion_input_1; move | | { match | | -> Result < _, crate
            ::TerraneForeignError > { let callback_value = callback();
            Ok(terrane_int_support::coerce:: < i64 > (&callback_value).map_err(| error |
            crate ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
            ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
            std::panic::panic_any(error.0) } } }, { let callback =
            __terrane_conversion_input_2; move | callback_argument_0 : String,
            callback_argument_1 : i64 | { match | | -> Result < _, crate
            ::TerraneForeignError > { let callback_value = callback(callback_argument_0,
            terrane_int_support::Int::from(i128::from(callback_argument_1)));
            Ok(callback_value) } () { Ok(value) => value, Err(error) =>
            std::panic::panic_any(error.0) } } })) })) { Ok(result) => result ?,
            Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-callable-recipe-witness",
            "terrane_callable_recipe_witness::compose")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 5 /* terrane-site: src/main.trn:46:13-46:55 */) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = std::sync::Arc::new(no_argument); __terrane_raised(| | -> Result < _, crate
            ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > {
            Ok(terrane_callable_recipe_witness::accepts_handler({ let callback =
            __terrane_conversion_input_0; move | | { match | | -> Result < _, crate
            ::TerraneForeignError > { let callback_value = callback();
            Ok(terrane_int_support::coerce:: < i64 > (&callback_value).map_err(| error |
            crate ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
            ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
            std::panic::panic_any(error.0) } } })) })) { Ok(result) => result ?,
            Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-callable-recipe-witness",
            "terrane_callable_recipe_witness::accepts_handler")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 6 /* terrane-site: src/main.trn:47:13-47:41 */) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = std::sync::Arc::new(one_argument); __terrane_raised(| | -> Result < _,
            crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > {
            Ok(terrane_callable_recipe_witness::accepts_handler({ let callback =
            __terrane_conversion_input_0; move | callback_argument_0 : i64 | { match | |
            -> Result < _, crate ::TerraneForeignError > { let callback_value =
            callback(terrane_int_support::Int::from(i128::from(callback_argument_0)));
            Ok(terrane_int_support::coerce:: < i64 > (&callback_value).map_err(| error |
            crate ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
            ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
            std::panic::panic_any(error.0) } } })) })) { Ok(result) => result ?,
            Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-callable-recipe-witness",
            "terrane_callable_recipe_witness::accepts_handler")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 7 /* terrane-site: src/main.trn:48:13-48:42 */) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_1
            = std::sync::Arc::new(view_value); __terrane_raised(| | -> Result < _, crate
            ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(terrane_callable_recipe_witness::render::
            < i64 > (&4, { let callback = __terrane_conversion_input_1; move |
            callback_argument_0 : &'_ i64 | { match | | -> Result < _, crate
            ::TerraneForeignError > { let callback_value =
            callback(&terrane_int_support::Int::from(i128::from(* callback_argument_0)));
            Ok(terrane_int_support::coerce:: < i64 > (&callback_value).map_err(| error |
            crate ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
            ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
            std::panic::panic_any(error.0) } } })) })) { Ok(result) => result ?,
            Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-callable-recipe-witness",
            "terrane_callable_recipe_witness::render::<i64>")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from(i128::from(value))) }
            } (), 8 /* terrane-site: src/main.trn:49:13-49:34 */) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = __terrane_raised(blank(), 9 /* terrane-site: src/main.trn:50:22-50:28 */);
            __terrane_raised(accept(__terrane_conversion_input_0), 10 /* terrane-site: src/main.trn:50:13-50:29 */) })
        );
        router = __terrane_raised(
            terrane_static_trn_4d6574686f64526f75746572_new::<_, _>(),
            11 /* terrane-site: src/main.trn:51:27-51:45 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await({
            let __terrane_source_input_0 = std::sync::Arc::new(move | | -> std::pin::Pin
            < Box < dyn Future < Output = _ > + Send > > { Box::pin(produce_async()) });
            { let __terrane_future = { let __terrane_call = router
            .dispatch(__terrane_raised(| | -> Result < _, crate ::TerraneForeignError > {
            Ok({ let callback = __terrane_source_input_0; move | | { let callback =
            callback.clone(); let callback_future = callback(); Box::pin(async move {
            match async { let callback_value = callback_future. await; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::coerce:: < i64 >
            (&callback_value).map_err(| error | crate ::TerraneForeignError(crate
            ::TerraneRaised::raised(error, crate ::TERRANE_NO_SITE))) ?) } . await {
            Ok(value) => value, Err(error) => std::panic::panic_any(error.0) } }) } }) }
            (), 12 /* terrane-site: src/main.trn:52:36-52:49 */)); Box::pin(async move
            { { let value = crate ::__terrane_dependency_await_unwind(__terrane_call).
            await .map_err(| payload | crate ::__terrane_dependency_panic(payload,
            "terrane-callable-recipe-witness",
            "terrane_callable_recipe_witness::MethodRouter::dispatch")) ?; Ok:: < _,
            crate ::TerraneForeignError > (value) } }) }; async move {
            __terrane_raised_err(__terrane_future. await, 13 /* terrane-site: src/main.trn:52:19-52:49 */) } } }). await, 13 /* terrane-site: src/main.trn:52:19-52:49 */))
        );
        router = __terrane_raised(
            terrane_static_trn_4d6574686f64526f75746572_new::<_, _>(),
            14 /* terrane-site: src/main.trn:53:27-53:45 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await({
            let __terrane_source_input_0 = std::sync::Arc::new(move | | -> std::pin::Pin
            < Box < dyn Future < Output = _ > + Send > > {
            Box::pin(produce_async_output()) }); { let __terrane_future = { let
            __terrane_call = router.dispatch(__terrane_raised(| | -> Result < _, crate
            ::TerraneForeignError > { Ok({ let callback = __terrane_source_input_0; move
            | | { let callback = callback.clone(); let callback_future = callback();
            Box::pin(async move { match async { let callback_value = callback_future.
            await; Ok:: < _, crate ::TerraneForeignError > (terrane_int_support::coerce::
            < i64 > (&callback_value).map_err(| error | crate ::TerraneForeignError(crate
            ::TerraneRaised::raised(error, crate ::TERRANE_NO_SITE))) ?) } . await {
            Ok(value) => value, Err(error) => std::panic::panic_any(error.0) } }) } }) }
            (), 15 /* terrane-site: src/main.trn:54:36-54:56 */)); Box::pin(async move
            { { let value = crate ::__terrane_dependency_await_unwind(__terrane_call).
            await .map_err(| payload | crate ::__terrane_dependency_panic(payload,
            "terrane-callable-recipe-witness",
            "terrane_callable_recipe_witness::MethodRouter::dispatch")) ?; Ok:: < _,
            crate ::TerraneForeignError > (value) } }) }; async move {
            __terrane_raised_err(__terrane_future. await, 16 /* terrane-site: src/main.trn:54:19-54:56 */) } } }). await, 16 /* terrane-site: src/main.trn:54:19-54:56 */))
        );
        router = __terrane_raised(
            terrane_static_trn_4d6574686f64526f75746572_new::<_, _>(),
            17 /* terrane-site: src/main.trn:55:27-55:45 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await({
            let __terrane_source_input_0 = std::sync::Arc::new(move | argument_0 :
            terrane_int_support::Int | -> std::pin::Pin < Box < dyn Future < Output = _ >
            + Send > > { Box::pin(produce_async_one_argument(argument_0)) }); { let
            __terrane_future = { let __terrane_call = router.dispatch(__terrane_raised(|
            | -> Result < _, crate ::TerraneForeignError > { Ok({ let callback =
            __terrane_source_input_0; move | callback_argument_0 : i64 | { let callback =
            callback.clone(); let callback_future =
            callback(terrane_int_support::Int::from(i128::from(callback_argument_0)));
            Box::pin(async move { match async { let callback_value = callback_future.
            await; Ok:: < _, crate ::TerraneForeignError > (terrane_int_support::coerce::
            < i64 > (&callback_value).map_err(| error | crate ::TerraneForeignError(crate
            ::TerraneRaised::raised(error, crate ::TERRANE_NO_SITE))) ?) } . await {
            Ok(value) => value, Err(error) => std::panic::panic_any(error.0) } }) } }) }
            (), 18 /* terrane-site: src/main.trn:56:36-56:62 */)); Box::pin(async move
            { { let value = crate ::__terrane_dependency_await_unwind(__terrane_call).
            await .map_err(| payload | crate ::__terrane_dependency_panic(payload,
            "terrane-callable-recipe-witness",
            "terrane_callable_recipe_witness::MethodRouter::dispatch")) ?; Ok:: < _,
            crate ::TerraneForeignError > (value) } }) }; async move {
            __terrane_raised_err(__terrane_future. await, 19 /* terrane-site: src/main.trn:56:19-56:62 */) } } }). await, 19 /* terrane-site: src/main.trn:56:19-56:62 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = __terrane_traced(__terrane_await({ let __terrane_source_input_0 =
            std::sync::Arc::new(move | argument_0 : terrane_int_support::Int | ->
            std::pin::Pin < Box < dyn Future < Output = _ > + Send > > {
            Box::pin(produce_async_one_argument(argument_0)) }); { let __terrane_future =
            { let __terrane_call =
            terrane_callable_recipe_witness::registered(__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { Ok({ let callback =
            __terrane_source_input_0; move | callback_argument_0 : i64 | { let callback =
            callback.clone(); let callback_future =
            callback(terrane_int_support::Int::from(i128::from(callback_argument_0)));
            Box::pin(async move { match async { let callback_value = callback_future.
            await; Ok:: < _, crate ::TerraneForeignError > (terrane_int_support::coerce::
            < i64 > (&callback_value).map_err(| error | crate ::TerraneForeignError(crate
            ::TerraneRaised::raised(error, crate ::TERRANE_NO_SITE))) ?) } . await {
            Ok(value) => value, Err(error) => std::panic::panic_any(error.0) } }) } }) }
            (), 20 /* terrane-site: src/main.trn:57:52-57:78 */)); Box::pin(async move
            { { let value = crate ::__terrane_dependency_await_unwind(__terrane_call).
            await .map_err(| payload | crate ::__terrane_dependency_panic(payload,
            "terrane-callable-recipe-witness",
            "terrane_callable_recipe_witness::registered")) ?; Ok:: < _, crate
            ::TerraneForeignError > (value) } }) }; async move {
            __terrane_raised_err(__terrane_future. await, 21 /* terrane-site: src/main.trn:57:40-57:78 */) } } }). await, 21 /* terrane-site: src/main.trn:57:40-57:78 */);
            __terrane_raised(registration_value(__terrane_conversion_input_0),
            22 /* terrane-site: src/main.trn:57:13-57:79 */) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = __terrane_traced(__terrane_await({ let __terrane_source_input_0 =
            std::sync::Arc::new(move | argument_0 : terrane_int_support::Int | ->
            std::pin::Pin < Box < dyn Future < Output = _ > + Send > > {
            Box::pin(produce_async_one_argument(argument_0)) }); { let __terrane_future =
            { let __terrane_call =
            terrane_callable_recipe_witness::registered(__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { Ok({ let callback =
            __terrane_source_input_0; move | callback_argument_0 : i64 | { let callback =
            callback.clone(); let callback_future =
            callback(terrane_int_support::Int::from(i128::from(callback_argument_0)));
            Box::pin(async move { match async { let callback_value = callback_future.
            await; Ok:: < _, crate ::TerraneForeignError > (terrane_int_support::coerce::
            < i64 > (&callback_value).map_err(| error | crate ::TerraneForeignError(crate
            ::TerraneRaised::raised(error, crate ::TERRANE_NO_SITE))) ?) } . await {
            Ok(value) => value, Err(error) => std::panic::panic_any(error.0) } }) } }) }
            (), 23 /* terrane-site: src/main.trn:58:63-58:89 */)); Box::pin(async move
            { { let value = crate ::__terrane_dependency_await_unwind(__terrane_call).
            await .map_err(| payload | crate ::__terrane_dependency_panic(payload,
            "terrane-callable-recipe-witness",
            "terrane_callable_recipe_witness::registered")) ?; Ok:: < _, crate
            ::TerraneForeignError > (value) } }) }; async move {
            __terrane_raised_err(__terrane_future. await, 24 /* terrane-site: src/main.trn:58:51-58:89 */) } } }). await, 24 /* terrane-site: src/main.trn:58:51-58:89 */);
            __terrane_raised(registration_value_with_state(__terrane_conversion_input_0),
            25 /* terrane-site: src/main.trn:58:13-58:90 */) })
        );
    });
}
