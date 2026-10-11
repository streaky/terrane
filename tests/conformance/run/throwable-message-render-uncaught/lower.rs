// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: throwable-message-render-uncaught
#[derive(Clone)]
pub struct SplitError {
    pub message: String,
}
impl SplitError {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("message field"),
        }
    }
    pub fn render(&self) -> String {
        return String::from("render text");
    }
}
fn main() {
    __terrane_uncaught({
        let value = SplitError::terrane_construct();
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
            0 /* terrane-site: case.trn:8:3-8:20 */,
        )
    });
}
