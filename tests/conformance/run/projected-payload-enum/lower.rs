// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF0S217 {
    Arm0(Event),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF0S217 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(_) => {
                unreachable!("semantic display refinement excludes this union arm")
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
fn main() {
    let text: TerraneUnionF0S217;
    let text_value: Option<String>;
    let binary: Event;
    let binary_value: Option<Vec<u8>>;
    let ping: Event;
    let pair_payload: EventPair;
    let pair: Event;
    let pair_value: Option<EventPair>;
    let named_payload: EventNamed;
    let named: Event;
    let named_value: Option<EventNamed>;
    let optional_payload: EventOptional;
    let optional: Event;
    let optional_value: Option<EventOptional>;
    let owned: Option<OwnedEvent>;
    let payload: Option<NonClonePayload>;
    let future: OpenEvent;
    let text_2: String;
    text = TerraneUnionF0S217::Arm0({
        let __terrane_conversion_input_0 = String::from("hello");
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                terrane_payload_enum_witness::Event::Text(
                                    __terrane_conversion_input_0,
                                ),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_payload_enum_witness",
                                    "terrane_payload_enum_witness::Event::Text",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),
            0 /* terrane-site: src/main.trn:6:10-6:30 */,
        )
    });
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(match &match &text { TerraneUnionF0S217::Arm0(value)
        => value, _ => unreachable!("flow-proven storage refinement") } {
        terrane_payload_enum_witness::Event::Text { .. } => "Text".to_owned(),
        terrane_payload_enum_witness::Event::Binary { .. } => "Binary".to_owned(),
        terrane_payload_enum_witness::Event::Ping { .. } => "Ping".to_owned(),
        terrane_payload_enum_witness::Event::Pair { .. } => "Pair".to_owned(),
        terrane_payload_enum_witness::Event::Named { .. } => "Named".to_owned(),
        terrane_payload_enum_witness::Event::Optional { .. } => "Optional".to_owned() })
        })) { Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane_payload_enum_witness",
        "terrane_payload_enum_witness::Event::variant-name")) }; Ok:: < _, crate
        ::TerraneForeignError > (value) } } (), 1 /* terrane-site: src/main.trn:7:11-7:29 */))
    );
    text_value = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            match match &text {
                                TerraneUnionF0S217::Arm0(value) => value,
                                _ => unreachable!("flow-proven storage refinement"),
                            }
                                .clone()
                            {
                                terrane_payload_enum_witness::Event::Text(value) => {
                                    Some(value)
                                }
                                _ => None,
                            },
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane_payload_enum_witness",
                                "terrane_payload_enum_witness::Event::into-Text",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        2 /* terrane-site: src/main.trn:8:28-8:43 */,
    );
    if (&text_value).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &text_value { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    binary = {
        let __terrane_conversion_input_0 = Vec::<u8>::from([65, 66]);
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                terrane_payload_enum_witness::Event::Binary(
                                    __terrane_conversion_input_0,
                                ),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_payload_enum_witness",
                                    "terrane_payload_enum_witness::Event::Binary",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),
            3 /* terrane-site: src/main.trn:11:12-11:32 */,
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(match &binary {
        terrane_payload_enum_witness::Event::Text { .. } => "Text".to_owned(),
        terrane_payload_enum_witness::Event::Binary { .. } => "Binary".to_owned(),
        terrane_payload_enum_witness::Event::Ping { .. } => "Ping".to_owned(),
        terrane_payload_enum_witness::Event::Pair { .. } => "Pair".to_owned(),
        terrane_payload_enum_witness::Event::Named { .. } => "Named".to_owned(),
        terrane_payload_enum_witness::Event::Optional { .. } => "Optional".to_owned() })
        })) { Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane_payload_enum_witness",
        "terrane_payload_enum_witness::Event::variant-name")) }; Ok:: < _, crate
        ::TerraneForeignError > (value) } } (), 4 /* terrane-site: src/main.trn:12:11-12:31 */))
    );
    binary_value = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            match binary {
                                terrane_payload_enum_witness::Event::Binary(value) => {
                                    Some(value)
                                }
                                _ => None,
                            },
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane_payload_enum_witness",
                                "terrane_payload_enum_witness::Event::into-Binary",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        5 /* terrane-site: src/main.trn:13:29-13:48 */,
    );
    if (&binary_value).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&(match &binary_value { Some(value)
            => value, _ => unreachable!("flow-proven storage refinement") } .len() as
            i128))
        );
    }
    ping = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(terrane_payload_enum_witness::Event::Ping) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane_payload_enum_witness",
                                "terrane_payload_enum_witness::Event::Ping",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        6 /* terrane-site: src/main.trn:16:10-16:22 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(match &ping {
        terrane_payload_enum_witness::Event::Text { .. } => "Text".to_owned(),
        terrane_payload_enum_witness::Event::Binary { .. } => "Binary".to_owned(),
        terrane_payload_enum_witness::Event::Ping { .. } => "Ping".to_owned(),
        terrane_payload_enum_witness::Event::Pair { .. } => "Pair".to_owned(),
        terrane_payload_enum_witness::Event::Named { .. } => "Named".to_owned(),
        terrane_payload_enum_witness::Event::Optional { .. } => "Optional".to_owned() })
        })) { Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane_payload_enum_witness",
        "terrane_payload_enum_witness::Event::variant-name")) }; Ok:: < _, crate
        ::TerraneForeignError > (value) } } (), 7 /* terrane-site: src/main.trn:17:11-17:29 */))
    );
    pair_payload = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(20_i128);
        let __terrane_conversion_input_1 = terrane_int_support::Int::from(22_i128);
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(
                    EventPair::terrane_construct(
                        terrane_int_support::coerce::<i64>(&__terrane_conversion_input_0)
                            .map_err(|error| crate::TerraneForeignError(
                                crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                            ))?,
                        terrane_int_support::coerce::<i64>(&__terrane_conversion_input_1)
                            .map_err(|error| crate::TerraneForeignError(
                                crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                            ))?,
                    ),
                )
            }(),
            8 /* terrane-site: src/main.trn:18:29-18:76 */,
        )
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(i128::from(*
        &pair_payload.item_n0)) + terrane_int_support::Int::from(i128::from(*
        &pair_payload.item_n1))))
    );
    pair = {
        let __terrane_conversion_input_0 = pair_payload;
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok({
                                let (field_0, field_1) = __terrane_conversion_input_0
                                    .terrane_into_fields();
                                terrane_payload_enum_witness::Event::Pair(field_0, field_1)
                            })
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_payload_enum_witness",
                                    "terrane_payload_enum_witness::Event::Pair",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),
            9 /* terrane-site: src/main.trn:20:16-20:41 */,
        )
    };
    pair_value = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            match pair {
                                terrane_payload_enum_witness::Event::Pair(
                                    field_0,
                                    field_1,
                                ) => {
                                    Some(
                                        __terrane_enum_payload_74657272616e655f7061796c6f61645f656e756d5f7769746e6573733a3a4576656e743a3a50616972237061796c6f6164(
                                            field_0,
                                            field_1,
                                        ),
                                    )
                                }
                                _ => None,
                            },
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane_payload_enum_witness",
                                "terrane_payload_enum_witness::Event::into-Pair",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        10 /* terrane-site: src/main.trn:21:32-21:47 */,
    );
    if (&pair_value).is_some() {
        println!("{}", String::from("pair"));
    }
    named_payload = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(42_i128);
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(
                    EventNamed::terrane_construct(
                        terrane_int_support::coerce::<i64>(&__terrane_conversion_input_0)
                            .map_err(|error| crate::TerraneForeignError(
                                crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                            ))?,
                    ),
                )
            }(),
            11 /* terrane-site: src/main.trn:24:31-24:63 */,
        )
    };
    println!("{}", terrane_scalar_support::scalar_text(&named_payload.value));
    named = {
        let __terrane_conversion_input_0 = named_payload;
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok({
                                let (field_0,) = __terrane_conversion_input_0
                                    .terrane_into_fields();
                                terrane_payload_enum_witness::Event::Named {
                                    r#value: field_0,
                                }
                            })
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_payload_enum_witness",
                                    "terrane_payload_enum_witness::Event::Named",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),
            12 /* terrane-site: src/main.trn:26:17-26:44 */,
        )
    };
    named_value = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            match named {
                                terrane_payload_enum_witness::Event::Named {
                                    r#value: field_0,
                                } => {
                                    Some(
                                        __terrane_enum_payload_74657272616e655f7061796c6f61645f656e756d5f7769746e6573733a3a4576656e743a3a4e616d6564237061796c6f6164(
                                            field_0,
                                        ),
                                    )
                                }
                                _ => None,
                            },
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane_payload_enum_witness",
                                "terrane_payload_enum_witness::Event::into-Named",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        13 /* terrane-site: src/main.trn:27:34-27:51 */,
    );
    if (&named_value).is_some() {
        println!("{}", String::from("named"));
    }
    optional_payload = {
        let __terrane_conversion_input_0 = Some(terrane_int_support::Int::from(42_i128));
        let __terrane_conversion_input_1 = Some(String::from("ok"));
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(
                    EventOptional::terrane_construct(
                        {
                            let __terrane_field_argument: Option<
                                terrane_int_support::Int,
                            > = __terrane_conversion_input_0;
                            __terrane_field_argument
                        }
                            .map(|value| -> Result<u32, crate::TerraneForeignError> {
                                Ok(
                                    terrane_int_support::coerce::<u32>(&value)
                                        .map_err(|error| crate::TerraneForeignError(
                                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                        ))?,
                                )
                            })
                            .transpose()?,
                        {
                            let __terrane_field_argument: Option<String> = __terrane_conversion_input_1;
                            __terrane_field_argument
                        },
                    ),
                )
            }(),
            14 /* terrane-site: src/main.trn:30:37-30:92 */,
        )
    };
    optional = {
        let __terrane_conversion_input_0 = optional_payload;
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok({
                                let (field_0, field_1) = __terrane_conversion_input_0
                                    .terrane_into_fields();
                                terrane_payload_enum_witness::Event::Optional {
                                    r#value: field_0,
                                    r#label: field_1,
                                }
                            })
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_payload_enum_witness",
                                    "terrane_payload_enum_witness::Event::Optional",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),
            15 /* terrane-site: src/main.trn:31:20-31:53 */,
        )
    };
    optional_value = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            match optional {
                                terrane_payload_enum_witness::Event::Optional {
                                    r#value: field_0,
                                    r#label: field_1,
                                } => {
                                    Some(
                                        __terrane_enum_payload_74657272616e655f7061796c6f61645f656e756d5f7769746e6573733a3a4576656e743a3a4f7074696f6e616c237061796c6f6164(
                                            field_0,
                                            field_1,
                                        ),
                                    )
                                }
                                _ => None,
                            },
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane_payload_enum_witness",
                                "terrane_payload_enum_witness::Event::into-Optional",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        16 /* terrane-site: src/main.trn:32:40-32:63 */,
    );
    if (&optional_value).is_some() {
        println!("{}", String::from("optional"));
    }
    if (&optional_value).is_some() {
        println!(
            "{}", { let mut __terrane_text = String::new(); match &match &optional_value
            { Some(value) => value, _ => unreachable!("flow-proven storage refinement") }
            .label_value { Some(__terrane_present) => {
            terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
            __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text
            }
        );
    }
    owned = Some({
        let __terrane_conversion_input_0 = {
            let __terrane_conversion_input_0 = String::from("owned");
            __terrane_raised(
                non_clone_payload(__terrane_conversion_input_0),
                17 /* terrane-site: src/main.trn:37:49-37:75 */,
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
                                terrane_payload_enum_witness::OwnedEvent::Payload(
                                    __terrane_conversion_input_0,
                                ),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_payload_enum_witness",
                                    "terrane_payload_enum_witness::OwnedEvent::Payload",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),
            18 /* terrane-site: src/main.trn:37:27-37:76 */,
        )
    });
    if (&owned).is_some() {
        payload = __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                match owned.expect("semantic optional narrowing") {
                                    terrane_payload_enum_witness::OwnedEvent::Payload(value) => {
                                        Some(value)
                                    }
                                    _ => None,
                                },
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_payload_enum_witness",
                                    "terrane_payload_enum_witness::OwnedEvent::into-Payload",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),
            19 /* terrane-site: src/main.trn:39:36-39:62 */,
        );
        if (&payload).is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&{ let
                __terrane_conversion_input_0 = payload
                .expect("semantic optional narrowing");
                __terrane_raised(consume_payload(__terrane_conversion_input_0),
                20 /* terrane-site: src/main.trn:41:15-41:44 */) })
            );
        }
    }
    future = __terrane_raised(
        future_event(),
        21 /* terrane-site: src/main.trn:42:22-42:35 */,
    );
    let __terrane_match_value_1661 = future.clone();
    match __terrane_match_value_1661 {
        terrane_payload_enum_witness::OpenEvent::Known(__terrane_pattern_1678_text) => {
            text_2 = __terrane_pattern_1678_text;
            println!("{}", terrane_scalar_support::scalar_text(&text_2));
        }
        _ => {
            println!("{}", String::from("future"));
        }
    }
}
