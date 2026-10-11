// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: throwable-self-construction
#[derive(Clone)]
pub struct BaseErrorStorage {
    pub message: String,
}
impl BaseErrorStorage {
    pub fn terrane_construct(message: String) -> Self {
        let mut __terrane_constructed_value = Self { message: String::from("") };
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
#[derive(Clone)]
pub enum BaseError {
    Own(BaseErrorStorage),
    ChildError(ChildError),
}
impl BaseError {
    pub fn terrane_construct(message: String) -> Self {
        Self::Own(BaseErrorStorage::terrane_construct(message))
    }
    pub fn terrane_static_raise(message: String) -> Result<(), TerraneError> {
        return Err({
            let value = BaseError::terrane_construct(message);
            let rendered = value.render();
            let original = value.terrane_field_message();
            let (message, rendered) = if &rendered == original {
                (rendered, None)
            } else {
                (original.clone(), Some(rendered))
            };
            TerraneError::raised_with_rendered_message(
                TerraneErrorKind::Custom(DescriptorId(0)),
                message,
                rendered,
                0 /* terrane-site: case.trn:11:5-11:24 */,
            )
        });
    }
    pub fn render(&self) -> String {
        match self {
            Self::Own(value) => value.render(),
            Self::ChildError(value) => value.render(),
        }
    }
    pub fn terrane_field_message(&self) -> &String {
        match self {
            Self::Own(value) => &value.message,
            Self::ChildError(value) => &value.message,
        }
    }
    pub fn terrane_field_message_mut(&mut self) -> &mut String {
        match self {
            Self::Own(value) => &mut value.message,
            Self::ChildError(value) => &mut value.message,
        }
    }
}
#[derive(Clone)]
pub struct ChildError {
    pub message: String,
}
impl ChildError {
    pub fn terrane_construct(message: String) -> Self {
        let mut __terrane_constructed_value = Self { message: String::from("") };
        __terrane_constructed_value.construct(message);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, message: String) {
        self.message = message;
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
    pub fn terrane_static_raise(message: String) -> Result<(), TerraneError> {
        return Err({
            let value = ChildError::terrane_construct(message);
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
                0 /* terrane-site: case.trn:11:5-11:24 */,
            )
        });
    }
}
#[derive(Clone)]
pub struct ZeroError {
    pub message: String,
}
impl ZeroError {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("zero self"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
    pub fn terrane_static_raise_bare() -> Result<(), TerraneError> {
        return Err({
            let value = ZeroError::terrane_construct();
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
                1 /* terrane-site: case.trn:20:5-20:15 */,
            )
        });
    }
    pub fn terrane_static_raise_empty() -> Result<(), TerraneError> {
        return Err({
            let value = ZeroError::terrane_construct();
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
                2 /* terrane-site: case.trn:22:5-22:16 */,
            )
        });
    }
}
fn main() {
    let mut failure: Option<TerraneError> = None;
    let mut failure_2: Option<TerraneError> = None;
    let mut failure_3: Option<TerraneError> = None;
    let mut failure_4: Option<TerraneError> = None;
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            __terrane_traced_completion!(
                BaseError::terrane_static_raise(String::from("from self")),
                3 /* terrane-site: case.trn:26:5-26:35 */
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
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            __terrane_traced_completion!(
                ChildError::terrane_static_raise(String::from("inherited self")),
                4 /* terrane-site: case.trn:30:5-30:41 */
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
                    && __terrane_error_1.kind
                        == TerraneErrorKind::Custom(DescriptorId(1))
                {
                    __terrane_handled_1 = true;
                    let _ = failure_2.insert(__terrane_error_1.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&failure_2.as_ref()
                        .expect("flow-proven available binding").clone().message()
                        .to_owned())
                    );
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&failure_2.as_ref()
                        .expect("flow-proven available binding").clone().render()
                        .contains(&String::from("::child-error: inherited self")))
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
            __terrane_traced_completion!(
                ZeroError::terrane_static_raise_bare(), 5 /* terrane-site: case.trn:35:5-35:28 */
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
                        == TerraneErrorKind::Custom(DescriptorId(2))
                {
                    __terrane_handled_2 = true;
                    let _ = failure_3.insert(__terrane_error_2.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&failure_3.as_ref()
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
            __terrane_traced_completion!(
                ZeroError::terrane_static_raise_empty(), 6 /* terrane-site: case.trn:39:5-39:29 */
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
                    && __terrane_error_3.kind
                        == TerraneErrorKind::Custom(DescriptorId(2))
                {
                    __terrane_handled_3 = true;
                    let _ = failure_4.insert(__terrane_error_3.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&failure_4.as_ref()
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
}
