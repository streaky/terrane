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
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        terrane_callable_recipe_witness::invoke(match | | -> Result < _, crate
        ::TerraneForeignError > { Ok({ let callback = std::sync::Arc::new(produce); move
        | | { match | | -> Result < _, crate ::TerraneForeignError > { let callback_value
        = callback(); Ok(terrane_int_support::coerce:: < i64 > (&callback_value)
        .map_err(| error | crate ::TerraneForeignError(crate
        ::TerraneRaised::raised(error, crate ::TERRANE_NO_SITE))) ?) } () { Ok(value) =>
        value, Err(error) => std::panic::panic_any(error.0) } } }) } () { Ok(value) =>
        value, Err(error) => std::panic::panic_any(error) }))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "terrane-callable-recipe-witness",
        "terrane_callable_recipe_witness::invoke")) }, 0 /* terrane-site: src/main.trn:33:13-33:28 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        terrane_callable_recipe_witness::transform(String::from("one"), match | | ->
        Result < _, crate ::TerraneForeignError > { Ok({ let callback =
        std::sync::Arc::new(length); move | callback_argument_0 : String | { match | | ->
        Result < _, crate ::TerraneForeignError > { let callback_value =
        callback(callback_argument_0); Ok(terrane_int_support::coerce:: < i64 >
        (&callback_value).map_err(| error | crate ::TerraneForeignError(crate
        ::TerraneRaised::raised(error, crate ::TERRANE_NO_SITE))) ?) } () { Ok(value) =>
        value, Err(error) => std::panic::panic_any(error.0) } } }) } () { Ok(value) =>
        value, Err(error) => std::panic::panic_any(error) }))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "terrane-callable-recipe-witness",
        "terrane_callable_recipe_witness::transform")) }, 1 /* terrane-site: src/main.trn:34:13-34:37 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        terrane_callable_recipe_witness::mutate(String::from("two"), match | | -> Result
        < _, crate ::TerraneForeignError > { Ok({ let callback =
        std::sync::Arc::new(length); move | callback_argument_0 : String | { match | | ->
        Result < _, crate ::TerraneForeignError > { let callback_value =
        callback(callback_argument_0); Ok(terrane_int_support::coerce:: < i64 >
        (&callback_value).map_err(| error | crate ::TerraneForeignError(crate
        ::TerraneRaised::raised(error, crate ::TERRANE_NO_SITE))) ?) } () { Ok(value) =>
        value, Err(error) => std::panic::panic_any(error.0) } } }) } () { Ok(value) =>
        value, Err(error) => std::panic::panic_any(error) }))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "terrane-callable-recipe-witness",
        "terrane_callable_recipe_witness::mutate")) }, 2 /* terrane-site: src/main.trn:35:13-35:34 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        terrane_callable_recipe_witness::install(match | | -> Result < _, crate
        ::TerraneForeignError > { Ok({ let callback = std::sync::Arc::new(move | | ->
        std::pin::Pin < Box < dyn Future < Output = _ > + Send > > {
        Box::pin(produce_async()) }); move | | { let callback = callback.clone(); let
        callback_future = callback(); Box::pin(async move { match async { let
        callback_value = callback_future. await; Ok:: < _, crate ::TerraneForeignError >
        (terrane_int_support::coerce:: < i64 > (&callback_value).map_err(| error | crate
        ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ?) } . await { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error.0) } }) } }) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error) }))) { Ok(value) =>
        Ok(terrane_int_support::Int::from_u128(value as u128)), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "terrane-callable-recipe-witness",
        "terrane_callable_recipe_witness::install")) }, 3 /* terrane-site: src/main.trn:36:13-36:35 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        terrane_callable_recipe_witness::run_correlated(match | | -> Result < _, crate
        ::TerraneForeignError > { Ok({ let callback = std::sync::Arc::new(initial); move
        | | { match | | -> Result < _, crate ::TerraneForeignError > { let callback_value
        = callback(); Ok(callback_value) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error.0) } } }) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error) }, match | | -> Result < _, crate
        ::TerraneForeignError > { Ok(terrane_int_support::coerce:: < i64 >
        (&terrane_int_support::Int::from(7_i128)).map_err(| error | crate
        ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error) }, match | | -> Result < _, crate
        ::TerraneForeignError > { Ok({ let callback = std::sync::Arc::new(combine); move
        | callback_argument_0 : String, callback_argument_1 : i64 | { match | | -> Result
        < _, crate ::TerraneForeignError > { let callback_value =
        callback(callback_argument_0,
        terrane_int_support::Int::from(i128::from(callback_argument_1)));
        Ok(callback_value) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error.0) } } }) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error) }))) { Ok(value) => Ok(value), Err(payload) =>
        Err(crate ::__terrane_dependency_panic(payload,
        "terrane-callable-recipe-witness",
        "terrane_callable_recipe_witness::run_correlated")) }, 4 /* terrane-site: src/main.trn:37:13-37:48 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        terrane_callable_recipe_witness::compose(match | | -> Result < _, crate
        ::TerraneForeignError > { Ok({ let callback = std::sync::Arc::new(initial); move
        | | { match | | -> Result < _, crate ::TerraneForeignError > { let callback_value
        = callback(); Ok(callback_value) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error.0) } } }) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error) }, match | | -> Result < _, crate
        ::TerraneForeignError > { Ok({ let callback =
        std::sync::Arc::new(initial_message); move | | { match | | -> Result < _, crate
        ::TerraneForeignError > { let callback_value = callback();
        Ok(terrane_int_support::coerce:: < i64 > (&callback_value).map_err(| error |
        crate ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error.0) } } }) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error) }, match | | -> Result < _, crate
        ::TerraneForeignError > { Ok({ let callback = std::sync::Arc::new(combine); move
        | callback_argument_0 : String, callback_argument_1 : i64 | { match | | -> Result
        < _, crate ::TerraneForeignError > { let callback_value =
        callback(callback_argument_0,
        terrane_int_support::Int::from(i128::from(callback_argument_1)));
        Ok(callback_value) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error.0) } } }) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error) }))) { Ok(value) => Ok(value), Err(payload) =>
        Err(crate ::__terrane_dependency_panic(payload,
        "terrane-callable-recipe-witness", "terrane_callable_recipe_witness::compose"))
        }, 5 /* terrane-site: src/main.trn:38:13-38:55 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        terrane_callable_recipe_witness::accepts_handler(match | | -> Result < _, crate
        ::TerraneForeignError > { Ok({ let callback = std::sync::Arc::new(no_argument);
        move | | { match | | -> Result < _, crate ::TerraneForeignError > { let
        callback_value = callback(); Ok(terrane_int_support::coerce:: < i64 >
        (&callback_value).map_err(| error | crate ::TerraneForeignError(crate
        ::TerraneRaised::raised(error, crate ::TERRANE_NO_SITE))) ?) } () { Ok(value) =>
        value, Err(error) => std::panic::panic_any(error.0) } } }) } () { Ok(value) =>
        value, Err(error) => std::panic::panic_any(error) }))) { Ok(value) => Ok(value),
        Err(payload) => Err(crate ::__terrane_dependency_panic(payload,
        "terrane-callable-recipe-witness",
        "terrane_callable_recipe_witness::accepts_handler")) }, 6 /* terrane-site: src/main.trn:39:13-39:41 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        terrane_callable_recipe_witness::accepts_handler(match | | -> Result < _, crate
        ::TerraneForeignError > { Ok({ let callback = std::sync::Arc::new(one_argument);
        move | callback_argument_0 : i64 | { match | | -> Result < _, crate
        ::TerraneForeignError > { let callback_value =
        callback(terrane_int_support::Int::from(i128::from(callback_argument_0)));
        Ok(terrane_int_support::coerce:: < i64 > (&callback_value).map_err(| error |
        crate ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error.0) } } }) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error) }))) { Ok(value) => Ok(value), Err(payload) =>
        Err(crate ::__terrane_dependency_panic(payload,
        "terrane-callable-recipe-witness",
        "terrane_callable_recipe_witness::accepts_handler")) }, 7 /* terrane-site: src/main.trn:40:13-40:42 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        terrane_callable_recipe_witness::render:: < i64 > (&4, match | | -> Result < _,
        crate ::TerraneForeignError > { Ok({ let callback =
        std::sync::Arc::new(view_value); move | callback_argument_0 : &'_ i64 | { match |
        | -> Result < _, crate ::TerraneForeignError > { let callback_value =
        callback(&terrane_int_support::Int::from(i128::from(* callback_argument_0)));
        Ok(terrane_int_support::coerce:: < i64 > (&callback_value).map_err(| error |
        crate ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error.0) } } }) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error) }))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "terrane-callable-recipe-witness",
        "terrane_callable_recipe_witness::render::<i64>")) }, 8 /* terrane-site: src/main.trn:41:13-41:34 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(blank(), 9 /* terrane-site: src/main.trn:42:22-42:28 */);
        __terrane_raised(accept(__terrane_native_input_0), 10 /* terrane-site: src/main.trn:42:13-42:29 */) })
    );
}
