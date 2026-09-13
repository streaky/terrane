// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: app/main.trn
// Namespace: app
fn main() {
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            __terrane_traced_completion!(
                fail(), 0 /* terrane-site: app/main.trn:8:5-8:10 */
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
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("wrong class"))
                    );
                }
                if !__terrane_handled_0
                    && __terrane_error_0.kind
                        == TerraneErrorKind::Custom(DescriptorId(1))
                {
                    __terrane_handled_0 = true;
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("caught-left"))
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
}
// Source: left/error.trn
// Namespace: left
#[derive(Clone)]
pub struct TerraneNs4LeftSharedError {
    pub message: String,
}
impl TerraneNs4LeftSharedError {
    pub fn terrane_construct(message: String) -> Self {
        let mut value = Self { message: String::from("") };
        value.construct(message);
        value
    }
    pub fn construct(&mut self, message: String) {
        self.message = message;
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
fn fail() -> Result<(), TerraneError> {
    return Err({
        let value = TerraneNs4LeftSharedError::terrane_construct(
            String::from("left failure"),
        );
        TerraneError::raised_with_message(
            TerraneErrorKind::Custom(DescriptorId(1)),
            value.render(),
            1 /* terrane-site: left/error.trn:12:3-12:45 */,
        )
    });
}
// Source: right/error.trn
// Namespace: right
#[derive(Clone)]
pub struct TerraneNs5RightSharedError {
    pub message: String,
}
impl TerraneNs5RightSharedError {
    pub fn terrane_construct(message: String) -> Self {
        let mut value = Self { message: String::from("") };
        value.construct(message);
        value
    }
    pub fn construct(&mut self, message: String) {
        self.message = message;
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
