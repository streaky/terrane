// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: local-function-results
fn first() -> terrane_int_support::Int {
    fn helper_terrane_local_function_results() -> terrane_int_support::Int {
        return terrane_int_support::Int::from(1_i128);
    }
    let value: terrane_int_support::Int;
    value = helper_terrane_local_function_results();
    return value.clone();
}
fn second() -> String {
    fn helper_terrane_local_function_results() -> String {
        return String::from("second");
    }
    let value: String;
    value = helper_terrane_local_function_results();
    return value;
}
fn guarded() -> terrane_int_support::Int {
    fn helper_terrane_local_function_results() -> terrane_int_support::Int {
        return terrane_int_support::Int::from(3_i128);
    }
    let mut value: Option<terrane_int_support::Int> = None;
    let mut __terrane_completion_0: TerraneCompletion<terrane_int_support::Int> = (|| {
        let __terrane_try_0: TerraneCompletion<terrane_int_support::Int> = (|| {
            let _ = value.insert(helper_terrane_local_function_results());
            return TerraneCompletion::Return(
                value.as_ref().expect("flow-proven available binding").clone().clone(),
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
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_0: TerraneCompletion<terrane_int_support::Int> = (|| {
        println!("{}", terrane_scalar_support::scalar_text(&String::from("F")));
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
fn main() {
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&first()),
        terrane_scalar_support::scalar_text(&second()),
        terrane_scalar_support::scalar_text(&guarded())
    );
}
