// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let initial: Border;
    let radius: Radius;
    let first: Border;
    let eight: f32;
    let second: Border;
    initial = __terrane_raised(
        make_border(),
        0 /* terrane-site: src/main.trn:6:22-6:34 */,
    );
    radius = {
        let __terrane_conversion_input_0 = 4.0_f32;
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(terrane_native_bound_witness::Radius {
                    value: __terrane_conversion_input_0,
                })
            }(),
            1 /* terrane-site: src/main.trn:7:21-7:49 */,
        )
    };
    first = {
        let __terrane_conversion_input_0 = radius;
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok(initial.rounded(__terrane_conversion_input_0)) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_native_bound_witness",
                                    "terrane_native_bound_witness::Border::rounded",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),
            2 /* terrane-site: src/main.trn:8:20-8:43 */,
        )
    };
    eight = 8.0_f32;
    second = {
        let __terrane_conversion_input_0 = eight;
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok(first.rounded(__terrane_conversion_input_0)) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_native_bound_witness",
                                    "terrane_native_bound_witness::Border::rounded",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),
            3 /* terrane-site: src/main.trn:10:21-10:41 */,
        )
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(radius_value(&second),
        4 /* terrane-site: src/main.trn:11:13-11:33 */))
    );
}
