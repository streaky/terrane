pub struct TerraneAsyncMutableState<Value> {
    value: std::sync::Arc<std::sync::Mutex<Value>>,
}

impl<Value> TerraneAsyncMutableState<Value> {
    fn new(value: Value) -> Self {
        Self {
            value: std::sync::Arc::new(std::sync::Mutex::new(value)),
        }
    }

    fn share(&self) -> std::sync::Arc<std::sync::Mutex<Value>> {
        self.value.clone()
    }
}

impl<Value: Clone> Clone for TerraneAsyncMutableState<Value> {
    fn clone(&self) -> Self {
        let value = self
            .value
            .lock()
            .expect("mutable async callable state lock poisoned")
            .clone();
        Self::new(value)
    }
}
