// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: borrowed-throwable-text
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
fn make_error(replacement: bool) -> TerraneError {
    let mut error: Option<TerraneError> = None;
    let __terrane_completion_0: TerraneCompletion<TerraneError> = (|| {
        let __terrane_try_0: TerraneCompletion<TerraneError> = (|| {
            if replacement {
                return TerraneCompletion::Error({
                    let value = ReplacementError::terrane_construct();
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
                        0 /* terrane-site: case.trn:17:13-17:36 */,
                    )
                });
            }
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
                    TerraneErrorKind::Custom(DescriptorId(1)),
                    message,
                    rendered,
                    1 /* terrane-site: case.trn:18:9-18:29 */,
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
                    && (__terrane_error_0.kind
                        == TerraneErrorKind::Custom(DescriptorId(1))
                        || __terrane_error_0.kind
                            == TerraneErrorKind::Custom(DescriptorId(0)))
                {
                    __terrane_handled_0 = true;
                    let _ = error.insert(__terrane_error_0.clone());
                    return TerraneCompletion::Return(
                        error.as_ref().expect("flow-proven available binding").clone(),
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
        TerraneCompletion::Normal => {
            __terrane_generated_defect("non-fallthrough try completed normally")
        }
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
fn replace(error: &mut TerraneError) -> String {
    *error = make_error(true);
    return String::from("original");
}
pub struct ErrorHolder {
    pub current: Option<TerraneError>,
}
impl ErrorHolder {
    pub fn terrane_construct(current: TerraneError) -> Self {
        let mut __terrane_constructed_value = Self { current: None };
        __terrane_constructed_value.construct(current);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, current: TerraneError) {
        {
            let __terrane_value: TerraneError = current;
            self.current = Some(__terrane_value);
        }
    }
    pub fn replace_current(&mut self) -> String {
        {
            let __terrane_value: TerraneError = make_error(true);
            self.current = Some(__terrane_value);
        }
        return String::from("original");
    }
}
fn observe(error: &TerraneError) -> String {
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_error_text_0 =
        error.message(); let __terrane_string_argument_0 = "orig"; __terrane_error_text_0
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        }), String::from("/"), terrane_scalar_support::scalar_text(&{ let
        __terrane_error_text_1 = error.descriptor_name(); let __terrane_string_argument_0
        = "original-error"; __terrane_error_text_1.contains(std::convert::AsRef:: < str >
        ::as_ref(&__terrane_string_argument_0)) })
    );
    return error.message().to_owned();
}
fn main() {
    let mut error: TerraneError;
    let owned: String;
    let mut held: std::sync::Arc<TerraneSharedCell<ErrorHolder>>;
    let observed: TerraneError;
    error = make_error(false);
    {
        let __terrane_error_text_2 = error.message();
        println!("{}", __terrane_error_text_2);
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_error_text_3 = error
        .message(); let __terrane_string_argument_0 = "orig"; __terrane_error_text_3
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_error_text_4 = error
        .descriptor_name(); let __terrane_string_argument_0 = "original-error";
        __terrane_error_text_4.contains(std::convert::AsRef:: < str >
        ::as_ref(&__terrane_string_argument_0)) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_5 =
        &String::from("original"); let __terrane_error_text_6 = error.message(); let
        __terrane_string_argument_0 = __terrane_error_text_6; __terrane_receiver_value_5
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_error_text_7 = error
        .message(); __terrane_error_text_7 == "original" })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_error_text_8 = error
        .message(); "original" == __terrane_error_text_8 })
    );
    {
        let __terrane_receiver_value_9 = &make_error(false);
        let __terrane_error_text_10 = __terrane_receiver_value_9.message();
        println!("{}", __terrane_error_text_10);
    };
    owned = error.message().to_owned();
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_error_text_11 = {
        error.message().to_owned() }; let __terrane_comparison_text_12 = &replace(&mut
        error); __terrane_error_text_11.as_str() == __terrane_comparison_text_12.as_str()
        })
    );
    println!("{}", terrane_scalar_support::scalar_text(&owned));
    {
        let __terrane_error_text_13 = error.message();
        println!("{}", __terrane_error_text_13);
    };
    error = make_error(false);
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_error_text_14 = {
        error.message().to_owned() }; let __terrane_string_argument_0 = &replace(&mut
        error); __terrane_error_text_14.as_str().contains(std::convert::AsRef:: < str >
        ::as_ref(&__terrane_string_argument_0)) })
    );
    error = make_error(false);
    {
        let __terrane_error_text_15 = { error.message().to_owned() };
        let __terrane_print_value_16 = terrane_scalar_support::scalar_text(
            &replace(&mut error),
        );
        println!("{}{}", __terrane_error_text_15.as_str(), __terrane_print_value_16);
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(1); __terrane_text.push_str("x"); { let
        __terrane_error_text_17 = error.message(); __terrane_text
        .push_str(__terrane_error_text_17); } __terrane_text })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_18 =
        &make_error(false); let __terrane_error_text_19 = __terrane_receiver_value_18
        .message(); let __terrane_receiver_value_20 = &make_error(true); let
        __terrane_error_text_21 = __terrane_receiver_value_20.message(); let
        __terrane_string_argument_0 = __terrane_error_text_21; __terrane_error_text_19
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        })
    );
    let __terrane_switch_value_1779 = &String::from("original");
    if {
        let __terrane_receiver_value_22 = &make_error(false);
        let __terrane_error_text_23 = __terrane_receiver_value_22.message();
        __terrane_switch_value_1779.as_str() == __terrane_error_text_23
    } {
        println!("{}", String::from("matched"));
    } else {
        println!("{}", String::from("wrong"));
    }
    held = std::sync::Arc::new(
        TerraneSharedCell::new(ErrorHolder::terrane_construct(make_error(false))),
    );
    {
        let __terrane_owner_24 = &held;
        let __terrane_read_guard_25 = __terrane_owner_24
            .lock()
            .expect("shared reference lock poisoned");
        let __terrane_error_text_26 = __terrane_read_guard_25
            .current
            .as_ref()
            .expect("required field initialized")
            .message();
        println!("{}", __terrane_error_text_26);
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_error_text_29 = { let
        __terrane_owner_27 = &held; let __terrane_read_guard_28 = __terrane_owner_27
        .lock().expect("shared reference lock poisoned"); __terrane_read_guard_28.current
        .as_ref().expect("required field initialized").message().to_owned() }; let
        __terrane_string_argument_0 = &{ let __terrane_owner_31 = held.clone(); let
        __terrane_call_result = __terrane_owner_31.lock()
        .expect("shared reference lock poisoned").replace_current();
        __terrane_call_result }; __terrane_error_text_29.as_str()
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        })
    );
    {
        let __terrane_owner_32 = &held;
        let __terrane_read_guard_33 = __terrane_owner_32
            .lock()
            .expect("shared reference lock poisoned");
        let __terrane_error_text_34 = __terrane_read_guard_33
            .current
            .as_ref()
            .expect("required field initialized")
            .message();
        println!("{}", __terrane_error_text_34);
    };
    held = std::sync::Arc::new(
        TerraneSharedCell::new(ErrorHolder::terrane_construct(make_error(false))),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_error_text_37 = { let
        __terrane_owner_35 = &held; let __terrane_read_guard_36 = __terrane_owner_35
        .lock().expect("shared reference lock poisoned"); __terrane_read_guard_36.current
        .as_ref().expect("required field initialized").message().to_owned() }; let
        __terrane_comparison_text_38 = &{ let __terrane_owner_40 = held.clone(); let
        __terrane_call_result = __terrane_owner_40.lock()
        .expect("shared reference lock poisoned").replace_current();
        __terrane_call_result }; __terrane_error_text_37.as_str() ==
        __terrane_comparison_text_38.as_str() })
    );
    held = std::sync::Arc::new(
        TerraneSharedCell::new(ErrorHolder::terrane_construct(make_error(false))),
    );
    {
        let __terrane_error_text_43 = {
            let __terrane_owner_41 = &held;
            let __terrane_read_guard_42 = __terrane_owner_41
                .lock()
                .expect("shared reference lock poisoned");
            __terrane_read_guard_42
                .current
                .as_ref()
                .expect("required field initialized")
                .message()
                .to_owned()
        };
        let __terrane_print_value_46 = terrane_scalar_support::scalar_text(
            &{
                let __terrane_owner_45 = held.clone();
                let __terrane_call_result = __terrane_owner_45
                    .lock()
                    .expect("shared reference lock poisoned")
                    .replace_current();
                __terrane_call_result
            },
        );
        println!("{}{}", __terrane_error_text_43.as_str(), __terrane_print_value_46);
    };
    observed = make_error(false);
    println!("{}", terrane_scalar_support::scalar_text(&observe(&observed)));
    {
        let __terrane_error_text_47 = observed.message();
        println!("{}", __terrane_error_text_47);
    };
}
