// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: conformance/loop-body-backedge-while
fn main() {
    let mut index: terrane_int_support::Int;
    let mut item: Option<String> = None;
    index = terrane_int_support::Int::from(0_i128);
    while index.clone() < terrane_int_support::Int::from(3_i128) {
        if index.clone() > terrane_int_support::Int::from(0_i128) {
            println!(
                "{}", terrane_scalar_support::scalar_text(&item.as_ref().unwrap_or_else(|
                | __terrane_uninitialized_binding("item", "case.trn", 8, 20)))
            );
        }
        if index.clone() == terrane_int_support::Int::from(0_i128) {
            let _ = item.insert(String::from("a"));
        } else {
            let _ = item.insert(String::from("b"));
        }
        index = index.clone() + terrane_int_support::Int::from(1_i128);
        continue;
    }
}
