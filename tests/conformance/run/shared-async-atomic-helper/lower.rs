// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: shared-async-atomic-helper
#[derive(Clone)]
pub struct Accumulator {
    pub total: terrane_int_support::Int,
}
impl Accumulator {
    pub fn terrane_construct() -> Self {
        Self {
            total: terrane_int_support::Int::from(0_i128),
        }
    }
    pub fn increment(&mut self) {
        {
            let __terrane_value = self.total.clone()
                + terrane_int_support::Int::from(1_i128);
            self.total = __terrane_value;
        }
    }
    pub async fn add_many(&mut self, count: terrane_int_support::Int) {
        let mut i: terrane_int_support::Int;
        i = terrane_int_support::Int::from(0_i128);
        '__terrane_break_0: while i.clone() < count.clone() {
            '__terrane_continue_0: {
                self.increment();
            }
            i = i.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    pub async fn __terrane_shared_async_add_many(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
        count: terrane_int_support::Int,
    ) {
        let mut i: terrane_int_support::Int;
        i = terrane_int_support::Int::from(0_i128);
        '__terrane_break_1: while i.clone() < count.clone() {
            '__terrane_continue_1: {
                {
                    let __terrane_call_result = __terrane_shared_handle
                        .lock()
                        .expect("shared async receiver lock poisoned")
                        .increment();
                    __terrane_call_result
                };
            }
            i = i.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
}
fn main() {
    __terrane_run(async move {
        let value: std::sync::Arc<TerraneSharedCell<Accumulator>>;
        let owner: std::sync::Arc<TerraneSharedCell<Accumulator>>;
        let alias: std::sync::Arc<TerraneSharedCell<Accumulator>>;
        let scope: TerraneTaskScope;
        let first: TerraneScopedTask<()>;
        let second: TerraneScopedTask<()>;
        let first_result: TerraneTaskOutcome<()>;
        let second_result: TerraneTaskOutcome<()>;
        value = std::sync::Arc::new(
            TerraneSharedCell::new(Accumulator::terrane_construct()),
        );
        owner = value.clone();
        alias = owner.clone();
        scope = TerraneTaskScope::new(None);
        first = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = {
                let __terrane_shared_call_handle = owner.clone();
                let __terrane_shared_argument_0 = terrane_int_support::Int::from(
                    20000_i128,
                );
                let __terrane_shared_argument_1 = __terrane_shared_argument_0;
                Accumulator::__terrane_shared_async_add_many(
                    __terrane_shared_call_handle,
                    __terrane_shared_argument_1,
                )
            };
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        __terrane_spawned_task,
                        __terrane_cancel,
                        __terrane_deadline,
                    )
                    .await
                {
                    Some(value) => TerraneTaskResult::Completed(value),
                    None => TerraneTaskResult::Cancelled,
                }
            })
        };
        second = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = {
                let __terrane_shared_call_handle = alias.clone();
                let __terrane_shared_argument_0 = terrane_int_support::Int::from(
                    20000_i128,
                );
                let __terrane_shared_argument_1 = __terrane_shared_argument_0;
                Accumulator::__terrane_shared_async_add_many(
                    __terrane_shared_call_handle,
                    __terrane_shared_argument_1,
                )
            };
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        __terrane_spawned_task,
                        __terrane_cancel,
                        __terrane_deadline,
                    )
                    .await
                {
                    Some(value) => TerraneTaskResult::Completed(value),
                    None => TerraneTaskResult::Cancelled,
                }
            })
        };
        first_result = __terrane_await(scope.join(first)).await;
        second_result = __terrane_await(scope.join(second)).await;
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&first_result.completed),
            terrane_scalar_support::scalar_text(&second_result.completed),
            terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = owner.lock()
            .expect("shared reference lock poisoned").total.clone(); __terrane_snapshot
            })
        );
    });
}
