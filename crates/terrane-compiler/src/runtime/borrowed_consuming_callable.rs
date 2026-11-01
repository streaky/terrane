pub struct TerraneBorrowedConsumingCallable<'a, Arguments, Output> {
    body: std::boxed::Box<dyn FnOnce(Arguments) -> Output + 'a>,
}

impl<'a, Arguments, Output> TerraneBorrowedConsumingCallable<'a, Arguments, Output> {
    fn new<Function>(function: Function) -> Self
    where
        Function: FnOnce(Arguments) -> Output + 'a,
    {
        Self {
            body: std::boxed::Box::new(function),
        }
    }

    fn call(self, arguments: Arguments) -> Output {
        (self.body)(arguments)
    }
}
