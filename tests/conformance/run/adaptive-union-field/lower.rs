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
        let current: Option<terrane_int_support::Int>;
        current = self.estimate.clone();
        if current.is_some() {
            return Some(
                match &current {
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
    let item: WorkItem;
    let estimate: Option<terrane_int_support::Int>;
    let next: Option<terrane_int_support::Int>;
    item = WorkItem::terrane_construct(Some(terrane_int_support::Int::from(4_i128)));
    estimate = item.estimate_units();
    if estimate.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &estimate { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    next = item.next_estimate();
    if next.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &next { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
}
