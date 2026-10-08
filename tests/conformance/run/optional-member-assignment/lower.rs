// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-scalar-support
// Source: case.trn
// Namespace: app
#[derive(Clone)]
pub struct OptionalValue {
    pub value: Option<terrane_int_support::Int>,
}
impl OptionalValue {
    pub fn terrane_construct(input: terrane_int_support::Int) -> Self {
        let mut __terrane_constructed_value = Self { value: None };
        __terrane_constructed_value.construct(input);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, input: terrane_int_support::Int) {
        self.value = Some(input.clone());
    }
}
fn main() {
    let holder_terrane_f0_s140: OptionalValue;
    let value_terrane_f0_s179: Option<terrane_int_support::Int>;
    holder_terrane_f0_s140 = OptionalValue::terrane_construct(
        terrane_int_support::Int::from(7_i128),
    );
    value_terrane_f0_s179 = holder_terrane_f0_s140.value.clone();
    if value_terrane_f0_s179.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &value_terrane_f0_s179 {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") })
        );
    } else {
        println!("{}", terrane_scalar_support::scalar_text(&String::from("none")));
    }
}
