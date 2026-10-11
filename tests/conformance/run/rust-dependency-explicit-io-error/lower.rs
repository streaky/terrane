// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: app
fn main() {
    let original: Error;
    let kind: ErrorKind;
    let occupied: bool;
    let mut failure: Option<TerraneError> = None;
    original = __terrane_raised(
        make_error(),
        0 /* terrane-site: case.trn:5:16-5:27 */,
    );
    kind = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| (&original).kind()),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "std",
                        "std::io::Error::kind",
                    ),
                )
            }
        },
        1 /* terrane-site: case.trn:6:12-6:26 */,
    );
    occupied = __terrane_raised(
        is_occupied(kind),
        2 /* terrane-site: case.trn:7:16-7:33 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&occupied));
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            __terrane_raised_completion!(
                fail(), 3 /* terrane-site: case.trn:10:9-10:14 */
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
                        == TerraneErrorKind::Custom(DescriptorId(2))
                {
                    __terrane_handled_0 = true;
                    let _ = failure.insert(__terrane_error_0.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&failure.as_ref()
                        .expect("flow-proven available binding").clone().message()
                        .to_owned())
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
