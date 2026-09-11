pub struct TerraneAsyncMutableState<Value> {
    value: std::sync::Arc<std::sync::Mutex<Value>>,
    invocation: std::sync::Arc<tokio::sync::Mutex<()>>,
}

impl<Value> TerraneAsyncMutableState<Value> {
    fn new(value: Value) -> Self {
        Self {
            value: std::sync::Arc::new(std::sync::Mutex::new(value)),
            invocation: std::sync::Arc::new(tokio::sync::Mutex::new(())),
        }
    }

    fn share(&self) -> Self {
        Self {
            value: self.value.clone(),
            invocation: self.invocation.clone(),
        }
    }

    fn replace(&self, value: Value) {
        *self
            .value
            .lock()
            .expect("mutable async callable state lock poisoned") = value;
    }
}

impl<Value: Clone> TerraneAsyncMutableState<Value> {
    fn snapshot(&self) -> Value {
        self.value
            .lock()
            .expect("mutable async callable state lock poisoned")
            .clone()
    }

    async fn with_receiver<Output, Operation>(&self, operation: Operation) -> Output
    where
        Value: Send,
        Operation: for<'receiver> FnOnce(
                &'receiver mut Value,
            ) -> std::pin::Pin<Box<dyn Future<Output = Output> + Send + 'receiver>>
            + Send,
    {
        let _invocation = self.invocation.lock().await;
        let mut receiver = self.snapshot();
        let result = operation(&mut receiver).await;
        self.replace(receiver);
        result
    }
}

impl<Value: Clone> Clone for TerraneAsyncMutableState<Value> {
    fn clone(&self) -> Self {
        Self::new(self.snapshot())
    }
}
