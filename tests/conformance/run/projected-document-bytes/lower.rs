// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let mut document: Document;
    let data: Vec<u8>;
    let mut invalid: Option<Document> = None;
    let mut failure: Option<TerraneError> = None;
    document = __terrane_raised(
        terrane_static_trn_446f63756d656e74_from_text(String::from("π")),
        0 /* terrane-site: src/main.trn:7:23-7:47 */,
    );
    data = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| document.save_to_bytes()),
        ) {
            Ok(Ok(value)) => Ok(value),
            Ok(Err(error)) => {
                Err(
                    crate::TerraneForeignError(
                        crate::TerraneError::custom_raised(
                            crate::DescriptorId(2),
                            error.to_string(),
                            crate::TERRANE_NO_SITE,
                        ),
                    ),
                )
            }
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::api::Document::save_to_bytes",
                    ),
                )
            }
        },
        1 /* terrane-site: src/main.trn:8:16-8:39 */,
    );
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&(data.len() as i128)),
        String::from(" "),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::byte_at(&data,
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        2 /* terrane-site: src/main.trn:9:28-9:35 */)), 2 /* terrane-site: src/main.trn:9:28-9:35 */)), String::from(" "),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::byte_at(&data,
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        3 /* terrane-site: src/main.trn:9:42-9:49 */)), 3 /* terrane-site: src/main.trn:9:42-9:49 */))
    );
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let _ = invalid
                .insert(
                    __terrane_raised_completion!(
                        terrane_static_trn_446f63756d656e74_from_text(String::from("")),
                        4 /* terrane-site: src/main.trn:11:15-11:38 */
                    ),
                );
            let _ = &invalid;
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
