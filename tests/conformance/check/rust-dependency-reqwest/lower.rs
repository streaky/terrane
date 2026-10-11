// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let mut response: Response;
    let response_status: StatusCode;
    let body: String;
    response = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| reqwest::blocking::get(
                String::from("http://127.0.0.1:38125/"),
            )),
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
                        "reqwest",
                        "reqwest::blocking::get",
                    ),
                )
            }
        },
        0 /* terrane-site: src/main.trn:5:16-5:45 */,
    );
    response.headers_mut();
    response_status = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| (&response).status()),
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
        1 /* terrane-site: src/main.trn:7:34-7:50 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&response_status)
        .is_success())) { Ok(value) => Ok(value), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "http", "http::StatusCode::is_success")) },
        2 /* terrane-site: src/main.trn:8:13-8:40 */))
    );
    body = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| response.text()),
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
                        "reqwest",
                        "reqwest::blocking::Response::text",
                    ),
                )
            }
        },
        3 /* terrane-site: src/main.trn:9:19-9:33 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&body));
}
