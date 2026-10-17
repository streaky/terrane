// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: object-field-owned-snapshot
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
    pub required: Option<Payload>,
    pub initialized: Payload,
}
impl Holder {
    pub fn terrane_construct() -> Self {
        let mut __terrane_constructed_value = Self {
            required: None,
            initialized: Payload::terrane_construct(),
        };
        __terrane_constructed_value.construct();
        __terrane_constructed_value
    }
    pub fn construct(&mut self) {
        self.required = Some(Payload::terrane_construct());
    }
    pub fn required_copy(&self) -> Payload {
        return self.required.as_ref().expect("required field initialized").clone();
    }
    pub fn initialized_copy(&self) -> Payload {
        return self.initialized.clone();
    }
    pub fn change_copy(&self) -> terrane_int_support::Int {
        let mut local: Payload;
        local = self.required.as_ref().expect("required field initialized").clone();
        local.value = terrane_int_support::Int::from(9_i128);
        return local.value.clone();
    }
    pub fn temporary_required_copy(&self) -> Payload {
        return Holder::terrane_construct()
            .required
            .as_ref()
            .expect("required field initialized")
            .clone();
    }
}
#[derive(Clone)]
pub struct PlainHolder {
    pub value: Payload,
}
impl PlainHolder {
    pub fn terrane_construct() -> Self {
        Self {
            value: Payload::terrane_construct(),
        }
    }
}
fn make_holder() -> PlainHolder {
    return PlainHolder::terrane_construct();
}
fn borrow_holder<'a>(value: &'a PlainHolder) -> &'a PlainHolder {
    return value;
}
fn main() {
    let original: Holder;
    let first: Payload;
    let second: Payload;
    let unchanged: Payload;
    let mut from_call: Payload;
    let mut from_constructor: Payload;
    let named: PlainHolder;
    let mut from_borrow: Payload;
    original = Holder::terrane_construct();
    first = original.required_copy();
    second = original.initialized_copy();
    println!("{}", terrane_scalar_support::scalar_text(&first.value));
    println!("{}", terrane_scalar_support::scalar_text(&second.value));
    println!("{}", terrane_scalar_support::scalar_text(&original.change_copy()));
    unchanged = original.required_copy();
    println!("{}", terrane_scalar_support::scalar_text(&unchanged.value));
    from_call = make_holder().value;
    from_call.value = terrane_int_support::Int::from(11_i128);
    from_constructor = PlainHolder::terrane_construct().value;
    from_constructor.value = terrane_int_support::Int::from(12_i128);
    println!("{}", terrane_scalar_support::scalar_text(&from_call.value));
    println!("{}", terrane_scalar_support::scalar_text(&from_constructor.value));
    println!(
        "{}", terrane_scalar_support::scalar_text(&original.temporary_required_copy()
        .value)
    );
    named = PlainHolder::terrane_construct();
    from_borrow = borrow_holder(&named).value.clone().clone();
    from_borrow.value = terrane_int_support::Int::from(13_i128);
    println!("{}", terrane_scalar_support::scalar_text(&from_borrow.value));
    println!("{}", terrane_scalar_support::scalar_text(&named.value.value));
}
