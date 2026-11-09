// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: structured-assignment-blocks
#[derive(Clone)]
pub struct Settings {
    pub width: terrane_int_support::Int,
    pub height: terrane_int_support::Int,
    pub title: String,
}
impl Settings {
    pub fn terrane_construct(width: terrane_int_support::Int) -> Self {
        let mut __terrane_constructed_value = Self {
            width: terrane_int_support::Int::from(10_i128),
            height: terrane_int_support::Int::from(20_i128),
            title: String::from("default"),
        };
        __terrane_constructed_value.construct(width);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, width: terrane_int_support::Int) {
        {
            let __terrane_value: terrane_int_support::Int = width.clone();
            self.width = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct Token {
    __terrane_lifetime: std::sync::Arc<()>,
    pub value: terrane_int_support::Int,
}
impl Token {
    pub fn terrane_construct() -> Self {
        let mut __terrane_constructed_value = Self {
            value: terrane_int_support::Int::from(1_i128),
            __terrane_lifetime: std::sync::Arc::new(()),
        };
        __terrane_constructed_value.construct();
        __terrane_constructed_value
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn construct(&mut self) {
        println!("{}", String::from("construct"));
    }
    pub fn destruct(&mut self) {
        println!("{}", String::from("drop"));
    }
}
impl Drop for Token {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) != 1 {
            return;
        }
        self.destruct();
    }
}
#[derive(Clone)]
pub struct Failure {
    pub message: String,
}
impl Failure {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("failed"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
fn fail() -> Result<terrane_int_support::Int, TerraneError> {
    return Err({
        let value = Failure::terrane_construct();
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
            0 /* terrane-site: case.trn:24:5-24:28 */,
        )
    });
}
fn make(width: terrane_int_support::Int) -> Settings {
    return {
        let mut instance = Settings::terrane_construct(width.clone());
        {
            let __terrane_value: terrane_int_support::Int = width.clone()
                + terrane_int_support::Int::from(1_i128);
            instance.height = __terrane_value;
        }
        instance
    };
}
fn consume(value: Settings) {
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&value.width), String::from(":"),
        terrane_scalar_support::scalar_text(&value.height)
    );
}
fn main() {
    let height: i64;
    let request: std::sync::Arc<TerraneSharedCell<Settings>>;
    let alias: std::sync::Arc<TerraneSharedCell<Settings>>;
    let ordinary: Settings;
    let plain: std::sync::Arc<TerraneSharedCell<Token>>;
    let observed: std::sync::Arc<TerraneSharedCell<Token>>;
    let nested: Settings;
    height = 7;
    request = std::sync::Arc::new(
        TerraneSharedCell::new({
            let mut instance = Settings::terrane_construct(
                terrane_int_support::Int::from(3_i128),
            );
            {
                let __terrane_value: terrane_int_support::Int = terrane_int_support::Int::from(
                    height as i128,
                );
                instance.height = __terrane_value;
            }
            {
                let __terrane_value: String = String::from("made");
                instance.title = __terrane_value;
            }
            instance
        }),
    );
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = {
        let __terrane_result = request.lock().expect("reference lock poisoned").width
        .clone(); __terrane_result }; __terrane_snapshot }), String::from(":"),
        terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = { let
        __terrane_result = request.lock().expect("reference lock poisoned").height
        .clone(); __terrane_result }; __terrane_snapshot }), String::from(":"),
        terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = { let
        __terrane_result = request.lock().expect("reference lock poisoned").title
        .clone(); __terrane_result }; __terrane_snapshot })
    );
    alias = request.clone();
    {
        let __terrane_owner_0 = request.clone();
        let __terrane_value: terrane_int_support::Int = {
            let __terrane_snapshot = {
                let __terrane_result = request
                    .lock()
                    .expect("reference lock poisoned")
                    .width
                    .clone();
                __terrane_result
            };
            __terrane_snapshot
        } + terrane_int_support::Int::from(1_i128);
        __terrane_owner_0.lock().expect("reference lock poisoned").width = __terrane_value;
    }
    {
        let __terrane_owner_1 = request.clone();
        let __terrane_value: terrane_int_support::Int = {
            let __terrane_snapshot = {
                let __terrane_result = request
                    .lock()
                    .expect("reference lock poisoned")
                    .width
                    .clone();
                __terrane_result
            };
            __terrane_snapshot
        } + terrane_int_support::Int::from(2_i128);
        __terrane_owner_1.lock().expect("reference lock poisoned").height = __terrane_value;
    }
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot =
        alias.lock().expect("shared reference lock poisoned").width.clone();
        __terrane_snapshot }), String::from(":"), terrane_scalar_support::scalar_text(&{
        let __terrane_snapshot = alias.lock().expect("shared reference lock poisoned")
        .height.clone(); __terrane_snapshot }), String::from(":"),
        terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = alias.lock()
        .expect("shared reference lock poisoned").title.clone(); __terrane_snapshot })
    );
    ordinary = Settings::terrane_construct(terrane_int_support::Int::from(10_i128));
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&ordinary.width),
        String::from(":"), terrane_scalar_support::scalar_text(&ordinary.height)
    );
    plain = std::sync::Arc::new(TerraneSharedCell::new(Token::terrane_construct()));
    observed = plain.clone();
    {
        let __terrane_owner_2 = plain.clone();
        let __terrane_value: terrane_int_support::Int = terrane_int_support::Int::from(
            3_i128,
        );
        __terrane_owner_2.lock().expect("reference lock poisoned").value = __terrane_value;
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = observed
        .lock().expect("shared reference lock poisoned").value.clone();
        __terrane_snapshot })
    );
    nested = make(terrane_int_support::Int::from(8_i128));
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&nested.width), String::from(":"),
        terrane_scalar_support::scalar_text(&nested.height)
    );
    consume({
        let mut instance = Settings::terrane_construct(
            terrane_int_support::Int::from(9_i128),
        );
        {
            let __terrane_value: terrane_int_support::Int = terrane_int_support::Int::from(
                11_i128,
            );
            instance.height = __terrane_value;
        }
        instance
    });
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            {
                let __terrane_owner_3 = request.clone();
                let __terrane_value: terrane_int_support::Int = terrane_int_support::Int::from(
                    42_i128,
                );
                __terrane_owner_3.lock().expect("reference lock poisoned").width = __terrane_value;
            }
            {
                let __terrane_owner_4 = request.clone();
                let __terrane_value: terrane_int_support::Int = __terrane_traced_completion!(
                    fail(), 1 /* terrane-site: case.trn:59:22-59:27 */
                );
                __terrane_owner_4.lock().expect("reference lock poisoned").height = __terrane_value;
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
                    println!(
                        "{}{}{}", terrane_scalar_support::scalar_text(&{ let
                        __terrane_snapshot = alias.lock()
                        .expect("shared reference lock poisoned").width.clone();
                        __terrane_snapshot }), String::from(":"),
                        terrane_scalar_support::scalar_text(&{ let __terrane_snapshot =
                        alias.lock().expect("shared reference lock poisoned").height
                        .clone(); __terrane_snapshot })
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
            {
                let _: Token = {
                    let mut instance = Token::terrane_construct();
                    {
                        let __terrane_value: terrane_int_support::Int = __terrane_traced_completion!(
                            fail(), 2 /* terrane-site: case.trn:64:39-64:44 */
                        );
                        instance.value = __terrane_value;
                    }
                    instance
                };
            }
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
                    println!("{}", String::from("init-failed"));
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
}
