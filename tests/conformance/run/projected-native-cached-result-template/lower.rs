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
    let concrete: witness::DefaultOnly<core::convert::Infallible>;
    defaulted = __terrane_raised(
        terrane_static_trn_4275696c646572_new::<()>(),
        0 /* terrane-site: src/main.trn:9:25-9:38 */,
    );
    attached = {
        let __terrane_native_input_0 = defaulted;
        let __terrane_native_input_1 = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| witness::make_lane::<
                    _,
                    (),
                >(
                    match || -> Result<_, crate::TerraneForeignError> {
                        Ok({
                            let callback = TerraneConsumingCallable::new(move |(): ()| callback());
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
                            "dependency::ForeignType::make_lane",
                        ),
                    )
                }
            },
            1 /* terrane-site: src/main.trn:10:35-10:54 */,
        );
        __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    __terrane_native_input_0.attach(__terrane_native_input_1)
                }),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "witness",
                            "witness::Builder::attach",
                        ),
                    )
                }
            },
            2 /* terrane-site: src/main.trn:10:16-10:55 */,
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&attached).value())) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload)
        => Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::Builder::value")) }, 3 /* terrane-site: src/main.trn:11:13-11:28 */))
    );
    concrete = __terrane_raised(
        terrane_static_trn_44656661756c744f6e6c79_new(),
        4 /* terrane-site: src/main.trn:12:28-12:45 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&concrete).value())) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload)
        => Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::DefaultOnly::value")) }, 5 /* terrane-site: src/main.trn:13:13-13:28 */))
    );
}
