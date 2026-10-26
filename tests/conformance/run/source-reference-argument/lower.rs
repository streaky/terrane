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
fn main() {
    let item: Holder;
    let observer: &Holder;
    let selected: &terrane_int_support::Int;
    let reader: std::sync::Arc<
        dyn Fn(&Holder) -> terrane_int_support::Int + Send + Sync,
    >;
    item = Holder::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&read(&item)));
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
