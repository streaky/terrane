// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let value: i64;
    value = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| serde_json::from_str::<
                i64,
            >(&String::from("17"))),
        ) {
            Ok(Ok(value)) => Ok(value),
            Ok(Err(error)) => {
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
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "serde-json",
                        "serde_json::from_str::<i64>",
                    ),
                )
            }
        },
        0 /* terrane-site: src/main.trn:5:17-5:31 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&value));
}
