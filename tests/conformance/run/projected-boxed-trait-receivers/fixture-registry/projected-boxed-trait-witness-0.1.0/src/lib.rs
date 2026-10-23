pub trait BoxedCounter: Send {
    fn read(self: Box<Self>) -> i64;
    fn increment(self: Box<Self>) -> i64;
}

pub fn read_boxed(value: Box<dyn BoxedCounter>) -> i64 {
    value.read()
}

pub fn increment_boxed(value: Box<dyn BoxedCounter>) -> i64 {
    value.increment()
}
