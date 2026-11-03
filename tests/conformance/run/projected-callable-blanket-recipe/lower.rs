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
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        std::sync::Arc::new(produce); __terrane_raised(| | -> Result < _, crate
        ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(terrane_callable_recipe_witness::invoke({ let
        callback = __terrane_conversion_input_0; move | | { match | | -> Result < _,
        crate ::TerraneForeignError > { let callback_value = callback();
        Ok(terrane_int_support::coerce:: < i64 > (&callback_value).map_err(| error |
        crate ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error.0) } } })) })) { Ok(result) => result ?, Err(payload)
        => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-callable-recipe-witness", "terrane_callable_recipe_witness::invoke")) };
        Ok:: < _, crate ::TerraneForeignError >
        (terrane_int_support::Int::from(i128::from(value))) } } (), 0 /* terrane-site: src/main.trn:33:13-33:28 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        String::from("one"); let __terrane_conversion_input_1 =
        std::sync::Arc::new(length); __terrane_raised(| | -> Result < _, crate
        ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > {
        Ok(terrane_callable_recipe_witness::transform(__terrane_conversion_input_0, { let
        callback = __terrane_conversion_input_1; move | callback_argument_0 : String | {
        match | | -> Result < _, crate ::TerraneForeignError > { let callback_value =
        callback(callback_argument_0); Ok(terrane_int_support::coerce:: < i64 >
        (&callback_value).map_err(| error | crate ::TerraneForeignError(crate
        ::TerraneRaised::raised(error, crate ::TERRANE_NO_SITE))) ?) } () { Ok(value) =>
        value, Err(error) => std::panic::panic_any(error.0) } } })) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-callable-recipe-witness", "terrane_callable_recipe_witness::transform"))
        }; Ok:: < _, crate ::TerraneForeignError >
        (terrane_int_support::Int::from(i128::from(value))) } } (), 1 /* terrane-site: src/main.trn:34:13-34:37 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        String::from("two"); let __terrane_conversion_input_1 =
        std::sync::Arc::new(length); __terrane_raised(| | -> Result < _, crate
        ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > {
        Ok(terrane_callable_recipe_witness::mutate(__terrane_conversion_input_0, { let
        callback = __terrane_conversion_input_1; move | callback_argument_0 : String | {
        match | | -> Result < _, crate ::TerraneForeignError > { let callback_value =
        callback(callback_argument_0); Ok(terrane_int_support::coerce:: < i64 >
        (&callback_value).map_err(| error | crate ::TerraneForeignError(crate
        ::TerraneRaised::raised(error, crate ::TERRANE_NO_SITE))) ?) } () { Ok(value) =>
        value, Err(error) => std::panic::panic_any(error.0) } } })) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-callable-recipe-witness", "terrane_callable_recipe_witness::mutate")) };
        Ok:: < _, crate ::TerraneForeignError >
        (terrane_int_support::Int::from(i128::from(value))) } } (), 2 /* terrane-site: src/main.trn:35:13-35:34 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        std::sync::Arc::new(move | | -> std::pin::Pin < Box < dyn Future < Output = _ > +
        Send > > { Box::pin(produce_async()) }); __terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(terrane_callable_recipe_witness::install({ let
        callback = __terrane_conversion_input_0; move | | { let callback = callback
        .clone(); let callback_future = callback(); Box::pin(async move { match async {
        let callback_value = callback_future. await; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::coerce:: < i64 > (&callback_value)
        .map_err(| error | crate ::TerraneForeignError(crate
        ::TerraneRaised::raised(error, crate ::TERRANE_NO_SITE))) ?) } . await {
        Ok(value) => value, Err(error) => std::panic::panic_any(error.0) } }) } })) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane-callable-recipe-witness",
        "terrane_callable_recipe_witness::install")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 3 /* terrane-site: src/main.trn:36:13-36:35 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        std::sync::Arc::new(initial); let __terrane_conversion_input_1 =
        terrane_int_support::Int::from(7_i128); let __terrane_conversion_input_2 =
        std::sync::Arc::new(combine); __terrane_raised(| | -> Result < _, crate
        ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(terrane_callable_recipe_witness::run_correlated({
        let callback = __terrane_conversion_input_0; move | | { match | | -> Result < _,
        crate ::TerraneForeignError > { let callback_value = callback();
        Ok(callback_value) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error.0) } } }, terrane_int_support::coerce:: < i64 >
        (&__terrane_conversion_input_1).map_err(| error | crate
        ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ?, { let callback = __terrane_conversion_input_2; move |
        callback_argument_0 : String, callback_argument_1 : i64 | { match | | -> Result <
        _, crate ::TerraneForeignError > { let callback_value =
        callback(callback_argument_0,
        terrane_int_support::Int::from(i128::from(callback_argument_1)));
        Ok(callback_value) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error.0) } } })) })) { Ok(result) => result ?, Err(payload)
        => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-callable-recipe-witness",
        "terrane_callable_recipe_witness::run_correlated")) }; Ok:: < _, crate
        ::TerraneForeignError > (value) } } (), 4 /* terrane-site: src/main.trn:37:13-37:48 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        std::sync::Arc::new(initial); let __terrane_conversion_input_1 =
        std::sync::Arc::new(initial_message); let __terrane_conversion_input_2 =
        std::sync::Arc::new(combine); __terrane_raised(| | -> Result < _, crate
        ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(terrane_callable_recipe_witness::compose({ let
        callback = __terrane_conversion_input_0; move | | { match | | -> Result < _,
        crate ::TerraneForeignError > { let callback_value = callback();
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
        std::panic::panic_any(error.0) } } })) })) { Ok(result) => result ?, Err(payload)
        => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-callable-recipe-witness", "terrane_callable_recipe_witness::compose"))
        }; Ok:: < _, crate ::TerraneForeignError > (value) } } (), 5 /* terrane-site: src/main.trn:38:13-38:55 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        std::sync::Arc::new(no_argument); __terrane_raised(| | -> Result < _, crate
        ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(terrane_callable_recipe_witness::accepts_handler({
        let callback = __terrane_conversion_input_0; move | | { match | | -> Result < _,
        crate ::TerraneForeignError > { let callback_value = callback();
        Ok(terrane_int_support::coerce:: < i64 > (&callback_value).map_err(| error |
        crate ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error.0) } } })) })) { Ok(result) => result ?, Err(payload)
        => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-callable-recipe-witness",
        "terrane_callable_recipe_witness::accepts_handler")) }; Ok:: < _, crate
        ::TerraneForeignError > (value) } } (), 6 /* terrane-site: src/main.trn:39:13-39:41 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        std::sync::Arc::new(one_argument); __terrane_raised(| | -> Result < _, crate
        ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(terrane_callable_recipe_witness::accepts_handler({
        let callback = __terrane_conversion_input_0; move | callback_argument_0 : i64 | {
        match | | -> Result < _, crate ::TerraneForeignError > { let callback_value =
        callback(terrane_int_support::Int::from(i128::from(callback_argument_0)));
        Ok(terrane_int_support::coerce:: < i64 > (&callback_value).map_err(| error |
        crate ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error.0) } } })) })) { Ok(result) => result ?, Err(payload)
        => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-callable-recipe-witness",
        "terrane_callable_recipe_witness::accepts_handler")) }; Ok:: < _, crate
        ::TerraneForeignError > (value) } } (), 7 /* terrane-site: src/main.trn:40:13-40:42 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_1 =
        std::sync::Arc::new(view_value); __terrane_raised(| | -> Result < _, crate
        ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(terrane_callable_recipe_witness::render:: < i64 >
        (&4, { let callback = __terrane_conversion_input_1; move | callback_argument_0 :
        &'_ i64 | { match | | -> Result < _, crate ::TerraneForeignError > { let
        callback_value = callback(&terrane_int_support::Int::from(i128::from(*
        callback_argument_0))); Ok(terrane_int_support::coerce:: < i64 >
        (&callback_value).map_err(| error | crate ::TerraneForeignError(crate
        ::TerraneRaised::raised(error, crate ::TERRANE_NO_SITE))) ?) } () { Ok(value) =>
        value, Err(error) => std::panic::panic_any(error.0) } } })) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-callable-recipe-witness", "terrane_callable_recipe_witness::render")) };
        Ok:: < _, crate ::TerraneForeignError >
        (terrane_int_support::Int::from(i128::from(value))) } } (), 8 /* terrane-site: src/main.trn:41:13-41:34 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        __terrane_raised(blank(), 9 /* terrane-site: src/main.trn:42:22-42:28 */);
        __terrane_raised(accept(__terrane_conversion_input_0), 10 /* terrane-site: src/main.trn:42:13-42:29 */) })
    );
}
