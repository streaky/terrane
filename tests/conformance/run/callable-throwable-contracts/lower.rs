// Generated deterministically by Terrane <version>.
// Runtime support: async.rs, executor_parallel.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: callable-throwable-contracts
#[derive(Clone)]
pub struct LocalError {
    pub message: String,
}
impl LocalError {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("local failure"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
#[derive(Clone)]
pub struct Unrelated {}
impl Unrelated {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn render(&self, value: terrane_int_support::Int) -> String {
        let _ = &value;
        return String::from("method");
    }
}
#[derive(Clone)]
pub struct Formatter {}
impl Formatter {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn render(
        &self,
        value: terrane_int_support::Int,
    ) -> Result<String, TerraneError> {
        if value.clone() < terrane_int_support::Int::from(0_i128) {
            return Err(
                TerraneError::raised(
                    TerraneErrorKind::CoercionError,
                    0 /* terrane-site: case.trn:16:7-16:27 */,
                ),
            );
        }
        return Ok(String::from("bound"));
    }
    pub fn invoke(
        &self,
        operation: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> Result<String, TerraneError> + Send + Sync,
        >,
        value: terrane_int_support::Int,
    ) -> Result<String, TerraneError> {
        return Ok(
            __terrane_traced_err(
                operation(value.clone()),
                1 /* terrane-site: case.trn:20:12-20:28 */,
            )?,
        );
    }
}
fn invoke(
    operation: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> Result<String, TerraneError> + Send + Sync,
    >,
    value: terrane_int_support::Int,
) -> Result<String, TerraneError> {
    return Ok(
        __terrane_traced_err(
            operation(value.clone()),
            2 /* terrane-site: case.trn:23:10-23:26 */,
        )?,
    );
}
fn safe_render(value: terrane_int_support::Int) -> String {
    let _ = &value;
    return String::from("safe");
}
fn invoke_infallible(
    operation: std::sync::Arc<dyn Fn(terrane_int_support::Int) -> String + Send + Sync>,
    value: terrane_int_support::Int,
) -> String {
    return operation(value.clone());
}
fn loud_render(value: terrane_int_support::Int) -> String {
    let _ = &value;
    return String::from("loud");
}
fn custom_render(value: terrane_int_support::Int) -> Result<String, TerraneError> {
    if value.clone() < terrane_int_support::Int::from(0_i128) {
        return Err({
            let value = LocalError::terrane_construct();
            TerraneError::raised_with_message(
                TerraneErrorKind::Custom(DescriptorId(0)),
                value.render(),
                3 /* terrane-site: case.trn:36:5-36:32 */,
            )
        });
    }
    return Ok(String::from("custom"));
}
#[derive(Clone)]
pub struct OperationHolder {
    pub operation: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> Result<String, TerraneError> + Send + Sync,
    >,
}
impl OperationHolder {
    pub fn terrane_construct() -> Self {
        Self {
            operation: std::sync::Arc::new(move |argument_0: terrane_int_support::Int| Ok(
                safe_render(argument_0),
            )),
        }
    }
}
#[derive(Clone)]
pub struct CallableHolder {
    pub render: std::sync::Arc<dyn Fn(terrane_int_support::Int) -> String + Send + Sync>,
}
impl CallableHolder {
    pub fn terrane_construct() -> Self {
        Self {
            render: std::sync::Arc::new(safe_render),
        }
    }
}
async fn async_render(value: terrane_int_support::Int) -> Result<String, TerraneError> {
    if value.clone() < terrane_int_support::Int::from(0_i128) {
        return Err(
            TerraneError::raised(
                TerraneErrorKind::CoercionError,
                4 /* terrane-site: case.trn:47:5-47:25 */,
            ),
        );
    }
    return Ok(String::from("async"));
}
fn make_render(
    value: terrane_int_support::Int,
) -> std::sync::Arc<
    dyn Fn(terrane_int_support::Int) -> Result<String, TerraneError> + Send + Sync,
> {
    let _ = &value;
    let service: Formatter = Formatter::terrane_construct();
    return {
        let receiver = service;
        std::sync::Arc::new(move |argument_0: terrane_int_support::Int| {
            receiver.render(argument_0)
        })
    };
}
async fn async_safe_render(value: terrane_int_support::Int) -> String {
    let _ = &value;
    return String::from("async-safe");
}
fn invoke_field(holder: OperationHolder) -> String {
    return __terrane_traced(
        (holder.operation)(terrane_int_support::Int::from(1_i128)),
        5 /* terrane-site: case.trn:58:10-58:29 */,
    );
}
fn invoke_maker(
    maker: std::sync::Arc<
        dyn Fn(
            terrane_int_support::Int,
        ) -> Result<
                std::sync::Arc<
                    dyn Fn(
                        terrane_int_support::Int,
                    ) -> Result<String, TerraneError> + Send + Sync,
                >,
                TerraneError,
            > + Send + Sync,
    >,
    value: terrane_int_support::Int,
) -> Result<String, TerraneError> {
    let operation: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> Result<String, TerraneError> + Send + Sync,
    > = __terrane_traced_err(
        maker(value.clone()),
        6 /* terrane-site: case.trn:61:15-61:27 */,
    )?;
    return Ok(
        __terrane_traced_err(
            operation(value.clone()),
            7 /* terrane-site: case.trn:62:10-62:26 */,
        )?,
    );
}
async fn invoke_async(
    operation: std::sync::Arc<
        dyn Fn(
            terrane_int_support::Int,
        ) -> std::pin::Pin<
                Box<dyn Future<Output = Result<String, TerraneError>> + Send>,
            > + Send + Sync,
    >,
    value: terrane_int_support::Int,
) -> Result<String, TerraneError> {
    return Ok(
        __terrane_traced_err(
            __terrane_await(operation(value.clone())).await,
            8 /* terrane-site: case.trn:65:16-65:32 */,
        )?,
    );
}
fn main() {
    __terrane_run(async move {
        let service: Formatter = Formatter::terrane_construct();
        let bound: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> Result<String, TerraneError> + Send + Sync,
        > = {
            let receiver = service.clone();
            std::sync::Arc::new(move |argument_0: terrane_int_support::Int| {
                receiver.render(argument_0)
            })
        };
        let closure: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> Result<String, TerraneError> + Send + Sync,
        > = {
            std::sync::Arc::new(move |
                value: terrane_int_support::Int,
            | -> Result<String, TerraneError> {
                if value.clone() < terrane_int_support::Int::from(0_i128) {
                    return Err(
                        TerraneError::raised(
                            TerraneErrorKind::CoercionError,
                            9 /* terrane-site: case.trn:72:7-72:27 */,
                        ),
                    );
                }
                return Ok(String::from("closure"));
            })
        };
        let broad_closure: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> Result<String, TerraneError> + Send + Sync,
        > = {
            let callable = {
                std::sync::Arc::new(move |value: terrane_int_support::Int| -> String {
                    if value.clone() < terrane_int_support::Int::from(0_i128) {
                        return String::from("negative");
                    }
                    return String::from("wide-closure");
                })
            }
                .clone();
            std::sync::Arc::new(move |argument_0: terrane_int_support::Int| Ok(
                callable(argument_0),
            ))
        };
        let broad_async: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> std::pin::Pin<
                    Box<dyn Future<Output = Result<String, TerraneError>> + Send>,
                > + Send + Sync,
        > = {
            let callable = {
                std::sync::Arc::new(move |
                    value: terrane_int_support::Int,
                | -> std::pin::Pin<Box<dyn Future<Output = String> + Send>> {
                    Box::pin(async move {
                        if value.clone() < terrane_int_support::Int::from(0_i128) {
                            return String::from("negative");
                        }
                        return String::from("wide-async");
                    })
                })
            }
                .clone();
            std::sync::Arc::new(move |
                argument_0: terrane_int_support::Int,
            | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
                {
                    let callable_future = callable(argument_0);
                    Box::pin(async move { Ok(callable_future.await) })
                }
            })
        };
        let alias: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> Result<String, TerraneError> + Send + Sync,
        > = bound.clone();
        let broad: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> Result<String, TerraneError> + Send + Sync,
        > = bound.clone();
        let async_operation: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> std::pin::Pin<
                    Box<dyn Future<Output = Result<String, TerraneError>> + Send>,
                > + Send + Sync,
        > = std::sync::Arc::new(move |
            argument_0: terrane_int_support::Int,
        | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
            Box::pin(async_render(argument_0))
        });
        let safe_operation: std::sync::Arc<
            dyn Fn(terrane_int_support::Int) -> String + Send + Sync,
        > = std::sync::Arc::new(safe_render);
        let async_safe_operation: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> std::pin::Pin<
                    Box<dyn Future<Output = Result<String, TerraneError>> + Send>,
                > + Send + Sync,
        > = std::sync::Arc::new(move |
            argument_0: terrane_int_support::Int,
        | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
            Box::pin(async move { Ok(async_safe_render(argument_0).await) })
        });
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = bound; "coercion-error"
            .to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = bound; "coercion-error"
            .to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = closure;
            "coercion-error".to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = closure;
            "coercion-error".to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = broad; "throwable"
            .to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = broad; "coercion-error"
            .to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = alias; "coercion-error"
            .to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = alias; "coercion-error"
            .to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = async_operation;
            "throwable".to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = async_operation;
            "coercion-error".to_owned() })
        );
        let custom: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> Result<String, TerraneError> + Send + Sync,
        > = std::sync::Arc::new(custom_render);
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = custom; "local-error"
            .to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = custom; "local-error"
            .to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(invoke(alias
            .clone(), terrane_int_support::Int::from(1_i128)), 10 /* terrane-site: case.trn:100:11-100:27 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(invoke(broad
            .clone(), terrane_int_support::Int::from(1_i128)), 11 /* terrane-site: case.trn:101:11-101:27 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(invoke(closure
            .clone(), terrane_int_support::Int::from(1_i128)), 12 /* terrane-site: case.trn:102:11-102:29 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await(invoke_async(async_operation
            .clone(), terrane_int_support::Int::from(1_i128))). await,
            13 /* terrane-site: case.trn:103:16-103:48 */))
        );
        let __terrane_completion_0: TerraneCompletion<()> = async {
            let __terrane_try_0: TerraneCompletion<()> = async {
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&__terrane_traced_completion!(__terrane_await(invoke_async(async_operation
                    .clone(), terrane_int_support::Int::from(- 1_i128))). await,
                    14 /* terrane-site: case.trn:105:18-105:51 */))
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
                        && __terrane_error_0.kind == TerraneErrorKind::CoercionError
                    {
                        __terrane_handled_0 = true;
                        println!(
                            "{}",
                            terrane_scalar_support::scalar_text(&String::from("async-caught"))
                        );
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
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(invoke(broad_closure
            .clone(), terrane_int_support::Int::from(1_i128)), 15 /* terrane-site: case.trn:108:11-108:35 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await(invoke_async(broad_async
            .clone(), terrane_int_support::Int::from(1_i128))). await,
            16 /* terrane-site: case.trn:109:16-109:44 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&safe_operation(terrane_int_support::Int::from(1_i128)))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(invoke_maker(std::sync::Arc::new(move
            | argument_0 : terrane_int_support::Int | Ok(make_render(argument_0))),
            terrane_int_support::Int::from(1_i128)), 17 /* terrane-site: case.trn:111:11-111:39 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(service
            .invoke(bound.clone(), terrane_int_support::Int::from(1_i128)),
            18 /* terrane-site: case.trn:112:11-112:35 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&invoke_infallible(safe_operation
            .clone(), terrane_int_support::Int::from(1_i128)))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await(async_safe_operation(terrane_int_support::Int::from(1_i128)))
            . await, 19 /* terrane-site: case.trn:114:16-114:39 */))
        );
        let holder: OperationHolder = OperationHolder::terrane_construct();
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced((holder
            .operation) (terrane_int_support::Int::from(1_i128)), 20 /* terrane-site: case.trn:116:11-116:30 */))
        );
        let mut callable: CallableHolder = CallableHolder::terrane_construct();
        callable.render = std::sync::Arc::new(loud_render);
        println!(
            "{}", terrane_scalar_support::scalar_text(&(callable.render)
            (terrane_int_support::Int::from(1_i128)))
        );
        let __terrane_completion_1: TerraneCompletion<()> = (|| {
            let __terrane_try_1: TerraneCompletion<()> = (|| {
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&__terrane_traced_completion!(invoke(custom
                    .clone(), terrane_int_support::Int::from(- 1_i128)),
                    21 /* terrane-site: case.trn:121:13-121:31 */))
                );
                TerraneCompletion::Normal
            })();
            match __terrane_try_1 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
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
                        println!(
                            "{}",
                            terrane_scalar_support::scalar_text(&String::from("custom-caught"))
                        );
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
        println!(
            "{}", terrane_scalar_support::scalar_text(&invoke_field(holder.clone()))
        );
    });
}
