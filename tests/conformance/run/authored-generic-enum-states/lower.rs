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
    match &state {
        OperationState::Pending => {
            return String::from("pending");
        }
        OperationState::Running(progress) => {
            println!("{}", terrane_scalar_support::scalar_text(&progress.clone()));
            return String::from("running");
        }
        OperationState::Completed(result) => {
            return result.clone();
        }
        OperationState::Failed(error) => {
            return error.clone();
        }
    }
}
fn optional_report(state: Option<OperationState<String>>) -> String {
    match &state {
        Some(OperationState::Pending) => {
            return String::from("pending");
        }
        Some(OperationState::Running(_)) => {
            return String::from("running");
        }
        Some(OperationState::Completed(result)) => {
            return result.clone();
        }
        Some(OperationState::Failed(error)) => {
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
    let __terrane_match_value_1124 = choice.clone();
    match __terrane_match_value_1124 {
        Decision::Accepted(explanation) => {
            return explanation;
        }
        Decision::Rejected(explanation) => {
            return explanation;
        }
    }
}
fn main() {
    let pending: OperationState<String> = OperationState::<String>::Pending;
    let running: OperationState<String> = {
        let __terrane_enum_payload_0 = terrane_int_support::Int::from(7_i128);
        OperationState::<String>::Running(__terrane_enum_payload_0)
    };
    let completed: OperationState<String> = {
        let __terrane_enum_payload_0 = String::from("done");
        OperationState::<String>::Completed(__terrane_enum_payload_0)
    };
    let failed: OperationState<String> = {
        let __terrane_enum_payload_0 = String::from("failed");
        OperationState::<String>::Failed(__terrane_enum_payload_0)
    };
    println!("{}", terrane_scalar_support::scalar_text(&report(pending)));
    println!("{}", terrane_scalar_support::scalar_text(&report(running)));
    println!("{}", terrane_scalar_support::scalar_text(&report(completed)));
    println!("{}", terrane_scalar_support::scalar_text(&report(failed)));
    println!("{}", terrane_scalar_support::scalar_text(&optional_report(None)));
    let accepted: Decision = {
        let __terrane_enum_payload_0 = String::from("accepted");
        Decision::Accepted(__terrane_enum_payload_0)
    };
    let rejected: Decision = {
        let __terrane_enum_payload_0 = String::from("rejected");
        Decision::Rejected(__terrane_enum_payload_0)
    };
    println!("{}", terrane_scalar_support::scalar_text(&explain(accepted)));
    println!("{}", terrane_scalar_support::scalar_text(&explain(rejected)));
}
