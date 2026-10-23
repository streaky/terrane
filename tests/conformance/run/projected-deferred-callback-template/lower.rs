// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn message() -> String {
    return String::from("deferred");
}
fn length(value: String) -> terrane_int_support::Int {
    return terrane_int_support::Int::from(
        terrane_string_support::length(&value) as i128,
    );
}
fn main() {
    let value: String;
    let size: terrane_int_support::Int;
    value = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| terrane_deferred_callback_witness::produce(
                match || -> Result<_, crate::TerraneForeignError> {
                    Ok({
                        let callback = std::sync::Arc::new(message);
                        move || {
                            match || -> Result<_, crate::TerraneForeignError> {
                                let callback_value = callback();
                                Ok(callback_value)
                            }() {
                                Ok(value) => value,
                                Err(error) => std::panic::panic_any(error.0),
                            }
                        }
                    })
                }() {
                    Ok(value) => value,
                    Err(error) => std::panic::panic_any(error),
                },
            )),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "dependency::ForeignType::produce",
                    ),
                )
            }
        },
        0 /* terrane-site: src/main.trn:12:20-12:36 */,
    );
    size = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| terrane_deferred_callback_witness::transform(
                value.clone(),
                match || -> Result<_, crate::TerraneForeignError> {
                    Ok({
                        let callback = std::sync::Arc::new(length);
                        move |callback_argument_0: String| {
                            match || -> Result<_, crate::TerraneForeignError> {
                                let callback_value = callback(callback_argument_0);
                                Ok(
                                    terrane_int_support::coerce::<i64>(&callback_value)
                                        .map_err(|error| crate::TerraneForeignError(
                                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                        ))?,
                                )
                            }() {
                                Ok(value) => value,
                                Err(error) => std::panic::panic_any(error.0),
                            }
                        }
                    })
                }() {
                    Ok(value) => value,
                    Err(error) => std::panic::panic_any(error),
                },
            )),
        ) {
            Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "dependency::ForeignType::transform",
                    ),
                )
            }
        },
        1 /* terrane-site: src/main.trn:13:16-13:40 */,
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&value), String::from(":"),
        terrane_scalar_support::scalar_text(&size)
    );
}
