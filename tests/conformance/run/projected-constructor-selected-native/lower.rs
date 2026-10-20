// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let address: Address;
    let mut envelope: witness::Envelope<witness::Address>;
    let text: witness::Envelope<String>;
    let frame: witness::Frame<witness::Address>;
    let restored: Address;
    let pair: witness::Pair<String, witness::Address>;
    let maybe: witness::Maybe<witness::Address>;
    let printable: witness::Printable<String>;
    let raw: String;
    let seed: i64;
    let mut numeric: witness::Envelope<i64>;
    let flag: witness::Envelope<bool>;
    let explicit: witness::Envelope<witness::Address>;
    let selected: witness::Frame<witness::Address>;
    address = witness::Address { port: 8080 };
    envelope = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::Envelope::<witness::Address> {
                payload: address.clone(),
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "Envelope::terrane_construct",
                    ),
                )
            }
        },
        0 /* terrane-site: src/main.trn:8:25-8:61 */,
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(address_port(&envelope),
        1 /* terrane-site: src/main.trn:9:13-9:35 */))
    );
    println!("{}", terrane_scalar_support::scalar_text(&envelope.payload.port));
    {
        let __terrane_value = witness::Address { port: 8081 };
        envelope.payload = __terrane_value;
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(address_port(&envelope),
        2 /* terrane-site: src/main.trn:12:13-12:35 */))
    );
    text = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::Envelope::<String> {
                payload: String::from("hello"),
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "Envelope::terrane_construct",
                    ),
                )
            }
        },
        3 /* terrane-site: src/main.trn:13:12-13:48 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(text_length(&text),
        4 /* terrane-site: src/main.trn:14:13-14:30 */))
    );
    frame = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::Frame::<witness::Address> {
                0: address.clone(),
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "Frame::terrane_construct",
                    ),
                )
            }
        },
        5 /* terrane-site: src/main.trn:15:13-15:36 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(frame_port(&frame),
        6 /* terrane-site: src/main.trn:16:13-16:30 */))
    );
    restored = __terrane_raised(
        into_address(envelope),
        7 /* terrane-site: src/main.trn:17:16-17:43 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&restored.port));
    pair = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::Pair::<String, witness::Address> {
                first: String::from("selected"),
                second: address.clone(),
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "Pair::terrane_construct",
                    ),
                )
            }
        },
        8 /* terrane-site: src/main.trn:19:12-19:63 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(pair_summary(&pair),
        9 /* terrane-site: src/main.trn:20:13-20:31 */))
    );
    maybe = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::Maybe::<witness::Address> {
                label: String::from("optional"),
                payload: address.clone().into(),
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "Maybe::terrane_construct",
                    ),
                )
            }
        },
        10 /* terrane-site: src/main.trn:21:13-21:73 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(maybe_port(&maybe),
        11 /* terrane-site: src/main.trn:22:13-22:30 */))
    );
    printable = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::Printable::<String> {
                payload: String::from("display"),
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "Printable::terrane_construct",
                    ),
                )
            }
        },
        12 /* terrane-site: src/main.trn:23:17-23:56 */,
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(printable_text(&printable),
        13 /* terrane-site: src/main.trn:24:13-24:38 */))
    );
    raw = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| text.into_payload()),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::Envelope::into_payload",
                    ),
                )
            }
        },
        14 /* terrane-site: src/main.trn:25:11-25:29 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&raw));
    seed = 41;
    numeric = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::Envelope::<i64> {
                payload: match || -> Result<_, crate::TerraneForeignError> {
                    Ok(
                        terrane_int_support::coerce::<
                            i64,
                        >(&terrane_int_support::Int::from(seed as i128))
                            .map_err(|error| crate::TerraneForeignError(
                                crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                            ))?,
                    )
                }() {
                    Ok(value) => value,
                    Err(error) => std::panic::panic_any(error),
                },
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "Envelope::terrane_construct",
                    ),
                )
            }
        },
        15 /* terrane-site: src/main.trn:28:15-28:48 */,
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(i128::from(*
        &numeric.payload)) + terrane_int_support::Int::from(1_i128)))
    );
    {
        let __terrane_value = __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                let __terrane_field_value = terrane_int_support::Int::from(
                    i128::from(*&numeric.payload),
                ) + terrane_int_support::Int::from(2_i128);
                Ok(
                    terrane_int_support::coerce::<i64>(&__terrane_field_value)
                        .map_err(|error| crate::TerraneForeignError(
                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                        ))?,
                )
            }(),
            16 /* terrane-site: src/main.trn:30:5-30:42 */,
        );
        numeric.payload = __terrane_value;
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(i128::from(*
        &numeric.payload)) + terrane_int_support::Int::from(1_i128)))
    );
    flag = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::Envelope::<bool> {
                payload: true,
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "Envelope::terrane_construct",
                    ),
                )
            }
        },
        17 /* terrane-site: src/main.trn:32:12-32:45 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&flag.payload));
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            {
                let __terrane_value = __terrane_raised_completion!(
                    | | -> Result < _, crate ::TerraneForeignError > { let
                    __terrane_field_value =
                    terrane_int_support::Int::from(9223372036854775808_i128);
                    Ok(terrane_int_support::coerce:: < i64 > (&__terrane_field_value)
                    .map_err(| error | crate ::TerraneForeignError(crate
                    ::TerraneRaised::raised(error, crate ::TERRANE_NO_SITE))) ?) } (),
                    18 /* terrane-site: src/main.trn:35:9-35:46 */
                );
                numeric.payload = __terrane_value;
            }
            TerraneCompletion::Normal
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
                        == TerraneErrorKind::IntegerConversionOverflow
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("native range rejected"));
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    println!("{}", terrane_scalar_support::scalar_text(&numeric.payload));
    explicit = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::Envelope::<witness::Address> {
                payload: address.clone(),
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "Envelope::terrane_construct",
                    ),
                )
            }
        },
        19 /* terrane-site: src/main.trn:39:36-39:72 */,
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(address_port(&explicit),
        20 /* terrane-site: src/main.trn:40:13-40:35 */))
    );
    selected = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::Frame::<witness::Address> {
                0: address,
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "Frame::terrane_construct",
                    ),
                )
            }
        },
        21 /* terrane-site: src/main.trn:41:16-41:52 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&selected.0.port));
}
