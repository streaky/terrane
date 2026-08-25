// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: interface-value-separation
pub trait ReadableProtocol {
    fn clone_box(&self) -> Box<dyn ReadableProtocol>;
    fn separate_box(&self) -> Box<dyn ReadableProtocol>;
    fn read(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn ReadableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
#[derive(Clone)]
pub struct Readable(Box<dyn ReadableProtocol>);
impl Readable {
    pub fn read(&self) -> terrane_int_support::Int {
        self.0.read()
    }
    fn terrane_separate(&self) -> Self {
        Self(self.0.separate_box())
    }
}
#[derive(Clone)]
pub struct Counter {
    __terrane_lifetime: std::sync::Arc<()>,
    pub value: terrane_int_support::Int,
}
impl Counter {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(7_i128),
            __terrane_lifetime: std::sync::Arc::new(()),
        }
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn read(&self) -> terrane_int_support::Int {
        return self.value.clone();
    }
    pub fn destruct(&self) {
        println!(
            "{}", terrane_scalar_support::scalar_text(& (String::from("destruct")))
        );
    }
}
impl ReadableProtocol for Counter {
    fn clone_box(&self) -> Box<dyn ReadableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn ReadableProtocol> {
        Box::new(self.terrane_separate())
    }
    fn read(&self) -> terrane_int_support::Int {
        Counter::read(self)
    }
}
impl From<Counter> for Readable {
    fn from(value: Counter) -> Self {
        Self(Box::new(value))
    }
}
impl Drop for Counter {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) == 1 {
            self.destruct();
        }
    }
}
fn main() {
    let original: Readable = Readable::from(Counter::terrane_construct());
    let copied: Readable = original.terrane_separate();
    println!("{}", terrane_scalar_support::scalar_text(& (original.read())));
    println!("{}", terrane_scalar_support::scalar_text(& (copied.read())));
}
