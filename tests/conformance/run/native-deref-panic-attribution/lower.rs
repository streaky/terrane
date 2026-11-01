// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let locks: Locks;
    locks = __terrane_raised(
        terrane_static_trn_4c6f636b73_new(),
        0 /* terrane-site: src/main.trn:5:13-5:24 */,
    );
    {
        let guard: std::sync::MutexGuard<'_, panic_api::Record> = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| (&locks).locked()),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "panic_api",
                            "panic_api::Locks::locked",
                        ),
                    )
                }
            },
            1 /* terrane-site: src/main.trn:6:33-6:46 */,
        );
        let __terrane_completion_0: TerraneCompletion<()> = (|| {
            let __terrane_try_0: TerraneCompletion<()> = (|| {
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&__terrane_raised_completion!(match
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| <
                    panic_api::Record > ::value(&guard))) { Ok(value) =>
                    Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload)
                    => Err(crate ::__terrane_dependency_panic(payload, "panic_api",
                    "panic_api::Record::value")) }, 2 /* terrane-site: src/main.trn:8:21-8:33 */))
                );
                TerraneCompletion::Normal
            })();
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
                            == TerraneErrorKind::Custom(DescriptorId(1))
                    {
                        __terrane_handled_0 = true;
                        let failure = __terrane_error_0.clone();
                        let message: String = failure.message().to_owned().clone();
                        println!(
                            "{}", terrane_scalar_support::scalar_text(&{ let
                            __terrane_string_argument_0 = "panic_api::Record::value";
                            message.contains(std::convert::AsRef:: < str >
                            ::as_ref(&__terrane_string_argument_0)) })
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
}
