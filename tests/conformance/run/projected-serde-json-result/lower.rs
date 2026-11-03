// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let value: i64;
    value = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            match match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| -> Result<_, crate::TerraneForeignError> {
                    Ok(serde_json::from_str::<i64>("17"))
                }),
            ) {
                Ok(result) => result?,
                Err(payload) => {
                    return Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "serde-json",
                            "serde_json::from_str",
                        ),
                    );
                }
            } {
                Ok(value) => Ok(value),
                Err(error) => {
                    Err(
                        crate::TerraneForeignError(
                            crate::TerraneError::custom_raised(
                                crate::DescriptorId(2),
                                error.to_string(),
                                crate::TERRANE_NO_SITE,
                            ),
                        ),
                    )
                }
            }
        }(),
        0 /* terrane-site: src/main.trn:5:17-5:31 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&value));
}
