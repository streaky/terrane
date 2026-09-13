// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: structured-legacy-failures
fn narrow_fixed() -> Result<i8, TerraneError> {
    let wide: i16 = 300;
    return Ok({
        let source_value = wide;
        __terrane_raised_err(
            i8::try_from(source_value)
                .map_err(|_| terrane_int_support::ArithmeticError::conversion_overflow(
                    &source_value,
                    "int16",
                    "int8",
                    "the value is outside the destination range",
                )),
            0 /* terrane-site: case.trn:5:10-5:14 */,
        )?
    });
}
fn narrow_float() -> Result<f32, TerraneError> {
    let wide_float: f64 = 340282400000000000000000000000000000000.0;
    return Ok({
        let source_value = wide_float;
        let converted = source_value as f32;
        if converted as f64 == source_value {
            converted
        } else {
            __terrane_raised_err(
                Err(
                    terrane_int_support::ArithmeticError::conversion_overflow(
                        &source_value,
                        "float64",
                        "float32",
                        "the floating value is not exactly representable",
                    ),
                ),
                1 /* terrane-site: case.trn:8:10-8:20 */,
            )?
        }
    });
}
fn divide() -> Result<terrane_int_support::Int, TerraneError> {
    let numerator: i64 = 1;
    let denominator: i64 = 0;
    return Ok(
        __terrane_raised_err(
            terrane_int_support::Int::from(numerator as i128)
                .euclidean_div(&terrane_int_support::Int::from(denominator as i128)),
            2 /* terrane-site: case.trn:12:10-12:33 */,
        )?,
    );
}
fn remainder() -> Result<terrane_int_support::Int, TerraneError> {
    let numerator: i64 = 1;
    let denominator: i64 = 0;
    return Ok(
        __terrane_raised_err(
            terrane_int_support::Int::from(numerator as i128)
                .modulo(&terrane_int_support::Int::from(denominator as i128)),
            3 /* terrane-site: case.trn:16:10-16:33 */,
        )?,
    );
}
fn round_value() -> Result<terrane_int_support::Int, TerraneError> {
    let one: f64 = 1.0;
    let zero: f64 = 0.0;
    let infinite: f64 = one / zero;
    return Ok(
        __terrane_raised_err(
            terrane_int_support::rounded_f64(
                infinite,
                terrane_int_support::FloatRounding::TiesEven,
            ),
            4 /* terrane-site: case.trn:21:10-21:25 */,
        )?,
    );
}
fn accepts_narrow(value: i8) -> terrane_int_support::Int {
    return terrane_int_support::Int::from(value as i128);
}
fn narrow_argument() -> Result<terrane_int_support::Int, TerraneError> {
    let wide: i16 = 300;
    return Ok(
        accepts_narrow({
            let source_value = wide;
            __terrane_raised_err(
                i8::try_from(source_value)
                    .map_err(|_| terrane_int_support::ArithmeticError::conversion_overflow(
                        &source_value,
                        "int16",
                        "int8",
                        "the value is outside the destination range",
                    )),
                5 /* terrane-site: case.trn:26:26-26:30 */,
            )?
        }),
    );
}
fn main() {
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_traced_completion!(narrow_fixed(),
                6 /* terrane-site: case.trn:29:13-29:26 */))
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
                        terrane_scalar_support::scalar_text(&String::from("fixed conversion caught"))
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
                terrane_scalar_support::scalar_text(&__terrane_traced_completion!(narrow_float(),
                7 /* terrane-site: case.trn:33:13-33:26 */))
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
                        terrane_scalar_support::scalar_text(&String::from("float narrowing caught"))
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
    let __terrane_completion_2: TerraneCompletion<()> = (|| {
        let __terrane_try_2: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_traced_completion!(divide(),
                8 /* terrane-site: case.trn:37:13-37:20 */))
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
                    && __terrane_error_2.kind == TerraneErrorKind::DivisionByZero
                {
                    __terrane_handled_2 = true;
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("int division caught"))
                    );
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
    let __terrane_completion_3: TerraneCompletion<()> = (|| {
        let __terrane_try_3: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_traced_completion!(remainder(),
                9 /* terrane-site: case.trn:41:13-41:23 */))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_3 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_3) => {
                let mut __terrane_handled_3 = false;
                if !__terrane_handled_3
                    && __terrane_error_3.kind == TerraneErrorKind::DivisionByZero
                {
                    __terrane_handled_3 = true;
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("int remainder caught"))
                    );
                }
                if !__terrane_handled_3 {
                    return TerraneCompletion::Error(__terrane_error_3);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_3 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    let __terrane_completion_4: TerraneCompletion<()> = (|| {
        let __terrane_try_4: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_traced_completion!(round_value(),
                10 /* terrane-site: case.trn:45:13-45:25 */))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_4 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_4) => {
                let mut __terrane_handled_4 = false;
                if !__terrane_handled_4
                    && __terrane_error_4.kind
                        == TerraneErrorKind::IntegerConversionOverflow
                {
                    __terrane_handled_4 = true;
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("rounding caught"))
                    );
                }
                if !__terrane_handled_4 {
                    return TerraneCompletion::Error(__terrane_error_4);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_4 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    let __terrane_completion_5: TerraneCompletion<()> = (|| {
        let __terrane_try_5: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_traced_completion!(narrow_argument(),
                11 /* terrane-site: case.trn:49:13-49:29 */))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_5 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_5) => {
                let mut __terrane_handled_5 = false;
                if !__terrane_handled_5
                    && __terrane_error_5.kind
                        == TerraneErrorKind::IntegerConversionOverflow
                {
                    __terrane_handled_5 = true;
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("argument conversion caught"))
                    );
                }
                if !__terrane_handled_5 {
                    return TerraneCompletion::Error(__terrane_error_5);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_5 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    println!("{}", terrane_scalar_support::scalar_text(&String::from("after")));
}
