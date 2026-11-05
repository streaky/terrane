// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: grouped-catch
#[derive(Clone)]
pub struct FirstError {
    pub message: String,
}
impl FirstError {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("first"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
#[derive(Clone)]
pub struct SecondError {
    pub message: String,
}
impl SecondError {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("second"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
#[derive(Clone)]
pub struct ThirdError {
    pub message: String,
}
impl ThirdError {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("third"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
#[derive(Clone)]
pub struct OtherError {
    pub message: String,
}
impl OtherError {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("other"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
fn fail(choice: terrane_int_support::Int) -> Result<(), TerraneError> {
    if choice == terrane_int_support::Int::from(1_i128) {
        return Err({
            let value = FirstError::terrane_construct();
            let rendered = value.render();
            let original = &value.message;
            let (message, rendered) = if &rendered == original {
                (rendered, None)
            } else {
                (original.clone(), Some(rendered))
            };
            TerraneError::raised_with_rendered_message(
                TerraneErrorKind::Custom(DescriptorId(0)),
                message,
                rendered,
                0 /* terrane-site: case.trn:26:9-26:26 */,
            )
        });
    }
    if choice == terrane_int_support::Int::from(2_i128) {
        return Err({
            let value = SecondError::terrane_construct();
            let rendered = value.render();
            let original = &value.message;
            let (message, rendered) = if &rendered == original {
                (rendered, None)
            } else {
                (original.clone(), Some(rendered))
            };
            TerraneError::raised_with_rendered_message(
                TerraneErrorKind::Custom(DescriptorId(1)),
                message,
                rendered,
                1 /* terrane-site: case.trn:28:9-28:27 */,
            )
        });
    }
    if choice == terrane_int_support::Int::from(3_i128) {
        return Err({
            let value = ThirdError::terrane_construct();
            let rendered = value.render();
            let original = &value.message;
            let (message, rendered) = if &rendered == original {
                (rendered, None)
            } else {
                (original.clone(), Some(rendered))
            };
            TerraneError::raised_with_rendered_message(
                TerraneErrorKind::Custom(DescriptorId(2)),
                message,
                rendered,
                2 /* terrane-site: case.trn:30:9-30:26 */,
            )
        });
    }
    return Err({
        let value = OtherError::terrane_construct();
        let rendered = value.render();
        let original = &value.message;
        let (message, rendered) = if &rendered == original {
            (rendered, None)
        } else {
            (original.clone(), Some(rendered))
        };
        TerraneError::raised_with_rendered_message(
            TerraneErrorKind::Custom(DescriptorId(3)),
            message,
            rendered,
            3 /* terrane-site: case.trn:31:5-31:22 */,
        )
    });
}
fn handled(choice: terrane_int_support::Int) -> Result<String, TerraneError> {
    let mut error: Option<TerraneError> = None;
    let __terrane_completion_0: TerraneCompletion<String> = (|| {
        let __terrane_try_0: TerraneCompletion<String> = (|| {
            __terrane_traced_completion!(
                fail(choice.clone()), 4 /* terrane-site: case.trn:35:9-35:21 */
            );
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
                    && (__terrane_error_0.kind
                        == TerraneErrorKind::Custom(DescriptorId(0))
                        || __terrane_error_0.kind
                            == TerraneErrorKind::Custom(DescriptorId(1))
                        || __terrane_error_0.kind
                            == TerraneErrorKind::Custom(DescriptorId(2)))
                {
                    __terrane_handled_0 = true;
                    let _ = error.insert(__terrane_error_0.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&error.as_ref()
                        .expect("flow-proven available binding").message().to_owned())
                    );
                    return TerraneCompletion::Return(
                        error
                            .as_ref()
                            .expect("flow-proven available binding")
                            .message()
                            .to_owned(),
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
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    return Ok(String::from("unexpected"));
}
fn reraised(choice: terrane_int_support::Int) -> Result<(), TerraneError> {
    let mut error: Option<TerraneError> = None;
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            __terrane_traced_completion!(
                fail(choice.clone()), 5 /* terrane-site: case.trn:43:9-43:21 */
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
                if !__terrane_handled_1
                    && (__terrane_error_1.kind
                        == TerraneErrorKind::Custom(DescriptorId(0))
                        || __terrane_error_1.kind
                            == TerraneErrorKind::Custom(DescriptorId(1))
                        || __terrane_error_1.kind
                            == TerraneErrorKind::Custom(DescriptorId(2)))
                {
                    __terrane_handled_1 = true;
                    let _ = error.insert(__terrane_error_1.clone());
                    return TerraneCompletion::Error(
                        error
                            .as_ref()
                            .expect("flow-proven available binding")
                            .clone()
                            .at(6 /* terrane-site: case.trn:45:9-45:20 */),
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
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    Ok(())
}
fn main() {
    let mut error: Option<TerraneError> = None;
    let mut error_2: Option<TerraneError> = None;
    let mut error_3: Option<TerraneError> = None;
    let mut original: Option<TerraneError> = None;
    let mut replacement: Option<TerraneError> = None;
    let mut cause: Option<Option<TerraneError>> = None;
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_traced(handled(terrane_int_support::Int::from(1_i128)),
        7 /* terrane-site: case.trn:48:13-48:23 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_traced(handled(terrane_int_support::Int::from(2_i128)),
        8 /* terrane-site: case.trn:49:13-49:23 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_traced(handled(terrane_int_support::Int::from(3_i128)),
        9 /* terrane-site: case.trn:50:13-50:23 */))
    );
    let __terrane_completion_2: TerraneCompletion<()> = (|| {
        let __terrane_try_2: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_traced_completion!(handled(terrane_int_support::Int::from(4_i128)),
                10 /* terrane-site: case.trn:52:17-52:27 */))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_2 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_2) => {
                let mut __terrane_handled_2 = false;
                if !__terrane_handled_2
                    && __terrane_error_2.kind
                        == TerraneErrorKind::Custom(DescriptorId(3))
                {
                    __terrane_handled_2 = true;
                    let _ = error.insert(__terrane_error_2.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&error.as_ref()
                        .expect("flow-proven available binding").message().to_owned())
                    );
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&error.as_ref()
                        .expect("flow-proven available binding").message().to_owned())
                    );
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
    let mut __terrane_completion_3: TerraneCompletion<()> = (|| {
        let __terrane_try_3: TerraneCompletion<()> = (|| {
            __terrane_traced_completion!(
                reraised(terrane_int_support::Int::from(2_i128)), 11 /* terrane-site: case.trn:57:9-57:20 */
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_3 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_3) => {
                let mut __terrane_handled_3 = false;
                if !__terrane_handled_3
                    && (__terrane_error_3.kind
                        == TerraneErrorKind::Custom(DescriptorId(0))
                        || __terrane_error_3.kind
                            == TerraneErrorKind::Custom(DescriptorId(1))
                        || __terrane_error_3.kind
                            == TerraneErrorKind::Custom(DescriptorId(2)))
                {
                    __terrane_handled_3 = true;
                    let _ = error_2.insert(__terrane_error_3.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&error_2.as_ref()
                        .expect("flow-proven available binding").message().to_owned())
                    );
                }
                if !__terrane_handled_3 {
                    return TerraneCompletion::Error(__terrane_error_3);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_3: TerraneCompletion<()> = (|| {
        println!("{}", String::from("finally"));
        TerraneCompletion::Normal
    })();
    match __terrane_finally_3 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_3 = replacement,
    }
    match __terrane_completion_3 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    let __terrane_completion_4: TerraneCompletion<()> = (|| {
        let __terrane_try_4: TerraneCompletion<()> = (|| {
            __terrane_traced_completion!(
                fail(terrane_int_support::Int::from(1_i128)), 12 /* terrane-site: case.trn:63:9-63:16 */
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_4 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_4) => {
                let mut __terrane_handled_4 = false;
                if !__terrane_handled_4
                    && (__terrane_error_4.kind
                        == TerraneErrorKind::Custom(DescriptorId(0))
                        || __terrane_error_4.kind
                            == TerraneErrorKind::Custom(DescriptorId(1))
                        || __terrane_error_4.kind
                            == TerraneErrorKind::Custom(DescriptorId(2)))
                {
                    __terrane_handled_4 = true;
                    println!("{}", String::from("unbound"));
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
    let __terrane_completion_5: TerraneCompletion<()> = (|| {
        let __terrane_try_5: TerraneCompletion<()> = (|| {
            __terrane_traced_completion!(
                fail(terrane_int_support::Int::from(4_i128)), 13 /* terrane-site: case.trn:67:9-67:16 */
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_5 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_5) => {
                let mut __terrane_handled_5 = false;
                if !__terrane_handled_5
                    && (__terrane_error_5.kind
                        == TerraneErrorKind::Custom(DescriptorId(0))
                        || __terrane_error_5.kind
                            == TerraneErrorKind::Custom(DescriptorId(1))
                        || __terrane_error_5.kind
                            == TerraneErrorKind::Custom(DescriptorId(2)))
                {
                    __terrane_handled_5 = true;
                    println!("{}", String::from("wrong handler"));
                }
                if !__terrane_handled_5 {
                    __terrane_handled_5 = true;
                    let _ = error_3.insert(__terrane_error_5.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&error_3.as_ref()
                        .expect("flow-proven available binding").message().to_owned())
                    );
                }
                if !__terrane_handled_5 {
                    return TerraneCompletion::Error(__terrane_error_5);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_5 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    let __terrane_completion_6: TerraneCompletion<()> = (|| {
        let __terrane_try_6: TerraneCompletion<()> = (|| {
            let __terrane_completion_7: TerraneCompletion<()> = (|| {
                let __terrane_try_7: TerraneCompletion<()> = (|| {
                    __terrane_traced_completion!(
                        fail(terrane_int_support::Int::from(1_i128)),
                        14 /* terrane-site: case.trn:74:13-74:20 */
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
                            && (__terrane_error_7.kind
                                == TerraneErrorKind::Custom(DescriptorId(0))
                                || __terrane_error_7.kind
                                    == TerraneErrorKind::Custom(DescriptorId(1))
                                || __terrane_error_7.kind
                                    == TerraneErrorKind::Custom(DescriptorId(2)))
                        {
                            __terrane_handled_7 = true;
                            let _ = original.insert(__terrane_error_7.clone());
                            println!(
                                "{}", terrane_scalar_support::scalar_text(&original.as_ref()
                                .expect("flow-proven available binding").message()
                                .to_owned())
                            );
                            println!(
                                "{}", terrane_scalar_support::scalar_text(&original.as_ref()
                                .expect("flow-proven available binding").message()
                                .to_owned())
                            );
                            return TerraneCompletion::Error(
                                {
                                    let value = OtherError::terrane_construct();
                                    let rendered = value.render();
                                    let original = &value.message;
                                    let (message, rendered) = if &rendered == original {
                                        (rendered, None)
                                    } else {
                                        (original.clone(), Some(rendered))
                                    };
                                    TerraneError::raised_with_rendered_message(
                                        TerraneErrorKind::Custom(DescriptorId(3)),
                                        message,
                                        rendered,
                                        15 /* terrane-site: case.trn:78:13-78:30 */,
                                    )
                                }
                                    .with_cause(__terrane_error_7.clone()),
                            );
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
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Error(error) => return TerraneCompletion::Error(error),
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
            }
            TerraneCompletion::Normal
        })();
        match __terrane_try_6 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_6) => {
                let mut __terrane_handled_6 = false;
                if !__terrane_handled_6
                    && __terrane_error_6.kind
                        == TerraneErrorKind::Custom(DescriptorId(3))
                {
                    __terrane_handled_6 = true;
                    let _ = replacement.insert(__terrane_error_6.clone());
                    let _ = cause
                        .insert(
                            replacement
                                .as_ref()
                                .expect("flow-proven available binding")
                                .detail
                                .as_ref()
                                .and_then(|detail| detail.cause.as_deref())
                                .cloned(),
                        );
                    if cause.as_ref().expect("flow-proven available binding").is_some() {
                        println!(
                            "{}", terrane_scalar_support::scalar_text(&match cause
                            .as_ref().expect("flow-proven available binding") {
                            Some(value) => value, _ =>
                            unreachable!("flow-proven storage refinement") } .message()
                            .to_owned())
                        );
                    } else {
                        println!("{}", String::from("missing cause"));
                    }
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
}
