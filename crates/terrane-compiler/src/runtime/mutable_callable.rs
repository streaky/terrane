trait TerraneMutableCallableBody<Arguments, Output>: Send {
    fn call(&mut self, arguments: Arguments) -> Output;
    fn clone_box(&self) -> Box<dyn TerraneMutableCallableBody<Arguments, Output>>;
}

impl<Arguments, Output, Function> TerraneMutableCallableBody<Arguments, Output> for Function
where
    Function: FnMut(Arguments) -> Output + Clone + Send + 'static,
{
    fn call(&mut self, arguments: Arguments) -> Output {
        self(arguments)
    }

    fn clone_box(&self) -> Box<dyn TerraneMutableCallableBody<Arguments, Output>> {
        Box::new(self.clone())
    }
}

pub struct TerraneMutableCallable<Arguments, Output> {
    body: std::sync::Mutex<Box<dyn TerraneMutableCallableBody<Arguments, Output>>>,
}

impl<Arguments, Output> TerraneMutableCallable<Arguments, Output> {
    fn new<Function>(function: Function) -> Self
    where
        Function: FnMut(Arguments) -> Output + Clone + Send + 'static,
    {
        Self {
            body: std::sync::Mutex::new(Box::new(function)),
        }
    }

    fn call(&self, arguments: Arguments) -> Output {
        self.body
            .lock()
            .expect("mutable callable lock poisoned")
            .call(arguments)
    }
}

impl<Arguments, Output> Clone for TerraneMutableCallable<Arguments, Output> {
    fn clone(&self) -> Self {
        let body = self
            .body
            .lock()
            .expect("mutable callable lock poisoned")
            .clone_box();
        Self {
            body: std::sync::Mutex::new(body),
        }
    }
}
