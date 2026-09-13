// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: finally-control-flow
fn early() -> terrane_int_support::Int {
    let mut __terrane_completion_0: TerraneCompletion<terrane_int_support::Int> = (|| {
        let __terrane_try_0: TerraneCompletion<terrane_int_support::Int> = (|| {
            return TerraneCompletion::Return(terrane_int_support::Int::from(7_i128));
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_0: TerraneCompletion<terrane_int_support::Int> = (|| {
        println!(
            "{}", terrane_scalar_support::scalar_text(&String::from("return finally"))
        );
        TerraneCompletion::Normal
    })();
    match __terrane_finally_0 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_0 = replacement,
    }
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {
            __terrane_generated_defect("non-fallthrough try completed normally")
        }
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
fn caught() -> terrane_int_support::Int {
    let mut __terrane_completion_1: TerraneCompletion<terrane_int_support::Int> = (|| {
        let __terrane_try_1: TerraneCompletion<terrane_int_support::Int> = (|| {
            return TerraneCompletion::Error(
                TerraneError::raised(
                    TerraneErrorKind::ArithmeticOverflow,
                    0 /* terrane-site: case.trn:10:5-10:30 */,
                ),
            );
        })();
        match __terrane_try_1 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_1) => {
                let mut __terrane_handled_1 = false;
                if !__terrane_handled_1
                    && __terrane_error_1.kind == TerraneErrorKind::ArithmeticOverflow
                {
                    __terrane_handled_1 = true;
                    return TerraneCompletion::Return(
                        terrane_int_support::Int::from(9_i128),
                    );
                }
                if !__terrane_handled_1 {
                    return TerraneCompletion::Error(__terrane_error_1);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_1: TerraneCompletion<terrane_int_support::Int> = (|| {
        println!(
            "{}", terrane_scalar_support::scalar_text(&String::from("catch finally"))
        );
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
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
fn main() {
    let value: terrane_int_support::Int = early();
    println!("{}", terrane_scalar_support::scalar_text(&value));
    let caught_value: terrane_int_support::Int = caught();
    println!("{}", terrane_scalar_support::scalar_text(&caught_value));
    let mut counter: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while counter.clone() < terrane_int_support::Int::from(3_i128) {
        counter = counter.clone() + terrane_int_support::Int::from(1_i128);
        let mut __terrane_completion_2: TerraneCompletion<()> = (|| {
            let __terrane_try_2: TerraneCompletion<()> = (|| {
                if counter.clone() == terrane_int_support::Int::from(1_i128) {
                    return TerraneCompletion::Continue;
                }
                if counter.clone() == terrane_int_support::Int::from(2_i128) {
                    return TerraneCompletion::Break;
                }
                TerraneCompletion::Normal
            })();
            match __terrane_try_2 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_2) => {
                    let mut __terrane_handled_2 = false;
                    if !__terrane_handled_2 {
                        return TerraneCompletion::Error(__terrane_error_2);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        let __terrane_finally_2: TerraneCompletion<()> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&String::from("loop finally"))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_finally_2 {
            TerraneCompletion::Normal => {}
            replacement => __terrane_completion_2 = replacement,
        }
        match __terrane_completion_2 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break => break,
            TerraneCompletion::Continue => continue,
        }
    }
    println!("{}", terrane_scalar_support::scalar_text(&String::from("done")));
}
