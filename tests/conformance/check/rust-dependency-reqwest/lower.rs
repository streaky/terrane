// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: src/main.trn
// Namespace: app
fn main() {
    let mut response: Response = __terrane_raised(
        get(String::from("http://127.0.0.1:38125/")),
        0 /* terrane-site: src/main.trn:5:16-5:45 */,
    );
    __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                response.headers_mut();
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "reqwest",
                        "reqwest::blocking::Response::headers_mut",
                    ),
                )
            }
        },
        1 /* terrane-site: src/main.trn:6:5-6:26 */,
    );
    let response_status: StatusCode = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| response.status()),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "reqwest",
                        "reqwest::blocking::Response::status",
                    ),
                )
            }
        },
        2 /* terrane-site: src/main.trn:7:34-7:50 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | response_status
        .is_success())) { Ok(value) => Ok(value), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "http", "http::StatusCode::is_success")) },
        3 /* terrane-site: src/main.trn:8:13-8:40 */))
    );
    let body: String = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| response.text()),
        ) {
            Ok(Ok(value)) => Ok(value),
            Ok(Err(error)) => {
                Err(
                    crate::TerraneForeignError(
                        crate::TerraneError::custom_raised(
                            crate::TERRANE_DEPENDENCY_ERROR,
                            format!(
                                "Rust dependency `reqwest` member `reqwest::blocking::Response::text` failed: {error}"
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
                        "reqwest",
                        "reqwest::blocking::Response::text",
                    ),
                )
            }
        },
        4 /* terrane-site: src/main.trn:9:19-9:33 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&body));
}
// Source: <terrane>/projected/deps/http.trn
// Namespace: deps/http
pub use http::Extensions;
pub use http::StatusCode;
pub use http::Version;
pub fn terrane_static_trn_457874656e73696f6e73_new() -> Result<
    Extensions,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| http::Extensions::new()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "http", "http::Extensions"))
        }
    }
}
pub fn terrane_static_trn_537461747573436f6465_from_u16(
    src: terrane_int_support::Int,
) -> Result<StatusCode, crate::TerraneForeignError> {
    let src = terrane_int_support::coerce::<u16>(&src)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| http::StatusCode::from_u16(src)),
    ) {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(error)) => {
            Err(
                crate::TerraneForeignError(
                    crate::TerraneError::custom_raised(
                        crate::TERRANE_DEPENDENCY_ERROR,
                        format!(
                            "Rust dependency `http` member `http::StatusCode` failed: {error}"
                        ),
                        crate::TERRANE_NO_SITE,
                    ),
                ),
            )
        }
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "http", "http::StatusCode"))
        }
    }
}
// Source: <terrane>/projected/deps/reqwest/blocking.trn
// Namespace: deps/reqwest/blocking
pub use reqwest::blocking::Response;
pub fn get(url: String) -> Result<Response, crate::TerraneForeignError> {
    let url = url;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| reqwest::blocking::get(url)),
    ) {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(error)) => {
            Err(
                crate::TerraneForeignError(
                    crate::TerraneError::custom_raised(
                        crate::TERRANE_DEPENDENCY_ERROR,
                        format!(
                            "Rust dependency `reqwest` member `reqwest::blocking::get` failed: {error}"
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
                    "reqwest",
                    "reqwest::blocking::get",
                ),
            )
        }
    }
}
