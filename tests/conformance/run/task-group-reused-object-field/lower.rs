// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: task-group-reused-object-field
#[derive(Clone)]
pub struct Payload {
    pub value: terrane_int_support::Int,
}
impl Payload {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(7_i128),
        }
    }
}
#[derive(Clone)]
pub struct Holder {
    pub item: Payload,
}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self {
            item: Payload::terrane_construct(),
        }
    }
}
async fn work(
    mut item: Payload,
    offset: terrane_int_support::Int,
) -> terrane_int_support::Int {
    {
        let __terrane_value = item.value.clone() + offset.clone();
        item.value = __terrane_value;
    }
    return item.value.clone();
}
fn main() {
    __terrane_run(async move {
        __terrane_group_frame(async move {
                let source: Holder;
                let group: TerraneTaskGroup<terrane_int_support::Int>;
                let mut index: terrane_int_support::Int;
                let outcomes: terrane_collection_support::List<
                    TerraneTaskOutcome<terrane_int_support::Int>,
                >;
                let mut total: terrane_int_support::Int;
                let mut outcome: TerraneTaskOutcome<terrane_int_support::Int>;
                let mut value: Option<terrane_int_support::Int>;
                source = Holder::terrane_construct();
                group = TerraneTaskGroup::<terrane_int_support::Int>::new();
                index = terrane_int_support::Int::from(1_i128);
                while index.clone() <= terrane_int_support::Int::from(3_i128) {
                    {
                        let __terrane_child = {
                            let __terrane_scope = group.scope();
                            let __terrane_cancel = __terrane_scope.cancellation();
                            let __terrane_deadline = __terrane_scope.deadline.clone();
                            let __terrane_spawned_task = work(
                                source.item.clone(),
                                index.clone(),
                            );
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
                    index = index.clone() + terrane_int_support::Int::from(1_i128);
                }
                outcomes = __terrane_await(std::boxed::Box::pin(group.prepare_drain()))
                    .await;
                total = terrane_int_support::Int::from(0_i128);
                let __terrane_iterable_0 = outcomes;
                let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
                    &__terrane_iterable_0,
                );
                loop {
                    outcome = match __terrane_iterator_0.next() {
                        terrane_collection_support::IterationStep::Item(item) => item,
                        terrane_collection_support::IterationStep::End => break,
                    };
                    value = outcome.value.clone();
                    if value.is_some() {
                        total = total.clone()
                            + match &value {
                                Some(value) => value,
                                _ => unreachable!("flow-proven storage refinement"),
                            }
                                .clone();
                    }
                }
                println!(
                    "{}", terrane_scalar_support::scalar_text(&(total.clone() ==
                    terrane_int_support::Int::from(27_i128)))
                );
                println!(
                    "{}", terrane_scalar_support::scalar_text(&(source.item.value.clone()
                    == terrane_int_support::Int::from(7_i128)))
                );
            })
            .await
    });
}
