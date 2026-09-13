// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: src/main.trn
// Namespace: app
fn main() {
    let value: i64 = __terrane_raised(
        from_str::<i64>(String::from("17")),
        0 /* terrane-site: src/main.trn:6:17-6:31 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&value));
}
// Source: <terrane>/projected/deps/serde-json.trn
// Namespace: deps/serde-json
pub fn from_str<T: for<'a> serde_core::de::Deserialize<'a>>(
    s: String,
) -> Result<T, crate::TerraneForeignError> {
    let s = s;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| serde_json::from_str::<T>(&s)),
    ) {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(error)) => {
            Err(
                crate::TerraneForeignError(
                    crate::TerraneError::custom_raised(
                        crate::TERRANE_DEPENDENCY_ERROR,
                        format!(
                            "Rust dependency `serde-json` member `serde_json::from_str` failed: {error}"
                        ),
                        crate::TERRANE_NO_SITE,
                    ),
                ),
            )
        }
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "serde-json",
                    "serde_json::from_str",
                ),
            )
        }
    }
}
