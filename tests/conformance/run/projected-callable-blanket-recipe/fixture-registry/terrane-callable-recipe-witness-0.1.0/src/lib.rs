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

pub trait AsyncHandler<T, State> {}

impl<F, Fut, Output, State> AsyncHandler<((),), State> for F
where
    F: FnOnce() -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Output> + Send,
    Output: std::fmt::Display,
{
}

pub fn install<H, T>(handler: H) -> u32
where
    H: AsyncHandler<T, ()>,
{
    let _ = handler;
    42
}

pub struct MethodRouter<State = (), Error = std::convert::Infallible> {
    state: std::marker::PhantomData<State>,
    error: std::marker::PhantomData<Error>,
}

pub fn blank() -> MethodRouter<(), std::convert::Infallible> {
    MethodRouter {
        state: std::marker::PhantomData,
        error: std::marker::PhantomData,
    }
}

pub fn accept(router: MethodRouter<()>) -> u32 {
    let _ = router;
    42
}
