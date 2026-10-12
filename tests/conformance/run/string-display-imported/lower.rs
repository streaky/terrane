// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: model/renderers.trn
// Namespace: model
#[derive(Clone)]
pub struct RenderError {
    pub message: String,
}
impl RenderError {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("imported renderer failed"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
#[derive(Clone)]
pub struct Label {}
impl Label {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn to_string(&self) -> String {
        return String::from("imported");
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
                0 /* terrane-site: model/renderers.trn:16:9-16:37 */,
            )
        });
    }
}
// Source: src/main.trn
// Namespace: app
fn main() {
    let value: Label;
    value = Label::terrane_construct();
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(6); __terrane_text.push_str("value:"); __terrane_text
        .push_str(&value.to_string()); __terrane_text })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_separator = &"|"; let
        mut __terrane_text = String::with_capacity(1); __terrane_text.push_str(&value
        .to_string()); __terrane_text.push_str(__terrane_separator); __terrane_text
        .push_str(&value.to_string()); __terrane_text })
    );
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
                String::with_capacity(7); __terrane_text.push_str("broken:");
                __terrane_text
                .push_str(&__terrane_traced_completion!(Broken::terrane_construct()
                .to_string(), 1 /* terrane-site: src/main.trn:10:29-10:45 */));
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
                    println!("{}", String::from("caught imported renderer"));
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
