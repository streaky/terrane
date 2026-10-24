pub use native_default_provider::{MutableValue, NativeValue};

pub struct Wrapped<T>(pub T);

impl<T: From<u8>> NativeValue for Wrapped<T> {
    type Owner = T;
}

pub struct Overridden;

impl Overridden {
    pub fn new() -> Self {
        Self
    }
}

impl NativeValue for Overridden {
    type Owner = u8;

    fn owner_value(&self) -> Self::Owner {
        3
    }

    fn label(&self) -> &'static str {
        "override"
    }
}

pub struct Mutator {
    value: u8,
}

impl Mutator {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}

impl NativeValue for Mutator {
    type Owner = u8;
}

impl native_default_provider::MutableValue for Mutator {
    fn update(&mut self, amount: u8) -> Result<u8, native_default_provider::NativeFailure> {
        if amount == 0 {
            return Err(std::io::Error::other("zero update rejected").into());
        }
        self.value += amount;
        Ok(self.value)
    }
}
