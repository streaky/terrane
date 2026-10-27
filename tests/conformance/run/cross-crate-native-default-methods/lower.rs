// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn update_twice(mutator: &mut Mutator) -> Result<u8, TerraneError> {
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised_err(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | < witness::Mutator as
        provider::MutableValue > ::update(&mut * mutator, match | | -> Result < _, crate
        ::TerraneForeignError > { Ok(terrane_int_support::coerce:: < u8 >
        (&terrane_int_support::Int::from(2_i128)).map_err(| error | crate
        ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error) }))) { Ok(Ok(value)) =>
        Ok(terrane_int_support::Int::from_u128(value as u128)), Ok(Err(error)) =>
        Err(crate ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
        ::TERRANE_DEPENDENCY_ERROR,
        format!("Rust dependency `{}` member `{}` failed: {error}", "witness",
        "witness::Mutator::update"), crate ::TERRANE_NO_SITE))), Err(payload) =>
        Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::Mutator::update")) }, 0 /* terrane-site: src/main.trn:7:13-7:30 */)
        ?)
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised_err(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | < witness::Mutator as
        provider::MutableValue > ::update(&mut * mutator, match | | -> Result < _, crate
        ::TerraneForeignError > { Ok(terrane_int_support::coerce:: < u8 >
        (&terrane_int_support::Int::from(3_i128)).map_err(| error | crate
        ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error) }))) { Ok(Ok(value)) =>
        Ok(terrane_int_support::Int::from_u128(value as u128)), Ok(Err(error)) =>
        Err(crate ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
        ::TERRANE_DEPENDENCY_ERROR,
        format!("Rust dependency `{}` member `{}` failed: {error}", "witness",
        "witness::Mutator::update"), crate ::TERRANE_NO_SITE))), Err(payload) =>
        Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::Mutator::update")) }, 1 /* terrane-site: src/main.trn:8:13-8:30 */)
        ?)
    );
    return Ok(
        __terrane_raised_err(
            terrane_int_support::coerce::<
                u8,
            >(
                &__terrane_raised_err(
                    match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| (&*mutator).value()),
                    ) {
                        Ok(value) => {
                            Ok(terrane_int_support::Int::from_u128(value as u128))
                        }
                        Err(payload) => {
                            Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "witness",
                                    "witness::Mutator::value",
                                ),
                            )
                        }
                    },
                    2 /* terrane-site: src/main.trn:9:12-9:26 */,
                )?,
            ),
            2 /* terrane-site: src/main.trn:9:12-9:26 */,
        )?,
    );
}
#[allow(dead_code)]
fn generic_panic_attributed(value: witness::Wrapped<u8>) -> bool {
    let mut wide: Option<u32> = None;
    let mut error: Option<TerraneError> = None;
    let __terrane_completion_0: TerraneCompletion<bool> = (|| {
        let __terrane_try_0: TerraneCompletion<bool> = (|| {
            let _ = wide
                .insert(
                    __terrane_raised_completion!(
                        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| <
                        witness::Wrapped < u8 > as provider::NativeValue > ::selected::<
                        u32 > (&value))) { Ok(value) => Ok(value), Err(payload) =>
                        Err(crate ::__terrane_dependency_panic(payload, "witness",
                        "witness::Wrapped::selected")) }, 3 /* terrane-site: src/main.trn:13:23-13:38 */
                    ),
                );
            return TerraneCompletion::Return(
                *wide.as_ref().expect("flow-proven available binding") == 0,
            );
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    __terrane_handled_0 = true;
                    let _ = error.insert(__terrane_error_0.clone());
                    return TerraneCompletion::Return(
                        error
                            .as_ref()
                            .expect("flow-proven available binding")
                            .clone()
                            .message()
                            .to_owned()
                            .contains(&String::from("Rust dependency `witness`"))
                            && error
                                .as_ref()
                                .expect("flow-proven available binding")
                                .clone()
                                .message()
                                .to_owned()
                                .contains(
                                    &String::from("member `witness::Wrapped::selected`"),
                                )
                            && error
                                .as_ref()
                                .expect("flow-proven available binding")
                                .clone()
                                .message()
                                .to_owned()
                                .contains(&String::from("generic selector panic")),
                    );
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {
            __terrane_generated_defect("non-fallthrough try completed normally")
        }
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
fn main() {
    let seed: u8;
    let value: witness::Wrapped<u8>;
    let owner: u8;
    let selected: u16;
    let overridden: Overridden;
    let explicit_owner: u8;
    let explicit_selected: u16;
    let mut mutator: Mutator;
    let mut error: Option<TerraneError> = None;
    seed = 9;
    value = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::Wrapped::<u8> { 0: seed }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::Wrapped::construct",
                    ),
                )
            }
        },
        4 /* terrane-site: src/main.trn:20:30-20:52 */,
    );
    owner = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| <witness::Wrapped<
                u8,
            > as provider::NativeValue>::owner_value(&value)),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::Wrapped::owner_value",
                    ),
                )
            }
        },
        5 /* terrane-site: src/main.trn:21:19-21:37 */,
    );
    selected = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| <witness::Wrapped<
                u8,
            > as provider::NativeValue>::selected::<u16>(&value)),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::Wrapped::selected",
                    ),
                )
            }
        },
        6 /* terrane-site: src/main.trn:22:23-22:38 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(2); __terrane_text.push_str("");
        terrane_scalar_support::ScalarDisplay::write_scalar(&owner, &mut __terrane_text);
        __terrane_text.push_str(":");
        terrane_scalar_support::ScalarDisplay::write_scalar(&selected, &mut
        __terrane_text); __terrane_text.push_str(":");
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | < witness::Wrapped < u8
        > as provider::NativeValue > ::label(&value))) { Ok(value) => Ok(value
        .to_owned()), Err(payload) => Err(crate ::__terrane_dependency_panic(payload,
        "witness", "witness::Wrapped::label")) }, 7 /* terrane-site: src/main.trn:23:49-23:61 */), &mut __terrane_text); __terrane_text })
    );
    overridden = __terrane_raised(
        terrane_static_trn_4f76657272696464656e_new(),
        8 /* terrane-site: src/main.trn:25:18-25:34 */,
    );
    explicit_owner = __terrane_raised(
        terrane_int_support::coerce::<
            u8,
        >(
            &__terrane_raised(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| <witness::Overridden as provider::NativeValue>::owner_value(
                        &overridden,
                    )),
                ) {
                    Ok(value) => Ok(terrane_int_support::Int::from_u128(value as u128)),
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::Overridden::owner_value",
                            ),
                        )
                    }
                },
                9 /* terrane-site: src/main.trn:26:28-26:51 */,
            ),
        ),
        9 /* terrane-site: src/main.trn:26:28-26:51 */,
    );
    explicit_selected = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| <witness::Overridden as provider::NativeValue>::selected::<
                u16,
            >(&overridden)),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::Overridden::selected",
                    ),
                )
            }
        },
        10 /* terrane-site: src/main.trn:27:32-27:52 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(2); __terrane_text.push_str("");
        terrane_scalar_support::ScalarDisplay::write_scalar(&explicit_owner, &mut
        __terrane_text); __terrane_text.push_str(":");
        terrane_scalar_support::ScalarDisplay::write_scalar(&explicit_selected, &mut
        __terrane_text); __terrane_text.push_str(":");
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | < witness::Overridden
        as provider::NativeValue > ::label(&overridden))) { Ok(value) => Ok(value
        .to_owned()), Err(payload) => Err(crate ::__terrane_dependency_panic(payload,
        "witness", "witness::Overridden::label")) }, 11 /* terrane-site: src/main.trn:28:67-28:84 */), &mut __terrane_text); __terrane_text })
    );
    mutator = __terrane_raised(
        terrane_static_trn_4d757461746f72_new(terrane_int_support::Int::from(10_i128)),
        12 /* terrane-site: src/main.trn:30:15-30:31 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced(update_twice(&mut
        mutator), 13 /* terrane-site: src/main.trn:31:13-31:34 */))
    );
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            __terrane_raised_completion!(
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | <
                witness::Mutator as provider::MutableValue > ::update(&mut mutator, match
                | | -> Result < _, crate ::TerraneForeignError > {
                Ok(terrane_int_support::coerce:: < u8 >
                (&terrane_int_support::Int::from(0_i128)).map_err(| error | crate
                ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
                ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
                std::panic::panic_any(error) }))) { Ok(Ok(value)) =>
                Ok(terrane_int_support::Int::from_u128(value as u128)), Ok(Err(error)) =>
                Err(crate ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
                ::TERRANE_DEPENDENCY_ERROR,
                format!("Rust dependency `{}` member `{}` failed: {error}", "witness",
                "witness::Mutator::update"), crate ::TERRANE_NO_SITE))), Err(payload) =>
                Err(crate ::__terrane_dependency_panic(payload, "witness",
                "witness::Mutator::update")) }, 14 /* terrane-site: src/main.trn:33:9-33:26 */
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_1 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_1) => {
                let mut __terrane_handled_1 = false;
                if !__terrane_handled_1 {
                    __terrane_handled_1 = true;
                    let _ = error.insert(__terrane_error_1.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&{ let mut
                        __terrane_text = String::with_capacity(21); __terrane_text
                        .push_str("native error caught: ");
                        terrane_scalar_support::ScalarDisplay::write_scalar(&error
                        .as_ref().expect("flow-proven available binding").clone()
                        .message().to_owned()
                        .contains(&String::from("zero update rejected")), &mut
                        __terrane_text); __terrane_text })
                    );
                }
                if !__terrane_handled_1 {
                    return TerraneCompletion::Error(__terrane_error_1);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_1 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
