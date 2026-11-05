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
            let __terrane_value = current;
            self.current = Some(__terrane_value);
        }
    }
    pub fn replace_current(&mut self) -> String {
        {
            let __terrane_value = make_error(true);
            self.current = Some(__terrane_value);
        }
        return String::from("original");
    }
}
fn main() {
    let mut error: TerraneError;
    let owned: String;
    let mut held: std::sync::Arc<TerraneSharedCell<ErrorHolder>>;
    error = make_error(false);
    {
        let __terrane_error_text_0 = error.message();
        println!("{}", __terrane_error_text_0);
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_error_text_1 = error
        .message(); let __terrane_string_argument_0 = "orig"; __terrane_error_text_1
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_error_text_2 = error
        .descriptor_name(); let __terrane_string_argument_0 = "original-error";
        __terrane_error_text_2.contains(std::convert::AsRef:: < str >
        ::as_ref(&__terrane_string_argument_0)) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_3 =
        &String::from("original"); let __terrane_error_text_4 = error.message(); let
        __terrane_string_argument_0 = __terrane_error_text_4; __terrane_receiver_value_3
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_error_text_5 = error
        .message(); __terrane_error_text_5 == "original" })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_error_text_6 = error
        .message(); "original" == __terrane_error_text_6 })
    );
    {
        let __terrane_receiver_value_7 = &make_error(false);
        let __terrane_error_text_8 = __terrane_receiver_value_7.message();
        println!("{}", __terrane_error_text_8);
    };
    owned = error.message().to_owned();
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_error_text_9 = { error
        .message().to_owned() }; let __terrane_comparison_text_10 = &replace(&mut error);
        __terrane_error_text_9.as_str() == __terrane_comparison_text_10.as_str() })
    );
    println!("{}", terrane_scalar_support::scalar_text(&owned));
    {
        let __terrane_error_text_11 = error.message();
        println!("{}", __terrane_error_text_11);
    };
    error = make_error(false);
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_error_text_12 = {
        error.message().to_owned() }; let __terrane_string_argument_0 = &replace(&mut
        error); __terrane_error_text_12.as_str().contains(std::convert::AsRef:: < str >
        ::as_ref(&__terrane_string_argument_0)) })
    );
    error = make_error(false);
    {
        let __terrane_error_text_13 = { error.message().to_owned() };
        let __terrane_print_value_14 = terrane_scalar_support::scalar_text(
            &replace(&mut error),
        );
        println!("{}{}", __terrane_error_text_13.as_str(), __terrane_print_value_14);
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(1); __terrane_text.push_str("x"); { let
        __terrane_error_text_15 = error.message(); __terrane_text
        .push_str(__terrane_error_text_15); } __terrane_text })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_16 =
        &make_error(false); let __terrane_error_text_17 = __terrane_receiver_value_16
        .message(); let __terrane_receiver_value_18 = &make_error(true); let
        __terrane_error_text_19 = __terrane_receiver_value_18.message(); let
        __terrane_string_argument_0 = __terrane_error_text_19; __terrane_error_text_17
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        })
    );
    let __terrane_switch_value_1612 = &String::from("original");
    if {
        let __terrane_receiver_value_20 = &make_error(false);
        let __terrane_error_text_21 = __terrane_receiver_value_20.message();
        __terrane_switch_value_1612.as_str() == __terrane_error_text_21
    } {
        println!("{}", String::from("matched"));
    } else {
        println!("{}", String::from("wrong"));
    }
    held = std::sync::Arc::new(
        TerraneSharedCell::new(ErrorHolder::terrane_construct(make_error(false))),
    );
    {
        let __terrane_owner_22 = &held;
        let __terrane_read_guard_23 = __terrane_owner_22
            .lock()
            .expect("shared reference lock poisoned");
        let __terrane_error_text_24 = __terrane_read_guard_23
            .current
            .as_ref()
            .expect("required field initialized")
            .message();
        println!("{}", __terrane_error_text_24);
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_error_text_27 = { let
        __terrane_owner_25 = &held; let __terrane_read_guard_26 = __terrane_owner_25
        .lock().expect("shared reference lock poisoned"); __terrane_read_guard_26.current
        .as_ref().expect("required field initialized").message().to_owned() }; let
        __terrane_string_argument_0 = &{ let __terrane_owner_29 = held.clone(); let
        __terrane_call_result = __terrane_owner_29.lock()
        .expect("shared reference lock poisoned").replace_current();
        __terrane_call_result }; __terrane_error_text_27.as_str()
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        })
    );
    {
        let __terrane_owner_30 = &held;
        let __terrane_read_guard_31 = __terrane_owner_30
            .lock()
            .expect("shared reference lock poisoned");
        let __terrane_error_text_32 = __terrane_read_guard_31
            .current
            .as_ref()
            .expect("required field initialized")
            .message();
        println!("{}", __terrane_error_text_32);
    };
    held = std::sync::Arc::new(
        TerraneSharedCell::new(ErrorHolder::terrane_construct(make_error(false))),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_error_text_35 = { let
        __terrane_owner_33 = &held; let __terrane_read_guard_34 = __terrane_owner_33
        .lock().expect("shared reference lock poisoned"); __terrane_read_guard_34.current
        .as_ref().expect("required field initialized").message().to_owned() }; let
        __terrane_comparison_text_36 = &{ let __terrane_owner_38 = held.clone(); let
        __terrane_call_result = __terrane_owner_38.lock()
        .expect("shared reference lock poisoned").replace_current();
        __terrane_call_result }; __terrane_error_text_35.as_str() ==
        __terrane_comparison_text_36.as_str() })
    );
    held = std::sync::Arc::new(
        TerraneSharedCell::new(ErrorHolder::terrane_construct(make_error(false))),
    );
    {
        let __terrane_error_text_41 = {
            let __terrane_owner_39 = &held;
            let __terrane_read_guard_40 = __terrane_owner_39
                .lock()
                .expect("shared reference lock poisoned");
            __terrane_read_guard_40
                .current
                .as_ref()
                .expect("required field initialized")
                .message()
                .to_owned()
        };
        let __terrane_print_value_44 = terrane_scalar_support::scalar_text(
            &{
                let __terrane_owner_43 = held.clone();
                let __terrane_call_result = __terrane_owner_43
                    .lock()
                    .expect("shared reference lock poisoned")
                    .replace_current();
                __terrane_call_result
            },
        );
        println!("{}{}", __terrane_error_text_41.as_str(), __terrane_print_value_44);
    };
}
