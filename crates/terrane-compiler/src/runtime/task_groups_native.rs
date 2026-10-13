use parking_lot::Mutex;

struct TerraneNativeTaskGroupState<T> {
    scope: TerraneTaskScope,
    tasks: Vec<TerraneScopedTask<T>>,
}

struct TerraneNativeTaskGroup<T> {
    state: Mutex<TerraneNativeTaskGroupState<T>>,
}

impl<T: Send + 'static> TerraneTaskCleanup for TerraneNativeTaskGroup<T> {
    fn cancel(&self) {
        self.state.lock().scope.cancel();
    }
    fn cleanup(
        self: std::sync::Arc<Self>,
    ) -> std::pin::Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
        Box::pin(async move {
            let (scope, tasks) = {
                let mut state = self.state.lock();
                state.scope.cancel();
                (state.scope.clone(), std::mem::take(&mut state.tasks))
            };
            for task in tasks {
                let _ = scope.join(task).await;
            }
        })
    }
}

pub struct TerraneTaskGroup<T: Send + 'static> {
    state: std::sync::Arc<TerraneNativeTaskGroup<T>>,
    drained: bool,
}

impl<T: Send + 'static> TerraneTaskGroup<T> {
    pub fn new() -> Self {
        let scope = TerraneTaskScope::new(None);
        let state = std::sync::Arc::new(TerraneNativeTaskGroup {
            state: Mutex::new(TerraneNativeTaskGroupState {
                scope,
                tasks: Vec::new(),
            }),
        });
        TERRANE_CANCELLATION_CONTEXT
            .try_with(|context| {
                context
                    .finalizers
                    .register_task_group(std::sync::Arc::downgrade(&state));
            })
            .ok();
        Self {
            state,
            drained: false,
        }
    }

    pub fn scope(&self) -> TerraneTaskScope {
        self.state.state.lock().scope.clone()
    }

    pub fn cancel(&self) {
        self.state.state.lock().scope.cancel();
    }

    pub fn spawn(&self, task: TerraneScopedTask<T>) {
        assert!(!self.drained, "cannot spawn into a drained task group");
        self.state.state.lock().tasks.push(task);
    }

    pub fn prepare_drain(
        self,
    ) -> impl Future<Output = terrane_collection_support::List<TerraneTaskOutcome<T>>> {
        let guard = __terrane_finally_guard();
        async move {
            let output = self.drain().await;
            drop(guard);
            terrane_collection_support::List::new(output)
        }
    }
    pub async fn drain(mut self) -> Vec<TerraneTaskOutcome<T>> {
        assert!(!self.drained, "task group drained once");
        self.drained = true;
        let (scope, tasks) = {
            let mut state = self.state.state.lock();
            (state.scope.clone(), std::mem::take(&mut state.tasks))
        };
        let mut outcomes = Vec::with_capacity(tasks.len());
        for task in tasks {
            outcomes.push(scope.join(task).await);
        }
        outcomes
    }
}

impl<T: Send + 'static> Drop for TerraneTaskGroup<T> {
    fn drop(&mut self) {
        if self.drained {
            return;
        }
        self.state.state.lock().scope.cancel();
        if TERRANE_CANCELLATION_CONTEXT
            .try_with(|context| {
                context.finalizers.retain_task_group(self.state.clone());
            })
            .is_ok()
        {
            return;
        }
        // Foreign callers without an async cancellation context cannot join here.
        // Cancel cooperatively before aborting retained handles as a last resort.
        let mut state = self.state.state.lock();
        state.scope.cancel();
        for task in state.tasks.drain(..) {
            if let Some(handle) = task.handle {
                handle.abort();
            }
        }
    }
}

#[allow(dead_code, reason = "group frame support is emitted with task scopes")]
async fn __terrane_group_frame<F: Future>(future: F) -> F::Output {
    if TERRANE_CANCELLATION_CONTEXT.try_with(|_| ()).is_ok() {
        future.await
    } else {
        __terrane_async_entry(future).await
    }
}
