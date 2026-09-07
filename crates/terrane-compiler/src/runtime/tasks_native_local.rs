#[derive(Clone)]
pub struct TerraneTaskScope {
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    deadline: Option<std::time::Instant>,
}

impl TerraneTaskScope {
    pub fn new(deadline_ms: Option<u64>) -> Self {
        Self {
            cancelled: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            deadline: deadline_ms
                .map(|milliseconds| std::time::Instant::now() + std::time::Duration::from_millis(milliseconds)),
        }
    }

    pub fn child_scope(&self, deadline_ms: u64) -> Self {
        let requested = std::time::Instant::now() + std::time::Duration::from_millis(deadline_ms);
        Self {
            cancelled: self.cancelled.clone(),
            deadline: Some(self.deadline.map_or(requested, |parent| parent.min(requested))),
        }
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
    }

    pub fn should_cancel(&self) -> bool {
        self.cancelled.load(std::sync::atomic::Ordering::Acquire)
            || self.deadline.is_some_and(|deadline| std::time::Instant::now() >= deadline)
    }

    pub async fn join<T>(&self, mut task: TerraneScopedTask<T>) -> TerraneTaskOutcome<T> {
        let result = task.handle.take().expect("scoped task joined once").await
            .expect("scoped task must not panic outside its Terrane boundary");
        outcome_from_result(self, result)
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
    handle: Option<tokio::task::JoinHandle<TerraneTaskResult<T>>>,
}

impl<T: 'static> TerraneScopedTask<T> {
    #[allow(dead_code, reason = "task spawn ABI is emitted before usage shaping")]
    fn spawn<F: Future<Output = TerraneTaskResult<T>> + 'static>(work: F) -> Self {
        Self { handle: Some(tokio::task::spawn_local(work)) }
    }
}

fn outcome_from_result<T>(scope: &TerraneTaskScope, result: TerraneTaskResult<T>) -> TerraneTaskOutcome<T> {
    match result {
        TerraneTaskResult::Completed(value) => TerraneTaskOutcome {
            completed: true,
            cancelled: scope.should_cancel(),
            value: Some(value),
            error: None,
        },
        TerraneTaskResult::Failed(error) => {
            scope.cancel();
            TerraneTaskOutcome { completed: false, cancelled: false, value: None, error: Some(error) }
        }
        TerraneTaskResult::Cancelled => TerraneTaskOutcome {
            completed: false,
            cancelled: true,
            value: None,
            error: None,
        },
    }
}

pub struct TerraneTaskOutcome<T> {
    pub completed: bool,
    pub cancelled: bool,
    pub value: Option<T>,
    pub error: Option<TerraneError>,
}
