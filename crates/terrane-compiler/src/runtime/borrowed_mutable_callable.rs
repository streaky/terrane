pub struct TerraneBorrowedMutableCallable<'a, Arguments, Output> {
    body: std::boxed::Box<dyn FnMut(Arguments) -> Output + 'a>,
}

impl<'a, Arguments, Output> TerraneBorrowedMutableCallable<'a, Arguments, Output> {
    fn new<Function>(function: Function) -> Self
    where
        Function: FnMut(Arguments) -> Output + 'a,
    {
        Self {
            body: std::boxed::Box::new(function),
        }
    }

    fn call(&mut self, arguments: Arguments) -> Output {
        (self.body)(arguments)
    }
}
