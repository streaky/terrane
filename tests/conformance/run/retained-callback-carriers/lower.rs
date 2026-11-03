// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
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
#[derive(Clone)]
pub struct OffsetSink {}
impl OffsetSink {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn accept(&self, incoming: &Event) -> terrane_int_support::Int {
        return {
            let __terrane_snapshot = terrane_int_support::Int::from(
                    i128::from(*&incoming.value),
                )
                .clone();
            __terrane_snapshot
        } + terrane_int_support::Int::from(10_i128);
    }
}
impl SinkProtocol for OffsetSink {
    fn clone_box(&self) -> Box<dyn SinkProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn SinkProtocol> {
        Box::new(self.clone())
    }
    fn accept(&self, event: &Event) -> terrane_int_support::Int {
        OffsetSink::accept(&*self, event)
    }
}
impl From<OffsetSink> for Sink {
    fn from(value: OffsetSink) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_retained_callback_api::Sink for OffsetSink {
    fn accept(&self, event: &terrane_retained_callback_api::Event) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <OffsetSink>::accept(&*self, event);
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
fn named_event(incoming: &Event) {
    println!("{}", terrane_scalar_support::scalar_text(&(* incoming).value));
}
fn add_tracked(
    listeners: &Registry,
    state: std::sync::Arc<TerraneSharedCell<Counter>>,
) -> std::sync::Weak<
    dyn terrane_retained_callback_api::EventListener<
        terrane_retained_callback_api::Event,
    > + core::marker::Send + core::marker::Sync,
> {
    let owned: Signal;
    let callback: std::sync::Arc<dyn Fn(&Event) -> () + Send + Sync>;
    owned = __terrane_raised(
        terrane_static_trn_5369676e616c_new(),
        0 /* terrane-site: src/main.trn:21:13-21:25 */,
    );
    callback = {
        let owned = owned;
        let state = state.clone();
        std::sync::Arc::new(move |incoming: &Event| -> () {
            {
                let __terrane_owner_0 = state.clone();
                let __terrane_value = {
                    let __terrane_snapshot = state
                        .lock()
                        .expect("shared reference lock poisoned")
                        .value
                        .clone();
                    __terrane_snapshot
                }
                    + {
                        let __terrane_snapshot = terrane_int_support::Int::from(
                                i128::from(*&incoming.value),
                            )
                            .clone();
                        __terrane_snapshot
                    }
                    + __terrane_raised(
                        || -> Result<_, crate::TerraneForeignError> {
                            {
                                let value = match std::panic::catch_unwind(
                                    std::panic::AssertUnwindSafe(|| -> Result<
                                        _,
                                        crate::TerraneForeignError,
                                    > { Ok((&owned).inspect()) }),
                                ) {
                                    Ok(result) => result?,
                                    Err(payload) => {
                                        return Err(
                                            crate::__terrane_dependency_panic(
                                                payload,
                                                "terrane_retained_callback_api",
                                                "terrane_retained_callback_api::Signal::inspect",
                                            ),
                                        );
                                    }
                                };
                                Ok::<
                                    _,
                                    crate::TerraneForeignError,
                                >(terrane_int_support::Int::from(i128::from(value)))
                            }
                        }(),
                        1 /* terrane-site: src/main.trn:23:55-23:69 */,
                    );
                __terrane_owner_0
                    .lock()
                    .expect("shared reference lock poisoned")
                    .value = __terrane_value;
            }
            ()
        })
    };
    return {
        let __terrane_conversion_input_0 = callback.clone();
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                (&*listeners)
                                    .register(
                                        std::sync::Arc::new({
                                            let callback = __terrane_conversion_input_0;
                                            move |
                                                callback_argument_0: &terrane_retained_callback_api::Event|
                                            {
                                                match || -> Result<_, crate::TerraneForeignError> {
                                                    let callback_value = callback(&*callback_argument_0);
                                                    Ok(callback_value)
                                                }() {
                                                    Ok(value) => value,
                                                    Err(error) => std::panic::panic_any(error.0),
                                                }
                                            }
                                        }),
                                    ),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_retained_callback_registry",
                                    "terrane_retained_callback_registry::Registry::register",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),
            2 /* terrane-site: src/main.trn:24:12-24:40 */,
        )
    };
}
fn forget_token(listeners: &Registry) {
    {
        let __terrane_conversion_input_0 = std::sync::Arc::new(named_event);
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let _ = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok({
                                let _ = (&*listeners)
                                    .register(
                                        std::sync::Arc::new({
                                            let callback = __terrane_conversion_input_0;
                                            move |
                                                callback_argument_0: &terrane_retained_callback_api::Event|
                                            {
                                                match || -> Result<_, crate::TerraneForeignError> {
                                                    let callback_value = callback(&*callback_argument_0);
                                                    Ok(callback_value)
                                                }() {
                                                    Ok(value) => value,
                                                    Err(error) => std::panic::panic_any(error.0),
                                                }
                                            }
                                        }),
                                    );
                            })
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_retained_callback_registry",
                                    "terrane_retained_callback_registry::Registry::register",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(())
                }
            }(),
            3 /* terrane-site: src/main.trn:27:5-27:36 */,
        )
    };
}
fn main() {
    let state: std::sync::Arc<TerraneSharedCell<Counter>>;
    let callback: std::sync::Arc<dyn Fn(&Event) -> () + Send + Sync>;
    let listeners: Registry;
    let token: std::sync::Weak<
        dyn terrane_retained_callback_api::EventListener<
            terrane_retained_callback_api::Event,
        > + core::marker::Send + core::marker::Sync,
    >;
    let copied_token: std::sync::Weak<
        dyn terrane_retained_callback_api::EventListener<
            terrane_retained_callback_api::Event,
        > + core::marker::Send + core::marker::Sync,
    >;
    let unrelated_owner: Registry;
    let unrelated: std::sync::Weak<
        dyn terrane_retained_callback_api::EventListener<
            terrane_retained_callback_api::Event,
        > + core::marker::Send + core::marker::Sync,
    >;
    let tracked: std::sync::Weak<
        dyn terrane_retained_callback_api::EventListener<
            terrane_retained_callback_api::Event,
        > + core::marker::Send + core::marker::Sync,
    >;
    let weak_owner: WeakRegistry;
    let reentrant: std::sync::Arc<dyn Fn(&Event) -> () + Send + Sync>;
    let reentrant_token: std::sync::Weak<
        dyn terrane_retained_callback_api::EventListener<
            terrane_retained_callback_api::Event,
        > + core::marker::Send + core::marker::Sync,
    >;
    let destroyed: std::sync::Weak<
        dyn terrane_retained_callback_api::EventListener<
            terrane_retained_callback_api::Event,
        > + core::marker::Send + core::marker::Sync,
    >;
    let changes: terrane_int_support::Int;
    let change: TerraneOwnedMutableInputsM<MutableEvent, ()>;
    let copied_change: TerraneOwnedMutableInputsM<MutableEvent, ()>;
    let mut changing_event: MutableEvent;
    let send_only_state: SendOnly;
    let send_only_callback: TerraneOwnedMutableInputsM<MutableEvent, ()>;
    let finish: TerraneConsumingCallable<(Event,), ()>;
    let erased: Sink;
    let finish_borrowed: TerraneOwnedConsumingInputsR<Event, ()>;
    let text_callback: TerraneOwnedMutableInputsR<String, ()>;
    let number_callback: TerraneOwnedMutableInputsM<i64, ()>;
    let saved: Option<Event>;
    let snapshotter: TerraneOwnedMutableInputsR<Event, ()>;
    let snapshot_event: Event;
    state = std::sync::Arc::new(TerraneSharedCell::new(Counter::terrane_construct()));
    callback = {
        let state = state.clone();
        std::sync::Arc::new(move |incoming: &Event| -> () {
            {
                let __terrane_owner_1 = state.clone();
                let __terrane_value = {
                    let __terrane_snapshot = state
                        .lock()
                        .expect("shared reference lock poisoned")
                        .value
                        .clone();
                    __terrane_snapshot
                }
                    + {
                        let __terrane_snapshot = terrane_int_support::Int::from(
                                i128::from(*&incoming.value),
                            )
                            .clone();
                        __terrane_snapshot
                    };
                __terrane_owner_1
                    .lock()
                    .expect("shared reference lock poisoned")
                    .value = __terrane_value;
            }
            ()
        })
    };
    listeners = __terrane_raised(
        terrane_static_trn_5265676973747279_new(),
        4 /* terrane-site: src/main.trn:33:17-33:31 */,
    );
    token = {
        let __terrane_conversion_input_0 = callback.clone();
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                (&listeners)
                                    .register(
                                        std::sync::Arc::new({
                                            let callback = __terrane_conversion_input_0;
                                            move |
                                                callback_argument_0: &terrane_retained_callback_api::Event|
                                            {
                                                match || -> Result<_, crate::TerraneForeignError> {
                                                    let callback_value = callback(&*callback_argument_0);
                                                    Ok(callback_value)
                                                }() {
                                                    Ok(value) => value,
                                                    Err(error) => std::panic::panic_any(error.0),
                                                }
                                            }
                                        }),
                                    ),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_retained_callback_registry",
                                    "terrane_retained_callback_registry::Registry::register",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),
            5 /* terrane-site: src/main.trn:34:13-34:41 */,
        )
    };
    copied_token = __terrane_raised(
        terrane_static_trn_5265676973747279_clone_token(&token),
        6 /* terrane-site: src/main.trn:35:20-35:48 */,
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_static_trn_5265676973747279_same_token(&token,
        &copied_token), 7 /* terrane-site: src/main.trn:36:13-36:54 */))
    );
    unrelated_owner = __terrane_raised(
        terrane_static_trn_5265676973747279_new(),
        8 /* terrane-site: src/main.trn:37:23-37:37 */,
    );
    unrelated = {
        let __terrane_conversion_input_0 = std::sync::Arc::new(named_event);
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                (&unrelated_owner)
                                    .register(
                                        std::sync::Arc::new({
                                            let callback = __terrane_conversion_input_0;
                                            move |
                                                callback_argument_0: &terrane_retained_callback_api::Event|
                                            {
                                                match || -> Result<_, crate::TerraneForeignError> {
                                                    let callback_value = callback(&*callback_argument_0);
                                                    Ok(callback_value)
                                                }() {
                                                    Ok(value) => value,
                                                    Err(error) => std::panic::panic_any(error.0),
                                                }
                                            }
                                        }),
                                    ),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_retained_callback_registry",
                                    "terrane_retained_callback_registry::Registry::register",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),
            9 /* terrane-site: src/main.trn:38:17-38:54 */,
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&listeners).remove(&unrelated)) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_retained_callback_registry",
        "terrane_retained_callback_registry::Registry::remove")) }; Ok:: < _, crate
        ::TerraneForeignError > (value) } } (), 10 /* terrane-site: src/main.trn:39:13-39:40 */))
    );
    __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let _ = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok({
                            let _ = (&listeners)
                                .emit(
                                    &{
                                        let __terrane_conversion_input_0 = terrane_int_support::Int::from(
                                            3_i128,
                                        );
                                        __terrane_raised(
                                            || -> Result<_, crate::TerraneForeignError> {
                                                Ok(terrane_retained_callback_api::Event {
                                                    value: terrane_int_support::coerce::<
                                                        i64,
                                                    >(&__terrane_conversion_input_0)
                                                        .map_err(|error| crate::TerraneForeignError(
                                                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                                        ))?,
                                                })
                                            }(),
                                            11 /* terrane-site: src/main.trn:40:22-40:47 */,
                                        )
                                    },
                                );
                        })
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane_retained_callback_registry",
                                "terrane_retained_callback_registry::Registry::emit",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),
        12 /* terrane-site: src/main.trn:40:5-40:48 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = state
        .lock().expect("shared reference lock poisoned").value.clone();
        __terrane_snapshot })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(event_clones(),
        13 /* terrane-site: src/main.trn:42:13-42:26 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&listeners).remove(&copied_token)) })) { Ok(result)
        => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane_retained_callback_registry",
        "terrane_retained_callback_registry::Registry::remove")) }; Ok:: < _, crate
        ::TerraneForeignError > (value) } } (), 14 /* terrane-site: src/main.trn:43:13-43:43 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&listeners).remove(&token)) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_retained_callback_registry",
        "terrane_retained_callback_registry::Registry::remove")) }; Ok:: < _, crate
        ::TerraneForeignError > (value) } } (), 15 /* terrane-site: src/main.trn:44:13-44:36 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_static_trn_5265676973747279_token_alive(&token),
        16 /* terrane-site: src/main.trn:45:13-45:41 */))
    );
    __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let _ = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok({
                            let _ = (&listeners)
                                .emit(
                                    &{
                                        let __terrane_conversion_input_0 = terrane_int_support::Int::from(
                                            4_i128,
                                        );
                                        __terrane_raised(
                                            || -> Result<_, crate::TerraneForeignError> {
                                                Ok(terrane_retained_callback_api::Event {
                                                    value: terrane_int_support::coerce::<
                                                        i64,
                                                    >(&__terrane_conversion_input_0)
                                                        .map_err(|error| crate::TerraneForeignError(
                                                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                                        ))?,
                                                })
                                            }(),
                                            17 /* terrane-site: src/main.trn:46:22-46:47 */,
                                        )
                                    },
                                );
                        })
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane_retained_callback_registry",
                                "terrane_retained_callback_registry::Registry::emit",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),
        18 /* terrane-site: src/main.trn:46:5-46:48 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = state
        .lock().expect("shared reference lock poisoned").value.clone();
        __terrane_snapshot })
    );
    tracked = add_tracked(&listeners, state.clone());
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_static_trn_5369676e616c_drops(),
        19 /* terrane-site: src/main.trn:50:13-50:27 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&listeners)
        .snapshot_then_remove_and_deliver(&tracked, &{ let __terrane_conversion_input_0 =
        terrane_int_support::Int::from(5_i128); __terrane_raised(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(terrane_retained_callback_api::Event { value :
        terrane_int_support::coerce:: < i64 > (&__terrane_conversion_input_0).map_err(|
        error | crate ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ? }) } (), 20 /* terrane-site: src/main.trn:51:67-51:92 */) })) })) { Ok(result) => result ?, Err(payload) =>
        return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_retained_callback_registry",
        "terrane_retained_callback_registry::Registry::snapshot_then_remove_and_deliver"))
        }; Ok:: < _, crate ::TerraneForeignError > (value) } } (), 21 /* terrane-site: src/main.trn:51:13-51:93 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = state
        .lock().expect("shared reference lock poisoned").value.clone();
        __terrane_snapshot })
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_static_trn_5369676e616c_drops(),
        22 /* terrane-site: src/main.trn:53:13-53:27 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_static_trn_5265676973747279_token_alive(&tracked),
        23 /* terrane-site: src/main.trn:54:13-54:43 */))
    );
    forget_token(&listeners);
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&listeners).count()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_retained_callback_registry",
        "terrane_retained_callback_registry::Registry::count")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 24 /* terrane-site: src/main.trn:57:13-57:29 */))
    );
    __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let _ = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok({
                            let _ = (&listeners)
                                .emit(
                                    &{
                                        let __terrane_conversion_input_0 = terrane_int_support::Int::from(
                                            7_i128,
                                        );
                                        __terrane_raised(
                                            || -> Result<_, crate::TerraneForeignError> {
                                                Ok(terrane_retained_callback_api::Event {
                                                    value: terrane_int_support::coerce::<
                                                        i64,
                                                    >(&__terrane_conversion_input_0)
                                                        .map_err(|error| crate::TerraneForeignError(
                                                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                                        ))?,
                                                })
                                            }(),
                                            25 /* terrane-site: src/main.trn:58:22-58:47 */,
                                        )
                                    },
                                );
                        })
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane_retained_callback_registry",
                                "terrane_retained_callback_registry::Registry::emit",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),
        26 /* terrane-site: src/main.trn:58:5-58:48 */,
    );
    weak_owner = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&listeners).weak_owner()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane_retained_callback_registry",
                                "terrane_retained_callback_registry::Registry::weak_owner",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        27 /* terrane-site: src/main.trn:59:18-59:39 */,
    );
    reentrant = {
        let state = state.clone();
        let weak_owner = weak_owner.clone();
        std::sync::Arc::new(move |incoming: &Event| -> () {
            let _ = &incoming;
            __terrane_raised(
                || -> Result<_, crate::TerraneForeignError> {
                    {
                        let _ = match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| -> Result<
                                _,
                                crate::TerraneForeignError,
                            > {
                                Ok({
                                    let _ = (&weak_owner).clear();
                                })
                            }),
                        ) {
                            Ok(result) => result?,
                            Err(payload) => {
                                return Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "terrane_retained_callback_registry",
                                        "terrane_retained_callback_registry::WeakRegistry::clear",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(())
                    }
                }(),
                28 /* terrane-site: src/main.trn:61:9-61:26 */,
            );
            {
                let __terrane_owner_2 = state.clone();
                let __terrane_value = {
                    let __terrane_snapshot = state
                        .lock()
                        .expect("shared reference lock poisoned")
                        .value
                        .clone();
                    __terrane_snapshot
                } + terrane_int_support::Int::from(1_i128);
                __terrane_owner_2
                    .lock()
                    .expect("shared reference lock poisoned")
                    .value = __terrane_value;
            }
            ()
        })
    };
    reentrant_token = {
        let __terrane_conversion_input_0 = reentrant.clone();
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                (&listeners)
                                    .register(
                                        std::sync::Arc::new({
                                            let callback = __terrane_conversion_input_0;
                                            move |
                                                callback_argument_0: &terrane_retained_callback_api::Event|
                                            {
                                                match || -> Result<_, crate::TerraneForeignError> {
                                                    let callback_value = callback(&*callback_argument_0);
                                                    Ok(callback_value)
                                                }() {
                                                    Ok(value) => value,
                                                    Err(error) => std::panic::panic_any(error.0),
                                                }
                                            }
                                        }),
                                    ),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_retained_callback_registry",
                                    "terrane_retained_callback_registry::Registry::register",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),
            29 /* terrane-site: src/main.trn:63:23-63:52 */,
        )
    };
    __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let _ = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok({
                            let _ = (&listeners)
                                .emit(
                                    &{
                                        let __terrane_conversion_input_0 = terrane_int_support::Int::from(
                                            9_i128,
                                        );
                                        __terrane_raised(
                                            || -> Result<_, crate::TerraneForeignError> {
                                                Ok(terrane_retained_callback_api::Event {
                                                    value: terrane_int_support::coerce::<
                                                        i64,
                                                    >(&__terrane_conversion_input_0)
                                                        .map_err(|error| crate::TerraneForeignError(
                                                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                                        ))?,
                                                })
                                            }(),
                                            30 /* terrane-site: src/main.trn:64:22-64:47 */,
                                        )
                                    },
                                );
                        })
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane_retained_callback_registry",
                                "terrane_retained_callback_registry::Registry::emit",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),
        31 /* terrane-site: src/main.trn:64:5-64:48 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&listeners).count()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_retained_callback_registry",
        "terrane_retained_callback_registry::Registry::count")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 32 /* terrane-site: src/main.trn:65:13-65:29 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = state
        .lock().expect("shared reference lock poisoned").value.clone();
        __terrane_snapshot })
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_static_trn_5265676973747279_token_alive(&reentrant_token),
        33 /* terrane-site: src/main.trn:67:13-67:51 */))
    );
    destroyed = add_tracked(&listeners, state.clone());
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_static_trn_5369676e616c_drops(),
        34 /* terrane-site: src/main.trn:70:13-70:27 */))
    );
    __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let _ = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok({
                            let _ = listeners.destroy();
                        })
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane_retained_callback_registry",
                                "terrane_retained_callback_registry::Registry::destroy",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),
        35 /* terrane-site: src/main.trn:71:5-71:23 */,
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_static_trn_5265676973747279_token_alive(&destroyed),
        36 /* terrane-site: src/main.trn:72:13-72:45 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_static_trn_5369676e616c_drops(),
        37 /* terrane-site: src/main.trn:73:13-73:27 */))
    );
    changes = terrane_int_support::Int::from(0_i128);
    change = {
        let mut changes = changes.clone();
        TerraneOwnedMutableInputsM::new(move |(incoming,): (&mut MutableEvent,)| -> () {
            let current: &mut MutableEvent;
            changes = changes.clone() + terrane_int_support::Int::from(1_i128);
            current = &mut *incoming;
            {
                let __terrane_value = changes == terrane_int_support::Int::from(2_i128);
                current.active = __terrane_value;
            }
            ()
        })
    };
    copied_change = change.clone();
    changing_event = {
        let __terrane_conversion_input_0 = false;
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(terrane_retained_callback_api::MutableEvent {
                    active: __terrane_conversion_input_0,
                })
            }(),
            38 /* terrane-site: src/main.trn:81:22-81:60 */,
        )
    };
    {
        let __terrane_conversion_input_0 = change;
        __terrane_raised(
            invoke_mut(__terrane_conversion_input_0, &mut changing_event),
            39 /* terrane-site: src/main.trn:82:5-82:44 */,
        )
    };
    println!("{}", terrane_scalar_support::scalar_text(&changing_event.active));
    {
        let __terrane_value = false;
        changing_event.active = __terrane_value;
    }
    {
        let __terrane_conversion_input_0 = copied_change;
        __terrane_raised(
            invoke_mut(__terrane_conversion_input_0, &mut changing_event),
            40 /* terrane-site: src/main.trn:85:5-85:51 */,
        )
    };
    println!("{}", terrane_scalar_support::scalar_text(&changing_event.active));
    send_only_state = __terrane_raised(
        terrane_static_trn_53656e644f6e6c79_new(),
        41 /* terrane-site: src/main.trn:87:23-87:38 */,
    );
    send_only_callback = {
        let send_only_state = send_only_state.clone();
        TerraneOwnedMutableInputsM::new(move |(incoming,): (&mut MutableEvent,)| -> () {
            {
                let __terrane_value = __terrane_raised(
                    || -> Result<_, crate::TerraneForeignError> {
                        {
                            let value = match std::panic::catch_unwind(
                                std::panic::AssertUnwindSafe(|| -> Result<
                                    _,
                                    crate::TerraneForeignError,
                                > { Ok((&send_only_state).bump()) }),
                            ) {
                                Ok(result) => result?,
                                Err(payload) => {
                                    return Err(
                                        crate::__terrane_dependency_panic(
                                            payload,
                                            "terrane_retained_callback_api",
                                            "terrane_retained_callback_api::SendOnly::bump",
                                        ),
                                    );
                                }
                            };
                            Ok::<
                                _,
                                crate::TerraneForeignError,
                            >(terrane_int_support::Int::from_u128(value as u128))
                        }
                    }(),
                    42 /* terrane-site: src/main.trn:89:28-89:49 */,
                ) == terrane_int_support::Int::from(2_i128);
                incoming.active = __terrane_value;
            }
            ()
        })
    };
    {
        let __terrane_value = false;
        changing_event.active = __terrane_value;
    }
    {
        let __terrane_conversion_input_0 = send_only_callback;
        __terrane_raised(
            invoke_send_mut(__terrane_conversion_input_0, &mut changing_event),
            43 /* terrane-site: src/main.trn:91:5-91:61 */,
        )
    };
    println!("{}", terrane_scalar_support::scalar_text(&changing_event.active));
    finish = {
        TerraneConsumingCallable::new(move |(incoming,): (Event,)| -> () {
            println!("{}", terrane_scalar_support::scalar_text(&incoming.value));
            ()
        })
    };
    {
        let __terrane_conversion_input_0 = finish;
        let __terrane_conversion_input_1 = {
            let __terrane_conversion_input_0 = terrane_int_support::Int::from(20_i128);
            __terrane_raised(
                || -> Result<_, crate::TerraneForeignError> {
                    Ok(terrane_retained_callback_api::Event {
                        value: terrane_int_support::coerce::<
                            i64,
                        >(&__terrane_conversion_input_0)
                            .map_err(|error| crate::TerraneForeignError(
                                crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                            ))?,
                    })
                }(),
                44 /* terrane-site: src/main.trn:95:32-95:58 */,
            )
        };
        __terrane_raised(
            invoke_once(__terrane_conversion_input_0, __terrane_conversion_input_1),
            45 /* terrane-site: src/main.trn:95:5-95:59 */,
        )
    };
    {
        let __terrane_conversion_input_0 = std::sync::Arc::new(named_event);
        __terrane_raised(
            invoke_shared(
                __terrane_conversion_input_0,
                &{
                    let __terrane_conversion_input_0 = terrane_int_support::Int::from(
                        30_i128,
                    );
                    __terrane_raised(
                        || -> Result<_, crate::TerraneForeignError> {
                            Ok(terrane_retained_callback_api::Event {
                                value: terrane_int_support::coerce::<
                                    i64,
                                >(&__terrane_conversion_input_0)
                                    .map_err(|error| crate::TerraneForeignError(
                                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                    ))?,
                            })
                        }(),
                        46 /* terrane-site: src/main.trn:96:34-96:60 */,
                    )
                },
            ),
            47 /* terrane-site: src/main.trn:96:5-96:61 */,
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        OffsetSink::terrane_construct();
        __terrane_raised(invoke_sink(__terrane_conversion_input_0, &{ let
        __terrane_conversion_input_0 = terrane_int_support::Int::from(2_i128);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > {
        Ok(terrane_retained_callback_api::Event { value : terrane_int_support::coerce:: <
        i64 > (&__terrane_conversion_input_0).map_err(| error | crate
        ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ? }) } (), 48 /* terrane-site: src/main.trn:97:52-97:77 */) }), 49 /* terrane-site: src/main.trn:97:13-97:78 */) })
    );
    erased = <Sink>::from(OffsetSink::terrane_construct());
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        erased; __terrane_raised(invoke_sink(__terrane_conversion_input_0, &{ let
        __terrane_conversion_input_0 = terrane_int_support::Int::from(3_i128);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > {
        Ok(terrane_retained_callback_api::Event { value : terrane_int_support::coerce:: <
        i64 > (&__terrane_conversion_input_0).map_err(| error | crate
        ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ? }) } (), 50 /* terrane-site: src/main.trn:99:35-99:60 */) }), 51 /* terrane-site: src/main.trn:99:13-99:61 */) })
    );
    finish_borrowed = {
        TerraneOwnedConsumingInputsR::new(move |(incoming,): (&Event,)| -> () {
            println!("{}", terrane_scalar_support::scalar_text(&(* incoming).value));
            ()
        })
    };
    {
        let __terrane_conversion_input_0 = finish_borrowed;
        __terrane_raised(
            invoke_borrowed_once(
                __terrane_conversion_input_0,
                &{
                    let __terrane_conversion_input_0 = terrane_int_support::Int::from(
                        40_i128,
                    );
                    __terrane_raised(
                        || -> Result<_, crate::TerraneForeignError> {
                            Ok(terrane_retained_callback_api::Event {
                                value: terrane_int_support::coerce::<
                                    i64,
                                >(&__terrane_conversion_input_0)
                                    .map_err(|error| crate::TerraneForeignError(
                                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                    ))?,
                            })
                        }(),
                        52 /* terrane-site: src/main.trn:102:50-102:76 */,
                    )
                },
            ),
            53 /* terrane-site: src/main.trn:102:5-102:77 */,
        )
    };
    {
        let __terrane_conversion_input_0 = std::sync::Arc::new(named_event);
        __terrane_raised(
            invoke_object_alias(
                __terrane_conversion_input_0,
                &{
                    let __terrane_conversion_input_0 = terrane_int_support::Int::from(
                        50_i128,
                    );
                    __terrane_raised(
                        || -> Result<_, crate::TerraneForeignError> {
                            Ok(terrane_retained_callback_api::Event {
                                value: terrane_int_support::coerce::<
                                    i64,
                                >(&__terrane_conversion_input_0)
                                    .map_err(|error| crate::TerraneForeignError(
                                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                    ))?,
                            })
                        }(),
                        54 /* terrane-site: src/main.trn:103:40-103:66 */,
                    )
                },
            ),
            55 /* terrane-site: src/main.trn:103:5-103:67 */,
        )
    };
    text_callback = {
        TerraneOwnedMutableInputsR::new(move |(text,): (&String,)| -> () {
            println!("{}", terrane_scalar_support::scalar_text(&* text));
            ()
        })
    };
    {
        let __terrane_conversion_input_0 = text_callback;
        __terrane_raised(
            invoke_text(__terrane_conversion_input_0),
            56 /* terrane-site: src/main.trn:106:5-106:36 */,
        )
    };
    number_callback = {
        TerraneOwnedMutableInputsM::new(move |(value,): (&mut i64,)| -> () {
            *value = 8;
            ()
        })
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        number_callback; __terrane_raised(invoke_number(__terrane_conversion_input_0),
        57 /* terrane-site: src/main.trn:109:13-109:48 */) })
    );
    saved = Some({
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(60_i128);
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(terrane_retained_callback_api::Event {
                    value: terrane_int_support::coerce::<
                        i64,
                    >(&__terrane_conversion_input_0)
                        .map_err(|error| crate::TerraneForeignError(
                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                        ))?,
                })
            }(),
            58 /* terrane-site: src/main.trn:110:24-110:50 */,
        )
    });
    snapshotter = {
        let mut saved = saved.clone();
        TerraneOwnedMutableInputsR::new(move |(incoming,): (&Event,)| -> () {
            if (&saved).is_some() {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&match &saved { Some(value)
                    => value, _ => unreachable!("flow-proven storage refinement") }
                    .value)
                );
            }
            saved = Some(
                __terrane_raised(
                    || -> Result<_, crate::TerraneForeignError> {
                        {
                            let value = match std::panic::catch_unwind(
                                std::panic::AssertUnwindSafe(|| -> Result<
                                    _,
                                    crate::TerraneForeignError,
                                > {
                                    Ok(
                                        <terrane_retained_callback_api::Event as core::clone::Clone>::clone(
                                            &*incoming,
                                        ),
                                    )
                                }),
                            ) {
                                Ok(result) => result?,
                                Err(payload) => {
                                    return Err(
                                        crate::__terrane_dependency_panic(
                                            payload,
                                            "terrane_retained_callback_api",
                                            "terrane_retained_callback_api::Event::clone",
                                        ),
                                    );
                                }
                            };
                            Ok::<_, crate::TerraneForeignError>(value)
                        }
                    }(),
                    59 /* terrane-site: src/main.trn:114:17-114:32 */,
                ),
            );
            if (&saved).is_some() {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&match &saved { Some(value)
                    => value, _ => unreachable!("flow-proven storage refinement") }
                    .value)
                );
            }
            ()
        })
    };
    snapshot_event = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(61_i128);
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(terrane_retained_callback_api::Event {
                    value: terrane_int_support::coerce::<
                        i64,
                    >(&__terrane_conversion_input_0)
                        .map_err(|error| crate::TerraneForeignError(
                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                        ))?,
                })
            }(),
            60 /* terrane-site: src/main.trn:117:22-117:48 */,
        )
    };
    snapshotter.call((&snapshot_event,));
}
