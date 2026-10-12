// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: errors/error.trn
// Namespace: generic-errors
#[derive(Clone)]
pub struct ImportedBoxedError<TerraneType54> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub message: String,
}
impl<TerraneType54> ImportedBoxedError<TerraneType54> {
    pub fn terrane_construct(message: String) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_phantom: std::marker::PhantomData,
            message: String::from(""),
        };
        __terrane_constructed_value.construct(message);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, message: String) {
        self.message = message;
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
// Source: src/main.trn
// Namespace: generic-throwable
#[derive(Clone)]
pub struct BoxedError<TerraneType54> {
    pub message: String,
    pub payload: Option<TerraneType54>,
}
impl<TerraneType54> BoxedError<TerraneType54> {
    pub fn terrane_construct(message: String, payload: TerraneType54) -> Self {
        let mut __terrane_constructed_value = Self {
            message: String::from(""),
            payload: None,
        };
        __terrane_constructed_value.construct(message, payload);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, message: String, payload: TerraneType54) {
        self.message = message;
        self.payload = Some(payload);
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
fn declared(failure: BoxedError<terrane_int_support::Int>) -> Result<(), TerraneError> {
    return Err({
        let value = failure;
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
            0 /* terrane-site: src/main.trn:15:3-15:16 */,
        )
    });
}
fn inferred(failure: BoxedError<terrane_int_support::Int>) -> Result<(), TerraneError> {
    return Err({
        let value = failure;
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
            1 /* terrane-site: src/main.trn:18:3-18:16 */,
        )
    });
}
fn raise_direct() -> Result<(), TerraneError> {
    return Err({
        let value = BoxedError::<
            terrane_int_support::Int,
        >::terrane_construct(
            String::from("typed callable bound"),
            terrane_int_support::Int::from(7_i128),
        );
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
            2 /* terrane-site: src/main.trn:21:3-21:54 */,
        )
    });
}
fn invoke_typed(
    operation: std::sync::Arc<dyn Fn() -> Result<(), TerraneError> + Send + Sync>,
) -> Result<(), TerraneError> {
    __terrane_traced_err(operation(), 3 /* terrane-site: src/main.trn:24:3-24:13 */)?;
    return Ok(());
}
fn raise_application(use_string: bool) -> Result<(), TerraneError> {
    if use_string {
        return Err({
            let value = BoxedError::<
                String,
            >::terrane_construct(String::from("string"), String::from("seven"));
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
                4 /* terrane-site: src/main.trn:29:5-29:51 */,
            )
        });
    }
    return Err({
        let value = BoxedError::<
            terrane_int_support::Int,
        >::terrane_construct(
            String::from("int"),
            terrane_int_support::Int::from(7_i128),
        );
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
            5 /* terrane-site: src/main.trn:30:3-30:37 */,
        )
    });
}
fn catch_application(use_string: bool) {
    let mut failure: Option<TerraneError> = None;
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            __terrane_traced_completion!(
                raise_application(use_string), 6 /* terrane-site: src/main.trn:34:5-34:34 */
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
                    && __terrane_error_0.kind
                        == TerraneErrorKind::Custom(DescriptorId(0))
                {
                    __terrane_handled_0 = true;
                    let _ = failure.insert(__terrane_error_0.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&failure.as_ref()
                        .expect("flow-proven available binding").clone().message()
                        .to_owned())
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
}
fn main() {
    let mut failure: Option<TerraneError> = None;
    let mut failure_2: Option<TerraneError> = None;
    let existing: BoxedError<terrane_int_support::Int>;
    let mut failure_3: Option<TerraneError> = None;
    let mut failure_4: Option<TerraneError> = None;
    let mut failure_5: Option<TerraneError> = None;
    let mut failure_6: Option<TerraneError> = None;
    let mut failure_7: Option<TerraneError> = None;
    let mut failure_8: Option<TerraneError> = None;
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            return TerraneCompletion::Error({
                let value = BoxedError::<
                    terrane_int_support::Int,
                >::terrane_construct(
                    String::from("direct generic"),
                    terrane_int_support::Int::from(7_i128),
                );
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
                    7 /* terrane-site: src/main.trn:40:5-40:50 */,
                )
            });
        })();
        match __terrane_try_1 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
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
                    let _ = failure.insert(__terrane_error_1.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&failure.as_ref()
                        .expect("flow-proven available binding").clone().message()
                        .to_owned())
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
    let __terrane_completion_2: TerraneCompletion<()> = (|| {
        let __terrane_try_2: TerraneCompletion<()> = (|| {
            return TerraneCompletion::Error({
                let value = BoxedError::<
                    terrane_int_support::Int,
                >::terrane_construct(
                    String::from("inline instance"),
                    terrane_int_support::Int::from(7_i128),
                );
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
                    8 /* terrane-site: src/main.trn:45:5-45:60 */,
                )
            });
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
                        == TerraneErrorKind::Custom(DescriptorId(0))
                {
                    __terrane_handled_2 = true;
                    let _ = failure_2.insert(__terrane_error_2.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&failure_2.as_ref()
                        .expect("flow-proven available binding").clone().message()
                        .to_owned())
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
    existing = BoxedError::<
        terrane_int_support::Int,
    >::terrane_construct(
        String::from("existing instance"),
        terrane_int_support::Int::from(7_i128),
    );
    let __terrane_completion_3: TerraneCompletion<()> = (|| {
        let __terrane_try_3: TerraneCompletion<()> = (|| {
            return TerraneCompletion::Error({
                let value = existing;
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
                    9 /* terrane-site: src/main.trn:51:5-51:19 */,
                )
            });
        })();
        match __terrane_try_3 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_3) => {
                let mut __terrane_handled_3 = false;
                if !__terrane_handled_3
                    && __terrane_error_3.kind
                        == TerraneErrorKind::Custom(DescriptorId(0))
                {
                    __terrane_handled_3 = true;
                    let _ = failure_3.insert(__terrane_error_3.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&failure_3.as_ref()
                        .expect("flow-proven available binding").clone().message()
                        .to_owned())
                    );
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
    let __terrane_completion_4: TerraneCompletion<()> = (|| {
        let __terrane_try_4: TerraneCompletion<()> = (|| {
            __terrane_traced_completion!(
                declared(BoxedError::< terrane_int_support::Int
                >::terrane_construct(String::from("declared bound"),
                terrane_int_support::Int::from(7_i128))), 10 /* terrane-site: src/main.trn:56:5-56:65 */
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
                    && __terrane_error_4.kind
                        == TerraneErrorKind::Custom(DescriptorId(0))
                {
                    __terrane_handled_4 = true;
                    let _ = failure_4.insert(__terrane_error_4.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&failure_4.as_ref()
                        .expect("flow-proven available binding").clone().message()
                        .to_owned())
                    );
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
                inferred(BoxedError::< terrane_int_support::Int
                >::terrane_construct(String::from("inferred effect"),
                terrane_int_support::Int::from(7_i128))), 11 /* terrane-site: src/main.trn:61:5-61:66 */
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
                    && __terrane_error_5.kind
                        == TerraneErrorKind::Custom(DescriptorId(0))
                {
                    __terrane_handled_5 = true;
                    let _ = failure_5.insert(__terrane_error_5.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&failure_5.as_ref()
                        .expect("flow-proven available binding").clone().message()
                        .to_owned())
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
            __terrane_traced_completion!(
                invoke_typed(std::sync::Arc::new(raise_direct)), 12 /* terrane-site: src/main.trn:66:5-66:31 */
            );
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
                        == TerraneErrorKind::Custom(DescriptorId(0))
                {
                    __terrane_handled_6 = true;
                    let _ = failure_6.insert(__terrane_error_6.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&failure_6.as_ref()
                        .expect("flow-proven available binding").clone().message()
                        .to_owned())
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
    let __terrane_completion_7: TerraneCompletion<()> = (|| {
        let __terrane_try_7: TerraneCompletion<()> = (|| {
            return TerraneCompletion::Error({
                let value = ImportedBoxedError::<
                    terrane_int_support::Int,
                >::terrane_construct(String::from("imported alias"));
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
                    13 /* terrane-site: src/main.trn:71:5-71:55 */,
                )
            });
        })();
        match __terrane_try_7 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_7) => {
                let mut __terrane_handled_7 = false;
                if !__terrane_handled_7
                    && __terrane_error_7.kind
                        == TerraneErrorKind::Custom(DescriptorId(1))
                {
                    __terrane_handled_7 = true;
                    let _ = failure_7.insert(__terrane_error_7.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&failure_7.as_ref()
                        .expect("flow-proven available binding").clone().message()
                        .to_owned())
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
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    catch_application(false);
    catch_application(true);
    let __terrane_completion_8: TerraneCompletion<()> = (|| {
        let __terrane_try_8: TerraneCompletion<()> = (|| {
            __terrane_traced_completion!(
                raise_application(true), 14 /* terrane-site: src/main.trn:78:5-78:28 */
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_8 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_8) => {
                let mut __terrane_handled_8 = false;
                if !__terrane_handled_8 {
                    __terrane_handled_8 = true;
                    let _ = failure_8.insert(__terrane_error_8.clone());
                    println!("{}", String::from("catch-all"));
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&failure_8.as_ref()
                        .expect("flow-proven available binding").clone().message()
                        .to_owned())
                    );
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
}
