// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: throwable-message-render
pub static TERRANE_STATIC_SPLITERROR_RENDERS: std::sync::LazyLock<
    TerraneSharedCell<terrane_int_support::Int>,
> = std::sync::LazyLock::new(|| TerraneSharedCell::new(
    terrane_int_support::Int::from(0_i128),
));
#[derive(Clone)]
pub struct SplitError {
    pub message: String,
    pub display: String,
}
impl SplitError {
    pub fn terrane_construct(message: String, display: String) -> Self {
        let mut __terrane_constructed_value = Self {
            message: String::from(""),
            display: String::from(""),
        };
        __terrane_constructed_value.construct(message, display);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, message: String, display: String) {
        {
            let __terrane_value = message;
            self.message = __terrane_value;
        }
        {
            let __terrane_value = display;
            self.display = __terrane_value;
        }
    }
    pub fn render(&self) -> String {
        {
            let __terrane_static_value = {
                let __terrane_value = TERRANE_STATIC_SPLITERROR_RENDERS
                    .lock()
                    .expect("static field lock poisoned")
                    .clone();
                __terrane_value
            } + terrane_int_support::Int::from(1_i128);
            *TERRANE_STATIC_SPLITERROR_RENDERS
                .lock()
                .expect("static field lock poisoned") = __terrane_static_value;
        }
        return self.display.clone();
    }
}
#[derive(Clone)]
pub struct WrapperError {
    pub message: String,
}
impl WrapperError {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("wrapper message"),
        }
    }
    pub fn render(&self) -> String {
        return String::from("wrapper display");
    }
}
fn raise_original() -> Result<(), TerraneError> {
    return Err({
        let value = SplitError::terrane_construct(
            String::from("original message"),
            String::from("original display"),
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
            0 /* terrane-site: case.trn:21:3-21:60 */,
        )
    });
}
fn main() {
    let mut caught: Option<TerraneError> = None;
    let mut shown: Option<String> = None;
    let mut caught_2: Option<TerraneError> = None;
    let mut caught_3: Option<TerraneError> = None;
    let mut caught_4: Option<TerraneError> = None;
    let mut outer: Option<TerraneError> = None;
    let mut cause: Option<Option<TerraneError>> = None;
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let __terrane_completion_1: TerraneCompletion<()> = (|| {
                let __terrane_try_1: TerraneCompletion<()> = (|| {
                    __terrane_traced_completion!(
                        raise_original(), 1 /* terrane-site: case.trn:26:7-26:22 */
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
                        if !__terrane_handled_1
                            && __terrane_error_1.kind
                                == TerraneErrorKind::Custom(DescriptorId(0))
                        {
                            __terrane_handled_1 = true;
                            let _ = caught.insert(__terrane_error_1.clone());
                            println!(
                                "{}", terrane_scalar_support::scalar_text(&caught.as_ref()
                                .expect("flow-proven available binding").clone().message()
                                .to_owned())
                            );
                            let _ = shown
                                .insert(
                                    caught
                                        .as_ref()
                                        .expect("flow-proven available binding")
                                        .clone()
                                        .render(),
                                );
                            println!(
                                "{}{}", terrane_scalar_support::scalar_text(&shown.as_ref()
                                .expect("flow-proven available binding").clone()
                                .contains(&String::from("::split-error: original display"))),
                                terrane_scalar_support::scalar_text(&shown.as_ref()
                                .expect("flow-proven available binding").clone()
                                .contains(&String::from("original message")))
                            );
                            return TerraneCompletion::Error(
                                __terrane_error_1
                                    .clone()
                                    .at(2 /* terrane-site: case.trn:31:7-31:12 */),
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
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Error(error) => return TerraneCompletion::Error(error),
                TerraneCompletion::Break | TerraneCompletion::Continue => {
                    __terrane_generated_defect("loop control escaped a non-loop try")
                }
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
                        == TerraneErrorKind::Custom(DescriptorId(0))
                {
                    __terrane_handled_0 = true;
                    let _ = caught_2.insert(__terrane_error_0.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&caught_2.as_ref()
                        .expect("flow-proven available binding").clone().message()
                        .to_owned())
                    );
                    let _ = shown
                        .insert(
                            caught_2
                                .as_ref()
                                .expect("flow-proven available binding")
                                .clone()
                                .render(),
                        );
                    println!(
                        "{}{}", terrane_scalar_support::scalar_text(&shown.as_ref()
                        .expect("flow-proven available binding").clone()
                        .contains(&String::from("::split-error: original display"))),
                        terrane_scalar_support::scalar_text(&shown.as_ref()
                        .expect("flow-proven available binding").clone()
                        .contains(&String::from("::raise-original (case.trn:")))
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
    let __terrane_completion_2: TerraneCompletion<()> = (|| {
        let __terrane_try_2: TerraneCompletion<()> = (|| {
            let __terrane_completion_3: TerraneCompletion<()> = (|| {
                let __terrane_try_3: TerraneCompletion<()> = (|| {
                    __terrane_traced_completion!(
                        raise_original(), 3 /* terrane-site: case.trn:38:7-38:22 */
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
                                == TerraneErrorKind::Custom(DescriptorId(0))
                        {
                            __terrane_handled_3 = true;
                            let _ = caught_3.insert(__terrane_error_3.clone());
                            return TerraneCompletion::Error(
                                caught_3
                                    .as_ref()
                                    .expect("flow-proven available binding")
                                    .clone()
                                    .at(4 /* terrane-site: case.trn:40:7-40:19 */),
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
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Error(error) => return TerraneCompletion::Error(error),
                TerraneCompletion::Break | TerraneCompletion::Continue => {
                    __terrane_generated_defect("loop control escaped a non-loop try")
                }
            }
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
                        == TerraneErrorKind::Custom(DescriptorId(0))
                {
                    __terrane_handled_2 = true;
                    let _ = caught_4.insert(__terrane_error_2.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&caught_4.as_ref()
                        .expect("flow-proven available binding").clone().message()
                        .to_owned())
                    );
                    println!(
                        "{}{}", terrane_scalar_support::scalar_text(&caught_4.as_ref()
                        .expect("flow-proven available binding").clone().render()
                        .contains(&String::from("::split-error: original display"))),
                        terrane_scalar_support::scalar_text(&caught_4.as_ref()
                        .expect("flow-proven available binding").clone().detail.as_ref()
                        .and_then(| detail | detail.cause.as_deref()).cloned().is_none())
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
    let __terrane_completion_4: TerraneCompletion<()> = (|| {
        let __terrane_try_4: TerraneCompletion<()> = (|| {
            let __terrane_completion_5: TerraneCompletion<()> = (|| {
                let __terrane_try_5: TerraneCompletion<()> = (|| {
                    __terrane_traced_completion!(
                        raise_original(), 5 /* terrane-site: case.trn:46:7-46:22 */
                    );
                    TerraneCompletion::Normal
                })();
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
                                == TerraneErrorKind::Custom(DescriptorId(0))
                        {
                            __terrane_handled_5 = true;
                            return TerraneCompletion::Error(
                                {
                                    let value = WrapperError::terrane_construct();
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
                                        6 /* terrane-site: case.trn:48:7-48:26 */,
                                    )
                                }
                                    .with_cause(__terrane_error_5.clone()),
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
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Error(error) => return TerraneCompletion::Error(error),
                TerraneCompletion::Break | TerraneCompletion::Continue => {
                    __terrane_generated_defect("loop control escaped a non-loop try")
                }
            }
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
                        == TerraneErrorKind::Custom(DescriptorId(1))
                {
                    __terrane_handled_4 = true;
                    let _ = outer.insert(__terrane_error_4.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&outer.as_ref()
                        .expect("flow-proven available binding").clone().message()
                        .to_owned())
                    );
                    let _ = cause
                        .insert(
                            outer
                                .as_ref()
                                .expect("flow-proven available binding")
                                .clone()
                                .detail
                                .as_ref()
                                .and_then(|detail| detail.cause.as_deref())
                                .cloned(),
                        );
                    if cause
                        .as_ref()
                        .expect("flow-proven available binding")
                        .clone()
                        .is_some()
                    {
                        println!(
                            "{}", terrane_scalar_support::scalar_text(&match cause
                            .as_ref().expect("flow-proven available binding") {
                            Some(value) => value, _ =>
                            unreachable!("flow-proven storage refinement") } .message()
                            .to_owned())
                        );
                        let _ = shown
                            .insert(
                                outer
                                    .as_ref()
                                    .expect("flow-proven available binding")
                                    .clone()
                                    .render(),
                            );
                        println!(
                            "{}{}", terrane_scalar_support::scalar_text(&shown.as_ref()
                            .expect("flow-proven available binding").clone()
                            .contains(&String::from("::wrapper-error: wrapper display"))),
                            terrane_scalar_support::scalar_text(&shown.as_ref()
                            .expect("flow-proven available binding").clone()
                            .contains(&String::from("caused by: /throwable-message-render::split-error: original display")))
                        );
                    } else {
                        println!("{}", String::from("missing cause"));
                    }
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
    println!(
        "{}", { let __terrane_result = terrane_scalar_support::scalar_text(&*
        TERRANE_STATIC_SPLITERROR_RENDERS.lock().expect("static field lock poisoned"));
        __terrane_result }
    );
}
