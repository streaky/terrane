trait TerraneMutableCallableBody<Arguments, Output>: Send {
    fn call(&mut self, arguments: Arguments) -> Output;
    fn clone_box(
        &self,
    ) -> std::boxed::Box<dyn TerraneMutableCallableBody<Arguments, Output>>;
}

impl<Arguments, Output, Function> TerraneMutableCallableBody<Arguments, Output> for Function
where
    Function: FnMut(Arguments) -> Output + Clone + Send + 'static,
{
    fn call(&mut self, arguments: Arguments) -> Output {
        self(arguments)
    }

    fn clone_box(
        &self,
    ) -> std::boxed::Box<dyn TerraneMutableCallableBody<Arguments, Output>> {
        std::boxed::Box::new(self.clone())
    }
}

pub struct TerraneMutableCallable<Arguments, Output> {
    body: std::sync::Mutex<
        std::boxed::Box<dyn TerraneMutableCallableBody<Arguments, Output>>,
    >,
}

impl<Arguments, Output> TerraneMutableCallable<Arguments, Output> {
    fn new<Function>(function: Function) -> Self
    where
        Function: FnMut(Arguments) -> Output + Clone + Send + 'static,
    {
        Self {
            body: std::sync::Mutex::new(std::boxed::Box::new(function)),
        }
    }

    // Semantic mode checking provides exclusive invocation. The mutex gives the
    // erased body interior mutability without aliasing callable values: Clone
    // forks the current closure environment. Re-entrant invocation would
    // deadlock, but cannot occur because a closure captures a separated value.
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
