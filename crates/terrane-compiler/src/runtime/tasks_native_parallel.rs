#[derive(Clone)]
pub struct TerraneTaskScope {
    cancellation: TerraneCancellation,
    deadline: Option<terrane_int_support::Int>,
}

impl TerraneTaskScope {
    pub fn new(deadline: Option<terrane_int_support::Int>) -> Self {
        Self {
            cancellation: TerraneCancellation::new(),
            deadline,
        }
    }

    pub fn child_scope(&self, requested: &terrane_int_support::Int) -> Self {
        Self {
            cancellation: self.cancellation.clone(),
            deadline: Some(
                self.deadline
                    .as_ref()
                    .map_or_else(|| requested.clone(), |parent| parent.clone().min(requested.clone())),
            ),
        }
    }

    pub fn cancel(&self) {
        self.cancellation.cancel();
    }

    pub fn should_cancel(&self) -> bool {
        self.cancellation.is_cancelled()
            || self.deadline.as_ref().is_some_and(terrane_time_deadline_expired)
    }

    fn cancellation(&self) -> TerraneCancellation {
        self.cancellation.clone()
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

impl<T: Send + 'static> TerraneScopedTask<T> {
    #[allow(dead_code, reason = "task spawn ABI is emitted before usage shaping")]
    fn spawn<F: Future<Output = TerraneTaskResult<T>> + Send + 'static>(work: F) -> Self {
        Self { handle: Some(tokio::spawn(work)) }
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
