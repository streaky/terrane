// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn consume_generic<TerraneType54>(value: Boxed<TerraneType54>) -> TerraneType54 {
    return __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(value.into_value()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::Boxed::into_value",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        0 /* terrane-site: src/main.trn:6:12-6:29 */,
    );
}
fn main() {
    let written: witness::Boxed<i128>;
    let projected_result: i128;
    let terrane_input: witness::Boxed<i128>;
    let terrane_result: i128;
    written = __terrane_raised(
        make_i128(),
        1 /* terrane-site: src/main.trn:9:31-9:41 */,
    );
    projected_result = {
        let __terrane_conversion_input_0 = written;
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok(witness::consume_native(__terrane_conversion_input_0)) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "witness",
                                    "witness::consume_native",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),
            2 /* terrane-site: src/main.trn:10:31-10:54 */,
        )
    };
    terrane_input = __terrane_raised(
        make_i128(),
        3 /* terrane-site: src/main.trn:11:37-11:47 */,
    );
    terrane_result = consume_generic(terrane_input);
    println!("{}", terrane_scalar_support::scalar_text(&projected_result));
    println!("{}", terrane_scalar_support::scalar_text(&terrane_result));
}
