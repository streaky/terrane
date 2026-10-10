// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-scalar-support
// Source: case.trn
// Namespace: authored-generic-enum-states
#[derive(Clone)]
pub enum OperationState<TerraneType54> {
    Pending,
    Running(terrane_int_support::Int),
    Completed(TerraneType54),
    Failed(String),
}
fn report(state: OperationState<String>) -> String {
    let progress: &terrane_int_support::Int;
    let result: &String;
    let error: &String;
    match &state {
        OperationState::Pending => {
            return String::from("pending");
        }
        OperationState::Running(__terrane_pattern_307_progress) => {
            progress = __terrane_pattern_307_progress;
            println!("{}", terrane_scalar_support::scalar_text(&progress.clone()));
            return String::from("running");
        }
        OperationState::Completed(__terrane_pattern_412_result) => {
            result = __terrane_pattern_412_result;
            return result.clone();
        }
        OperationState::Failed(__terrane_pattern_486_error) => {
            error = __terrane_pattern_486_error;
            return error.clone();
        }
    }
}
fn optional_report(state: Option<OperationState<String>>) -> String {
    let result: &String;
    let error: &String;
    match &state {
        Some(OperationState::Pending) => {
            return String::from("pending");
        }
        Some(OperationState::Running(_)) => {
            return String::from("running");
        }
        Some(OperationState::Completed(__terrane_pattern_785_result)) => {
            result = __terrane_pattern_785_result;
            return result.clone();
        }
        Some(OperationState::Failed(__terrane_pattern_859_error)) => {
            error = __terrane_pattern_859_error;
            return error.clone();
        }
        None => {
            return String::from("absent");
        }
    }
}
#[derive(Clone)]
pub enum Decision {
    Accepted(String),
    Rejected(String),
}
fn explain(choice: Decision) -> String {
    let explanation: String;
    let explanation_2: String;
    let __terrane_match_value_1093 = choice.clone();
    match __terrane_match_value_1093 {
        Decision::Accepted(__terrane_pattern_1114_explanation) => {
            explanation = __terrane_pattern_1114_explanation;
            return explanation;
        }
        Decision::Rejected(__terrane_pattern_1190_explanation) => {
            explanation_2 = __terrane_pattern_1190_explanation;
            return explanation_2;
        }
    }
}
fn main() {
    let pending: OperationState<String>;
    let running: OperationState<String>;
    let completed: OperationState<String>;
    let failed: OperationState<String>;
    let accepted: Decision;
    let rejected: Decision;
    pending = OperationState::<String>::Pending;
    running = {
        let __terrane_enum_payload_0 = terrane_int_support::Int::from(7_i128);
        OperationState::<String>::Running(__terrane_enum_payload_0)
    };
    completed = {
        let __terrane_enum_payload_0 = String::from("done");
        OperationState::<String>::Completed(__terrane_enum_payload_0)
    };
    failed = {
        let __terrane_enum_payload_0 = String::from("failed");
        OperationState::<String>::Failed(__terrane_enum_payload_0)
    };
    println!("{}", terrane_scalar_support::scalar_text(&report(pending)));
    println!("{}", terrane_scalar_support::scalar_text(&report(running)));
    println!("{}", terrane_scalar_support::scalar_text(&report(completed)));
    println!("{}", terrane_scalar_support::scalar_text(&report(failed)));
    println!("{}", terrane_scalar_support::scalar_text(&optional_report(None)));
    accepted = {
        let __terrane_enum_payload_0 = String::from("accepted");
        Decision::Accepted(__terrane_enum_payload_0)
    };
    rejected = {
        let __terrane_enum_payload_0 = String::from("rejected");
        Decision::Rejected(__terrane_enum_payload_0)
    };
    println!("{}", terrane_scalar_support::scalar_text(&explain(accepted)));
    println!("{}", terrane_scalar_support::scalar_text(&explain(rejected)));
}
