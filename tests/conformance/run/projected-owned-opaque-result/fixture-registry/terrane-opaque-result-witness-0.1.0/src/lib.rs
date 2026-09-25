pub trait Value {
    type Output;

    fn value(&self) -> Self::Output;
}

struct Hidden(u32);
impl Value for Hidden {
    type Output = u32;

    fn value(&self) -> Self::Output {
        self.0
    }
}

pub struct Wrapper<T: Value> {
    inner: T,
}

impl<T: Value<Output = u32>> Wrapper<T> {
    pub fn value(&self) -> u32 {
        self.inner.value()
    }
}

pub fn make(value: u32) -> Wrapper<impl Value<Output = u32>> {
    Wrapper {
        inner: Hidden(value),
    }
}

