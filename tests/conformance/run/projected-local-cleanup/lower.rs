// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF0S288 {
    Arm0(terrane_int_support::Int),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF0S288 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF0S722 {
    Arm0(terrane_int_support::Int),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF0S722 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF0S866 {
    Arm0(terrane_int_support::Int),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF0S866 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
#[derive(Clone)]
pub struct Application {}
impl Application {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn values(&self) {
        let signed: Value;
        let unsigned: Value;
        let text: Value;
        let value: TerraneUnionF0S288;
        let value_2: terrane_int_support::Int;
        let value_3: String;
        let value_4: TerraneUnionF0S722;
        let value_5: TerraneUnionF0S866;
        signed = __terrane_raised(
            signed_value(),
            0 /* terrane-site: src/main.trn:7:18-7:31 */,
        );
        unsigned = __terrane_raised(
            unsigned_value(),
            1 /* terrane-site: src/main.trn:8:20-8:35 */,
        );
        text = __terrane_raised(
            text_value(),
            2 /* terrane-site: src/main.trn:9:16-9:27 */,
        );
        match &signed {
            witness::Value::Integer(__terrane_pattern_267_value) => {
                let __terrane_pattern_267_value = terrane_int_support::Int::from(
                    i128::from(*__terrane_pattern_267_value),
                );
                value = TerraneUnionF0S288::Arm0(__terrane_pattern_267_value);
                println!(
                    "{}", terrane_scalar_support::scalar_text(&match &value {
                    TerraneUnionF0S288::Arm0(value) => value, _ =>
                    unreachable!("flow-proven storage refinement") })
                );
            }
            _ => {
                println!("{}", terrane_scalar_support::scalar_text(&0));
            }
        }
        match &unsigned {
            witness::Value::Unsigned(__terrane_pattern_409_value) => {
                let __terrane_pattern_409_value = terrane_int_support::Int::from_u128(
                    *__terrane_pattern_409_value as u128,
                );
                value_2 = __terrane_pattern_409_value;
                println!("{}", terrane_scalar_support::scalar_text(&value_2));
            }
            _ => {
                println!("{}", terrane_scalar_support::scalar_text(&0));
            }
        }
        let __terrane_match_value_521 = text;
        match __terrane_match_value_521 {
            witness::Value::Text(__terrane_pattern_549_value) => {
                value_3 = __terrane_pattern_549_value;
                println!("{}", terrane_scalar_support::scalar_text(&value_3));
            }
            _ => {
                println!("{}", String::from("wrong variant"));
            }
        }
        let __terrane_match_value_671 = signed;
        match __terrane_match_value_671 {
            witness::Value::Integer(__terrane_pattern_701_value) => {
                let __terrane_pattern_701_value = terrane_int_support::Int::from(
                    i128::from(__terrane_pattern_701_value),
                );
                value_4 = TerraneUnionF0S722::Arm0(__terrane_pattern_701_value);
                println!(
                    "{}", terrane_scalar_support::scalar_text(&match &value_4 {
                    TerraneUnionF0S722::Arm0(value) => value, _ =>
                    unreachable!("flow-proven storage refinement") })
                );
            }
            _ => {
                println!("{}", terrane_scalar_support::scalar_text(&0));
            }
        }
        let __terrane_match_value_812 = unsigned;
        match __terrane_match_value_812 {
            witness::Value::Unsigned(__terrane_pattern_844_value) => {
                let __terrane_pattern_844_value = terrane_int_support::Int::from_u128(
                    __terrane_pattern_844_value as u128,
                );
                value_5 = TerraneUnionF0S866::Arm0(__terrane_pattern_844_value);
                println!(
                    "{}", terrane_scalar_support::scalar_text(&match &value_5 {
                    TerraneUnionF0S866::Arm0(value) => value, _ =>
                    unreachable!("flow-proven storage refinement") })
                );
            }
            _ => {
                println!("{}", terrane_scalar_support::scalar_text(&0));
            }
        }
    }
    pub async fn inspect(&self) -> Result<(), TerraneError> {
        let native: Native;
        let mut text: Option<String> = None;
        let mut data: Option<Vec<u8>> = None;
        let mut checksum: Option<terrane_int_support::Int> = None;
        native = __terrane_raised_err(
            terrane_static_trn_4e6174697665_new(),
            3 /* terrane-site: src/main.trn:37:18-37:30 */,
        )?;
        let mut __terrane_finally_guard_0 = __terrane_finally_guard();
        let __terrane_maybe_completion_0: Option<TerraneCompletion<()>> = __terrane_cancel_operation(
                &__terrane_finally_guard_0,
                async {
                    let __terrane_try_0: TerraneCompletion<()> = async {
                        let _ = text.insert(String::from("local-text"));
                        let _ = data.insert(Vec::<u8>::from([65, 66, 67, 68]));
                        let _ = checksum
                            .insert(
                                __terrane_traced_completion!(
                                    __terrane_await({ let __terrane_future = { let
                                    __terrane_call = (&native).inspect(&* text.as_ref()
                                    .expect("flow-proven available binding"), &* data.as_ref()
                                    .expect("flow-proven available binding")); Box::pin(async
                                    move { match crate
                                    ::__terrane_dependency_await_unwind(__terrane_call). await {
                                    Ok(Ok(value)) =>
                                    Ok(terrane_int_support::Int::from(i128::from(value))),
                                    Ok(Err(error)) => Err(crate ::TerraneForeignError(crate
                                    ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                                    .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                                    Err(crate ::__terrane_dependency_panic(payload, "witness",
                                    "witness::Native::inspect")) } }) }; async move {
                                    __terrane_raised_err(__terrane_future. await,
                                    4 /* terrane-site: src/main.trn:41:30-41:56 */) } }).
                                    await, 4 /* terrane-site: src/main.trn:41:30-41:56 */
                                ),
                            );
                        println!(
                            "{}", terrane_scalar_support::scalar_text(&checksum.as_ref()
                            .expect("flow-proven available binding"))
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
                            if !__terrane_handled_0 {
                                return TerraneCompletion::Error(__terrane_error_0);
                            }
                        }
                    }
                    TerraneCompletion::Normal
                },
            )
            .await;
        let __terrane_cancelled_0 = __terrane_maybe_completion_0.is_none();
        let mut __terrane_completion_0 = __terrane_maybe_completion_0
            .unwrap_or(TerraneCompletion::Normal);
        let __terrane_finally_0: TerraneCompletion<()> = async {
            __terrane_await(cleanup()).await;
            TerraneCompletion::Normal
        }
            .await;
        match __terrane_finally_0 {
            TerraneCompletion::Normal => {}
            replacement => __terrane_completion_0 = replacement,
        }
        if __terrane_cancelled_0
            && matches!(&__terrane_completion_0, TerraneCompletion::Normal)
        {
            __terrane_finish_cancelled_finally(__terrane_finally_guard_0).await;
        }
        __terrane_finally_guard_0.finish();
        match __terrane_completion_0 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return Ok(value),
            TerraneCompletion::Error(error) => return Err(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        Ok(())
    }
    pub async fn __terrane_shared_async_inspect(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
    ) -> Result<(), TerraneError> {
        let native: Native;
        let mut text: Option<String> = None;
        let mut data: Option<Vec<u8>> = None;
        let mut checksum: Option<terrane_int_support::Int> = None;
        native = __terrane_raised_err(
            terrane_static_trn_4e6174697665_new(),
            3 /* terrane-site: src/main.trn:37:18-37:30 */,
        )?;
        let mut __terrane_finally_guard_1 = __terrane_finally_guard();
        let __terrane_maybe_completion_1: Option<TerraneCompletion<()>> = __terrane_cancel_operation(
                &__terrane_finally_guard_1,
                async {
                    let __terrane_try_1: TerraneCompletion<()> = async {
                        let _ = text.insert(String::from("local-text"));
                        let _ = data.insert(Vec::<u8>::from([65, 66, 67, 68]));
                        let _ = checksum
                            .insert(
                                __terrane_traced_completion!(
                                    __terrane_await({ let __terrane_future = { let
                                    __terrane_call = (&native).inspect(&* text.as_ref()
                                    .expect("flow-proven available binding"), &* data.as_ref()
                                    .expect("flow-proven available binding")); Box::pin(async
                                    move { match crate
                                    ::__terrane_dependency_await_unwind(__terrane_call). await {
                                    Ok(Ok(value)) =>
                                    Ok(terrane_int_support::Int::from(i128::from(value))),
                                    Ok(Err(error)) => Err(crate ::TerraneForeignError(crate
                                    ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                                    .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                                    Err(crate ::__terrane_dependency_panic(payload, "witness",
                                    "witness::Native::inspect")) } }) }; async move {
                                    __terrane_raised_err(__terrane_future. await,
                                    4 /* terrane-site: src/main.trn:41:30-41:56 */) } }).
                                    await, 4 /* terrane-site: src/main.trn:41:30-41:56 */
                                ),
                            );
                        println!(
                            "{}", terrane_scalar_support::scalar_text(&checksum.as_ref()
                            .expect("flow-proven available binding"))
                        );
                        TerraneCompletion::Normal
                    }
                        .await;
                    match __terrane_try_1 {
                        TerraneCompletion::Return(value) => {
                            return TerraneCompletion::Return(value);
                        }
                        TerraneCompletion::Break => return TerraneCompletion::Break,
                        TerraneCompletion::Continue => return TerraneCompletion::Continue,
                        TerraneCompletion::Normal => {}
                        TerraneCompletion::Error(__terrane_error_1) => {
                            let mut __terrane_handled_1 = false;
                            if !__terrane_handled_1 {
                                return TerraneCompletion::Error(__terrane_error_1);
                            }
                        }
                    }
                    TerraneCompletion::Normal
                },
            )
            .await;
        let __terrane_cancelled_1 = __terrane_maybe_completion_1.is_none();
        let mut __terrane_completion_1 = __terrane_maybe_completion_1
            .unwrap_or(TerraneCompletion::Normal);
        let __terrane_finally_1: TerraneCompletion<()> = async {
            __terrane_await(cleanup()).await;
            TerraneCompletion::Normal
        }
            .await;
        match __terrane_finally_1 {
            TerraneCompletion::Normal => {}
            replacement => __terrane_completion_1 = replacement,
        }
        if __terrane_cancelled_1
            && matches!(&__terrane_completion_1, TerraneCompletion::Normal)
        {
            __terrane_finish_cancelled_finally(__terrane_finally_guard_1).await;
        }
        __terrane_finally_guard_1.finish();
        match __terrane_completion_1 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return Ok(value),
            TerraneCompletion::Error(error) => return Err(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        Ok(())
    }
}
async fn cleanup() {
    println!("{}", String::from("cleanup"));
}
fn main() {
    __terrane_run(async move {
        let app: std::sync::Arc<TerraneSharedCell<Application>>;
        let shared: std::sync::Arc<TerraneSharedCell<Application>>;
        app = std::sync::Arc::new(
            TerraneSharedCell::new(Application::terrane_construct()),
        );
        __terrane_traced(
            __terrane_await({
                    let __terrane_shared_call_handle = app.clone();
                    Application::__terrane_shared_async_inspect(
                        __terrane_shared_call_handle,
                    )
                })
                .await,
            5 /* terrane-site: src/main.trn:51:11-51:23 */,
        );
        {
            let __terrane_owner = app.clone();
            let __terrane_call_result = __terrane_owner
                .lock()
                .expect("reference lock poisoned")
                .values();
            __terrane_call_result
        };
        shared = app.clone();
        __terrane_traced(
            __terrane_await({
                    let __terrane_shared_call_handle = shared.clone();
                    Application::__terrane_shared_async_inspect(
                        __terrane_shared_call_handle,
                    )
                })
                .await,
            6 /* terrane-site: src/main.trn:54:11-54:26 */,
        );
        {
            let __terrane_owner = shared.clone();
            let __terrane_call_result = __terrane_owner
                .lock()
                .expect("shared reference lock poisoned")
                .values();
            __terrane_call_result
        };
    });
}
