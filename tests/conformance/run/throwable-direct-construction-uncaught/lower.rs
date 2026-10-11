// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: throwable-direct-construction-uncaught
#[derive(Clone)]
pub struct UncaughtError {
    pub message: String,
}
impl UncaughtError {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("uncaught direct message"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
fn main() {
    __terrane_uncaught({
        let value = UncaughtError::terrane_construct();
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
            0 /* terrane-site: case.trn:10:3-10:23 */,
        )
    });
}
