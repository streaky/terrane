// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: string-display-composition
#[derive(Clone)]
pub struct RenderError {
    pub message: String,
}
impl RenderError {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("cannot render"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
#[derive(Clone)]
pub struct Label {
    pub text: String,
}
impl Label {
    pub fn terrane_construct(text: String) -> Self {
        let mut __terrane_constructed_value = Self { text: String::new() };
        __terrane_constructed_value.construct(text);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, text: String) {
        {
            let __terrane_value = text;
            self.text = __terrane_value;
        }
    }
    pub fn to_string(&self) -> String {
        println!(
            "{}{}", String::from("render:"), terrane_scalar_support::scalar_text(&self
            .text)
        );
        return self.text.clone();
    }
}
pub trait DisplayableProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn DisplayableProtocol>;
    fn separate_box(&self) -> Box<dyn DisplayableProtocol>;
    fn to_string(&self) -> Result<String, TerraneError>;
}
impl Clone for Box<dyn DisplayableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Displayable(Box<dyn DisplayableProtocol>);
impl Clone for Displayable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Displayable {
    pub fn to_string(&self) -> Result<String, TerraneError> {
        self.0.to_string()
    }
}
#[derive(Clone)]
pub struct Broken {}
impl Broken {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn to_string(&self) -> Result<String, TerraneError> {
        return Err({
            let value = RenderError::terrane_construct();
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
                0 /* terrane-site: case.trn:23:9-23:37 */,
            )
        });
    }
}
impl DisplayableProtocol for Broken {
    fn clone_box(&self) -> Box<dyn DisplayableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DisplayableProtocol> {
        Box::new(self.clone())
    }
    fn to_string(&self) -> Result<String, TerraneError> {
        Broken::to_string(&*self)
    }
}
impl From<Broken> for Displayable {
    fn from(value: Broken) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct Calm {}
impl Calm {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn to_string(&self) -> String {
        return String::from("calm");
    }
}
impl DisplayableProtocol for Calm {
    fn clone_box(&self) -> Box<dyn DisplayableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DisplayableProtocol> {
        Box::new(self.clone())
    }
    fn to_string(&self) -> Result<String, TerraneError> {
        Ok(Calm::to_string(&*self))
    }
}
impl From<Calm> for Displayable {
    fn from(value: Calm) -> Self {
        Self(Box::new(value))
    }
}
fn number(value: terrane_int_support::Int) -> terrane_int_support::Int {
    println!(
        "{}{}", String::from("number:"), terrane_scalar_support::scalar_text(&value)
    );
    return value.clone();
}
fn render_generic<TerraneType54: DisplayableProtocol>(
    value: TerraneType54,
) -> Result<String, TerraneError> {
    return Ok({
        let mut __terrane_text = String::with_capacity(8);
        __terrane_text.push_str("generic:");
        __terrane_text
            .push_str(
                &__terrane_traced_err(
                    value.to_string(),
                    1 /* terrane-site: case.trn:34:25-34:30 */,
                )?,
            );
        __terrane_text
    });
}
fn main() {
    let item: Label;
    let erased: Displayable;
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(6);
        terrane_scalar_support::ScalarDisplay::write_scalar(&(terrane_int_support::Int::from(2_i128)
        + terrane_int_support::Int::from(3_i128)), &mut __terrane_text); __terrane_text
        .push_str(" items"); __terrane_text })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(7); __terrane_text.push_str("items: ");
        terrane_scalar_support::ScalarDisplay::write_scalar(&2, &mut __terrane_text);
        terrane_scalar_support::ScalarDisplay::write_scalar(&3, &mut __terrane_text);
        __terrane_text })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(7); __terrane_text.push_str("items: ");
        terrane_scalar_support::ScalarDisplay::write_scalar(&(terrane_int_support::Int::from(2_i128)
        + terrane_int_support::Int::from(3_i128)), &mut __terrane_text); __terrane_text
        })
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(1); __terrane_text.push_str("2");
        terrane_scalar_support::ScalarDisplay::write_scalar(&3, &mut __terrane_text);
        __terrane_text }), String::from("|"), terrane_scalar_support::scalar_text(&{ let
        mut __terrane_text = String::with_capacity(1);
        terrane_scalar_support::ScalarDisplay::write_scalar(&3, &mut __terrane_text);
        __terrane_text.push_str("2"); __terrane_text })
    );
    println!(
        "{}{}{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_scalar_support::scalar_text(&true)),
        String::from("|"),
        terrane_scalar_support::scalar_text(&terrane_scalar_support::scalar_text(&())),
        String::from("|"),
        terrane_scalar_support::scalar_text(&terrane_scalar_support::scalar_text(&42))
    );
    item = Label::terrane_construct(String::from("label"));
    println!("{}", terrane_scalar_support::scalar_text(&item.to_string()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(13); __terrane_text.push_str("before "); __terrane_text
        .push_str(&item.to_string()); __terrane_text.push_str(" after"); __terrane_text
        })
    );
    println!("{}", item.to_string());
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(7); __terrane_text.push_str("prefix "); __terrane_text
        .push_str(&item.to_string());
        terrane_scalar_support::ScalarDisplay::write_scalar(&7, &mut __terrane_text);
        __terrane_text })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_separator = &":"; let
        mut __terrane_text = String::with_capacity(1); __terrane_text.push_str(&item
        .to_string()); __terrane_text.push_str(__terrane_separator);
        terrane_scalar_support::ScalarDisplay::write_scalar(&7, &mut __terrane_text);
        __terrane_text })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(8); __terrane_text.push_str("ordered ");
        terrane_scalar_support::ScalarDisplay::write_scalar(&number(terrane_int_support::Int::from(2_i128)),
        &mut __terrane_text); __terrane_text.push_str(&item.to_string());
        terrane_scalar_support::ScalarDisplay::write_scalar(&number(terrane_int_support::Int::from(3_i128)),
        &mut __terrane_text); __terrane_text })
    );
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
                String::with_capacity(7); __terrane_text.push_str("broken ");
                __terrane_text
                .push_str(&__terrane_traced_completion!(Broken::terrane_construct()
                .to_string(), 2 /* terrane-site: case.trn:50:29-50:45 */));
                __terrane_text })
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
                    println!("{}", String::from("caught addition"));
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
            println!(
                "{}", __terrane_traced_completion!(Broken::terrane_construct()
                .to_string(), 3 /* terrane-site: case.trn:54:17-54:33 */)
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
                        == TerraneErrorKind::Custom(DescriptorId(0))
                {
                    __terrane_handled_1 = true;
                    println!("{}", String::from("caught print"));
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
            {
                let _: String = {
                    let mut __terrane_text = String::with_capacity(0);
                    __terrane_text.push_str("");
                    __terrane_text
                        .push_str(
                            &__terrane_traced_completion!(
                                Broken::terrane_construct().to_string(),
                                4 /* terrane-site: case.trn:58:25-58:41 */
                            ),
                        );
                    __terrane_text
                };
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
                    println!("{}", String::from("caught concat"));
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
            {
                let _: String = {
                    let __terrane_separator = &":";
                    let mut __terrane_text = String::with_capacity(0);
                    __terrane_text
                        .push_str(
                            &__terrane_traced_completion!(
                                Broken::terrane_construct().to_string(),
                                5 /* terrane-site: case.trn:62:24-62:40 */
                            ),
                        );
                    __terrane_text
                };
            }
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
                        == TerraneErrorKind::Custom(DescriptorId(0))
                {
                    __terrane_handled_3 = true;
                    println!("{}", String::from("caught join"));
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
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_traced_completion!(render_generic(Broken::terrane_construct()),
                6 /* terrane-site: case.trn:66:17-66:51 */))
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
                    println!("{}", String::from("caught generic"));
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
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_traced(render_generic(Calm::terrane_construct()),
        7 /* terrane-site: case.trn:69:13-69:45 */))
    );
    erased = <Displayable>::from(Calm::terrane_construct());
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(10); __terrane_text.push_str("interface:"); __terrane_text
        .push_str(&__terrane_traced(erased.to_string(), 8 /* terrane-site: case.trn:71:27-71:33 */)); __terrane_text })
    );
}
