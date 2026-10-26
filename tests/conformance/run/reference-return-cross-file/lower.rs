// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: reference-return-cross-file
fn locate<'a>(
    points: &'a terrane_collection_support::Map<i64, Point>,
    id: i64,
) -> Result<&'a Point, TerraneError> {
    let __terrane_completion_0: TerraneCompletion<&Point> = (|| {
        let __terrane_try_0: TerraneCompletion<&Point> = (|| {
            return TerraneCompletion::Return(
                __terrane_raised_completion!(
                    points.get(&id).ok_or(terrane_collection_support::MissingKey),
                    0 /* terrane-site: src/main.trn:8:20-8:30 */
                ),
            );
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
                    return TerraneCompletion::Error(
                        {
                            let value = LookupError::terrane_construct();
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
                                1 /* terrane-site: src/main.trn:10:9-10:28 */,
                            )
                        }
                            .with_cause(__terrane_error_0.clone()),
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
        TerraneCompletion::Normal => {
            __terrane_generated_defect("non-fallthrough try completed normally")
        }
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
fn main() {
    let mut points: terrane_collection_support::Map<i64, Point>;
    let borrowed: &Point;
    points = terrane_collection_support::Map::<i64, Point>::new(vec![]);
    {
        let __terrane_collection_argument_0 = 1;
        let __terrane_collection_argument_1 = Point::terrane_construct();
        points.set(__terrane_collection_argument_0, __terrane_collection_argument_1);
    };
    borrowed = __terrane_traced(
        locate(&points, 1),
        2 /* terrane-site: src/main.trn:15:16-15:37 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = borrowed
        .latitude; __terrane_snapshot })
    );
}
// Source: src/model.trn
// Namespace: reference-return-cross-file
#[derive(Clone)]
pub struct Point {
    pub latitude: f64,
}
impl Point {
    pub fn terrane_construct() -> Self {
        Self { latitude: 51.5 }
    }
}
#[allow(dead_code)]
fn describe(name: String) -> String {
    return {
        let mut __terrane_text = String::with_capacity(32);
        __terrane_text.push_str("Unknown camera catalogue entry: ");
        terrane_scalar_support::ScalarDisplay::write_scalar(&name, &mut __terrane_text);
        __terrane_text
    };
}
#[derive(Clone)]
pub struct LookupError {
    pub message: String,
}
impl LookupError {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("unknown point"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
