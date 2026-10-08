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
    let progress_terrane_f0_s369: &terrane_int_support::Int;
    let result_terrane_f0_s476: &String;
    let error_terrane_f0_s547: &String;
    match &state {
        OperationState::Pending => {
            return String::from("pending");
        }
        OperationState::Running(progress) => {
            progress_terrane_f0_s369 = progress;
            println!(
                "{}", terrane_scalar_support::scalar_text(&progress_terrane_f0_s369
                .clone())
            );
            return String::from("running");
        }
        OperationState::Completed(result) => {
            result_terrane_f0_s476 = result;
            return result_terrane_f0_s476.clone();
        }
        OperationState::Failed(error) => {
            error_terrane_f0_s547 = error;
            return error_terrane_f0_s547.clone();
        }
    }
}
fn optional_report(state: Option<OperationState<String>>) -> String {
    let result_terrane_f0_s849: &String;
    let error_terrane_f0_s920: &String;
    match &state {
        Some(OperationState::Pending) => {
            return String::from("pending");
        }
        Some(OperationState::Running(_)) => {
            return String::from("running");
        }
        Some(OperationState::Completed(result)) => {
            result_terrane_f0_s849 = result;
            return result_terrane_f0_s849.clone();
        }
        Some(OperationState::Failed(error)) => {
            error_terrane_f0_s920 = error;
            return error_terrane_f0_s920.clone();
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
    let explanation_terrane_f0_s1170: String;
    let explanation_terrane_f0_s1246: String;
    let __terrane_match_value_1124 = choice.clone();
    match __terrane_match_value_1124 {
        Decision::Accepted(explanation) => {
            explanation_terrane_f0_s1170 = explanation;
            return explanation_terrane_f0_s1170;
        }
        Decision::Rejected(explanation) => {
            explanation_terrane_f0_s1246 = explanation;
            return explanation_terrane_f0_s1246;
        }
    }
}
fn main() {
    let pending_terrane_f0_s1309: OperationState<String>;
    let running_terrane_f0_s1384: OperationState<String>;
    let completed_terrane_f0_s1472: OperationState<String>;
    let failed_terrane_f0_s1541: OperationState<String>;
    let accepted_terrane_f0_s1783: Decision;
    let rejected_terrane_f0_s1852: Decision;
    pending_terrane_f0_s1309 = OperationState::<String>::Pending;
    running_terrane_f0_s1384 = {
        let __terrane_enum_payload_0 = terrane_int_support::Int::from(7_i128);
        OperationState::<String>::Running(__terrane_enum_payload_0)
    };
    completed_terrane_f0_s1472 = {
        let __terrane_enum_payload_0 = String::from("done");
        OperationState::<String>::Completed(__terrane_enum_payload_0)
    };
    failed_terrane_f0_s1541 = {
        let __terrane_enum_payload_0 = String::from("failed");
        OperationState::<String>::Failed(__terrane_enum_payload_0)
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&report(pending_terrane_f0_s1309))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&report(running_terrane_f0_s1384))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&report(completed_terrane_f0_s1472))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&report(failed_terrane_f0_s1541))
    );
    println!("{}", terrane_scalar_support::scalar_text(&optional_report(None)));
    accepted_terrane_f0_s1783 = {
        let __terrane_enum_payload_0 = String::from("accepted");
        Decision::Accepted(__terrane_enum_payload_0)
    };
    rejected_terrane_f0_s1852 = {
        let __terrane_enum_payload_0 = String::from("rejected");
        Decision::Rejected(__terrane_enum_payload_0)
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&explain(accepted_terrane_f0_s1783))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&explain(rejected_terrane_f0_s1852))
    );
}
