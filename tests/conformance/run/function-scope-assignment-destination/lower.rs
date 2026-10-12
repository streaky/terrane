// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: function-scope-assignment-destination
fn optional_result(choose: bool) -> Option<String> {
    let mut result: Option<String>;
    let mut count: terrane_int_support::Int;
    result = None;
    count = terrane_int_support::Int::from(0_i128);
    while count.clone() < terrane_int_support::Int::from(2_i128) {
        if choose {
            result = Some(String::from("found"));
        }
        count = count.clone() + terrane_int_support::Int::from(1_i128);
    }
    return result;
}
fn fixed_width_result(choose: bool) -> u8 {
    let mut result: u8;
    result = 1;
    if choose {
        result = 2;
    }
    return result;
}
fn create_value(fail: bool) -> Result<String, TerraneError> {
    if fail {
        return Err(
            TerraneError::raised(
                TerraneErrorKind::CoercionError,
                0 /* terrane-site: case.trn:18:9-18:39 */,
            ),
        );
    }
    return Ok(String::from("created"));
}
fn fallible_initialization(fail: bool) {
    let mut value: Option<String> = None;
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let _ = value
                .insert(
                    __terrane_traced_completion!(
                        create_value(fail), 1 /* terrane-site: case.trn:22:17-22:35 */
                    ),
                );
            println!(
                "{}", terrane_scalar_support::scalar_text(&value.as_ref()
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
                    && __terrane_error_0.kind == TerraneErrorKind::CoercionError
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("failed"));
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
    return ();
}
fn main() {
    let absent: Option<String>;
    let present: Option<String>;
    absent = optional_result(false);
    if absent.is_none() {
        println!("{}", String::from("absent"));
    }
    present = optional_result(true);
    if present.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &present { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    println!("{}", terrane_scalar_support::scalar_text(&fixed_width_result(true)));
    fallible_initialization(false);
    fallible_initialization(true);
}
