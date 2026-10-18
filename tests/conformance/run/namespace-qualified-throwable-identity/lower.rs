// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
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
                    println!("{}", String::from("wrong class"));
                }
                if !__terrane_handled_0
                    && __terrane_error_0.kind
                        == TerraneErrorKind::Custom(DescriptorId(1))
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("caught-left"));
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
pub struct TerraneNsLeftSharedErrorIdentity2F6C656674007368617265642D6572726F72 {
    pub message: String,
}
impl TerraneNsLeftSharedErrorIdentity2F6C656674007368617265642D6572726F72 {
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
fn fail() -> Result<(), TerraneError> {
    return Err({
        let value = TerraneNsLeftSharedErrorIdentity2F6C656674007368617265642D6572726F72::terrane_construct(
            String::from("left failure"),
        );
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
            1 /* terrane-site: left/error.trn:12:3-12:45 */,
        )
    });
}
// Source: right/error.trn
// Namespace: right
#[derive(Clone)]
pub struct TerraneNsRightSharedErrorIdentity2F7269676874007368617265642D6572726F72 {
    pub message: String,
}
impl TerraneNsRightSharedErrorIdentity2F7269676874007368617265642D6572726F72 {
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
