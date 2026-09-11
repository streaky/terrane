// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: interface-invocation-mode-widening
pub trait MutableCounterProtocol {
    fn clone_box(&self) -> Box<dyn MutableCounterProtocol>;
    fn separate_box(&self) -> Box<dyn MutableCounterProtocol>;
    fn value(&mut self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn MutableCounterProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
#[derive(Clone)]
pub struct MutableCounter(Box<dyn MutableCounterProtocol>);
impl MutableCounter {
    pub fn value(&mut self) -> terrane_int_support::Int {
        self.0.value()
    }
}
pub trait ConsumingLabelProtocol {
    fn clone_box(&self) -> Box<dyn ConsumingLabelProtocol>;
    fn separate_box(&self) -> Box<dyn ConsumingLabelProtocol>;
    fn label(self: Box<Self>) -> String;
}
impl Clone for Box<dyn ConsumingLabelProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
#[derive(Clone)]
pub struct ConsumingLabel(Box<dyn ConsumingLabelProtocol>);
impl ConsumingLabel {
    pub fn label(self) -> String {
        self.0.label()
    }
}
#[derive(Clone)]
pub struct Sample {}
impl Sample {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn value(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(7_i128);
    }
    pub fn label(&self) -> String {
        return String::from("ready");
    }
}
impl MutableCounterProtocol for Sample {
    fn clone_box(&self) -> Box<dyn MutableCounterProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn MutableCounterProtocol> {
        Box::new(self.clone())
    }
    fn value(&mut self) -> terrane_int_support::Int {
        Sample::value(&*self)
    }
}
impl From<Sample> for MutableCounter {
    fn from(value: Sample) -> Self {
        Self(Box::new(value))
    }
}
impl ConsumingLabelProtocol for Sample {
    fn clone_box(&self) -> Box<dyn ConsumingLabelProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn ConsumingLabelProtocol> {
        Box::new(self.clone())
    }
    fn label(self: Box<Self>) -> String {
        Sample::label(&*self)
    }
}
impl From<Sample> for ConsumingLabel {
    fn from(value: Sample) -> Self {
        Self(Box::new(value))
    }
}
fn main() {
    let mut mutable_view: MutableCounter = MutableCounter::from(
        Sample::terrane_construct(),
    );
    let consuming_view: ConsumingLabel = ConsumingLabel::from(
        Sample::terrane_construct(),
    );
    println!("{}", terrane_scalar_support::scalar_text(&mutable_view.value()));
    println!("{}", terrane_scalar_support::scalar_text(&consuming_view.label()));
}
