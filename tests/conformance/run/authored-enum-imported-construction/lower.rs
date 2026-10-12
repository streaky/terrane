// Generated deterministically by Terrane <version>.
// Source: model/state.trn
// Namespace: model
#[derive(Clone)]
pub enum State<TerraneType54> {
    Pending,
    Completed(TerraneType54),
}
#[derive(Clone)]
pub enum UnitState {
    Accepted,
    Rejected,
}
// Source: src/main.trn
// Namespace: app
fn report(value: State<String>) -> String {
    let result: &String;
    match &value {
        State::Pending => {
            return String::from("pending");
        }
        State::Completed(__terrane_pattern_237_result) => {
            result = __terrane_pattern_237_result;
            return result.clone();
        }
    }
}
fn main() {
    let waiting: State<String>;
    let completed: State<String>;
    let inferred: State<String>;
    let bare: UnitState;
    waiting = State::<String>::Pending;
    completed = {
        let __terrane_enum_payload_0 = String::from("done");
        State::<String>::Completed(__terrane_enum_payload_0)
    };
    inferred = {
        let __terrane_enum_payload_0 = String::from("inferred");
        State::<String>::Completed(__terrane_enum_payload_0)
    };
    bare = UnitState::Rejected;
    println!("{}", terrane_scalar_support::scalar_text(&report(waiting)));
    println!("{}", terrane_scalar_support::scalar_text(&report(completed)));
    println!("{}", terrane_scalar_support::scalar_text(&report(inferred)));
    let __terrane_match_value_696 = bare.clone();
    match __terrane_match_value_696 {
        UnitState::Rejected => {
            println!("{}", String::from("rejected"));
        }
        _ => {
            println!("{}", String::from("other"));
        }
    }
}
