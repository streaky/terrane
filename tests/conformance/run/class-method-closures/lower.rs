// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: class-method-closures
#[derive(Clone)]
pub struct Maker {
}
impl Maker {
    pub fn terrane_construct() -> Self {
        Self {
        }
    }
    pub fn offset(&self, base: terrane_int_support::Int) -> std::sync::Arc<dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int> {
        let _ = &self;
        return { let base = base.clone(); std::sync::Arc::new(move |value: terrane_int_support::Int| -> terrane_int_support::Int {
            return base.clone() + value.clone();
        }) };
    }
}
fn main() {
    let value: Maker = Maker::terrane_construct();
    let add: std::sync::Arc<dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int> = value.offset(terrane_int_support::Int::from(10_i128));
    let result: terrane_int_support::Int = add(terrane_int_support::Int::from(5_i128));
    println!("{}", terrane_scalar_support::scalar_text(&(result)));
}
