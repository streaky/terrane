// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let value: HeaderValue;
    value = __terrane_raised(
        terrane_static_trn_48656164657256616c7565_from_static("Accept"),
        0 /* terrane-site: src/main.trn:5:13-5:49 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { match match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&value).to_str()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload, "http",
        "http::HeaderValue::to_str")) } { Ok(value) => Ok(value.to_owned()), Err(error)
        => Err(crate ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
        ::DescriptorId(2), error.to_string(), crate ::TERRANE_NO_SITE))) } } (),
        1 /* terrane-site: src/main.trn:6:13-6:26 */))
    );
}
