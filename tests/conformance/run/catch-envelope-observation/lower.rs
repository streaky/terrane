// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: catch-envelope-observation
#[derive(Clone)]
pub struct OriginalError {
    pub message: String,
}
impl OriginalError {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("original"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
#[derive(Clone)]
pub struct ReplacementError {
    pub message: String,
}
impl ReplacementError {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("replacement"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
#[derive(Clone)]
pub struct FinalError {
    pub message: String,
}
impl FinalError {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("final"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
fn main() {
    let mut error: Option<TerraneError> = None;
    let mut rendered: Option<std::sync::Arc<dyn Fn() -> String + Send + Sync>> = None;
    let mut replacement: Option<TerraneError> = None;
    let mut outer: Option<TerraneError> = None;
    let mut cause: Option<Option<TerraneError>> = None;
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let __terrane_completion_1: TerraneCompletion<()> = (|| {
                let __terrane_try_1: TerraneCompletion<()> = (|| {
                    return TerraneCompletion::Error({
                        let value = OriginalError::terrane_construct();
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
                            0 /* terrane-site: case.trn:22:13-22:33 */,
                        )
                    });
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
                        if !__terrane_handled_1
                            && __terrane_error_1.kind
                                == TerraneErrorKind::Custom(DescriptorId(0))
                        {
                            __terrane_handled_1 = true;
                            let _ = error.insert(__terrane_error_1.clone());
                            {
                                let __terrane_error_text_0 = error
                                    .as_ref()
                                    .expect("flow-proven available binding")
                                    .message();
                                println!("{}", __terrane_error_text_0);
                            };
                            {
                                let __terrane_error_text_1 = error
                                    .as_ref()
                                    .expect("flow-proven available binding")
                                    .message();
                                println!("{}", __terrane_error_text_1);
                            };
                            println!(
                                "{}", terrane_scalar_support::scalar_text(&{ let
                                __terrane_error_text_2 = error.as_ref()
                                .expect("flow-proven available binding").descriptor_name();
                                let __terrane_string_argument_0 = "original-error";
                                __terrane_error_text_2.contains(std::convert::AsRef:: < str
                                > ::as_ref(&__terrane_string_argument_0)) })
                            );
                            let _ = rendered
                                .insert(
                                    std::sync::Arc::new({
                                        let error = error
                                            .as_ref()
                                            .expect("flow-proven available binding")
                                            .clone();
                                        move || error.render()
                                    }),
                                );
                            let __terrane_completion_2: TerraneCompletion<()> = (|| {
                                let __terrane_try_2: TerraneCompletion<()> = (|| {
                                    return TerraneCompletion::Error(
                                        {
                                            let value = ReplacementError::terrane_construct();
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
                                                1 /* terrane-site: case.trn:29:17-29:40 */,
                                            )
                                        }
                                            .with_cause(__terrane_error_1.clone()),
                                    );
                                })();
                                match __terrane_try_2 {
                                    TerraneCompletion::Return(value) => {
                                        return TerraneCompletion::Return(value);
                                    }
                                    TerraneCompletion::Break => return TerraneCompletion::Break,
                                    TerraneCompletion::Continue => {
                                        return TerraneCompletion::Continue;
                                    }
                                    TerraneCompletion::Normal => {}
                                    TerraneCompletion::Error(__terrane_error_2) => {
                                        let mut __terrane_handled_2 = false;
                                        if !__terrane_handled_2
                                            && __terrane_error_2.kind
                                                == TerraneErrorKind::Custom(DescriptorId(1))
                                        {
                                            __terrane_handled_2 = true;
                                            let _ = replacement.insert(__terrane_error_2.clone());
                                            let _ = error
                                                .insert(
                                                    replacement
                                                        .as_ref()
                                                        .expect("flow-proven available binding")
                                                        .clone(),
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
                                TerraneCompletion::Return(value) => {
                                    return TerraneCompletion::Return(value);
                                }
                                TerraneCompletion::Error(error) => {
                                    return TerraneCompletion::Error(error);
                                }
                                TerraneCompletion::Break => return TerraneCompletion::Break,
                                TerraneCompletion::Continue => {
                                    return TerraneCompletion::Continue;
                                }
                            }
                            {
                                let __terrane_error_text_3 = error
                                    .as_ref()
                                    .expect("flow-proven available binding")
                                    .message();
                                println!("{}", __terrane_error_text_3);
                            };
                            println!(
                                "{}", terrane_scalar_support::scalar_text(&{ let
                                __terrane_receiver_value_4 = &rendered.as_ref()
                                .expect("flow-proven available binding").clone() (); let
                                __terrane_string_argument_0 = "original";
                                __terrane_receiver_value_4.contains(std::convert::AsRef:: <
                                str > ::as_ref(&__terrane_string_argument_0)) })
                            );
                            return TerraneCompletion::Error(
                                {
                                    let value = FinalError::terrane_construct();
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
                                        2 /* terrane-site: case.trn:34:13-34:30 */,
                                    )
                                }
                                    .with_cause(__terrane_error_1.clone()),
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
                    __terrane_generated_defect("non-fallthrough try completed normally")
                }
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Error(error) => return TerraneCompletion::Error(error),
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
            }
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
                    let _ = outer.insert(__terrane_error_0.clone());
                    {
                        let __terrane_error_text_5 = outer
                            .as_ref()
                            .expect("flow-proven available binding")
                            .message();
                        println!("{}", __terrane_error_text_5);
                    };
                    let _ = cause
                        .insert(
                            outer
                                .as_ref()
                                .expect("flow-proven available binding")
                                .detail
                                .as_ref()
                                .and_then(|detail| detail.cause.as_deref())
                                .cloned(),
                        );
                    if cause.as_ref().expect("flow-proven available binding").is_some() {
                        {
                            let __terrane_error_text_6 = match cause
                                .as_ref()
                                .expect("flow-proven available binding")
                            {
                                Some(value) => value,
                                _ => unreachable!("flow-proven storage refinement"),
                            }
                                .message();
                            println!("{}", __terrane_error_text_6);
                        };
                    } else {
                        println!("{}", String::from("missing cause"));
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
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    {
        let __terrane_error_text_7 = error
            .as_ref()
            .expect("flow-proven available binding")
            .message();
        println!("{}", __terrane_error_text_7);
    };
}
