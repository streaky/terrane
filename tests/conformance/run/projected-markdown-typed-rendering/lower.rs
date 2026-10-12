// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let gfm: Options;
    let rendered: String;
    let mut invalid: Option<String> = None;
    let mut failure: Option<TerraneError> = None;
    gfm = __terrane_raised(
        terrane_static_trn_4f7074696f6e73_gfm(),
        0 /* terrane-site: src/main.trn:8:9-8:22 */,
    );
    rendered = __terrane_raised(
        to_html_with_options(String::from("~Venus~Mars!"), &gfm),
        1 /* terrane-site: src/main.trn:9:14-9:55 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&rendered));
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let _ = invalid
                .insert(
                    __terrane_raised_completion!(
                        to_html_with_options(String::from("{"),
                        &__terrane_raised_completion!(mdx_options(),
                        2 /* terrane-site: src/main.trn:12:43-12:55 */)),
                        3 /* terrane-site: src/main.trn:12:15-12:56 */
                    ),
                );
            println!(
                "{}", terrane_scalar_support::scalar_text(&invalid.as_ref()
                .expect("flow-proven available binding"))
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
