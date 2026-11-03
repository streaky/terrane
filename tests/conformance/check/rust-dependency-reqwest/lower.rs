// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let mut response: reqwest::blocking::Response;
    let response_status: StatusCode;
    let body: String;
    response = {
        let __terrane_conversion_input_0 = String::from("http://127.0.0.1:38125/");
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                match match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(reqwest::blocking::get(__terrane_conversion_input_0)) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "reqwest",
                                "reqwest::blocking::get",
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
            0 /* terrane-site: src/main.trn:5:16-5:45 */,
        )
    };
    response.headers_mut();
    response_status = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&response).status()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "reqwest",
                                "reqwest::blocking::Response::status",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        1 /* terrane-site: src/main.trn:7:34-7:50 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&response_status).is_success()) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "http", "http::StatusCode::is_success")) }; Ok:: < _, crate ::TerraneForeignError
        > (value) } } (), 2 /* terrane-site: src/main.trn:8:13-8:40 */))
    );
    body = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            match match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| -> Result<_, crate::TerraneForeignError> {
                    Ok(response.text())
                }),
            ) {
                Ok(result) => result?,
                Err(payload) => {
                    return Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "reqwest",
                            "reqwest::blocking::Response::text",
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
        3 /* terrane-site: src/main.trn:9:19-9:33 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&body));
}
