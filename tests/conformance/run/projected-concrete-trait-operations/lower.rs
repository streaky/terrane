// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let mut token: Token;
    let other: Token;
    let mut response: Response;
    let element: Element;
    let basket: Basket;
    let selected: Selected;
    token = __terrane_raised(
        terrane_static_trn_546f6b656e_from_str(String::from("good")),
        0 /* terrane-site: src/main.trn:6:19-6:42 */,
    );
    other = __terrane_raised(
        terrane_static_trn_546f6b656e_from_str(String::from("good")),
        1 /* terrane-site: src/main.trn:7:19-7:42 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&token)
        .contains(&other))) { Ok(value) => Ok(value), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness", "witness::Token::contains")) },
        2 /* terrane-site: src/main.trn:8:13-8:38 */))
    );
    response = __terrane_raised(
        terrane_static_trn_546f6b656e_create(),
        3 /* terrane-site: src/main.trn:10:25-10:39 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&response).value())) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload)
        => Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::Response::value")) }, 4 /* terrane-site: src/main.trn:11:13-11:28 */))
    );
    token = __terrane_raised(
        terrane_static_trn_546f6b656e_from_str(String::from("good")),
        5 /* terrane-site: src/main.trn:13:13-13:36 */,
    );
    response = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| <witness::Token as witness::IntoResponse>::into_response(
                token,
            )),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::Token::into_response",
                    ),
                )
            }
        },
        6 /* terrane-site: src/main.trn:14:16-14:36 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&response).value())) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload)
        => Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::Response::value")) }, 7 /* terrane-site: src/main.trn:15:13-15:28 */))
    );
    token = __terrane_raised(
        terrane_static_trn_546f6b656e_from_str(String::from("good")),
        8 /* terrane-site: src/main.trn:17:13-17:36 */,
    );
    element = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| <witness::Token as core::convert::Into<
                witness::Element,
            >>::into(token)),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::Token::into",
                    ),
                )
            }
        },
        9 /* terrane-site: src/main.trn:18:23-18:34 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&element).value())) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload)
        => Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::Element::value")) }, 10 /* terrane-site: src/main.trn:19:13-19:27 */))
    );
    basket = __terrane_raised(
        terrane_static_trn_4261736b6574_new(),
        11 /* terrane-site: src/main.trn:21:21-21:33 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&basket).len())) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload)
        => Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::Basket::len")) }, 12 /* terrane-site: src/main.trn:22:13-22:24 */))
    );
    selected = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::destination::<witness::Selected>()),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "dependency::ForeignType::destination",
                    ),
                )
            }
        },
        13 /* terrane-site: src/main.trn:24:25-24:37 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&selected).value())) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload)
        => Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::Selected::value")) }, 14 /* terrane-site: src/main.trn:25:13-25:28 */))
    );
}
