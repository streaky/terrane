pub trait NativeValue {
    type Owner: From<u8>;

    fn owner_value(&self) -> Self::Owner {
        Self::Owner::from(5)
    }

    fn selected<U: From<u8>>(&self) -> U {
        assert!(std::mem::size_of::<U>() <= 2, "generic selector panic");
        U::from(7)
    }

    fn label(&self) -> &'static str {
        "default"
    }
}

pub type NativeFailure = Box<dyn std::error::Error + Send + Sync>;

pub trait MutableValue {
    fn update(&mut self, amount: u8) -> Result<u8, NativeFailure>;
}
