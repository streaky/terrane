// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn callback() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(7_i128);
}
fn main() {
    let defaulted: witness::Builder<()>;
    let attached: witness::Builder<()>;
    let concrete: DefaultOnly;
    defaulted = __terrane_raised(
        terrane_static_trn_4275696c646572_new::<()>(),
        0 /* terrane-site: src/main.trn:9:25-9:38 */,
    );
    attached = {
        let __terrane_native_input_0 = defaulted;
        let __terrane_conversion_input_0 = {
            let __terrane_conversion_input_0 = TerraneConsumingCallable::new(move |
                (): ()|
            callback());
            __terrane_raised(
                || -> Result<_, crate::TerraneForeignError> {
                    {
                        let value = match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| -> Result<
                                _,
                                crate::TerraneForeignError,
                            > {
                                Ok(
                                    witness::make_lane::<
                                        _,
                                        _,
                                    >({
                                        let callback = __terrane_conversion_input_0;
                                        move || {
                                            match || -> Result<_, crate::TerraneForeignError> {
                                                let callback_value = callback.call(());
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
                                    }),
                                )
                            }),
                        ) {
                            Ok(result) => result?,
                            Err(payload) => {
                                return Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "witness",
                                        "witness::make_lane",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),
                1 /* terrane-site: src/main.trn:10:35-10:54 */,
            )
        };
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                __terrane_native_input_0
                                    .attach(__terrane_conversion_input_0),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "witness",
                                    "witness::Builder::attach",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),
            2 /* terrane-site: src/main.trn:10:16-10:55 */,
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&attached).value()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::Builder::value")) }; Ok:: < _, crate ::TerraneForeignError >
        (terrane_int_support::Int::from(i128::from(value))) } } (), 3 /* terrane-site: src/main.trn:11:13-11:28 */))
    );
    concrete = __terrane_raised(
        terrane_static_trn_44656661756c744f6e6c79_new(),
        4 /* terrane-site: src/main.trn:12:28-12:45 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&concrete).value()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::DefaultOnly::value")) }; Ok:: < _, crate ::TerraneForeignError >
        (terrane_int_support::Int::from(i128::from(value))) } } (), 5 /* terrane-site: src/main.trn:13:13-13:28 */))
    );
}
