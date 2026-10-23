// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct Counter {
    pub value: terrane_int_support::Int,
}
impl Counter {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(17_i128),
        }
    }
    pub fn read(&self) -> terrane_int_support::Int {
        return self.value.clone();
    }
    pub fn increment(&mut self) -> terrane_int_support::Int {
        {
            let __terrane_value = self.value.clone()
                + terrane_int_support::Int::from(5_i128);
            self.value = __terrane_value;
        }
        return self.value.clone();
    }
}
impl BoxedCounterProtocol for Counter {
    fn read(self: Box<Self>) -> terrane_int_support::Int {
        Counter::read(&*self)
    }
    fn increment(mut self: Box<Self>) -> terrane_int_support::Int {
        Counter::increment(&mut *self)
    }
}
impl From<Counter> for BoxedCounter {
    fn from(value: Counter) -> Self {
        Self(Box::new(value))
    }
}
impl witness::BoxedCounter for Counter {
    fn read(self: Box<Self>) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Counter>::read(&*self);
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn increment(mut self: Box<Self>) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Counter>::increment(&mut *self);
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
fn rust_read(value: Counter) -> i64 {
    let result: i64;
    result = witness::read_boxed(Box::new(value));
    return result;
}
fn rust_increment(value: Counter) -> i64 {
    let result: i64;
    result = witness::increment_boxed(Box::new(value));
    return result;
}
fn main() {
    let read_value: Counter;
    let increment_value: Counter;
    read_value = Counter::terrane_construct();
    increment_value = Counter::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&rust_read(read_value)));
    println!(
        "{}", terrane_scalar_support::scalar_text(&rust_increment(increment_value))
    );
}
