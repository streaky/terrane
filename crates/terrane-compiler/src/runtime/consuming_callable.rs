trait TerraneConsumingCallableBody<Arguments, Output>: Send {
    fn call(self: Box<Self>, arguments: Arguments) -> Output;
}

impl<Arguments, Output, Function> TerraneConsumingCallableBody<Arguments, Output> for Function
where
    Function: FnOnce(Arguments) -> Output + Send + 'static,
{
    fn call(self: Box<Self>, arguments: Arguments) -> Output {
        self(arguments)
    }
}

pub struct TerraneConsumingCallable<Arguments, Output> {
    body: Box<dyn TerraneConsumingCallableBody<Arguments, Output>>,
}

impl<Arguments, Output> TerraneConsumingCallable<Arguments, Output> {
    fn new<Function>(function: Function) -> Self
    where
        Function: FnOnce(Arguments) -> Output + Send + 'static,
    {
        Self {
            body: Box::new(function),
        }
    }

    fn call(self, arguments: Arguments) -> Output {
        self.body.call(arguments)
    }
}
