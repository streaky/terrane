// Generated deterministically by Terrane <version>.
// Source: contracts/a-values.trn
// Namespace: contracts
fn offset() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(2_i128);
}
// Source: contracts/z-protocol.trn
// Namespace: contracts
pub trait CounterProtocol: Send + Sync {
    fn finish(self: Box<Self>) -> terrane_int_support::Int;
}
pub struct Counter(Box<dyn CounterProtocol>);
impl Counter {
    pub fn finish(self) -> terrane_int_support::Int {
        self.0.finish()
    }
}
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct Tally {
    pub value: terrane_int_support::Int,
}
impl Tally {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(40_i128),
        }
    }
    pub fn finish(self) -> terrane_int_support::Int {
        return self.value.clone() + offset();
    }
}
impl CounterProtocol for Tally {
    fn finish(self: Box<Self>) -> terrane_int_support::Int {
        Tally::finish(*self)
    }
}
impl From<Tally> for Counter {
    fn from(value: Tally) -> Self {
        Self(Box::new(value))
    }
}
fn finish(value: Counter) -> terrane_int_support::Int {
    return value.finish();
}
fn main() {
    let item: Tally;
    let erased: Counter;
    item = Tally::terrane_construct();
    erased = <Counter>::from(item);
    println!("{}", terrane_scalar_support::scalar_text(&finish(erased)));
}
