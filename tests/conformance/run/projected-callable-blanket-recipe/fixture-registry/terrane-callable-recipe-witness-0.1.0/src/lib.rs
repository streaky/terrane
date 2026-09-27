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

pub trait Initializer<State> {
    fn initialize(self) -> State;
}

impl<F, State> Initializer<State> for F
where
    F: FnOnce() -> State,
{
    fn initialize(self) -> State {
        self()
    }
}

pub trait Reducer<State, Message> {
    fn reduce(self, state: State, message: Message) -> State;
}

impl<F, State, Message> Reducer<State, Message> for F
where
    F: FnOnce(State, Message) -> State,
{
    fn reduce(self, state: State, message: Message) -> State {
        self(state, message)
    }
}

pub fn run_correlated<State, Message>(
    initializer: impl Initializer<State>,
    message: Message,
    reducer: impl Reducer<State, Message>,
) -> State {
    reducer.reduce(initializer.initialize(), message)
}

pub fn compose<State, Message>(
    initializer: impl Initializer<State>,
    message: impl Initializer<Message>,
    reducer: impl Reducer<State, Message>,
) -> State {
    reducer.reduce(initializer.initialize(), message.initialize())
}

pub trait Handler<Arguments, Output> {}

impl<F, Output> Handler<(), Output> for F where F: FnOnce() -> Output {}

impl<F, Argument, Output> Handler<(Argument,), Output> for F where
    F: FnOnce(Argument) -> Output
{
}

pub fn accepts_handler<H, Arguments, Output>(_handler: H) -> bool
where
    H: Handler<Arguments, Output>,
{
    true
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
