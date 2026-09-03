// Generated deterministically by Terrane <version>.
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct TerraneDescriptor {
    identity: &'static str,
    name: &'static str,
    kind: &'static str,
}
// Source: case.trn
// Namespace: class-instance-independence
#[derive(Clone)]
pub struct Counter {
    pub value: terrane_int_support::Int,
}
impl Counter {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(0_i128),
        }
    }
    pub fn increase(&mut self) -> terrane_int_support::Int {
        self.value = self.value.clone() + terrane_int_support::Int::from(1_i128);
        return self.value.clone();
    }
}
fn main() {
    let mut first: Counter = Counter::terrane_construct();
    let mut second: Counter = Counter::terrane_construct();
    let first_value: terrane_int_support::Int = first.increase();
    let second_value: terrane_int_support::Int = second.increase();
    let next_first: terrane_int_support::Int = first.increase();
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&first_value),
        terrane_scalar_support::scalar_text(&second_value),
        terrane_scalar_support::scalar_text(&next_first)
    );
}
