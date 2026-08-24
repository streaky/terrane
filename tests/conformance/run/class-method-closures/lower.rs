// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: class-method-closures
#[derive(Clone)]
pub struct Maker {
    pub base: terrane_int_support::Int,
}
impl Maker {
    pub fn terrane_construct() -> Self {
        Self {
            base: terrane_int_support::Int::from(10_i128),
        }
    }
    pub fn offset(&self) -> std::sync::Arc<dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int> {
        return { let this = self.clone(); std::sync::Arc::new(move |value: terrane_int_support::Int| -> terrane_int_support::Int {
            return (this.base).clone() + value.clone();
        }) };
    }
}
fn main() {
    let value: Maker = Maker::terrane_construct();
    let add: std::sync::Arc<dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int> = value.offset();
    let result: terrane_int_support::Int = add(terrane_int_support::Int::from(5_i128));
    println!("{}", terrane_scalar_support::scalar_text(&(result)));
}
