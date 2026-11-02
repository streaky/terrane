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
    value = {
        let __terrane_conversion_input_0 = std::sync::Arc::new(message);
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                terrane_deferred_callback_witness::produce({
                                    let callback = __terrane_conversion_input_0;
                                    move || {
                                        match || -> Result<_, crate::TerraneForeignError> {
                                            let callback_value = callback();
                                            Ok(callback_value)
                                        }() {
                                            Ok(value) => value,
                                            Err(error) => std::panic::panic_any(error.0),
                                        }
                                    }
                                }),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane-deferred-callback-witness",
                                    "terrane_deferred_callback_witness::produce",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),
            0 /* terrane-site: src/main.trn:12:20-12:36 */,
        )
    };
    size = {
        let __terrane_conversion_input_0 = value.clone();
        let __terrane_conversion_input_1 = std::sync::Arc::new(length);
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                terrane_deferred_callback_witness::transform(
                                    __terrane_conversion_input_0,
                                    {
                                        let callback = __terrane_conversion_input_1;
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
                                    },
                                ),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane-deferred-callback-witness",
                                    "terrane_deferred_callback_witness::transform",
                                ),
                            );
                        }
                    };
                    Ok::<
                        _,
                        crate::TerraneForeignError,
                    >(terrane_int_support::Int::from(i128::from(value)))
                }
            }(),
            1 /* terrane-site: src/main.trn:13:16-13:40 */,
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&value), String::from(":"),
        terrane_scalar_support::scalar_text(&size)
    );
}
