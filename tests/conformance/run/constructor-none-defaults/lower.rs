// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: constructor-none-defaults
#[derive(Clone)]
pub struct Payload {
    pub value: terrane_int_support::Int,
}
impl Payload {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(7_i128),
        }
    }
}
#[derive(Clone)]
pub struct Holder {
    pub tag: String,
    pub value: Option<Payload>,
}
impl Holder {
    pub fn terrane_construct(tag: String, value: Option<Payload>) -> Self {
        let mut __terrane_constructed_value = Self {
            tag: String::new(),
            value: None,
        };
        __terrane_constructed_value.construct(tag, value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, tag: String, value: Option<Payload>) {
        {
            let __terrane_value = tag;
            self.tag = __terrane_value;
        }
        {
            let __terrane_value = value;
            self.value = __terrane_value;
        }
    }
}
fn describe(value: Option<Payload>) -> String {
    if (&value).is_some() {
        return {
            let mut __terrane_text = String::with_capacity(6);
            __terrane_text.push_str("value=");
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &match &value {
                    Some(value) => value,
                    _ => unreachable!("flow-proven storage refinement"),
                }
                    .value,
                &mut __terrane_text,
            );
            __terrane_text
        };
    }
    return String::from("missing");
}
fn main() {
    let omitted: Holder;
    let partial: Holder;
    let present: Holder;
    let owned: Option<Payload>;
    omitted = Holder::terrane_construct(String::from("default"), None);
    partial = Holder::terrane_construct(String::from("partial"), None);
    present = Holder::terrane_construct(
        String::from("present"),
        Some(Payload::terrane_construct()),
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&omitted.tag), String::from(":"),
        terrane_scalar_support::scalar_text(&omitted.value.is_none())
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&partial.tag), String::from(":"),
        terrane_scalar_support::scalar_text(&partial.value.is_none())
    );
    owned = present.value.clone();
    if (&owned).is_some() {
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&present.tag),
            String::from(":"), terrane_scalar_support::scalar_text(&match &owned {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") }
            .value)
        );
    }
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&describe(None)),
        String::from(":"), terrane_scalar_support::scalar_text(&describe(None)),
        String::from(":"),
        terrane_scalar_support::scalar_text(&describe(Some(Payload::terrane_construct())))
    );
}
