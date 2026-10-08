// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-scalar-support
// Source: case.trn
// Namespace: adaptive-union-field
#[derive(Clone)]
pub struct WorkItem {
    pub estimate: Option<terrane_int_support::Int>,
}
impl WorkItem {
    pub fn terrane_construct(estimate: Option<terrane_int_support::Int>) -> Self {
        let mut __terrane_constructed_value = Self { estimate: None };
        __terrane_constructed_value.construct(estimate);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, estimate: Option<terrane_int_support::Int>) {
        self.estimate = estimate;
    }
    pub fn estimate_units(&self) -> Option<terrane_int_support::Int> {
        return self.estimate.clone();
    }
    pub fn next_estimate(&self) -> Option<terrane_int_support::Int> {
        let current_terrane_f0_s267: Option<terrane_int_support::Int>;
        current_terrane_f0_s267 = self.estimate.clone();
        if current_terrane_f0_s267.is_some() {
            return Some(
                match &current_terrane_f0_s267 {
                    Some(value) => value,
                    _ => unreachable!("flow-proven storage refinement"),
                }
                    .clone() + terrane_int_support::Int::from(1_i128),
            );
        }
        return None;
    }
}
fn main() {
    let item_terrane_f0_s398: WorkItem;
    let estimate_terrane_f0_s431: Option<terrane_int_support::Int>;
    let next_terrane_f0_s524: Option<terrane_int_support::Int>;
    item_terrane_f0_s398 = WorkItem::terrane_construct(
        Some(terrane_int_support::Int::from(4_i128)),
    );
    estimate_terrane_f0_s431 = item_terrane_f0_s398.estimate_units();
    if estimate_terrane_f0_s431.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &estimate_terrane_f0_s431 {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    next_terrane_f0_s524 = item_terrane_f0_s398.next_estimate();
    if next_terrane_f0_s524.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &next_terrane_f0_s524 {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
}
