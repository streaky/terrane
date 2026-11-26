// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: shared-statement-owner
#[derive(Clone)]
pub struct Counter {
    pub value: terrane_int_support::Int,
}
impl Counter {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(0_i128),
        }
    }
}
fn owned_copy(mut item: Counter) -> terrane_int_support::Int {
    let observed: &Counter;
    observed = &item;
    let _ = &observed;
    {
        let __terrane_value: terrane_int_support::Int = terrane_int_support::Int::from(
            41_i128,
        );
        item.value = __terrane_value;
    }
    return item.value.clone();
}
fn assign(
    owner: std::sync::Arc<TerraneSharedCell<Counter>>,
    value: terrane_int_support::Int,
) {
    {
        let __terrane_owner_0 = &owner;
        let __terrane_value: terrane_int_support::Int = &value
            + &terrane_int_support::Int::from(1_i128);
        __terrane_owner_0.lock().expect("shared reference lock poisoned").value = __terrane_value;
    }
}
fn replace(
    owners: &mut terrane_collection_support::List<
        std::sync::Arc<TerraneSharedCell<Counter>>,
    >,
    next: std::sync::Arc<TerraneSharedCell<Counter>>,
) -> terrane_int_support::Int {
    {
        let __terrane_index = __terrane_raised(
            terrane_collection_support::index_from_int(
                &terrane_int_support::Int::from(0_i128),
            ),
            0 /* terrane-site: case.trn:16:5-16:21 */,
        );
        let __terrane_value = next.clone();
        let _ = __terrane_raised(
            owners.set(__terrane_index, __terrane_value),
            0 /* terrane-site: case.trn:16:5-16:21 */,
        );
    }
    return terrane_int_support::Int::from(7_i128);
}
fn touch(owner: std::sync::Arc<TerraneSharedCell<Counter>>) -> terrane_int_support::Int {
    {
        let __terrane_owner_1 = owner.clone();
        let __terrane_value: terrane_int_support::Int = {
            let __terrane_snapshot = owner
                .lock()
                .expect("shared reference lock poisoned")
                .value
                .clone();
            __terrane_snapshot
        } + terrane_int_support::Int::from(1_i128);
        __terrane_owner_1.lock().expect("shared reference lock poisoned").value = __terrane_value;
    }
    return {
        let __terrane_snapshot = owner
            .lock()
            .expect("shared reference lock poisoned")
            .value
            .clone();
        __terrane_snapshot
    };
}
fn consume(
    owner: std::sync::Arc<TerraneSharedCell<Counter>>,
) -> terrane_int_support::Int {
    return {
        let __terrane_snapshot = owner
            .lock()
            .expect("shared reference lock poisoned")
            .value
            .clone();
        __terrane_snapshot
    } + terrane_int_support::Int::from(1_i128);
}
async fn delayed() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(23_i128);
}
fn main() {
    __terrane_run(async move {
        let first: std::sync::Arc<TerraneSharedCell<Counter>>;
        let second: std::sync::Arc<TerraneSharedCell<Counter>>;
        let mut selected: std::sync::Arc<TerraneSharedCell<Counter>>;
        let mut amount: terrane_int_support::Int;
        let simple: std::sync::Arc<TerraneSharedCell<Counter>>;
        let mut owners: terrane_collection_support::List<
            std::sync::Arc<TerraneSharedCell<Counter>>,
        >;
        let moved: std::sync::Arc<TerraneSharedCell<Counter>>;
        let captured: std::sync::Arc<TerraneSharedCell<Counter>>;
        let read: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
        let plain: Counter;
        let probe: std::sync::Arc<TerraneSharedCell<Counter>>;
        let mut index: terrane_int_support::Int;
        first = std::sync::Arc::new(
            TerraneSharedCell::new(Counter::terrane_construct()),
        );
        second = std::sync::Arc::new(
            TerraneSharedCell::new(Counter::terrane_construct()),
        );
        selected = first.clone();
        amount = terrane_int_support::Int::from(2_i128);
        simple = first.clone();
        {
            let __terrane_owner_2 = &simple;
            let __terrane_value: terrane_int_support::Int = &amount
                + &terrane_int_support::Int::from(1_i128);
            __terrane_owner_2.lock().expect("shared reference lock poisoned").value = __terrane_value;
        }
        {
            let __terrane_owner_3 = &simple;
            let __terrane_value: terrane_int_support::Int = &amount
                + &terrane_int_support::Int::from(2_i128);
            __terrane_owner_3.lock().expect("shared reference lock poisoned").value = __terrane_value;
        }
        assign(simple.clone(), terrane_int_support::Int::from(8_i128));
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = first
            .lock().expect("shared reference lock poisoned").value.clone();
            __terrane_snapshot })
        );
        owners = terrane_collection_support::List::<
            std::sync::Arc<TerraneSharedCell<Counter>>,
        >::new(vec![selected.clone()]);
        {
            let __terrane_owner_4 = __terrane_raised(
                    owners
                        .get_or_error(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(
                                    &terrane_int_support::Int::from(0_i128),
                                ),
                                1 /* terrane-site: case.trn:41:5-41:14 */,
                            ),
                        ),
                    1 /* terrane-site: case.trn:41:5-41:14 */,
                )
                .clone();
            let __terrane_value: terrane_int_support::Int = replace(
                &mut owners,
                second.clone(),
            );
            __terrane_owner_4.lock().expect("shared reference lock poisoned").value = __terrane_value;
        }
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot =
            first.lock().expect("shared reference lock poisoned").value.clone();
            __terrane_snapshot }), String::from(" "),
            terrane_scalar_support::scalar_text(&{ let __terrane_snapshot =
            __terrane_raised(owners
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            2 /* terrane-site: case.trn:42:30-42:39 */)), 2 /* terrane-site: case.trn:42:30-42:39 */).lock().expect("shared reference lock poisoned")
            .value.clone(); __terrane_snapshot })
        );
        selected = __terrane_raised(
                owners
                    .get_or_error(
                        __terrane_raised(
                            terrane_collection_support::index_from_int(
                                &terrane_int_support::Int::from(0_i128),
                            ),
                            3 /* terrane-site: case.trn:43:16-43:25 */,
                        ),
                    ),
                3 /* terrane-site: case.trn:43:16-43:25 */,
            )
            .clone();
        {
            let __terrane_owner_5 = selected.clone();
            let __terrane_value: terrane_int_support::Int = touch(first.clone());
            __terrane_owner_5.lock().expect("shared reference lock poisoned").value = __terrane_value;
        }
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot =
            first.lock().expect("shared reference lock poisoned").value.clone();
            __terrane_snapshot }), String::from(" "),
            terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = selected
            .lock().expect("shared reference lock poisoned").value.clone();
            __terrane_snapshot })
        );
        moved = first.clone();
        {
            let __terrane_owner_6 = moved.clone();
            let __terrane_value: terrane_int_support::Int = consume(moved);
            __terrane_owner_6.lock().expect("shared reference lock poisoned").value = __terrane_value;
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = first
            .lock().expect("shared reference lock poisoned").value.clone();
            __terrane_snapshot })
        );
        {
            let __terrane_owner_7 = selected.clone();
            let __terrane_value: terrane_int_support::Int = __terrane_await(delayed())
                .await;
            __terrane_owner_7.lock().expect("shared reference lock poisoned").value = __terrane_value;
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot =
            selected.lock().expect("shared reference lock poisoned").value.clone();
            __terrane_snapshot })
        );
        captured = std::sync::Arc::new(
            TerraneSharedCell::new(Counter::terrane_construct()),
        );
        read = {
            let captured = captured.clone();
            std::sync::Arc::new(move || -> terrane_int_support::Int {
                return {
                    let __terrane_snapshot = captured
                        .lock()
                        .expect("shared reference lock poisoned")
                        .value
                        .clone();
                    __terrane_snapshot
                };
            })
        };
        {
            let __terrane_owner_8 = captured.clone();
            let __terrane_value: terrane_int_support::Int = terrane_int_support::Int::from(
                31_i128,
            );
            __terrane_owner_8.lock().expect("shared reference lock poisoned").value = __terrane_value;
        }
        println!("{}", terrane_scalar_support::scalar_text(&read()));
        plain = Counter::terrane_construct();
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&owned_copy(plain.clone())),
            String::from(" "), terrane_scalar_support::scalar_text(&plain.value)
        );
        probe = std::sync::Arc::new(
            TerraneSharedCell::new(Counter::terrane_construct()),
        );
        amount = &amount + &terrane_int_support::Int::from(1_i128);
        {
            let __terrane_owner_9 = &probe;
            let __terrane_value: terrane_int_support::Int = &amount
                + &terrane_int_support::Int::from(1_i128);
            __terrane_owner_9.lock().expect("shared reference lock poisoned").value = __terrane_value;
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = probe
            .lock().expect("shared reference lock poisoned").value.clone();
            __terrane_snapshot })
        );
        index = terrane_int_support::Int::from(0_i128);
        while index < terrane_int_support::Int::from(3_i128) {
            {
                let __terrane_owner_10 = &probe;
                let __terrane_value: terrane_int_support::Int = &index
                    + &terrane_int_support::Int::from(1_i128);
                __terrane_owner_10
                    .lock()
                    .expect("shared reference lock poisoned")
                    .value = __terrane_value;
            }
            index = &index + &terrane_int_support::Int::from(1_i128);
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = probe
            .lock().expect("shared reference lock poisoned").value.clone();
            __terrane_snapshot })
        );
    });
}
