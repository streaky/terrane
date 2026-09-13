// Generated deterministically by Terrane <version>.
// Runtime support: async_native.rs, executor_parallel.rs, tasks_native_parallel.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: task-outcome-class-presence
#[derive(Clone)]
pub struct Point {
    pub x: terrane_int_support::Int,
}
impl Point {
    pub fn terrane_construct(x: terrane_int_support::Int) -> Self {
        let mut value = Self {
            x: terrane_int_support::Int::from(0_i128),
        };
        value.construct(x);
        value
    }
    pub fn construct(&mut self, x: terrane_int_support::Int) {
        self.x = x.clone();
    }
}
async fn make() -> Point {
    return Point::terrane_construct(terrane_int_support::Int::from(5_i128));
}
fn main() {
    __terrane_run(async move {
        let scope: TerraneTaskScope = TerraneTaskScope::new(None);
        let child: TerraneScopedTask<Point> = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        std::sync::Arc::new(move || -> std::pin::Pin<
                            Box<dyn Future<Output = _> + Send>,
                        > { Box::pin(make()) })(),
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
        let outcome: TerraneTaskOutcome<Point> = __terrane_await(scope.join(child))
            .await;
        let value: Option<Point> = outcome.value.clone();
        if value.is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&String::from("present"))
            );
        }
        if value.is_none() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&String::from("missing"))
            );
        }
    });
}
