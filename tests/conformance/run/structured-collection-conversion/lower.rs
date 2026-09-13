// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: structured-collection-conversion
fn build(wide: i16) -> Result<terrane_collection_support::List<i8>, TerraneError> {
    return Ok(
        terrane_collection_support::List::<
            i8,
        >::new(
            vec![
                { let source_value = wide;
                __terrane_raised_err(i8::try_from(source_value).map_err(| _ |
                terrane_int_support::ArithmeticError::conversion_overflow(&source_value,
                "int16", "int8", "the value is outside the destination range")),
                0 /* terrane-site: case.trn:5:16-5:20 */) ? }
            ],
        ),
    );
}
fn append_value(
    wide: i16,
) -> Result<terrane_collection_support::List<i8>, TerraneError> {
    let mut values: terrane_collection_support::List<i8> = terrane_collection_support::List::<
        i8,
    >::new(vec![1]);
    return Ok({
        let collection = &mut values;
        collection
            .append({
                let source_value = wide;
                __terrane_raised_err(
                    i8::try_from(source_value)
                        .map_err(|_| terrane_int_support::ArithmeticError::conversion_overflow(
                            &source_value,
                            "int16",
                            "int8",
                            "the value is outside the destination range",
                        )),
                    1 /* terrane-site: case.trn:8:25-8:29 */,
                )?
            });
        collection.clone()
    });
}
fn main() {
    let wide: i16 = 300;
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_raised_completion!(__terrane_traced_completion!(build(wide),
                2 /* terrane-site: case.trn:12:13-12:24 */)
                .get_or_error(__terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
                3 /* terrane-site: case.trn:12:12-12:28 */)), 3 /* terrane-site: case.trn:12:12-12:28 */))
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
                        == TerraneErrorKind::IntegerConversionOverflow
                {
                    __terrane_handled_0 = true;
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("constructor conversion caught"))
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
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_raised_completion!(__terrane_traced_completion!(append_value(wide),
                4 /* terrane-site: case.trn:16:13-16:31 */)
                .get_or_error(__terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
                5 /* terrane-site: case.trn:16:12-16:35 */)), 5 /* terrane-site: case.trn:16:12-16:35 */))
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
                    && __terrane_error_1.kind
                        == TerraneErrorKind::IntegerConversionOverflow
                {
                    __terrane_handled_1 = true;
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("member conversion caught"))
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
    println!("{}", terrane_scalar_support::scalar_text(&String::from("after")));
}
