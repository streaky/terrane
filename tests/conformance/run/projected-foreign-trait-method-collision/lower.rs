// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let first_2: First;
    let second_2: Second;
    let first_value: terrane_int_support::Int;
    let second_value: terrane_int_support::Int;
    first_2 = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(1_i128);
        __terrane_raised(
            first(__terrane_conversion_input_0),
            0 /* terrane-site: src/main.trn:6:13-6:21 */,
        )
    };
    second_2 = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(2_i128);
        __terrane_raised(
            second(__terrane_conversion_input_0),
            1 /* terrane-site: src/main.trn:7:14-7:23 */,
        )
    };
    first_value = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(11_i128);
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                <terrane_trait_collision_impls::First as terrane_trait_collision_contract::ops::Transform>::transform(
                                    &first_2,
                                    terrane_int_support::coerce::<
                                        i64,
                                    >(&__terrane_conversion_input_0)
                                        .map_err(|error| crate::TerraneForeignError(
                                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                        ))?,
                                ),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_trait_collision_impls",
                                    "terrane_trait_collision_impls::First::transform",
                                ),
                            );
                        }
                    };
                    Ok::<
                        _,
                        crate::TerraneForeignError,
                    >(terrane_int_support::Int::from(i128::from(value)))
                }
            }(),
            2 /* terrane-site: src/main.trn:8:23-8:42 */,
        )
    };
    second_value = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(22_i128);
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                <terrane_trait_collision_impls::Second as terrane_trait_collision_contract::ops::Transform>::transform(
                                    &second_2,
                                    terrane_int_support::coerce::<
                                        i64,
                                    >(&__terrane_conversion_input_0)
                                        .map_err(|error| crate::TerraneForeignError(
                                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                        ))?,
                                ),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_trait_collision_impls",
                                    "terrane_trait_collision_impls::Second::transform",
                                ),
                            );
                        }
                    };
                    Ok::<
                        _,
                        crate::TerraneForeignError,
                    >(terrane_int_support::Int::from(i128::from(value)))
                }
            }(),
            3 /* terrane-site: src/main.trn:9:24-9:44 */,
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&first_value), String::from(" "),
        terrane_scalar_support::scalar_text(&second_value)
    );
}
