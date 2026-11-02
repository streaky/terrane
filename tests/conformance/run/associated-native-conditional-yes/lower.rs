// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let input: Allowed;
    let answer: bool;
    input = __terrane_raised(allowed(), 0 /* terrane-site: src/main.trn:4:13-4:21 */);
    answer = {
        let __terrane_conversion_input_0 = input;
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok(witness::execute(__terrane_conversion_input_0)) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "witness",
                                    "witness::execute",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),
            1 /* terrane-site: src/main.trn:5:14-5:28 */,
        )
    };
    println!("{}", terrane_scalar_support::scalar_text(&answer));
}
