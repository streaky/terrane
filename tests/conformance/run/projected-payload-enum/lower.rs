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
    text = TerraneUnionF0S217::Arm0(
        __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| terrane_payload_enum_witness::Event::Text(
                    String::from("hello"),
                )),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "terrane_payload_enum_witness",
                            "terrane_payload_enum_witness::Event::Text",
                        ),
                    )
                }
            },
            0 /* terrane-site: src/main.trn:6:10-6:30 */,
        ),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | match &match &text {
        TerraneUnionF0S217::Arm0(value) => value, _ =>
        unreachable!("flow-proven storage refinement") } {
        terrane_payload_enum_witness::Event::Text { .. } => "Text".to_owned(),
        terrane_payload_enum_witness::Event::Binary { .. } => "Binary".to_owned(),
        terrane_payload_enum_witness::Event::Ping { .. } => "Ping".to_owned(),
        terrane_payload_enum_witness::Event::Pair { .. } => "Pair".to_owned(),
        terrane_payload_enum_witness::Event::Named { .. } => "Named".to_owned(),
        terrane_payload_enum_witness::Event::Optional { .. } => "Optional".to_owned() }))
        { Ok(value) => Ok(value), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "terrane_payload_enum_witness",
        "terrane_payload_enum_witness::Event::variant-name")) }, 1 /* terrane-site: src/main.trn:7:11-7:29 */))
    );
    text_value = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| match match &text {
                TerraneUnionF0S217::Arm0(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            }
                .clone()
            {
                terrane_payload_enum_witness::Event::Text(value) => Some(value),
                _ => None,
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_payload_enum_witness",
                        "terrane_payload_enum_witness::Event::into-Text",
                    ),
                )
            }
        },
        2 /* terrane-site: src/main.trn:8:28-8:43 */,
    );
    if text_value.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &text_value { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    binary = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| terrane_payload_enum_witness::Event::Binary(
                Vec::from([65, 66]),
            )),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_payload_enum_witness",
                        "terrane_payload_enum_witness::Event::Binary",
                    ),
                )
            }
        },
        3 /* terrane-site: src/main.trn:11:12-11:32 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | match &binary {
        terrane_payload_enum_witness::Event::Text { .. } => "Text".to_owned(),
        terrane_payload_enum_witness::Event::Binary { .. } => "Binary".to_owned(),
        terrane_payload_enum_witness::Event::Ping { .. } => "Ping".to_owned(),
        terrane_payload_enum_witness::Event::Pair { .. } => "Pair".to_owned(),
        terrane_payload_enum_witness::Event::Named { .. } => "Named".to_owned(),
        terrane_payload_enum_witness::Event::Optional { .. } => "Optional".to_owned() }))
        { Ok(value) => Ok(value), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "terrane_payload_enum_witness",
        "terrane_payload_enum_witness::Event::variant-name")) }, 4 /* terrane-site: src/main.trn:12:11-12:31 */))
    );
    binary_value = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| match binary {
                terrane_payload_enum_witness::Event::Binary(value) => Some(value),
                _ => None,
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_payload_enum_witness",
                        "terrane_payload_enum_witness::Event::into-Binary",
                    ),
                )
            }
        },
        5 /* terrane-site: src/main.trn:13:29-13:48 */,
    );
    if binary_value.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&(match &binary_value { Some(value)
            => value, _ => unreachable!("flow-proven storage refinement") } .len() as
            i128))
        );
    }
    ping = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| terrane_payload_enum_witness::Event::Ping),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_payload_enum_witness",
                        "terrane_payload_enum_witness::Event::Ping",
                    ),
                )
            }
        },
        6 /* terrane-site: src/main.trn:16:10-16:22 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | match &ping {
        terrane_payload_enum_witness::Event::Text { .. } => "Text".to_owned(),
        terrane_payload_enum_witness::Event::Binary { .. } => "Binary".to_owned(),
        terrane_payload_enum_witness::Event::Ping { .. } => "Ping".to_owned(),
        terrane_payload_enum_witness::Event::Pair { .. } => "Pair".to_owned(),
        terrane_payload_enum_witness::Event::Named { .. } => "Named".to_owned(),
        terrane_payload_enum_witness::Event::Optional { .. } => "Optional".to_owned() }))
        { Ok(value) => Ok(value), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "terrane_payload_enum_witness",
        "terrane_payload_enum_witness::Event::variant-name")) }, 7 /* terrane-site: src/main.trn:17:11-17:29 */))
    );
    pair_payload = EventPair::terrane_construct(20, 22);
    println!(
        "{}", terrane_scalar_support::scalar_text(&(pair_payload.item_n0.clone() +
        pair_payload.item_n1.clone()))
    );
    pair = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let (field_0, field_1) = pair_payload.terrane_into_fields();
                terrane_payload_enum_witness::Event::Pair(field_0, field_1)
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_payload_enum_witness",
                        "terrane_payload_enum_witness::Event::Pair",
                    ),
                )
            }
        },
        8 /* terrane-site: src/main.trn:20:16-20:41 */,
    );
    pair_value = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| match pair {
                terrane_payload_enum_witness::Event::Pair(field_0, field_1) => {
                    Some(
                        __terrane_enum_payload_74657272616e655f7061796c6f61645f656e756d5f7769746e6573733a3a4576656e743a3a50616972237061796c6f6164(
                            field_0,
                            field_1,
                        ),
                    )
                }
                _ => None,
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_payload_enum_witness",
                        "terrane_payload_enum_witness::Event::into-Pair",
                    ),
                )
            }
        },
        9 /* terrane-site: src/main.trn:21:32-21:47 */,
    );
    if pair_value.is_some() {
        println!("{}", String::from("pair"));
    }
    named_payload = EventNamed::terrane_construct(42);
    println!("{}", terrane_scalar_support::scalar_text(&named_payload.value));
    named = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let (field_0,) = named_payload.terrane_into_fields();
                terrane_payload_enum_witness::Event::Named {
                    r#value: field_0,
                }
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_payload_enum_witness",
                        "terrane_payload_enum_witness::Event::Named",
                    ),
                )
            }
        },
        10 /* terrane-site: src/main.trn:26:17-26:44 */,
    );
    named_value = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| match named {
                terrane_payload_enum_witness::Event::Named { r#value: field_0 } => {
                    Some(
                        __terrane_enum_payload_74657272616e655f7061796c6f61645f656e756d5f7769746e6573733a3a4576656e743a3a4e616d6564237061796c6f6164(
                            field_0,
                        ),
                    )
                }
                _ => None,
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_payload_enum_witness",
                        "terrane_payload_enum_witness::Event::into-Named",
                    ),
                )
            }
        },
        11 /* terrane-site: src/main.trn:27:34-27:51 */,
    );
    if named_value.is_some() {
        println!("{}", String::from("named"));
    }
    optional_payload = EventOptional::terrane_construct(42, None::<String>);
    optional = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let (field_0, field_1) = optional_payload.terrane_into_fields();
                terrane_payload_enum_witness::Event::Optional {
                    r#value: field_0,
                    r#label: field_1,
                }
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_payload_enum_witness",
                        "terrane_payload_enum_witness::Event::Optional",
                    ),
                )
            }
        },
        12 /* terrane-site: src/main.trn:31:20-31:53 */,
    );
    optional_value = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| match optional {
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
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_payload_enum_witness",
                        "terrane_payload_enum_witness::Event::into-Optional",
                    ),
                )
            }
        },
        13 /* terrane-site: src/main.trn:32:40-32:63 */,
    );
    if optional_value.is_some() {
        println!("{}", String::from("optional"));
    }
    owned = Some(
        __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| terrane_payload_enum_witness::OwnedEvent::Payload(
                    __terrane_raised(
                        non_clone_payload(String::from("owned")),
                        14 /* terrane-site: src/main.trn:35:49-35:75 */,
                    ),
                )),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "terrane_payload_enum_witness",
                            "terrane_payload_enum_witness::OwnedEvent::Payload",
                        ),
                    )
                }
            },
            15 /* terrane-site: src/main.trn:35:27-35:76 */,
        ),
    );
    if owned.is_some() {
        payload = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| match owned
                    .expect("semantic optional narrowing")
                {
                    terrane_payload_enum_witness::OwnedEvent::Payload(value) => {
                        Some(value)
                    }
                    _ => None,
                }),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "terrane_payload_enum_witness",
                            "terrane_payload_enum_witness::OwnedEvent::into-Payload",
                        ),
                    )
                }
            },
            16 /* terrane-site: src/main.trn:37:36-37:62 */,
        );
        if payload.is_some() {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_raised(consume_payload(payload
                .expect("semantic optional narrowing")), 17 /* terrane-site: src/main.trn:39:15-39:44 */))
            );
        }
    }
    future = __terrane_raised(
        future_event(),
        18 /* terrane-site: src/main.trn:40:22-40:35 */,
    );
    let __terrane_match_value_1589 = future.clone();
    match __terrane_match_value_1589 {
        terrane_payload_enum_witness::OpenEvent::Known(__terrane_pattern_1606_text) => {
            text_2 = __terrane_pattern_1606_text;
            println!("{}", terrane_scalar_support::scalar_text(&text_2));
        }
        _ => {
            println!("{}", String::from("future"));
        }
    }
}
