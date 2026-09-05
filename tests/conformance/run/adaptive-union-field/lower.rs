// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: adaptive-union-field
#[derive(Clone)]
pub struct WorkItem {
    pub estimate: Option<terrane_int_support::Int>,
}
impl WorkItem {
    pub fn terrane_construct(estimate: Option<terrane_int_support::Int>) -> Self {
        let mut value = Self { estimate: None };
        value.construct(estimate);
        value
    }
    pub fn construct(&mut self, estimate: Option<terrane_int_support::Int>) {
        self.estimate = estimate;
    }
    pub fn estimate_units(&self) -> Option<terrane_int_support::Int> {
        return self.estimate.clone();
    }
    pub fn next_estimate(&self) -> Option<terrane_int_support::Int> {
        let current: Option<terrane_int_support::Int> = self.estimate.clone();
        if current.is_some() {
            return Some(
                (*current.as_ref().expect("semantic optional narrowing")).clone()
                    + terrane_int_support::Int::from(1_i128),
            );
        }
        return None;
    }
}
fn main() {
    let item: WorkItem = WorkItem::terrane_construct(
        Some(terrane_int_support::Int::from(4_i128)),
    );
    let estimate: Option<terrane_int_support::Int> = item.estimate_units();
    if estimate.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* estimate.as_ref()
            .expect("semantic optional narrowing"))
        );
    }
    let next: Option<terrane_int_support::Int> = item.next_estimate();
    if next.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* next.as_ref()
            .expect("semantic optional narrowing"))
        );
    }
}
