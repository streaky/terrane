#[derive(Clone)]
pub struct Holder<T = u8> {
    value: T,
}

impl<T> Holder<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn into_value(self) -> T {
        self.value
    }
}

pub fn make_owner() -> Holder<u8> {
    Holder::new(19)
}

pub fn owner_byte(value: Holder<u8>) -> u8 {
    value.into_value()
}
