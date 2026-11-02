// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn absent() -> Descriptor {
    return {
        let __terrane_conversion_input_1 = None;
        let __terrane_conversion_input_0 = String::from("empty");
        __terrane_raised(
            Descriptor::terrane_construct(
                __terrane_conversion_input_0,
                __terrane_conversion_input_1,
            ),
            0 /* terrane-site: src/main.trn:7:12-7:62 */,
        )
    };
}
fn choose(
    values: terrane_collection_support::Tuple<u32>,
) -> Result<Descriptor, TerraneError> {
    return Ok({
        let __terrane_conversion_input_1 = Some(values);
        let __terrane_conversion_input_0 = String::from("held");
        __terrane_raised_err(
            Descriptor::terrane_construct(
                __terrane_conversion_input_0,
                __terrane_conversion_input_1,
            ),
            1 /* terrane-site: src/main.trn:9:12-9:63 */,
        )?
    });
}
fn mark_config(mark: String) -> Result<Marked, TerraneError> {
    return Ok({
        let __terrane_conversion_input_1 = mark;
        let __terrane_conversion_input_0 = String::from("marked");
        __terrane_raised_err(
            Marked::terrane_construct(
                __terrane_conversion_input_0,
                __terrane_conversion_input_1,
            ),
            2 /* terrane-site: src/main.trn:11:12-11:57 */,
        )?
    });
}
fn rows_config(
    values: terrane_collection_support::List<terrane_collection_support::Tuple<u32>>,
) -> Result<Rows, TerraneError> {
    return Ok({
        let __terrane_conversion_input_1 = values;
        let __terrane_conversion_input_0 = String::from("rows");
        __terrane_raised_err(
            Rows::terrane_construct(
                __terrane_conversion_input_0,
                __terrane_conversion_input_1,
            ),
            3 /* terrane-site: src/main.trn:13:12-13:55 */,
        )?
    });
}
fn main() {
    let empty: Descriptor;
    let invalid: terrane_collection_support::Tuple<u32>;
    let marked: Marked;
    let invalid_rows: terrane_collection_support::List<
        terrane_collection_support::Tuple<u32>,
    >;
    let valid_rows: terrane_collection_support::List<
        terrane_collection_support::Tuple<u32>,
    >;
    let good_rows: Rows;
    empty = absent();
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(describe(&empty),
        4 /* terrane-site: src/main.trn:16:13-16:28 */))
    );
    invalid = terrane_collection_support::Tuple::<u32>::new(vec![1]);
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            __terrane_traced_completion!(
                choose(invalid), 5 /* terrane-site: src/main.trn:19:9-19:24 */
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
                        == TerraneErrorKind::Custom(DescriptorId(0))
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("shape-caught"));
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
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            __terrane_traced_completion!(
                mark_config(String::from("ab")), 6 /* terrane-site: src/main.trn:23:9-23:26 */
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_1 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_1) => {
                let mut __terrane_handled_1 = false;
                if !__terrane_handled_1
                    && __terrane_error_1.kind == TerraneErrorKind::CoercionError
                {
                    __terrane_handled_1 = true;
                    println!("{}", String::from("coercion-caught"));
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
    marked = __terrane_traced(
        mark_config(String::from("Ω")),
        7 /* terrane-site: src/main.trn:26:14-26:30 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(mark_text(&marked),
        8 /* terrane-site: src/main.trn:27:13-27:30 */))
    );
    invalid_rows = terrane_collection_support::List::<
        terrane_collection_support::Tuple<u32>,
    >::new(vec![terrane_collection_support::Tuple::< u32 >::new(vec![1])]);
    let __terrane_completion_2: TerraneCompletion<()> = (|| {
        let __terrane_try_2: TerraneCompletion<()> = (|| {
            __terrane_traced_completion!(
                rows_config(invalid_rows), 9 /* terrane-site: src/main.trn:30:9-30:34 */
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_2 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_2) => {
                let mut __terrane_handled_2 = false;
                if !__terrane_handled_2
                    && __terrane_error_2.kind
                        == TerraneErrorKind::Custom(DescriptorId(0))
                {
                    __terrane_handled_2 = true;
                    println!("{}", String::from("nested-shape-caught"));
                }
                if !__terrane_handled_2 {
                    return TerraneCompletion::Error(__terrane_error_2);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_2 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    valid_rows = terrane_collection_support::List::<
        terrane_collection_support::Tuple<u32>,
    >::new(vec![terrane_collection_support::Tuple::< u32 >::new(vec![1, 1])]);
    good_rows = __terrane_traced(
        rows_config(valid_rows),
        10 /* terrane-site: src/main.trn:34:17-34:40 */,
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(describe_rows(&good_rows),
        11 /* terrane-site: src/main.trn:35:13-35:37 */))
    );
}
