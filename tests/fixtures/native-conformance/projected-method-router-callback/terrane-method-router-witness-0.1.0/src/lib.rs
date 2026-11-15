pub type Auxiliary<Output = u8> = Output;

pub trait AsyncHandler<Arguments, State>: Clone + Send + Sync + Sized + 'static {
    fn auxiliary(&self) -> Auxiliary {
        7
    }

    type Future: Future<Output = String> + Send;

    fn call(self, input: i64, state: State) -> Self::Future;

    fn native_suffix(&self) -> &'static str {
        ":native"
    }
}

impl<F, Fut, Output, State> AsyncHandler<(), State> for F
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

pub struct MethodRouter<State = (), Error = std::convert::Infallible> {
    state: std::marker::PhantomData<State>,
    error: std::marker::PhantomData<Error>,
}

impl<State: Default, Error> MethodRouter<State, Error> {
    pub fn new() -> Self {
        Self {
            state: std::marker::PhantomData,
            error: std::marker::PhantomData,
        }
    }

    pub async fn dispatch<H, Arguments>(self, handler: H) -> String
    where
        H: AsyncHandler<Arguments, State>,
    {
        let suffix = handler.native_suffix();
        handler.call(41, State::default()).await + suffix
    }
}
