// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: constructor-if-initialization
#[derive(Clone)]
pub struct RequiredValue {
    pub text: String,
}
impl RequiredValue {
    pub fn terrane_construct(text: String) -> Self {
        let mut __terrane_constructed_value = Self { text: String::new() };
        __terrane_constructed_value.construct(text);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, text: String) {
        self.text = text;
    }
}
#[derive(Clone)]
pub struct Configured {
    pub slot: Option<RequiredValue>,
}
impl Configured {
    pub fn terrane_construct(value: String) -> Self {
        let mut __terrane_constructed_value = Self { slot: None };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: String) {
        if value.as_str() == "a" {
            self.slot = Some(RequiredValue::terrane_construct(String::from("first")));
        } else if value.as_str() == "b" {
            self.slot = Some(RequiredValue::terrane_construct(String::from("second")));
        } else {
            self.slot = Some(RequiredValue::terrane_construct(String::from("other")));
        }
    }
}
#[derive(Clone)]
pub struct ConfiguredWithRead {
    pub slot: Option<RequiredValue>,
}
impl ConfiguredWithRead {
    pub fn terrane_construct(value: String) -> Self {
        let mut __terrane_constructed_value = Self { slot: None };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: String) {
        if value.as_str() == "a" {
            self.slot = Some(RequiredValue::terrane_construct(String::from("first")));
        } else {
            self.slot = Some(RequiredValue::terrane_construct(String::from("other")));
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&self.slot.as_ref()
            .expect("required field initialized").text)
        );
    }
}
fn main() {
    let first: Configured;
    let second: Configured;
    let third: Configured;
    let read_first: ConfiguredWithRead;
    let read_other: ConfiguredWithRead;
    first = Configured::terrane_construct(String::from("a"));
    second = Configured::terrane_construct(String::from("b"));
    third = Configured::terrane_construct(String::from("c"));
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&first.slot.as_ref()
        .expect("required field initialized").text), String::from("|"),
        terrane_scalar_support::scalar_text(&second.slot.as_ref()
        .expect("required field initialized").text), String::from("|"),
        terrane_scalar_support::scalar_text(&third.slot.as_ref()
        .expect("required field initialized").text)
    );
    read_first = ConfiguredWithRead::terrane_construct(String::from("a"));
    read_other = ConfiguredWithRead::terrane_construct(String::from("b"));
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&read_first.slot.as_ref()
        .expect("required field initialized").text), String::from("|"),
        terrane_scalar_support::scalar_text(&read_other.slot.as_ref()
        .expect("required field initialized").text)
    );
}
