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
    let mut first_terrane_f0_s189: Counter;
    let mut second_terrane_f0_s219: Counter;
    let first_value_terrane_f0_s250: terrane_int_support::Int;
    let second_value_terrane_f0_s288: terrane_int_support::Int;
    let next_first_terrane_f0_s328: terrane_int_support::Int;
    first_terrane_f0_s189 = Counter::terrane_construct();
    second_terrane_f0_s219 = Counter::terrane_construct();
    first_value_terrane_f0_s250 = first_terrane_f0_s189.increase();
    second_value_terrane_f0_s288 = second_terrane_f0_s219.increase();
    next_first_terrane_f0_s328 = first_terrane_f0_s189.increase();
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&first_value_terrane_f0_s250),
        terrane_scalar_support::scalar_text(&second_value_terrane_f0_s288),
        terrane_scalar_support::scalar_text(&next_first_terrane_f0_s328)
    );
}
