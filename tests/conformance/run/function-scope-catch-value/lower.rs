// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: function-scope-catch-value
fn caught_value(choose: bool) -> String {
    let mut caught: Option<TerraneError> = None;
    let __terrane_completion_0: TerraneCompletion<String> = (|| {
        let __terrane_try_0: TerraneCompletion<String> = (|| {
            if choose {
                return TerraneCompletion::Error(
                    TerraneError::raised(
                        TerraneErrorKind::CoercionError,
                        0 /* terrane-site: case.trn:6:13-6:33 */,
                    ),
                );
            }
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
                    let _ = caught.insert(__terrane_error_0.clone());
                    println!("{}", String::from("caught"));
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
    return caught
        .as_ref()
        .unwrap_or_else(|| __terrane_uninitialized_binding("caught", "case.trn", 9, 12))
        .clone()
        .message()
        .to_owned()
        .clone();
}
fn main() {
    println!("{}", terrane_scalar_support::scalar_text(&caught_value(true)));
}
