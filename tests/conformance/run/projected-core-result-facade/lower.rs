// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    __terrane_run(async move {
        let good: terrane_int_support::Int;
        let mut error: Option<TerraneError> = None;
        let present: Option<terrane_int_support::Int>;
        let absent: Option<terrane_int_support::Int>;
        let chosen: facade::Either<i64, String>;
        let nominal: facade::Either<i64, String>;
        let letter: Option<String>;
        let invalid: Option<String>;
        let owner: Reader;
        let mut error_2: Option<TerraneError> = None;
        let elapsed: Duration;
        let constructed: Duration;
        let packet: facade::Response<i64>;
        let mut error_3: Option<TerraneError> = None;
        let mut error_4: Option<TerraneError> = None;
        let bridge: BridgeReader;
        let mut error_5: Option<TerraneError> = None;
        let mut error_6: Option<TerraneError> = None;
        good = __terrane_raised(
            parse("41"),
            0 /* terrane-site: src/main.trn:14:16-14:27 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&good));
        let __terrane_completion_0: TerraneCompletion<()> = (|| {
            let __terrane_try_0: TerraneCompletion<()> = (|| {
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&__terrane_raised_completion!(parse("bad"),
                    1 /* terrane-site: src/main.trn:17:17-17:29 */))
                );
                TerraneCompletion::Normal
            })();
            match __terrane_try_0 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_0) => {
                    let mut __terrane_handled_0 = false;
                    if !__terrane_handled_0 {
                        __terrane_handled_0 = true;
                        let _ = error.insert(__terrane_error_0.clone());
                        println!(
                            "{}", terrane_scalar_support::scalar_text(&{ let mut
                            __terrane_text = String::with_capacity(8); __terrane_text
                            .push_str("caught: "); { let __terrane_error_text_0 = error
                            .as_ref().expect("flow-proven available binding").message();
                            __terrane_text.push_str(__terrane_error_text_0); }
                            __terrane_text })
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
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        present = {
            let __terrane_conversion_input_0 = true;
            __terrane_raised(
                maybe(__terrane_conversion_input_0),
                2 /* terrane-site: src/main.trn:20:24-20:35 */,
            )
        };
        println!("{}", terrane_scalar_support::scalar_text(&(&present).is_some()));
        absent = {
            let __terrane_conversion_input_0 = false;
            __terrane_raised(
                maybe(__terrane_conversion_input_0),
                3 /* terrane-site: src/main.trn:22:23-22:35 */,
            )
        };
        println!("{}", terrane_scalar_support::scalar_text(&(&absent).is_none()));
        chosen = {
            let __terrane_conversion_input_0 = true;
            __terrane_raised(
                pick(__terrane_conversion_input_0),
                4 /* terrane-site: src/main.trn:24:14-24:24 */,
            )
        };
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(describe(&chosen),
            5 /* terrane-site: src/main.trn:25:13-25:29 */))
        );
        nominal = {
            let __terrane_conversion_input_0 = false;
            __terrane_raised(
                pick(__terrane_conversion_input_0),
                6 /* terrane-site: src/main.trn:26:46-26:57 */,
            )
        };
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(describe(&nominal),
            7 /* terrane-site: src/main.trn:27:13-27:30 */))
        );
        letter = {
            let __terrane_conversion_input_0 = terrane_int_support::Int::from(65_i128);
            __terrane_raised(
                character(__terrane_conversion_input_0),
                8 /* terrane-site: src/main.trn:28:26-28:39 */,
            )
        };
        println!(
            "{}", { let mut __terrane_text = String::new(); match &letter {
            Some(__terrane_present) => {
            terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
            __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text
            }
        );
        invalid = {
            let __terrane_conversion_input_0 = terrane_int_support::Int::from(
                1114112_i128,
            );
            __terrane_raised(
                character(__terrane_conversion_input_0),
                9 /* terrane-site: src/main.trn:30:27-30:45 */,
            )
        };
        println!("{}", terrane_scalar_support::scalar_text(&(&invalid).is_none()));
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = terrane_collection_support::List:: < terrane_int_support::Int >
            ::new(vec![terrane_int_support::Int::from(117_i128),
            terrane_int_support::Int::from(116_i128),
            terrane_int_support::Int::from(102_i128),
            terrane_int_support::Int::from(56_i128)]);
            __terrane_raised(text(__terrane_conversion_input_0), 10 /* terrane-site: src/main.trn:32:13-32:44 */) })
        );
        let __terrane_completion_1: TerraneCompletion<()> = (|| {
            let __terrane_try_1: TerraneCompletion<()> = (|| {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let
                    __terrane_conversion_input_0 = terrane_collection_support::List:: <
                    terrane_int_support::Int >
                    ::new(vec![terrane_int_support::Int::from(255_i128)]);
                    __terrane_raised_completion!(text(__terrane_conversion_input_0),
                    11 /* terrane-site: src/main.trn:34:17-34:34 */) })
                );
                TerraneCompletion::Normal
            })();
            match __terrane_try_1 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_1) => {
                    let mut __terrane_handled_1 = false;
                    if !__terrane_handled_1 {
                        __terrane_handled_1 = true;
                        println!("{}", String::from("invalid utf8 caught"));
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
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&{ let
            __terrane_conversion_input_0 = true;
            __terrane_raised(external(__terrane_conversion_input_0),
            12 /* terrane-site: src/main.trn:37:13-37:27 */) }), String::from(":"),
            terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
            true; __terrane_raised(renamed(__terrane_conversion_input_0),
            13 /* terrane-site: src/main.trn:37:36-37:49 */) })
        );
        let __terrane_completion_2: TerraneCompletion<()> = (|| {
            let __terrane_try_2: TerraneCompletion<()> = (|| {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let
                    __terrane_conversion_input_0 = false;
                    __terrane_raised_completion!(external(__terrane_conversion_input_0),
                    14 /* terrane-site: src/main.trn:39:17-39:32 */) })
                );
                TerraneCompletion::Normal
            })();
            match __terrane_try_2 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_2) => {
                    let mut __terrane_handled_2 = false;
                    if !__terrane_handled_2
                        && __terrane_error_2.kind
                            == TerraneErrorKind::Custom(DescriptorId(2))
                    {
                        __terrane_handled_2 = true;
                        println!("{}", String::from("typed external failure"));
                    }
                    if !__terrane_handled_2 {
                        return TerraneCompletion::Error(__terrane_error_2);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        match __terrane_completion_2 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        println!(
            "{}", { let mut __terrane_text = String::new(); match &{ let
            __terrane_conversion_input_0 = true;
            __terrane_raised(external_maybe(__terrane_conversion_input_0),
            15 /* terrane-site: src/main.trn:42:13-42:33 */) } {
            Some(__terrane_present) => {
            terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
            __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text
            }
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = false; __terrane_raised(external_maybe(__terrane_conversion_input_0),
            16 /* terrane-site: src/main.trn:43:13-43:34 */) } .is_none())
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = false; let __terrane_conversion_input_1 = false;
            __terrane_raised(nested(__terrane_conversion_input_0,
            __terrane_conversion_input_1), 17 /* terrane-site: src/main.trn:44:13-44:33 */) } .is_none())
        );
        println!(
            "{}", { let mut __terrane_text = String::new(); match &{ let
            __terrane_conversion_input_0 = true; let __terrane_conversion_input_1 = true;
            __terrane_raised(nested(__terrane_conversion_input_0,
            __terrane_conversion_input_1), 18 /* terrane-site: src/main.trn:45:13-45:31 */) } { Some(__terrane_present) => {
            terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
            __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text
            }
        );
        let __terrane_completion_3: TerraneCompletion<()> = (|| {
            let __terrane_try_3: TerraneCompletion<()> = (|| {
                println!(
                    "{}", { let mut __terrane_text = String::new(); match &{ let
                    __terrane_conversion_input_0 = true; let __terrane_conversion_input_1
                    = false;
                    __terrane_raised_completion!(nested(__terrane_conversion_input_0,
                    __terrane_conversion_input_1), 19 /* terrane-site: src/main.trn:47:17-47:36 */) } { Some(__terrane_present) => {
                    terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present,
                    &mut __terrane_text); }, None => __terrane_text.push_str("none"), }
                    __terrane_text }
                );
                TerraneCompletion::Normal
            })();
            match __terrane_try_3 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_3) => {
                    let mut __terrane_handled_3 = false;
                    if !__terrane_handled_3
                        && __terrane_error_3.kind
                            == TerraneErrorKind::Custom(DescriptorId(2))
                    {
                        __terrane_handled_3 = true;
                        println!("{}", String::from("typed nested failure"));
                    }
                    if !__terrane_handled_3 {
                        return TerraneCompletion::Error(__terrane_error_3);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        match __terrane_completion_3 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        owner = __terrane_raised(
            reader(),
            20 /* terrane-site: src/main.trn:50:13-50:20 */,
        );
        println!(
            "{}{}{}{}{}", terrane_scalar_support::scalar_text(&{ let
            __terrane_conversion_input_0 = true;
            __terrane_raised(terrane_static_trn_526561646572_open(__terrane_conversion_input_0),
            21 /* terrane-site: src/main.trn:51:13-51:31 */) }), String::from(":"),
            terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
            true; __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > {
            match match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
            Result < _, crate ::TerraneForeignError > { Ok((&owner)
            .read(__terrane_conversion_input_0)) })) { Ok(result) => result ?,
            Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "facade", "facade::Reader::read")) } { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(error) =>
            Err(crate ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
            ::DescriptorId(2), error.to_string(), crate ::TERRANE_NO_SITE))) } } (),
            22 /* terrane-site: src/main.trn:51:40-51:56 */) }), String::from(":"),
            terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await({ let
            __terrane_future = { let __terrane_call = (&owner).asynchronous(true);
            Box::pin(async move { match crate
            ::__terrane_dependency_await_unwind(__terrane_call). await .map_err(| payload
            | crate ::__terrane_dependency_panic(payload, "facade",
            "facade::Reader::asynchronous")) ? { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(error) =>
            Err(crate ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
            ::DescriptorId(2), error.to_string(), crate ::TERRANE_NO_SITE))) } }) };
            async move { __terrane_raised_err(__terrane_future. await,
            23 /* terrane-site: src/main.trn:51:71-51:95 */) } }). await,
            23 /* terrane-site: src/main.trn:51:71-51:95 */))
        );
        let __terrane_completion_4: TerraneCompletion<()> = (|| {
            let __terrane_try_4: TerraneCompletion<()> = (|| {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let
                    __terrane_conversion_input_0 = false; __terrane_raised_completion!(|
                    | -> Result < _, crate ::TerraneForeignError > { match match
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result <
                    _, crate ::TerraneForeignError > { Ok((&owner)
                    .read(__terrane_conversion_input_0)) })) { Ok(result) => result ?,
                    Err(payload) => return Err(crate
                    ::__terrane_dependency_panic(payload, "facade",
                    "facade::Reader::read")) } { Ok(value) =>
                    Ok(terrane_int_support::Int::from(i128::from(value))), Err(error) =>
                    Err(crate ::TerraneForeignError(crate
                    ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                    .to_string(), crate ::TERRANE_NO_SITE))) } } (),
                    24 /* terrane-site: src/main.trn:53:17-53:34 */) })
                );
                TerraneCompletion::Normal
            })();
            match __terrane_try_4 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_4) => {
                    let mut __terrane_handled_4 = false;
                    if !__terrane_handled_4
                        && __terrane_error_4.kind
                            == TerraneErrorKind::Custom(DescriptorId(2))
                    {
                        __terrane_handled_4 = true;
                        println!("{}", String::from("typed method failure"));
                    }
                    if !__terrane_handled_4 {
                        return TerraneCompletion::Error(__terrane_error_4);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        match __terrane_completion_4 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        let __terrane_completion_5: TerraneCompletion<()> = async {
            let __terrane_try_5: TerraneCompletion<()> = async {
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&__terrane_traced_completion!(__terrane_await({
                    let __terrane_future = { let __terrane_call = (&owner)
                    .asynchronous(false); Box::pin(async move { match crate
                    ::__terrane_dependency_await_unwind(__terrane_call). await .map_err(|
                    payload | crate ::__terrane_dependency_panic(payload, "facade",
                    "facade::Reader::asynchronous")) ? { Ok(value) =>
                    Ok(terrane_int_support::Int::from(i128::from(value))), Err(error) =>
                    Err(crate ::TerraneForeignError(crate
                    ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                    .to_string(), crate ::TERRANE_NO_SITE))) } }) }; async move {
                    __terrane_raised_err(__terrane_future. await, 25 /* terrane-site: src/main.trn:57:23-57:48 */) } }). await, 25 /* terrane-site: src/main.trn:57:23-57:48 */))
                );
                TerraneCompletion::Normal
            }
                .await;
            match __terrane_try_5 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_5) => {
                    let mut __terrane_handled_5 = false;
                    if !__terrane_handled_5
                        && __terrane_error_5.kind
                            == TerraneErrorKind::Custom(DescriptorId(2))
                    {
                        __terrane_handled_5 = true;
                        println!("{}", String::from("typed async failure"));
                    }
                    if !__terrane_handled_5 {
                        return TerraneCompletion::Error(__terrane_error_5);
                    }
                }
            }
            TerraneCompletion::Normal
        }
            .await;
        match __terrane_completion_5 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = true; __terrane_raised(opaque(__terrane_conversion_input_0),
            26 /* terrane-site: src/main.trn:60:13-60:25 */) })
        );
        let __terrane_completion_6: TerraneCompletion<()> = (|| {
            let __terrane_try_6: TerraneCompletion<()> = (|| {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let
                    __terrane_conversion_input_0 = false;
                    __terrane_raised_completion!(opaque(__terrane_conversion_input_0),
                    27 /* terrane-site: src/main.trn:62:17-62:30 */) })
                );
                TerraneCompletion::Normal
            })();
            match __terrane_try_6 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_6) => {
                    let mut __terrane_handled_6 = false;
                    if !__terrane_handled_6
                        && __terrane_error_6.kind
                            == TerraneErrorKind::Custom(DescriptorId(0))
                    {
                        __terrane_handled_6 = true;
                        let _ = error_2.insert(__terrane_error_6.clone());
                        println!(
                            "{}", terrane_scalar_support::scalar_text(&{ let
                            __terrane_error_text_1 = error_2.as_ref()
                            .expect("flow-proven available binding").message(); let
                            __terrane_string_argument_0 = "opaque alias failed";
                            __terrane_error_text_1.contains(std::convert::AsRef:: < str >
                            ::as_ref(&__terrane_string_argument_0)) })
                        );
                    }
                    if !__terrane_handled_6 {
                        return TerraneCompletion::Error(__terrane_error_6);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        match __terrane_completion_6 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        elapsed = __terrane_raised(
            duration(),
            28 /* terrane-site: src/main.trn:65:15-65:24 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&elapsed).as_secs()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "core",
            "core::time::Duration::as_secs")) }; Ok:: < _, crate ::TerraneForeignError >
            (terrane_int_support::Int::from_u128(value as u128)) } } (),
            29 /* terrane-site: src/main.trn:66:13-66:29 */))
        );
        constructed = {
            let __terrane_conversion_input_0 = terrane_int_support::Int::from(5_i128);
            __terrane_raised(
                terrane_static_trn_4475726174696f6e_from_secs(
                    __terrane_conversion_input_0,
                ),
                30 /* terrane-site: src/main.trn:67:19-67:41 */,
            )
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&constructed).as_secs()) })) { Ok(result)
            => result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "core",
            "core::time::Duration::as_secs")) }; Ok:: < _, crate ::TerraneForeignError >
            (terrane_int_support::Int::from_u128(value as u128)) } } (),
            31 /* terrane-site: src/main.trn:68:13-68:33 */))
        );
        packet = __terrane_raised(
            response(),
            32 /* terrane-site: src/main.trn:69:23-69:32 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&packet.value));
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = true; __terrane_raised(new_version(__terrane_conversion_input_0),
            33 /* terrane-site: src/main.trn:71:13-71:30 */) })
        );
        let __terrane_completion_7: TerraneCompletion<()> = (|| {
            let __terrane_try_7: TerraneCompletion<()> = (|| {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let
                    __terrane_conversion_input_0 = false;
                    __terrane_raised_completion!(new_version(__terrane_conversion_input_0),
                    34 /* terrane-site: src/main.trn:73:17-73:35 */) })
                );
                TerraneCompletion::Normal
            })();
            match __terrane_try_7 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_7) => {
                    let mut __terrane_handled_7 = false;
                    if !__terrane_handled_7
                        && __terrane_error_7.kind
                            == TerraneErrorKind::Custom(DescriptorId(3))
                    {
                        __terrane_handled_7 = true;
                        println!("{}", String::from("typed newer failure"));
                    }
                    if !__terrane_handled_7 {
                        return TerraneCompletion::Error(__terrane_error_7);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        match __terrane_completion_7 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = true; __terrane_raised(opaque_transitive(__terrane_conversion_input_0),
            35 /* terrane-site: src/main.trn:76:13-76:36 */) })
        );
        let __terrane_completion_8: TerraneCompletion<()> = (|| {
            let __terrane_try_8: TerraneCompletion<()> = (|| {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let
                    __terrane_conversion_input_0 = false;
                    __terrane_raised_completion!(opaque_transitive(__terrane_conversion_input_0),
                    36 /* terrane-site: src/main.trn:78:17-78:41 */) })
                );
                TerraneCompletion::Normal
            })();
            match __terrane_try_8 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_8) => {
                    let mut __terrane_handled_8 = false;
                    if !__terrane_handled_8
                        && __terrane_error_8.kind
                            == TerraneErrorKind::Custom(DescriptorId(0))
                    {
                        __terrane_handled_8 = true;
                        let _ = error_3.insert(__terrane_error_8.clone());
                        {
                            let __terrane_error_text_2 = error_3
                                .as_ref()
                                .expect("flow-proven available binding")
                                .message();
                            println!("{}", __terrane_error_text_2);
                        };
                    }
                    if !__terrane_handled_8 {
                        return TerraneCompletion::Error(__terrane_error_8);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        match __terrane_completion_8 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = true; __terrane_raised(public_transitive(__terrane_conversion_input_0),
            37 /* terrane-site: src/main.trn:81:13-81:36 */) })
        );
        let __terrane_completion_9: TerraneCompletion<()> = (|| {
            let __terrane_try_9: TerraneCompletion<()> = (|| {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let
                    __terrane_conversion_input_0 = false;
                    __terrane_raised_completion!(public_transitive(__terrane_conversion_input_0),
                    38 /* terrane-site: src/main.trn:83:17-83:41 */) })
                );
                TerraneCompletion::Normal
            })();
            match __terrane_try_9 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_9) => {
                    let mut __terrane_handled_9 = false;
                    if !__terrane_handled_9
                        && __terrane_error_9.kind
                            == TerraneErrorKind::Custom(DescriptorId(4))
                    {
                        __terrane_handled_9 = true;
                        let _ = error_4.insert(__terrane_error_9.clone());
                        println!(
                            "{}", terrane_scalar_support::scalar_text(&{ let mut
                            __terrane_text = String::with_capacity(19); __terrane_text
                            .push_str("public transitive: "); { let
                            __terrane_error_text_3 = error_4.as_ref()
                            .expect("flow-proven available binding").message();
                            __terrane_text.push_str(__terrane_error_text_3); }
                            __terrane_text })
                        );
                    }
                    if !__terrane_handled_9 {
                        return TerraneCompletion::Error(__terrane_error_9);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        match __terrane_completion_9 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = true; __terrane_raised(old_transitive(__terrane_conversion_input_0),
            39 /* terrane-site: src/main.trn:86:13-86:33 */) })
        );
        let __terrane_completion_10: TerraneCompletion<()> = (|| {
            let __terrane_try_10: TerraneCompletion<()> = (|| {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let
                    __terrane_conversion_input_0 = false;
                    __terrane_raised_completion!(old_transitive(__terrane_conversion_input_0),
                    40 /* terrane-site: src/main.trn:88:17-88:38 */) })
                );
                TerraneCompletion::Normal
            })();
            match __terrane_try_10 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_10) => {
                    let mut __terrane_handled_10 = false;
                    if !__terrane_handled_10
                        && __terrane_error_10.kind
                            == TerraneErrorKind::Custom(DescriptorId(5))
                    {
                        __terrane_handled_10 = true;
                        println!("{}", String::from("typed old hidden failure"));
                    }
                    if !__terrane_handled_10 {
                        return TerraneCompletion::Error(__terrane_error_10);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        match __terrane_completion_10 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        bridge = __terrane_raised(
            bridge_reader(),
            41 /* terrane-site: src/main.trn:91:14-91:28 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = true;
            __terrane_raised(terrane_static_trn_427269646765526561646572_opaque_static(__terrane_conversion_input_0),
            42 /* terrane-site: src/main.trn:92:13-92:46 */) })
        );
        let __terrane_completion_11: TerraneCompletion<()> = (|| {
            let __terrane_try_11: TerraneCompletion<()> = (|| {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let
                    __terrane_conversion_input_0 = false;
                    __terrane_raised_completion!(terrane_static_trn_427269646765526561646572_opaque_static(__terrane_conversion_input_0),
                    43 /* terrane-site: src/main.trn:94:17-94:51 */) })
                );
                TerraneCompletion::Normal
            })();
            match __terrane_try_11 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_11) => {
                    let mut __terrane_handled_11 = false;
                    if !__terrane_handled_11
                        && __terrane_error_11.kind
                            == TerraneErrorKind::Custom(DescriptorId(0))
                    {
                        __terrane_handled_11 = true;
                        let _ = error_5.insert(__terrane_error_11.clone());
                        println!(
                            "{}", terrane_scalar_support::scalar_text(&{ let
                            __terrane_error_text_4 = error_5.as_ref()
                            .expect("flow-proven available binding").message(); let
                            __terrane_string_argument_0 = "transitive hidden failure";
                            __terrane_error_text_4.contains(std::convert::AsRef:: < str >
                            ::as_ref(&__terrane_string_argument_0)) })
                        );
                    }
                    if !__terrane_handled_11 {
                        return TerraneCompletion::Error(__terrane_error_11);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        match __terrane_completion_11 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = true; __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > {
            match match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
            Result < _, crate ::TerraneForeignError > { Ok((&bridge)
            .typed(__terrane_conversion_input_0)) })) { Ok(result) => result ?,
            Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "facade", "facade::BridgeReader::typed")) } { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(error) =>
            Err(crate ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
            ::DescriptorId(4), error.to_string(), crate ::TERRANE_NO_SITE))) } } (),
            44 /* terrane-site: src/main.trn:97:13-97:31 */) })
        );
        let __terrane_completion_12: TerraneCompletion<()> = (|| {
            let __terrane_try_12: TerraneCompletion<()> = (|| {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let
                    __terrane_conversion_input_0 = false; __terrane_raised_completion!(|
                    | -> Result < _, crate ::TerraneForeignError > { match match
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result <
                    _, crate ::TerraneForeignError > { Ok((&bridge)
                    .typed(__terrane_conversion_input_0)) })) { Ok(result) => result ?,
                    Err(payload) => return Err(crate
                    ::__terrane_dependency_panic(payload, "facade",
                    "facade::BridgeReader::typed")) } { Ok(value) =>
                    Ok(terrane_int_support::Int::from(i128::from(value))), Err(error) =>
                    Err(crate ::TerraneForeignError(crate
                    ::TerraneError::custom_raised(crate ::DescriptorId(4), error
                    .to_string(), crate ::TERRANE_NO_SITE))) } } (),
                    45 /* terrane-site: src/main.trn:99:17-99:36 */) })
                );
                TerraneCompletion::Normal
            })();
            match __terrane_try_12 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_12) => {
                    let mut __terrane_handled_12 = false;
                    if !__terrane_handled_12
                        && __terrane_error_12.kind
                            == TerraneErrorKind::Custom(DescriptorId(4))
                    {
                        __terrane_handled_12 = true;
                        println!("{}", String::from("typed transitive method failure"));
                    }
                    if !__terrane_handled_12 {
                        return TerraneCompletion::Error(__terrane_error_12);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        match __terrane_completion_12 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await({
            let __terrane_future = { let __terrane_call = (&bridge).opaque_async(true);
            Box::pin(async move { match crate
            ::__terrane_dependency_await_unwind(__terrane_call). await .map_err(| payload
            | crate ::__terrane_dependency_panic(payload, "facade",
            "facade::BridgeReader::opaque_async")) ? { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(error) =>
            Err(crate ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
            ::TERRANE_DEPENDENCY_ERROR,
            format!("Rust dependency `{}` member `{}` failed: {error}", "facade",
            "facade::BridgeReader::opaque_async"), crate ::TERRANE_NO_SITE))) } }) };
            async move { __terrane_raised_err(__terrane_future. await,
            46 /* terrane-site: src/main.trn:102:19-102:44 */) } }). await,
            46 /* terrane-site: src/main.trn:102:19-102:44 */))
        );
        let __terrane_completion_13: TerraneCompletion<()> = async {
            let __terrane_try_13: TerraneCompletion<()> = async {
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&__terrane_traced_completion!(__terrane_await({
                    let __terrane_future = { let __terrane_call = (&bridge)
                    .opaque_async(false); Box::pin(async move { match crate
                    ::__terrane_dependency_await_unwind(__terrane_call). await .map_err(|
                    payload | crate ::__terrane_dependency_panic(payload, "facade",
                    "facade::BridgeReader::opaque_async")) ? { Ok(value) =>
                    Ok(terrane_int_support::Int::from(i128::from(value))), Err(error) =>
                    Err(crate ::TerraneForeignError(crate
                    ::TerraneError::custom_raised(crate ::TERRANE_DEPENDENCY_ERROR,
                    format!("Rust dependency `{}` member `{}` failed: {error}", "facade",
                    "facade::BridgeReader::opaque_async"), crate ::TERRANE_NO_SITE))) }
                    }) }; async move { __terrane_raised_err(__terrane_future. await,
                    47 /* terrane-site: src/main.trn:104:23-104:49 */) } }). await,
                    47 /* terrane-site: src/main.trn:104:23-104:49 */))
                );
                TerraneCompletion::Normal
            }
                .await;
            match __terrane_try_13 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_13) => {
                    let mut __terrane_handled_13 = false;
                    if !__terrane_handled_13
                        && __terrane_error_13.kind
                            == TerraneErrorKind::Custom(DescriptorId(0))
                    {
                        __terrane_handled_13 = true;
                        let _ = error_6.insert(__terrane_error_13.clone());
                        println!(
                            "{}", terrane_scalar_support::scalar_text(&{ let
                            __terrane_error_text_5 = error_6.as_ref()
                            .expect("flow-proven available binding").message(); let
                            __terrane_string_argument_0 = "transitive hidden failure";
                            __terrane_error_text_5.contains(std::convert::AsRef:: < str >
                            ::as_ref(&__terrane_string_argument_0)) })
                        );
                    }
                    if !__terrane_handled_13 {
                        return TerraneCompletion::Error(__terrane_error_13);
                    }
                }
            }
            TerraneCompletion::Normal
        }
            .await;
        match __terrane_completion_13 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
    });
}
