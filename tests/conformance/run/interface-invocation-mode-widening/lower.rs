// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: interface-invocation-mode-widening
pub trait MutableCounterProtocol {
    fn clone_box(&self) -> Box<dyn MutableCounterProtocol>;
    fn separate_box(&self) -> Box<dyn MutableCounterProtocol>;
    fn value(&mut self, offset: terrane_int_support::Int) -> terrane_int_support::Int;
}
impl Clone for Box<dyn MutableCounterProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct MutableCounter(Box<dyn MutableCounterProtocol>);
impl Clone for MutableCounter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl MutableCounter {
    pub fn value(
        &mut self,
        offset: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        self.0.value(offset)
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
pub struct ConsumingLabel(Box<dyn ConsumingLabelProtocol>);
impl Clone for ConsumingLabel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ConsumingLabel {
    pub fn label(self) -> String {
        self.0.label()
    }
}
pub trait ConsumingCounterProtocol {
    fn clone_box(&self) -> Box<dyn ConsumingCounterProtocol>;
    fn separate_box(&self) -> Box<dyn ConsumingCounterProtocol>;
    fn redeem(self: Box<Self>) -> terrane_int_support::Int;
}
impl Clone for Box<dyn ConsumingCounterProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct ConsumingCounter(Box<dyn ConsumingCounterProtocol>);
impl Clone for ConsumingCounter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ConsumingCounter {
    pub fn redeem(self) -> terrane_int_support::Int {
        self.0.redeem()
    }
}
#[derive(Clone)]
pub struct Sample {
    pub total: terrane_int_support::Int,
}
impl Sample {
    pub fn terrane_construct() -> Self {
        Self {
            total: terrane_int_support::Int::from(0_i128),
        }
    }
    pub fn value(&self, offset: terrane_int_support::Int) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(7_i128) + offset.clone();
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
    fn value(&mut self, offset: terrane_int_support::Int) -> terrane_int_support::Int {
        Sample::value(&*self, offset)
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
#[derive(Clone)]
pub struct MutableSample {
    pub total: terrane_int_support::Int,
}
impl MutableSample {
    pub fn terrane_construct() -> Self {
        Self {
            total: terrane_int_support::Int::from(0_i128),
        }
    }
    pub fn redeem(&mut self) -> terrane_int_support::Int {
        self.total = self.total.clone() + terrane_int_support::Int::from(1_i128);
        return self.total.clone();
    }
}
impl ConsumingCounterProtocol for MutableSample {
    fn clone_box(&self) -> Box<dyn ConsumingCounterProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn ConsumingCounterProtocol> {
        Box::new(self.clone())
    }
    fn redeem(mut self: Box<Self>) -> terrane_int_support::Int {
        MutableSample::redeem(&mut *self)
    }
}
impl From<MutableSample> for ConsumingCounter {
    fn from(value: MutableSample) -> Self {
        Self(Box::new(value))
    }
}
fn main() {
    let mut mutable_view: MutableCounter = <MutableCounter>::from(
        Sample::terrane_construct(),
    );
    let consuming_view: ConsumingLabel = <ConsumingLabel>::from(
        Sample::terrane_construct(),
    );
    let mutable_consuming_view: ConsumingCounter = <ConsumingCounter>::from(
        MutableSample::terrane_construct(),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&mutable_view
        .value(terrane_int_support::Int::from(5_i128)))
    );
    println!("{}", terrane_scalar_support::scalar_text(&consuming_view.label()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&mutable_consuming_view.redeem())
    );
}
