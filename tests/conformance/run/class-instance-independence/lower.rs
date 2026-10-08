// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-scalar-support
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
    let mut first: Counter;
    let mut second: Counter;
    let first_value: terrane_int_support::Int;
    let second_value: terrane_int_support::Int;
    let next_first: terrane_int_support::Int;
    first = Counter::terrane_construct();
    second = Counter::terrane_construct();
    first_value = first.increase();
    second_value = second.increase();
    next_first = first.increase();
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&first_value),
        terrane_scalar_support::scalar_text(&second_value),
        terrane_scalar_support::scalar_text(&next_first)
    );
}
