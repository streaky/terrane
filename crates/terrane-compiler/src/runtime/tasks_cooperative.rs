#[derive(Clone)]
pub struct TerraneTaskScope {
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    deadline: Option<terrane_int_support::Int>,
}
impl TerraneTaskScope {
    pub fn new(deadline: Option<terrane_int_support::Int>) -> Self {
        Self {
            cancelled: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            deadline,
        }
    }
    pub fn child_scope(&self, requested: &terrane_int_support::Int) -> Self {
        let deadline = Some(self.deadline.as_ref().map_or_else(
            || requested.clone(),
            |parent| parent.clone().min(requested.clone()),
        ));
        Self {
            cancelled: self.cancelled.clone(),
            deadline,
        }
    }
    pub fn cancel(&self) {
        self.cancelled
            .store(true, std::sync::atomic::Ordering::Release);
    }
    pub fn should_cancel(&self) -> bool {
        self.cancelled.load(std::sync::atomic::Ordering::Acquire)
            || self
                .deadline
                .as_ref()
                .is_some_and(terrane_time_deadline_expired)
    }
    pub fn join<T>(&self, mut task: TerraneScopedTask<T>) -> TerraneTaskOutcome<T> {
        let result = task.result.take().expect("scoped task joined once");
        match result {
            TerraneTaskResult::Completed(value) => TerraneTaskOutcome {
                completed: true,
                cancelled: self.should_cancel(),
                value: Some(value),
                error: None,
            },
            TerraneTaskResult::Failed(error) => {
                self.cancel();
                TerraneTaskOutcome {
                    completed: false,
                    cancelled: false,
                    value: None,
                    error: Some(error),
                }
            }
            TerraneTaskResult::Cancelled => TerraneTaskOutcome {
                completed: false,
                cancelled: true,
                value: None,
                error: None,
            },
        }
    }
}
#[allow(
    dead_code,
    reason = "task result ABI is emitted before per-variant usage shaping"
)]
enum TerraneTaskResult<T> {
    Completed(T),
    Failed(TerraneError),
    Cancelled,
}
pub struct TerraneScopedTask<T> {
    result: Option<TerraneTaskResult<T>>,
}
impl<T> TerraneScopedTask<T> {
    #[allow(dead_code, reason = "task spawn ABI is emitted before usage shaping")]
    fn spawn<F: FnOnce() -> TerraneTaskResult<T>>(work: F) -> Self {
        Self {
            result: Some(work()),
        }
    }
}

#[derive(Clone)]
pub struct TerraneTaskOutcome<T> {
    pub completed: bool,
    pub cancelled: bool,
    pub value: Option<T>,
    pub error: Option<TerraneError>,
}

pub struct TerraneTaskGroup<T> {
    scope: TerraneTaskScope,
    tasks: parking_lot::Mutex<Vec<TerraneScopedTask<T>>>,
}
impl<T> TerraneTaskGroup<T> {
    pub fn new() -> Self {
        Self {
            scope: TerraneTaskScope::new(None),
            tasks: parking_lot::Mutex::new(Vec::new()),
        }
    }
    pub fn scope(&self) -> TerraneTaskScope {
        self.scope.clone()
    }
    pub fn cancel(&self) {
        self.scope.cancel();
    }
    pub fn spawn(&self, task: TerraneScopedTask<T>) {
        self.tasks.lock().push(task);
    }
    pub fn prepare_drain(
        self,
    ) -> impl Future<Output = terrane_collection_support::List<TerraneTaskOutcome<T>>> {
        async move { terrane_collection_support::List::new(self.drain().await) }
    }
    pub async fn drain(self) -> Vec<TerraneTaskOutcome<T>> {
        let tasks = std::mem::take(&mut *self.tasks.lock());
        tasks
            .into_iter()
            .map(|task| self.scope.join(task))
            .collect()
    }
}
impl<T> Drop for TerraneTaskGroup<T> {
    fn drop(&mut self) {
        self.scope.cancel();
        if let Ok(tasks) = self.tasks.get_mut() {
            tasks.clear();
        }
    }
}
