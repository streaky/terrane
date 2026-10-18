// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: unsafe-method-name-isolation
#[derive(Clone)]
pub struct ConfigError {
    pub message: String,
}
impl ConfigError {
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
#[derive(Clone)]
pub struct Item {
    pub value: terrane_int_support::Int,
}
impl Item {
    pub fn terrane_construct(input: terrane_int_support::Int) -> Self {
        let mut __terrane_constructed_value = Self {
            value: terrane_int_support::Int::from(0_i128),
        };
        __terrane_constructed_value.construct(input);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, input: terrane_int_support::Int) {
        {
            let __terrane_value = input.clone();
            self.value = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct Unrelated {}
impl Unrelated {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    #[allow(unsafe_code)]
    pub unsafe fn _terrane_unsafe_72656e646572(&self) -> String {
        return String::from("unsafe");
    }
}
fn main() {
    let mut failure: Option<TerraneError> = None;
    let ignored: LogValue;
    let created: Item;
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            return TerraneCompletion::Error({
                let value = ConfigError::terrane_construct(String::from("caught"));
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
                    0 /* terrane-site: case.trn:24:5-24:42 */,
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
                        == TerraneErrorKind::Custom(DescriptorId(0))
                {
                    __terrane_handled_0 = true;
                    let _ = failure.insert(__terrane_error_0.clone());
                    println!("{}", String::from("caught"));
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
    ignored = log_text(String::from("hello"));
    let _ = &ignored;
    println!("{}", terrane_scalar_support::scalar_text(&1));
    created = Item::terrane_construct(terrane_int_support::Int::from(4_i128));
    println!("{}", terrane_scalar_support::scalar_text(&created.value));
}
