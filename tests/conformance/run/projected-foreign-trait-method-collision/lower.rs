// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let first_2: First;
    let second_2: Second;
    let first_value: terrane_int_support::Int;
    let second_value: terrane_int_support::Int;
    first_2 = __terrane_raised(
        first(terrane_int_support::Int::from(1_i128)),
        0 /* terrane-site: src/main.trn:6:13-6:21 */,
    );
    second_2 = __terrane_raised(
        second(terrane_int_support::Int::from(2_i128)),
        1 /* terrane-site: src/main.trn:7:14-7:23 */,
    );
    first_value = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| <terrane_trait_collision_impls::First as terrane_trait_collision_contract::ops::Transform>::transform(
                &first_2,
                match || -> Result<_, crate::TerraneForeignError> {
                    Ok(
                        terrane_int_support::coerce::<
                            i64,
                        >(&terrane_int_support::Int::from(11_i128))
                            .map_err(|error| crate::TerraneForeignError(
                                crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                            ))?,
                    )
                }() {
                    Ok(value) => value,
                    Err(error) => std::panic::panic_any(error),
                },
            )),
        ) {
            Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_trait_collision_impls",
                        "terrane_trait_collision_impls::First::transform",
                    ),
                )
            }
        },
        2 /* terrane-site: src/main.trn:8:23-8:42 */,
    );
    second_value = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| <terrane_trait_collision_impls::Second as terrane_trait_collision_contract::ops::Transform>::transform(
                &second_2,
                match || -> Result<_, crate::TerraneForeignError> {
                    Ok(
                        terrane_int_support::coerce::<
                            i64,
                        >(&terrane_int_support::Int::from(22_i128))
                            .map_err(|error| crate::TerraneForeignError(
                                crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                            ))?,
                    )
                }() {
                    Ok(value) => value,
                    Err(error) => std::panic::panic_any(error),
                },
            )),
        ) {
            Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_trait_collision_impls",
                        "terrane_trait_collision_impls::Second::transform",
                    ),
                )
            }
        },
        3 /* terrane-site: src/main.trn:9:24-9:44 */,
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&first_value),
        terrane_scalar_support::scalar_text(&String::from(" ")),
        terrane_scalar_support::scalar_text(&second_value)
    );
}
