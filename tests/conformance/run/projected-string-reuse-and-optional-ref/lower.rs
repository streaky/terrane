// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn parse_twice(text: String) -> terrane_int_support::Int {
    let mut __trn_5f6669727374: Option<HeaderValue> = None;
    let mut __trn_5f7365636f6e64: Option<HeaderValue> = None;
    let __terrane_completion_0: TerraneCompletion<terrane_int_support::Int> = (|| {
        let __terrane_try_0: TerraneCompletion<terrane_int_support::Int> = (|| {
            let _ = __trn_5f6669727374
                .insert(
                    __terrane_raised_completion!(
                        terrane_static_trn_48656164657256616c7565_from_str(text.clone()),
                        0 /* terrane-site: src/main.trn:8:18-8:45 */
                    ),
                );
            let _ = &__trn_5f6669727374;
            return TerraneCompletion::Return(terrane_int_support::Int::from(1_i128));
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0
                    && __terrane_error_0.kind
                        == TerraneErrorKind::Custom(DescriptorId(2))
                {
                    __terrane_handled_0 = true;
                    let __terrane_completion_1: TerraneCompletion<
                        terrane_int_support::Int,
                    > = (|| {
                        let __terrane_try_1: TerraneCompletion<
                            terrane_int_support::Int,
                        > = (|| {
                            let _ = __trn_5f7365636f6e64
                                .insert(
                                    __terrane_raised_completion!(
                                        terrane_static_trn_48656164657256616c7565_from_str(text
                                        .clone()), 1 /* terrane-site: src/main.trn:12:23-12:50 */
                                    ),
                                );
                            let _ = &__trn_5f7365636f6e64;
                            return TerraneCompletion::Return(
                                terrane_int_support::Int::from(2_i128),
                            );
                        })();
                        match __terrane_try_1 {
                            TerraneCompletion::Return(value) => {
                                return TerraneCompletion::Return(value);
                            }
                            TerraneCompletion::Break => return TerraneCompletion::Break,
                            TerraneCompletion::Continue => {
                                return TerraneCompletion::Continue;
                            }
                            TerraneCompletion::Normal => {}
                            TerraneCompletion::Error(__terrane_error_1) => {
                                let mut __terrane_handled_1 = false;
                                if !__terrane_handled_1
                                    && __terrane_error_1.kind
                                        == TerraneErrorKind::Custom(DescriptorId(2))
                                {
                                    __terrane_handled_1 = true;
                                    if text.contains(&String::from("\n")) {
                                        return TerraneCompletion::Return(
                                            terrane_int_support::Int::from(3_i128),
                                        );
                                    }
                                    return TerraneCompletion::Return(
                                        terrane_int_support::Int::from(4_i128),
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
                        TerraneCompletion::Normal => {
                            __terrane_generated_defect(
                                "non-fallthrough try completed normally",
                            )
                        }
                        TerraneCompletion::Return(value) => {
                            return TerraneCompletion::Return(value);
                        }
                        TerraneCompletion::Error(error) => {
                            return TerraneCompletion::Error(error);
                        }
                        TerraneCompletion::Break | TerraneCompletion::Continue => {
                            __terrane_generated_defect(
                                "loop control escaped a non-loop try",
                            )
                        }
                    }
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
fn sensitive(header: &HeaderValue) -> bool {
    return __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| (&header).is_sensitive()),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "http",
                        "http::HeaderValue::is_sensitive",
                    ),
                )
            }
        },
        2 /* terrane-site: src/main.trn:20:12-20:32 */,
    );
}
fn mark_sensitive(header: &mut HeaderValue) {
    __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let _ = header.set_sensitive(true);
            }),
        ) {
            Ok(()) => Ok(()),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "http",
                        "http::HeaderValue::set_sensitive",
                    ),
                )
            }
        },
        3 /* terrane-site: src/main.trn:23:5-23:31 */,
    );
}
fn main() {
    let mut header: Option<HeaderValue>;
    println!(
        "{}", terrane_scalar_support::scalar_text(&parse_twice(String::from("valid")))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&parse_twice(String::from("bad\nvalue")))
    );
    header = Some(
        __terrane_raised(
            terrane_static_trn_48656164657256616c7565_from_str(String::from("value")),
            4 /* terrane-site: src/main.trn:28:31-28:61 */,
        ),
    );
    if header.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&sensitive(match &header {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") }))
        );
        mark_sensitive(
            match &mut header {
                Some(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            },
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&sensitive(match &header {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") }))
        );
    } else {
        println!("{}", String::from("unexpected absence"));
    }
}
