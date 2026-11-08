// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: static-async-throwing-dispatch
#[derive(Clone)]
pub struct Failure {
    pub message: String,
}
impl Failure {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("async field failure"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
async fn safe() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(7_i128);
}
async fn bad() -> Result<terrane_int_support::Int, TerraneError> {
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
            0 /* terrane-site: case.trn:10:5-10:28 */,
        )
    });
}
pub static TERRANE_STATIC_HOLDER_F: std::sync::LazyLock<
    TerraneSharedCell<
        std::sync::Arc<
            dyn Fn(
            ) -> std::pin::Pin<
                    Box<
                        dyn Future<
                            Output = Result<terrane_int_support::Int, TerraneError>,
                        > + Send,
                    >,
                > + Send + Sync,
        >,
    >,
> = std::sync::LazyLock::new(|| TerraneSharedCell::new(
    std::sync::Arc::new(move || -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
        Box::pin(async move { Ok(safe().await) })
    }),
));
#[derive(Clone)]
pub struct Holder {}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self {}
    }
}
fn main() {
    __terrane_run(async move {
        let mut error: Option<TerraneError> = None;
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await({
            let __terrane_value = TERRANE_STATIC_HOLDER_F.lock()
            .expect("static field lock poisoned").clone(); __terrane_value } ()). await,
            1 /* terrane-site: case.trn:14:19-14:31 */))
        );
        {
            let __terrane_static_value = std::sync::Arc::new(move || -> std::pin::Pin<
                Box<dyn Future<Output = _> + Send>,
            > { Box::pin(bad()) });
            *TERRANE_STATIC_HOLDER_F.lock().expect("static field lock poisoned") = __terrane_static_value;
        }
        let __terrane_completion_0: TerraneCompletion<()> = async {
            let __terrane_try_0: TerraneCompletion<()> = async {
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&__terrane_traced_completion!(__terrane_await({
                    let __terrane_value = TERRANE_STATIC_HOLDER_F.lock()
                    .expect("static field lock poisoned").clone(); __terrane_value } ()).
                    await, 2 /* terrane-site: case.trn:17:23-17:35 */))
                );
                TerraneCompletion::Normal
            }
                .await;
            match __terrane_try_0 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
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
                        let _ = error.insert(__terrane_error_0.clone());
                        {
                            let __terrane_error_text_0 = error
                                .as_ref()
                                .expect("flow-proven available binding")
                                .message();
                            println!("{}", __terrane_error_text_0);
                        };
                    }
                    if !__terrane_handled_0 {
                        return TerraneCompletion::Error(__terrane_error_0);
                    }
                }
            }
            TerraneCompletion::Normal
        }
            .await;
        match __terrane_completion_0 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
    });
}
