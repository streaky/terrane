// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: errors/error.trn
// Namespace: imported-errors
#[derive(Clone)]
pub struct ImportedError {
    pub message: String,
}
impl ImportedError {
    pub fn terrane_construct(message: String) -> Self {
        let mut __terrane_constructed_value = Self { message: String::from("") };
        __terrane_constructed_value.construct(message);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, message: String) {
        {
            let __terrane_value = message;
            self.message = __terrane_value;
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
// Source: src/main.trn
// Namespace: throwable-direct-construction
#[derive(Clone)]
pub struct ConsumerError {
    pub declaration: DocumentValue,
    pub message: String,
}
impl ConsumerError {
    pub fn terrane_construct(declaration: DocumentValue, message: String) -> Self {
        let mut __terrane_constructed_value = Self {
            declaration: make_document_none(),
            message: String::from(""),
        };
        __terrane_constructed_value.construct(declaration, message);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, declaration: DocumentValue, message: String) {
        {
            let __terrane_value = declaration;
            self.declaration = __terrane_value;
        }
        {
            let __terrane_value = message;
            self.message = __terrane_value;
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
#[derive(Clone)]
pub struct DefaultError {
    pub message: String,
}
impl DefaultError {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("default error"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
#[derive(Clone)]
pub struct ExplicitError {
    pub message: String,
}
impl ExplicitError {
    pub fn terrane_construct() -> Self {
        let mut __terrane_constructed_value = Self { message: String::from("") };
        __terrane_constructed_value.construct();
        __terrane_constructed_value
    }
    pub fn construct(&mut self) {
        {
            let __terrane_value = String::from("explicit constructor");
            self.message = __terrane_value;
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
fn invoke_local_construction() -> Result<(), TerraneError> {
    fn local_construction() -> Result<(), TerraneError> {
        return Err({
            let value = ImportedError::terrane_construct(
                String::from("local construction"),
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
                0 /* terrane-site: src/main.trn:29:5-29:46 */,
            )
        });
    }
    __terrane_traced_err(
        local_construction(),
        1 /* terrane-site: src/main.trn:30:3-30:22 */,
    )?;
    Ok(())
}
fn closure_construction() -> Result<(), TerraneError> {
    let raise: std::sync::Arc<dyn Fn() -> Result<(), TerraneError> + Send + Sync>;
    raise = {
        std::sync::Arc::new(move || -> Result<(), TerraneError> {
            return Err({
                let value = ImportedError::terrane_construct(
                    String::from("closure construction"),
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
                    2 /* terrane-site: src/main.trn:34:5-34:48 */,
                )
            });
        })
    };
    __terrane_traced_err(raise(), 3 /* terrane-site: src/main.trn:35:3-35:9 */)?;
    Ok(())
}
fn bare_default_throw() -> Result<(), TerraneError> {
    return Err({
        let value = DefaultError::terrane_construct();
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
            4 /* terrane-site: src/main.trn:38:3-38:22 */,
        )
    });
}
fn invoke_bare_default_throw() -> Result<(), TerraneError> {
    __terrane_traced_err(
        bare_default_throw(),
        5 /* terrane-site: src/main.trn:41:3-41:22 */,
    )?;
    Ok(())
}
fn main() {
    let mut error: Option<TerraneError> = None;
    let mut error_2: Option<TerraneError> = None;
    let mut error_3: Option<TerraneError> = None;
    let mut error_4: Option<TerraneError> = None;
    let mut error_5: Option<TerraneError> = None;
    let mut error_6: Option<TerraneError> = None;
    let mut error_7: Option<TerraneError> = None;
    let mut error_8: Option<TerraneError> = None;
    let existing: ConsumerError;
    let mut error_9: Option<TerraneError> = None;
    let mut error_10: Option<TerraneError> = None;
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            return TerraneCompletion::Error({
                let value = ConsumerError::terrane_construct(
                    make_document_none(),
                    String::from(
                        "CLI command must be synchronous, non-throwing, and safe",
                    ),
                );
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
                    6 /* terrane-site: src/main.trn:45:5-45:107 */,
                )
            });
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
                    let _ = error.insert(__terrane_error_0.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&error.as_ref()
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
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            return TerraneCompletion::Error({
                let value = ConsumerError::terrane_construct(
                    make_document_none(),
                    String::from("named arguments"),
                );
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
                    7 /* terrane-site: src/main.trn:49:5-49:91 */,
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
                        == TerraneErrorKind::Custom(DescriptorId(2))
                {
                    __terrane_handled_1 = true;
                    let _ = error_2.insert(__terrane_error_1.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&error_2.as_ref()
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
                let value = DefaultError::terrane_construct();
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
                    8 /* terrane-site: src/main.trn:53:5-53:24 */,
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
                        == TerraneErrorKind::Custom(DescriptorId(1))
                {
                    __terrane_handled_2 = true;
                    let _ = error_3.insert(__terrane_error_2.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&error_3.as_ref()
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
    let __terrane_completion_3: TerraneCompletion<()> = (|| {
        let __terrane_try_3: TerraneCompletion<()> = (|| {
            return TerraneCompletion::Error({
                let value = ExplicitError::terrane_construct();
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
                    9 /* terrane-site: src/main.trn:57:5-57:26 */,
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
                        == TerraneErrorKind::Custom(DescriptorId(3))
                {
                    __terrane_handled_3 = true;
                    let _ = error_4.insert(__terrane_error_3.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&error_4.as_ref()
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
            return TerraneCompletion::Error({
                let value = ImportedError::terrane_construct(
                    String::from("imported alias"),
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
                    10 /* terrane-site: src/main.trn:61:5-61:42 */,
                )
            });
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
                    let _ = error_5.insert(__terrane_error_4.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&error_5.as_ref()
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
                invoke_local_construction(), 11 /* terrane-site: src/main.trn:65:5-65:31 */
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
                    let _ = error_6.insert(__terrane_error_5.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&error_6.as_ref()
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
                closure_construction(), 12 /* terrane-site: src/main.trn:69:5-69:26 */
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
                    let _ = error_7.insert(__terrane_error_6.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&error_7.as_ref()
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
            __terrane_traced_completion!(
                invoke_bare_default_throw(), 13 /* terrane-site: src/main.trn:73:5-73:31 */
            );
            TerraneCompletion::Normal
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
                    let _ = error_8.insert(__terrane_error_7.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&error_8.as_ref()
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
    existing = ConsumerError::terrane_construct(
        make_document_none(),
        String::from("existing instance"),
    );
    let __terrane_completion_8: TerraneCompletion<()> = (|| {
        let __terrane_try_8: TerraneCompletion<()> = (|| {
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
                    TerraneErrorKind::Custom(DescriptorId(2)),
                    message,
                    rendered,
                    14 /* terrane-site: src/main.trn:78:5-78:19 */,
                )
            });
        })();
        match __terrane_try_8 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_8) => {
                let mut __terrane_handled_8 = false;
                if !__terrane_handled_8
                    && __terrane_error_8.kind
                        == TerraneErrorKind::Custom(DescriptorId(2))
                {
                    __terrane_handled_8 = true;
                    let _ = error_9.insert(__terrane_error_8.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&error_9.as_ref()
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
    let __terrane_completion_9: TerraneCompletion<()> = (|| {
        let __terrane_try_9: TerraneCompletion<()> = (|| {
            return TerraneCompletion::Error(
                TerraneError::raised(
                    TerraneErrorKind::CoercionError,
                    15 /* terrane-site: src/main.trn:82:5-82:25 */,
                ),
            );
        })();
        match __terrane_try_9 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_9) => {
                let mut __terrane_handled_9 = false;
                if !__terrane_handled_9
                    && __terrane_error_9.kind == TerraneErrorKind::CoercionError
                {
                    __terrane_handled_9 = true;
                    let _ = error_10.insert(__terrane_error_9.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&error_10.as_ref()
                        .expect("flow-proven available binding").clone().message()
                        .to_owned())
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
}
