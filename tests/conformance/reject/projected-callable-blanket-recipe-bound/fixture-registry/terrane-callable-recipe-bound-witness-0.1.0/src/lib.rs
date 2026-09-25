pub trait Factory<Output> {
    fn create(self) -> Output;
}

impl<F, Output> Factory<Output> for F
where
    F: FnOnce() -> Output,
    Output: std::fmt::Display,
{
    fn create(self) -> Output {
        self()
    }
}

pub fn invoke<Output>(factory: impl Factory<Output>) -> Output {
    factory.create()
}
