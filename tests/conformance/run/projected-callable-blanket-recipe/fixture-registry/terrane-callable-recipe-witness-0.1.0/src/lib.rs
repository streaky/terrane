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

pub trait Transformer<Input, Output> {
    fn transform(self, input: Input) -> Output;
}

impl<F, Input, Output> Transformer<Input, Output> for F
where
    F: FnOnce(Input) -> Output,
{
    fn transform(self, input: Input) -> Output {
        self(input)
    }
}

pub fn transform<Input, Output>(
    input: Input,
    transformer: impl Transformer<Input, Output>,
) -> Output {
    transformer.transform(input)
}

pub trait Mutator<Input, Output> {
    fn mutate(&mut self, input: Input) -> Output;
}

impl<F, Input, Output> Mutator<Input, Output> for F
where
    F: FnMut(Input) -> Output,
{
    fn mutate(&mut self, input: Input) -> Output {
        self(input)
    }
}

pub fn mutate<Input, Output>(
    input: Input,
    mut mutator: impl Mutator<Input, Output>,
) -> Output {
    mutator.mutate(input)
}
