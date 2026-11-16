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

pub type Auxiliary<Output = u8> = Output;

pub trait AsyncHandler<T, State>: Clone + Send + Sync + Sized + 'static {
    fn auxiliary(&self) -> Auxiliary {
        7
    }
    type Future: Future<Output = String> + Send;
    fn call(self, input: i64, state: State) -> Self::Future;
    fn native_suffix(&self) -> &'static str {
        ":native"
    }
}

impl<F, Fut, Output, State> AsyncHandler<((),), State> for F
where
    F: FnOnce() -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Output> + Send + 'static,
    Output: std::fmt::Display,
{
    type Future = std::pin::Pin<Box<dyn Future<Output = String> + Send>>;
    fn call(self, _input: i64, _state: State) -> Self::Future {
        Box::pin(async move { self().await.to_string() })
    }
}

impl<F, Fut, Output, State> AsyncHandler<(i64,), State> for F
where
    F: FnOnce(i64) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Output> + Send + 'static,
    Output: std::fmt::Display,
{
    type Future = std::pin::Pin<Box<dyn Future<Output = String> + Send>>;
    fn call(self, input: i64, _state: State) -> Self::Future {
        Box::pin(async move { self(input).await.to_string() })
    }
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
    rendered: Option<String>,
}

pub fn blank() -> MethodRouter<(), std::convert::Infallible> {
    MethodRouter {
        state: std::marker::PhantomData,
        error: std::marker::PhantomData,
        rendered: None,
    }
}

pub fn accept(router: MethodRouter<()>) -> u32 {
    let _ = router;
    42
}

impl<State: Default, Error> MethodRouter<State, Error> {
    pub fn new() -> Self {
        Self {
            state: std::marker::PhantomData,
            error: std::marker::PhantomData,
            rendered: None,
        }
    }
    pub async fn dispatch<H, T>(self, handler: H) -> String
    where
        H: AsyncHandler<T, State>,
    {
        let suffix = handler.native_suffix();
        handler.call(41, State::default()).await + suffix
    }
}

pub async fn registered<H, T, State: Default>(handler: H) -> MethodRouter<State>
where
    H: AsyncHandler<T, State>,
{
    let suffix = handler.native_suffix();
    let rendered = handler.call(41, State::default()).await + suffix;
    MethodRouter {
        state: std::marker::PhantomData,
        error: std::marker::PhantomData,
        rendered: Some(rendered),
    }
}

pub fn registration_value(router: MethodRouter) -> String {
    router.rendered.expect("registered callback result")
}

pub fn registration_value_with_state(router: MethodRouter<String>) -> String {
    router.rendered.expect("registered callback result")
}

pub struct View<'a> {
    value: i64,
    marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> From<i64> for View<'a> {
    fn from(value: i64) -> Self {
        Self {
            value,
            marker: std::marker::PhantomData,
        }
    }
}

pub trait ViewFn<'a, State> {
    fn view(&self, state: &'a State) -> View<'a>;
}

impl<'a, F, State: 'a, Widget> ViewFn<'a, State> for F
where
    F: Fn(&'a State) -> Widget,
    Widget: Into<View<'a>>,
{
    fn view(&self, state: &'a State) -> View<'a> {
        self(state).into()
    }
}

pub fn render<State>(state: &State, view: impl for<'a> ViewFn<'a, State>) -> i64 {
    view.view(state).value
}
