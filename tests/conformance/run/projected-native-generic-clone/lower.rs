// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn copy(source: &witness::CloneBox<u8>) -> witness::CloneBox<u8> {
    return __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| <witness::CloneBox<
                u8,
            > as core::clone::Clone>::clone(&*source)),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::CloneBox::clone",
                    ),
                )
            }
        },
        0 /* terrane-site: src/main.trn:7:12-7:25 */,
    );
}
fn main() {
    __terrane_run(async move {
        let value: u8;
        let original: witness::CloneBox<u8>;
        let copied: witness::CloneBox<u8>;
        let mut local: Option<witness::CloneBox<u8>> = None;
        let mut failure: Option<TerraneError> = None;
        value = 42;
        original = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| witness::make(value)),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "dependency",
                            "dependency::ForeignType::make",
                        ),
                    )
                }
            },
            1 /* terrane-site: src/main.trn:11:34-11:56 */,
        );
        copied = copy(&original);
        println!("{}", terrane_scalar_support::scalar_text(&copied.value));
        let __terrane_completion_0: TerraneCompletion<()> = async {
            let __terrane_try_0: TerraneCompletion<()> = async {
                let _ = local
                    .insert(
                        __terrane_raised_completion!(
                            match
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                            witness::make(value))) { Ok(value) => Ok(value), Err(payload)
                            => Err(crate ::__terrane_dependency_panic(payload,
                            "dependency", "dependency::ForeignType::make")) },
                            2 /* terrane-site: src/main.trn:15:35-15:57 */
                        ),
                    );
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&__terrane_traced_completion!(__terrane_await({
                    let __terrane_future = { let __terrane_call = (&local.as_ref()
                    .expect("flow-proven available binding")).read(true); Box::pin(async
                    move { match crate
                    ::__terrane_dependency_await_unwind(std::future::IntoFuture::into_future(__terrane_call))
                    . await { Ok(Ok(value)) => Ok(value), Ok(Err(error)) => Err(crate
                    ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
                    ::TERRANE_DEPENDENCY_ERROR,
                    format!("Rust dependency `{}` member `{}` failed: {error}",
                    "witness", "witness::CloneBox::read"), crate ::TERRANE_NO_SITE))),
                    Err(payload) => Err(crate ::__terrane_dependency_panic(payload,
                    "witness", "witness::CloneBox::read")) } }) }; async move {
                    __terrane_raised_err(__terrane_future. await, 3 /* terrane-site: src/main.trn:16:22-16:48 */) } }). await, 3 /* terrane-site: src/main.trn:16:22-16:48 */))
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
                        __terrane_handled_0 = true;
                        println!("{}", String::from("unexpected native failure"));
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
        let __terrane_completion_1: TerraneCompletion<()> = (|| {
            let __terrane_try_1: TerraneCompletion<()> = (|| {
                __terrane_raised_completion!(
                    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                    (&original).reject(true))) { Ok(Ok(value)) => Ok(value),
                    Ok(Err(error)) => Err(crate ::TerraneForeignError(crate
                    ::TerraneError::custom_raised(crate ::TERRANE_DEPENDENCY_ERROR,
                    format!("Rust dependency `{}` member `{}` failed: {error}",
                    "witness", "witness::CloneBox::reject"), crate ::TERRANE_NO_SITE))),
                    Err(payload) => Err(crate ::__terrane_dependency_panic(payload,
                    "witness", "witness::CloneBox::reject")) }, 4 /* terrane-site: src/main.trn:20:9-20:40 */
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
                        let _ = failure.insert(__terrane_error_1.clone());
                        println!(
                            "{}", terrane_scalar_support::scalar_text(&failure.as_ref()
                            .expect("flow-proven available binding").clone().message()
                            .to_owned().contains(&String::from("CloneBox::reject")))
                        );
                        println!(
                            "{}", terrane_scalar_support::scalar_text(&failure.as_ref()
                            .expect("flow-proven available binding").clone().message()
                            .to_owned().contains(&String::from("<")))
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
    });
}
