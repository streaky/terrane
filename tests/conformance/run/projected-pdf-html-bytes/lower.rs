// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let mut document: Option<Pdf> = None;
    let mut data: Option<Vec<u8>> = None;
    let mut failure: Option<TerraneError> = None;
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let _ = document
                .insert(
                    __terrane_raised_completion!(
                        terrane_static_trn_506466_from_html(String::from("<h1>Terrane PDF</h1><p>direct projection</p>")),
                        0 /* terrane-site: src/main.trn:8:20-8:82 */
                    ),
                );
            let _ = data
                .insert(
                    __terrane_raised_completion!(
                        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                        document.as_mut().expect("flow-proven available binding")
                        .save_to_bytes())) { Ok(Ok(value)) => Ok(value), Ok(Err(error))
                        => Err(crate ::TerraneForeignError(crate
                        ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                        .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                        Err(crate ::__terrane_dependency_panic(payload, "pdf_oxide",
                        "pdf_oxide::api::Pdf::save_to_bytes")) }, 1 /* terrane-site: src/main.trn:9:18-9:41 */
                    ),
                );
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&(__terrane_raised_completion!(terrane_collection_support::byte_at(&data
                .as_ref().expect("flow-proven available binding").clone(),
                __terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
                2 /* terrane-site: src/main.trn:10:12-10:19 */)),
                2 /* terrane-site: src/main.trn:10:12-10:19 */) == 37
                &&__terrane_raised_completion!(terrane_collection_support::byte_at(&data
                .as_ref().expect("flow-proven available binding").clone(),
                __terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
                3 /* terrane-site: src/main.trn:10:30-10:37 */)),
                3 /* terrane-site: src/main.trn:10:30-10:37 */) == 80
                &&__terrane_raised_completion!(terrane_collection_support::byte_at(&data
                .as_ref().expect("flow-proven available binding").clone(),
                __terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
                4 /* terrane-site: src/main.trn:10:48-10:55 */)),
                4 /* terrane-site: src/main.trn:10:48-10:55 */) == 68
                &&__terrane_raised_completion!(terrane_collection_support::byte_at(&data
                .as_ref().expect("flow-proven available binding").clone(),
                __terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
                5 /* terrane-site: src/main.trn:10:66-10:73 */)),
                5 /* terrane-site: src/main.trn:10:66-10:73 */) == 70))
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
