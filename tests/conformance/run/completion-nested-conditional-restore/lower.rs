// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: completion-nested-conditional-restore
#[derive(Clone)]
pub struct Err {
    pub message: String,
}
impl Err {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("failure"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
fn eat(
    value: terrane_collection_support::List<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    return terrane_int_support::Int::from(value.length());
}
fn restored(
    value: terrane_collection_support::List<terrane_int_support::Int>,
    restore: bool,
) -> terrane_int_support::Int {
    let mut value: Option<terrane_collection_support::List<terrane_int_support::Int>> = Some(
        value,
    );
    let __terrane_completion_0: TerraneCompletion<terrane_int_support::Int> = (|| {
        let __terrane_try_0: TerraneCompletion<terrane_int_support::Int> = (|| {
            let mut __terrane_completion_1: TerraneCompletion<
                terrane_int_support::Int,
            > = (|| {
                let __terrane_try_1: TerraneCompletion<terrane_int_support::Int> = (|| {
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&eat(value.as_ref()
                        .expect("flow-proven available binding").clone()))
                    );
                    return TerraneCompletion::Error({
                        let value = Err::terrane_construct();
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
                            0 /* terrane-site: case.trn:17:13-17:32 */,
                        )
                    });
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
                        if !__terrane_handled_1 {
                            return TerraneCompletion::Error(__terrane_error_1);
                        }
                    }
                }
                TerraneCompletion::Normal
            })();
            let __terrane_finally_1: TerraneCompletion<terrane_int_support::Int> = (|| {
                if restore {
                    let _ = value
                        .insert(
                            terrane_collection_support::List::<
                                terrane_int_support::Int,
                            >::new(vec![terrane_int_support::Int::from(2_i128)]),
                        );
                }
                TerraneCompletion::Normal
            })();
            match __terrane_finally_1 {
                TerraneCompletion::Normal => {}
                replacement => __terrane_completion_1 = replacement,
            }
            match __terrane_completion_1 {
                TerraneCompletion::Normal => {
                    __terrane_generated_defect("non-fallthrough try completed normally")
                }
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Error(error) => return TerraneCompletion::Error(error),
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
            }
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    __terrane_handled_0 = true;
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&__terrane_raised_completion!(value
                        .as_ref().expect("flow-proven available binding")
                        .get_or_error(__terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
                        1 /* terrane-site: case.trn:22:16-22:24 */)),
                        1 /* terrane-site: case.trn:22:16-22:24 */))
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
    return __terrane_raised(
        value
            .as_ref()
            .expect("flow-proven available binding")
            .get_or_error(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(0_i128),
                    ),
                    2 /* terrane-site: case.trn:23:12-23:20 */,
                ),
            ),
        2 /* terrane-site: case.trn:23:12-23:20 */,
    );
}
fn main() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&restored(terrane_collection_support::List::
        < terrane_int_support::Int > ::new(vec![terrane_int_support::Int::from(1_i128)]),
        false))
    );
}
