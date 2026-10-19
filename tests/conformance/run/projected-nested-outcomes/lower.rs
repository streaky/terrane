// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn inspect_next(mode: i64) {
    let mut connection: Channel;
    let mut value: Option<Option<String>> = None;
    let mut failure: Option<TerraneError> = None;
    connection = __terrane_raised(
        channel(terrane_int_support::Int::from(mode as i128)),
        0 /* terrane-site: src/main.trn:6:16-6:29 */,
    );
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let _ = value
                .insert(
                    __terrane_raised_completion!(
                        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                        connection.next())) { Ok(None) => Ok(None), Ok(Some(Ok(value)))
                        => Ok(Some(value)), Ok(Some(Err(error))) => Err(crate
                        ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
                        ::DescriptorId(2), error.to_string(), crate ::TERRANE_NO_SITE))),
                        Err(payload) => Err(crate ::__terrane_dependency_panic(payload,
                        "terrane_nested_outcome_witness",
                        "terrane_nested_outcome_witness::Channel::next")) },
                        1 /* terrane-site: src/main.trn:8:25-8:41 */
                    ),
                );
            if value.as_ref().expect("flow-proven available binding").is_some() {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&match value.as_ref()
                    .expect("flow-proven available binding") { Some(value) => value, _ =>
                    unreachable!("flow-proven storage refinement") })
                );
            } else {
                println!("{}", String::from("end"));
            }
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
fn inspect_outer(mode: i64) {
    let connection: Channel;
    let mut value: Option<Option<String>> = None;
    let mut failure: Option<TerraneError> = None;
    connection = __terrane_raised(
        channel(terrane_int_support::Int::from(mode as i128)),
        2 /* terrane-site: src/main.trn:17:16-17:29 */,
    );
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            let _ = value
                .insert(
                    __terrane_raised_completion!(
                        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                        (&connection).outer())) { Ok(Ok(value)) => Ok(value),
                        Ok(Err(error)) => Err(crate ::TerraneForeignError(crate
                        ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                        .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                        Err(crate ::__terrane_dependency_panic(payload,
                        "terrane_nested_outcome_witness",
                        "terrane_nested_outcome_witness::Channel::outer")) },
                        3 /* terrane-site: src/main.trn:19:25-19:42 */
                    ),
                );
            if value.as_ref().expect("flow-proven available binding").is_some() {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&match value.as_ref()
                    .expect("flow-proven available binding") { Some(value) => value, _ =>
                    unreachable!("flow-proven storage refinement") })
                );
            } else {
                println!("{}", String::from("absent"));
            }
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
                        == TerraneErrorKind::Custom(DescriptorId(2))
                {
                    __terrane_handled_1 = true;
                    let _ = failure.insert(__terrane_error_1.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&failure.as_ref()
                        .expect("flow-proven available binding").clone().message()
                        .to_owned())
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
}
fn main() {
    inspect_next(0);
    inspect_next(1);
    inspect_next(2);
    inspect_outer(0);
    inspect_outer(1);
    inspect_outer(3);
}
