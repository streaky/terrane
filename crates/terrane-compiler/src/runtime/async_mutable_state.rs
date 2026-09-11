pub struct TerraneAsyncMutableState<Value> {
    value: std::sync::Arc<tokio::sync::Mutex<Value>>,
}

impl<Value> TerraneAsyncMutableState<Value> {
    fn new(value: Value) -> Self {
        Self {
            value: std::sync::Arc::new(tokio::sync::Mutex::new(value)),
        }
    }

    fn share(&self) -> std::sync::Arc<tokio::sync::Mutex<Value>> {
        self.value.clone()
    }
}

impl<Value: Clone> Clone for TerraneAsyncMutableState<Value> {
    fn clone(&self) -> Self {
        let value = self
            .value
            .try_lock()
            .expect("mutable async callable cannot be separated during an active invocation")
            .clone();
        Self::new(value)
    }
}
