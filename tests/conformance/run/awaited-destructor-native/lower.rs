// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: awaited-destructor-native
#[derive(Clone)]
pub struct Resource {
    __terrane_lifetime: std::sync::Arc<()>,
}
impl Resource {
    pub fn terrane_construct() -> Self {
        Self {
            __terrane_lifetime: std::sync::Arc::new(()),
        }
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    #[allow(unused_mut)]
    pub async fn destruct(mut self: &mut Self) {
        __terrane_await(
                terrane_time_sleep_after(
                    __terrane_traced(
                            Duration::terrane_static_milliseconds(
                                terrane_int_support::Int::from(1_i128),
                            ),
                            0 /* terrane-site: case.trn:6:30-6:55 */,
                        )
                        .total_nanoseconds
                        .clone(),
                ),
            )
            .await;
        println!("{}", String::from("destructed"));
    }
}
impl Drop for Resource {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) != 1 {
            return;
        }
        __terrane_await_destructor(async {
            self.destruct().await;
        });
    }
}
async fn release() {
    let value: std::sync::Arc<TerraneSharedCell<Resource>>;
    let owner: std::sync::Arc<TerraneSharedCell<Resource>>;
    let alias: std::sync::Arc<TerraneSharedCell<Resource>>;
    value = std::sync::Arc::new(TerraneSharedCell::new(Resource::terrane_construct()));
    owner = value.clone();
    alias = owner.clone();
    {
        let _: std::sync::Arc<TerraneSharedCell<Resource>> = alias.clone();
    }
    println!("{}", String::from("body"));
}
fn main() {
    __terrane_run(async move {
        __terrane_group_frame(async move {
                let group: TerraneTaskGroup<()>;
                let outcomes: terrane_collection_support::List<TerraneTaskOutcome<()>>;
                group = TerraneTaskGroup::<()>::new();
                {
                    let __terrane_child = {
                        let __terrane_scope = group.scope();
                        let __terrane_cancel = __terrane_scope.cancellation();
                        let __terrane_deadline = __terrane_scope.deadline.clone();
                        let __terrane_spawned_task = release();
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
                    group.spawn(__terrane_child);
                };
                outcomes = __terrane_await(std::boxed::Box::pin(group.prepare_drain()))
                    .await;
                println!(
                    "{}", terrane_scalar_support::scalar_text(&__terrane_raised(outcomes
                    .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
                    1 /* terrane-site: case.trn:20:12-20:23 */)),
                    1 /* terrane-site: case.trn:20:12-20:23 */).completed)
                );
                println!("{}", String::from("after"));
            })
            .await
    });
}
