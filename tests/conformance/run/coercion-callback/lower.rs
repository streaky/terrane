// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: coercion-callback
fn source() -> terrane_int_support::Int {
    println!("{}", terrane_scalar_support::scalar_text(&String::from("source")));
    return terrane_int_support::Int::from(7_i128);
}
fn render(__trn_5f76616c7565: terrane_int_support::Int) -> String {
    let _ = &__trn_5f76616c7565;
    return String::from("converted");
}
#[derive(Clone)]
pub struct Renderer {}
impl Renderer {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn render(&self, __trn_5f76616c7565: terrane_int_support::Int) -> String {
        let _ = &__trn_5f76616c7565;
        return String::from("bound");
    }
}
fn fail(__trn_5f76616c7565: terrane_int_support::Int) -> Result<String, TerraneError> {
    let _ = &__trn_5f76616c7565;
    return Err(
        TerraneError::raised(
            TerraneErrorKind::CoercionError,
            0 /* terrane-site: case.trn:17:3-17:23 */,
        ),
    );
}
fn wrap() -> Result<String, TerraneError> {
    return Ok(
        __terrane_traced_err(
            std::sync::Arc::new(fail)(terrane_int_support::Int::from(7_i128)),
            1 /* terrane-site: case.trn:20:11-20:19 */,
        )?,
    );
}
fn main() {
    println!(
        "{}", terrane_scalar_support::scalar_text(&std::sync::Arc::new(render)
        (source()))
    );
    let closure: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> String + Send + Sync,
    > = {
        std::sync::Arc::new(move |
            __trn_5f76616c7565: terrane_int_support::Int,
        | -> String {
            return String::from("closure");
        })
    };
    let service: Renderer = Renderer::terrane_construct();
    let bound: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> String + Send + Sync,
    > = {
        let receiver = service;
        std::sync::Arc::new(move |argument_0: terrane_int_support::Int| {
            receiver.render(argument_0)
        })
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&closure.clone()
        (terrane_int_support::Int::from(8_i128)))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&bound.clone()
        (terrane_int_support::Int::from(9_i128)))
    );
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_traced_completion!(wrap(),
                2 /* terrane-site: case.trn:31:13-31:18 */))
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
                    && __terrane_error_0.kind == TerraneErrorKind::CoercionError
                {
                    __terrane_handled_0 = true;
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("caught"))
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
