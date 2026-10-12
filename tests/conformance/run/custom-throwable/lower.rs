// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: custom-throwable
#[derive(Clone)]
pub struct ConfigError {
    pub message: String,
    pub path: String,
}
impl ConfigError {
    pub fn terrane_construct(path: String, message: String) -> Self {
        let mut __terrane_constructed_value = Self {
            message: String::from(""),
            path: String::from(""),
        };
        __terrane_constructed_value.construct(path, message);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, path: String, message: String) {
        self.path = path;
        self.message = message;
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
fn load(path: String) -> Result<String, TerraneError> {
    return Err({
        let value = ConfigError::terrane_construct(
            path,
            String::from("configuration is invalid"),
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
            0 /* terrane-site: case.trn:14:3-14:63 */,
        )
    });
}
fn main() {
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            __terrane_traced_completion!(
                load(String::from("settings.toml")), 1 /* terrane-site: case.trn:18:5-18:25 */
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
}
