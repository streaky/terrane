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
    let holder: OptionalValue = OptionalValue::terrane_construct(
        terrane_int_support::Int::from(7_i128),
    );
    let value: Option<terrane_int_support::Int> = holder.value.clone();
    if value.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&value.as_ref()
            .expect("semantic optional narrowing").clone())
        );
    } else {
        println!("{}", terrane_scalar_support::scalar_text(&String::from("none")));
    }
}
