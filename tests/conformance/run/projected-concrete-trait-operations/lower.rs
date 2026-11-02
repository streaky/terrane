// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let mut token: Token;
    let other: Token;
    let borrowed_other: &Token;
    let owned: OwnedToken;
    let mut response: Response;
    let element: Element;
    let basket: Basket;
    let selected: Selected;
    token = __terrane_raised(
        terrane_static_trn_546f6b656e_from_str("good"),
        0 /* terrane-site: src/main.trn:6:19-6:42 */,
    );
    other = __terrane_raised(
        terrane_static_trn_546f6b656e_from_str("good"),
        1 /* terrane-site: src/main.trn:7:19-7:42 */,
    );
    borrowed_other = &other;
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&token).contains(&* borrowed_other)) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "witness", "witness::Token::contains")) };
        Ok:: < _, crate ::TerraneForeignError > (value) } } (), 2 /* terrane-site: src/main.trn:9:13-9:43 */))
    );
    owned = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(witness::OwnedToken::Good) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::OwnedToken::Good",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        3 /* terrane-site: src/main.trn:10:13-10:30 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        owned; __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let
        value = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result
        < _, crate ::TerraneForeignError > { Ok((&token)
        .contains(__terrane_conversion_input_0)) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::Token::contains")) }; Ok:: < _, crate ::TerraneForeignError > (value) }
        } (), 4 /* terrane-site: src/main.trn:11:13-11:34 */) })
    );
    response = __terrane_raised(
        terrane_static_trn_546f6b656e_create(),
        5 /* terrane-site: src/main.trn:13:25-13:39 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&response).value()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::Response::value")) }; Ok:: < _, crate ::TerraneForeignError >
        (terrane_int_support::Int::from(i128::from(value))) } } (), 6 /* terrane-site: src/main.trn:14:13-14:28 */))
    );
    token = __terrane_raised(
        terrane_static_trn_546f6b656e_from_str("good"),
        7 /* terrane-site: src/main.trn:16:13-16:36 */,
    );
    response = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            <witness::Token as witness::IntoResponse>::into_response(
                                token,
                            ),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::Token::into_response",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        8 /* terrane-site: src/main.trn:17:16-17:36 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&response).value()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::Response::value")) }; Ok:: < _, crate ::TerraneForeignError >
        (terrane_int_support::Int::from(i128::from(value))) } } (), 9 /* terrane-site: src/main.trn:18:13-18:28 */))
    );
    token = __terrane_raised(
        terrane_static_trn_546f6b656e_from_str("good"),
        10 /* terrane-site: src/main.trn:20:13-20:36 */,
    );
    element = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            <witness::Token as core::convert::Into<
                                witness::Element,
                            >>::into(token),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::Token::into",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        11 /* terrane-site: src/main.trn:21:23-21:34 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&element).value()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::Element::value")) }; Ok:: < _, crate ::TerraneForeignError >
        (terrane_int_support::Int::from(i128::from(value))) } } (),
        12 /* terrane-site: src/main.trn:22:13-22:27 */))
    );
    basket = __terrane_raised(
        terrane_static_trn_4261736b6574_new(),
        13 /* terrane-site: src/main.trn:24:21-24:33 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&basket).len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::Basket::len")) }; Ok:: < _, crate ::TerraneForeignError >
        (terrane_int_support::Int::from(i128::from(value))) } } (),
        14 /* terrane-site: src/main.trn:25:13-25:24 */))
    );
    selected = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(witness::destination::<witness::Selected>()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::destination",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        15 /* terrane-site: src/main.trn:27:25-27:37 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&selected).value()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::Selected::value")) }; Ok:: < _, crate ::TerraneForeignError >
        (terrane_int_support::Int::from(i128::from(value))) } } (),
        16 /* terrane-site: src/main.trn:28:13-28:28 */))
    );
}
