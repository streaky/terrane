// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: mutating-interface-methods
pub trait AdjustableProtocol {
    fn clone_box(&self) -> Box<dyn AdjustableProtocol>;
    fn separate_box(&self) -> Box<dyn AdjustableProtocol>;
    fn increase(&mut self, amount: terrane_int_support::Int) -> terrane_int_support::Int;
    fn read(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn AdjustableProtocol> { fn clone(&self) -> Self { self.clone_box() } }
#[derive(Clone)]
pub struct Adjustable(Box<dyn AdjustableProtocol>);
impl Adjustable {
    pub fn increase(&mut self, amount: terrane_int_support::Int) -> terrane_int_support::Int {
        self.0.increase(amount)
    }
    pub fn read(&self) -> terrane_int_support::Int {
        self.0.read()
    }
}
#[derive(Clone)]
pub struct Counter {
    pub value: terrane_int_support::Int,
}
impl Counter {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn increase(&mut self, amount: terrane_int_support::Int) -> terrane_int_support::Int {
        self.value = (self.value).clone() + amount.clone();
        return (self.value).clone();
    }
    pub fn read(&self) -> terrane_int_support::Int {
        return (self.value).clone();
    }
}
impl AdjustableProtocol for Counter {
    fn clone_box(&self) -> Box<dyn AdjustableProtocol> { Box::new(self.clone()) }
    fn separate_box(&self) -> Box<dyn AdjustableProtocol> { Box::new(self.clone()) }
    fn increase(&mut self, amount: terrane_int_support::Int) -> terrane_int_support::Int {
        Counter::increase(self, amount)
    }
    fn read(&self) -> terrane_int_support::Int {
        Counter::read(self, )
    }
}
impl From<Counter> for Adjustable { fn from(value: Counter) -> Self { Self(Box::new(value)) } }
fn main() {
    let mut value: Adjustable = Adjustable::from(Counter::terrane_construct());
    println!("{}", terrane_scalar_support::scalar_text(&(value.increase(terrane_int_support::Int::from(4_i128)))));
    println!("{}", terrane_scalar_support::scalar_text(&(value.read())));
}
