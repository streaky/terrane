// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: source-reference-argument
#[derive(Clone)]
pub struct Holder {
    pub value: terrane_int_support::Int,
}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(7_i128),
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
fn main() {
    let item: Holder;
    let observer: &Holder;
    item = Holder::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&read(&item)));
    observer = &item;
    println!("{}", terrane_scalar_support::scalar_text(&forward(observer)));
}
