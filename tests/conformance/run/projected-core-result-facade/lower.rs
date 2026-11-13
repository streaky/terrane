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
        good = __terrane_raised(
            parse("41"),
            0 /* terrane-site: src/main.trn:9:16-9:27 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&good));
        let __terrane_completion_0: TerraneCompletion<()> = (|| {
            let __terrane_try_0: TerraneCompletion<()> = (|| {
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&__terrane_raised_completion!(parse("bad"),
                    1 /* terrane-site: src/main.trn:12:17-12:29 */))
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
                2 /* terrane-site: src/main.trn:15:24-15:35 */,
            )
        };
        println!("{}", terrane_scalar_support::scalar_text(&(&present).is_some()));
        absent = {
            let __terrane_conversion_input_0 = false;
            __terrane_raised(
                maybe(__terrane_conversion_input_0),
                3 /* terrane-site: src/main.trn:17:23-17:35 */,
            )
        };
        println!("{}", terrane_scalar_support::scalar_text(&(&absent).is_none()));
        chosen = {
            let __terrane_conversion_input_0 = true;
            __terrane_raised(
                pick(__terrane_conversion_input_0),
                4 /* terrane-site: src/main.trn:19:14-19:24 */,
            )
        };
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(describe(&chosen),
            5 /* terrane-site: src/main.trn:20:13-20:29 */))
        );
        nominal = {
            let __terrane_conversion_input_0 = false;
            __terrane_raised(
                pick(__terrane_conversion_input_0),
                6 /* terrane-site: src/main.trn:21:46-21:57 */,
            )
        };
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(describe(&nominal),
            7 /* terrane-site: src/main.trn:22:13-22:30 */))
        );
        letter = {
            let __terrane_conversion_input_0 = terrane_int_support::Int::from(65_i128);
            __terrane_raised(
                character(__terrane_conversion_input_0),
                8 /* terrane-site: src/main.trn:23:26-23:39 */,
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
                9 /* terrane-site: src/main.trn:25:27-25:45 */,
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
            __terrane_raised(text(__terrane_conversion_input_0), 10 /* terrane-site: src/main.trn:27:13-27:44 */) })
        );
        let __terrane_completion_1: TerraneCompletion<()> = (|| {
            let __terrane_try_1: TerraneCompletion<()> = (|| {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let
                    __terrane_conversion_input_0 = terrane_collection_support::List:: <
                    terrane_int_support::Int >
                    ::new(vec![terrane_int_support::Int::from(255_i128)]);
                    __terrane_raised_completion!(text(__terrane_conversion_input_0),
                    11 /* terrane-site: src/main.trn:29:17-29:34 */) })
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
            12 /* terrane-site: src/main.trn:32:13-32:27 */) }), String::from(":"),
            terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
            true; __terrane_raised(renamed(__terrane_conversion_input_0),
            13 /* terrane-site: src/main.trn:32:36-32:49 */) })
        );
        let __terrane_completion_2: TerraneCompletion<()> = (|| {
            let __terrane_try_2: TerraneCompletion<()> = (|| {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let
                    __terrane_conversion_input_0 = false;
                    __terrane_raised_completion!(external(__terrane_conversion_input_0),
                    14 /* terrane-site: src/main.trn:34:17-34:32 */) })
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
            15 /* terrane-site: src/main.trn:37:13-37:33 */) } {
            Some(__terrane_present) => {
            terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
            __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text
            }
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = false; __terrane_raised(external_maybe(__terrane_conversion_input_0),
            16 /* terrane-site: src/main.trn:38:13-38:34 */) } .is_none())
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0
            = false; let __terrane_conversion_input_1 = false;
            __terrane_raised(nested(__terrane_conversion_input_0,
            __terrane_conversion_input_1), 17 /* terrane-site: src/main.trn:39:13-39:33 */) } .is_none())
        );
        println!(
            "{}", { let mut __terrane_text = String::new(); match &{ let
            __terrane_conversion_input_0 = true; let __terrane_conversion_input_1 = true;
            __terrane_raised(nested(__terrane_conversion_input_0,
            __terrane_conversion_input_1), 18 /* terrane-site: src/main.trn:40:13-40:31 */) } { Some(__terrane_present) => {
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
                    __terrane_conversion_input_1), 19 /* terrane-site: src/main.trn:42:17-42:36 */) } { Some(__terrane_present) => {
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
            20 /* terrane-site: src/main.trn:45:13-45:20 */,
        );
        println!(
            "{}{}{}{}{}", terrane_scalar_support::scalar_text(&{ let
            __terrane_conversion_input_0 = true;
            __terrane_raised(terrane_static_trn_526561646572_open(__terrane_conversion_input_0),
            21 /* terrane-site: src/main.trn:46:13-46:31 */) }), String::from(":"),
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
            22 /* terrane-site: src/main.trn:46:40-46:56 */) }), String::from(":"),
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
            23 /* terrane-site: src/main.trn:46:71-46:95 */) } }). await,
            23 /* terrane-site: src/main.trn:46:71-46:95 */))
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
                    24 /* terrane-site: src/main.trn:48:17-48:34 */) })
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
                    __terrane_raised_err(__terrane_future. await, 25 /* terrane-site: src/main.trn:52:23-52:48 */) } }). await, 25 /* terrane-site: src/main.trn:52:23-52:48 */))
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
            26 /* terrane-site: src/main.trn:55:13-55:25 */) })
        );
        let __terrane_completion_6: TerraneCompletion<()> = (|| {
            let __terrane_try_6: TerraneCompletion<()> = (|| {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let
                    __terrane_conversion_input_0 = false;
                    __terrane_raised_completion!(opaque(__terrane_conversion_input_0),
                    27 /* terrane-site: src/main.trn:57:17-57:30 */) })
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
    });
}
