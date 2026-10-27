// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: source-reference-argument
#[derive(Clone)]
pub struct Holder {
    pub value: terrane_int_support::Int,
    pub other: terrane_int_support::Int,
}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(7_i128),
            other: terrane_int_support::Int::from(9_i128),
        }
    }
    pub fn read_integer(
        &self,
        value: &terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        return value.clone();
    }
}
#[derive(Clone)]
pub struct IntegerHolder {
    pub value: terrane_int_support::Int,
}
impl IntegerHolder {
    pub fn terrane_construct(value: &terrane_int_support::Int) -> Self {
        let mut __terrane_constructed_value = Self {
            value: terrane_int_support::Int::from(0_i128),
        };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: &terrane_int_support::Int) {
        {
            let __terrane_value = value.clone();
            self.value = __terrane_value;
        }
    }
}
fn read(item: &Holder) -> terrane_int_support::Int {
    return {
        let __terrane_snapshot = item.value.clone();
        __terrane_snapshot
    };
}
fn forward(item: &Holder) -> terrane_int_support::Int {
    return read(item);
}
fn forward_field<'a>(item: &'a Holder) -> &'a terrane_int_support::Int {
    return field(item);
}
fn field<'a>(item: &'a Holder) -> &'a terrane_int_support::Int {
    return &item.value;
}
fn read_int(value: &terrane_int_support::Int) -> terrane_int_support::Int {
    return value.clone();
}
fn forward_int(value: &terrane_int_support::Int) -> terrane_int_support::Int {
    return read_int(value);
}
fn borrow_int<'a>(value: &'a terrane_int_support::Int) -> &'a terrane_int_support::Int {
    return value;
}
fn main() {
    let n: terrane_int_support::Int;
    let explicit: terrane_int_support::Int;
    let scalar_reader: std::sync::Arc<
        dyn Fn(&terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
    >;
    let callable_value: terrane_int_support::Int;
    let constructor_value: terrane_int_support::Int;
    let constructed: IntegerHolder;
    let named_value: terrane_int_support::Int;
    let returned_value: terrane_int_support::Int;
    let borrowed_int: &terrane_int_support::Int;
    let item: Holder;
    let method_value: terrane_int_support::Int;
    let observer: &Holder;
    let selected: &terrane_int_support::Int;
    let reader: std::sync::Arc<
        dyn Fn(&Holder) -> terrane_int_support::Int + Send + Sync,
    >;
    n = terrane_int_support::Int::from(5_i128);
    println!("{}", terrane_scalar_support::scalar_text(&read_int(&n)));
    explicit = terrane_int_support::Int::from(6_i128);
    println!("{}", terrane_scalar_support::scalar_text(&forward_int(&explicit)));
    scalar_reader = {
        std::sync::Arc::new(move |
            value: &terrane_int_support::Int,
        | -> terrane_int_support::Int {
            return value.clone();
        })
    };
    callable_value = terrane_int_support::Int::from(2_i128);
    println!("{}", terrane_scalar_support::scalar_text(&scalar_reader(&callable_value)));
    constructor_value = terrane_int_support::Int::from(13_i128);
    constructed = IntegerHolder::terrane_construct(&constructor_value);
    println!("{}", terrane_scalar_support::scalar_text(&constructed.value));
    named_value = terrane_int_support::Int::from(17_i128);
    println!("{}", terrane_scalar_support::scalar_text(&read_int(&named_value)));
    returned_value = terrane_int_support::Int::from(19_i128);
    borrowed_int = borrow_int(&returned_value);
    println!("{}", terrane_scalar_support::scalar_text(&read_int(borrowed_int)));
    item = Holder::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&read(&item)));
    method_value = terrane_int_support::Int::from(23_i128);
    println!(
        "{}", terrane_scalar_support::scalar_text(&item.read_integer(&method_value))
    );
    observer = &item;
    println!("{}", terrane_scalar_support::scalar_text(&forward(observer)));
    selected = forward_field(&item);
    println!("{}", terrane_scalar_support::scalar_text(&* selected));
    println!("{}", terrane_scalar_support::scalar_text(&read(&item)));
    reader = {
        std::sync::Arc::new(move |value: &Holder| -> terrane_int_support::Int {
            return {
                let __terrane_snapshot = value.value.clone();
                __terrane_snapshot
            };
        })
    };
    println!("{}", terrane_scalar_support::scalar_text(&reader(&item)));
    println!("{}", terrane_scalar_support::scalar_text(&item.other));
}
