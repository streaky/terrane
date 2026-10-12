// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: switch-boundaries
#[derive(Clone)]
pub struct Marker {
    __terrane_lifetime: std::sync::Arc<()>,
    pub name: String,
}
impl Marker {
    pub fn terrane_construct(name: String) -> Self {
        let mut __terrane_constructed_value = Self {
            name: String::new(),
            __terrane_lifetime: std::sync::Arc::new(()),
        };
        __terrane_constructed_value.construct(name);
        __terrane_constructed_value
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn construct(&mut self, name: String) {
        self.name = name;
    }
    pub fn to_string(&self) -> String {
        return self.name.clone();
    }
    pub fn destruct(&mut self) {
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
            String::with_capacity(5); __terrane_text.push_str("drop:");
            terrane_scalar_support::ScalarDisplay::write_scalar(&self.name, &mut
            __terrane_text); __terrane_text })
        );
    }
}
impl Drop for Marker {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) != 1 {
            return;
        }
        self.destruct();
    }
}
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
        let __terrane_switch_value_429 = &value;
        if __terrane_switch_value_429.as_str() == "a" {
            self.slot = Some(RequiredValue::terrane_construct(String::from("first")));
        } else {
            self.slot = Some(RequiredValue::terrane_construct(String::from("other")));
        }
    }
}
fn consume(item: Marker) -> bool {
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(9); __terrane_text.push_str("consumed:"); __terrane_text
        .push_str(&item.to_string()); __terrane_text })
    );
    return false;
}
fn absent() -> Option<String> {
    println!("{}", String::from("none-expression"));
    return None;
}
fn resource_case(ready: bool) {
    let item: Marker;
    item = Marker::terrane_construct(String::from("retained"));
    if ready {
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
            String::with_capacity(9); __terrane_text.push_str("resource:");
            __terrane_text.push_str(&item.to_string()); __terrane_text })
        );
        println!("{}", item.to_string());
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
            String::with_capacity(0); __terrane_text.push_str(""); __terrane_text
            .push_str(&item.to_string()); __terrane_text })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_separator = &"|";
            let mut __terrane_text = String::with_capacity(1); __terrane_text
            .push_str(&item.to_string()); __terrane_text.push_str(__terrane_separator);
            __terrane_text.push_str(&item.to_string()); __terrane_text })
        );
    } else if consume(item) {
        println!("{}", String::from("wrong"));
    }
}
fn main() {
    let selector: u8;
    let other: u16;
    let first: Configured;
    let second: Configured;
    selector = 255;
    other = 511;
    let __terrane_switch_value_1140 = &selector;
    if &(*__terrane_switch_value_1140 as u16) == &other {
        println!("{}", String::from("wrong-truncated"));
    } else if __terrane_switch_value_1140 == &255 {
        println!("{}", String::from("wide-comparison"));
    }
    let __terrane_switch_value_1270 = &absent();
    if {
        let _ = ();
        __terrane_switch_value_1270.is_none()
    } {
        println!("{}", String::from("optional-none"));
    }
    let __terrane_switch_value_1344 = &();
    if absent().is_none() {
        println!("{}", String::from("none-both"));
    }
    first = Configured::terrane_construct(String::from("a"));
    second = Configured::terrane_construct(String::from("b"));
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&first.slot.as_ref()
        .expect("required field initialized").text), String::from("|"),
        terrane_scalar_support::scalar_text(&second.slot.as_ref()
        .expect("required field initialized").text)
    );
    resource_case(true);
}
