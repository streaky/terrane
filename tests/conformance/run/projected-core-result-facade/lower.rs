// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let good: terrane_int_support::Int;
    let mut error: Option<TerraneError> = None;
    let present: Option<terrane_int_support::Int>;
    let absent: Option<terrane_int_support::Int>;
    let chosen: facade::Either<i64, String>;
    let nominal: facade::Either<i64, String>;
    let letter: Option<String>;
    let invalid: Option<String>;
    good = __terrane_raised(
        parse("41"),
        0 /* terrane-site: src/main.trn:8:16-8:27 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&good));
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_raised_completion!(parse("bad"),
                1 /* terrane-site: src/main.trn:11:17-11:29 */))
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
                if !__terrane_handled_0 {
                    __terrane_handled_0 = true;
                    let _ = error.insert(__terrane_error_0.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&{ let mut
                        __terrane_text = String::with_capacity(8); __terrane_text
                        .push_str("caught: "); { let __terrane_error_text_0 = error
                        .as_ref().expect("flow-proven available binding").message();
                        __terrane_text.push_str(__terrane_error_text_0); } __terrane_text
                        })
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
    present = {
        let __terrane_conversion_input_0 = true;
        __terrane_raised(
            maybe(__terrane_conversion_input_0),
            2 /* terrane-site: src/main.trn:14:24-14:35 */,
        )
    };
    println!("{}", terrane_scalar_support::scalar_text(&(&present).is_some()));
    absent = {
        let __terrane_conversion_input_0 = false;
        __terrane_raised(
            maybe(__terrane_conversion_input_0),
            3 /* terrane-site: src/main.trn:16:23-16:35 */,
        )
    };
    println!("{}", terrane_scalar_support::scalar_text(&(&absent).is_none()));
    chosen = {
        let __terrane_conversion_input_0 = true;
        __terrane_raised(
            pick(__terrane_conversion_input_0),
            4 /* terrane-site: src/main.trn:18:14-18:24 */,
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(describe(&chosen),
        5 /* terrane-site: src/main.trn:19:13-19:29 */))
    );
    nominal = {
        let __terrane_conversion_input_0 = false;
        __terrane_raised(
            pick(__terrane_conversion_input_0),
            6 /* terrane-site: src/main.trn:20:46-20:57 */,
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(describe(&nominal),
        7 /* terrane-site: src/main.trn:21:13-21:30 */))
    );
    letter = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(65_i128);
        __terrane_raised(
            character(__terrane_conversion_input_0),
            8 /* terrane-site: src/main.trn:22:26-22:39 */,
        )
    };
    println!(
        "{}", { let mut __terrane_text = String::new(); match &letter {
        Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    invalid = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(1114112_i128);
        __terrane_raised(
            character(__terrane_conversion_input_0),
            9 /* terrane-site: src/main.trn:24:27-24:45 */,
        )
    };
    println!("{}", terrane_scalar_support::scalar_text(&(&invalid).is_none()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        terrane_collection_support::List:: < terrane_int_support::Int >
        ::new(vec![terrane_int_support::Int::from(117_i128),
        terrane_int_support::Int::from(116_i128),
        terrane_int_support::Int::from(102_i128),
        terrane_int_support::Int::from(56_i128)]);
        __terrane_raised(text(__terrane_conversion_input_0), 10 /* terrane-site: src/main.trn:26:13-26:44 */) })
    );
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&{ let
                __terrane_conversion_input_0 = terrane_collection_support::List:: <
                terrane_int_support::Int >
                ::new(vec![terrane_int_support::Int::from(255_i128)]);
                __terrane_raised_completion!(text(__terrane_conversion_input_0),
                11 /* terrane-site: src/main.trn:28:17-28:34 */) })
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
                if !__terrane_handled_1 {
                    __terrane_handled_1 = true;
                    println!("{}", String::from("invalid utf8 caught"));
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
}
