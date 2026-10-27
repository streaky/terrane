// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct SourceCounter {
    pub count: terrane_int_support::Int,
}
impl SourceCounter {
    pub fn terrane_construct() -> Self {
        Self {
            count: terrane_int_support::Int::from(40_i128),
        }
    }
    pub fn advance(&mut self) -> terrane_int_support::Int {
        {
            let __terrane_value = self.count.clone()
                + terrane_int_support::Int::from(1_i128);
            self.count = __terrane_value;
        }
        return self.count.clone();
    }
}
impl AdvanceProtocol for SourceCounter {
    fn clone_box(&self) -> Box<dyn AdvanceProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn AdvanceProtocol> {
        Box::new(self.clone())
    }
    fn advance(&mut self) -> terrane_int_support::Int {
        SourceCounter::advance(&mut *self)
    }
}
impl From<SourceCounter> for Advance {
    fn from(value: SourceCounter) -> Self {
        Self(Box::new(value))
    }
}
impl witness::Advance for SourceCounter {
    fn advance(&mut self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <SourceCounter>::advance(&mut *self);
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
#[derive(Clone)]
pub struct EnumHolder {
    pub stage: Stage,
}
impl EnumHolder {
    pub fn terrane_construct() -> Self {
        Self {
            stage: witness::Stage::Idle,
        }
    }
}
fn execute_local() -> Result<i64, TerraneError> {
    let mut connection: Connection;
    let bound: Query;
    connection = __terrane_raised_err(
        terrane_static_trn_436f6e6e656374696f6e_new(),
        0 /* terrane-site: src/main.trn:16:18-16:34 */,
    )?;
    bound = __terrane_raised_err(
        query(String::from("bound-local")),
        1 /* terrane-site: src/main.trn:17:13-17:33 */,
    )?;
    return Ok(
        __terrane_raised_err(
            terrane_int_support::coerce::<
                i64,
            >(
                &__terrane_raised_err(
                    match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| bound.fetch(&mut connection)),
                    ) {
                        Ok(value) => {
                            Ok(terrane_int_support::Int::from(i128::from(value)))
                        }
                        Err(payload) => {
                            Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "witness",
                                    "witness::Query::fetch",
                                ),
                            )
                        }
                    },
                    2 /* terrane-site: src/main.trn:18:12-18:35 */,
                )?,
            ),
            2 /* terrane-site: src/main.trn:18:12-18:35 */,
        )?,
    );
}
fn execute_reference(connection: &mut Connection) -> Result<i64, TerraneError> {
    let bound: Query;
    bound = __terrane_raised_err(
        query(String::from("bound-reference")),
        3 /* terrane-site: src/main.trn:21:13-21:37 */,
    )?;
    return Ok(
        __terrane_raised_err(
            terrane_int_support::coerce::<
                i64,
            >(
                &__terrane_raised_err(
                    match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| bound.fetch(&mut *connection)),
                    ) {
                        Ok(value) => {
                            Ok(terrane_int_support::Int::from(i128::from(value)))
                        }
                        Err(payload) => {
                            Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "witness",
                                    "witness::Query::fetch",
                                ),
                            )
                        }
                    },
                    4 /* terrane-site: src/main.trn:22:12-22:35 */,
                )?,
            ),
            4 /* terrane-site: src/main.trn:22:12-22:35 */,
        )?,
    );
}
fn execute_formal_reference(connection: &mut Connection) -> Result<i64, TerraneError> {
    return Ok(
        __terrane_raised_err(
            terrane_int_support::coerce::<
                i64,
            >(
                &__terrane_raised_err(
                    match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| witness::advance(
                            &mut *connection,
                        )),
                    ) {
                        Ok(value) => {
                            Ok(terrane_int_support::Int::from(i128::from(value)))
                        }
                        Err(payload) => {
                            Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "witness",
                                    "witness::advance",
                                ),
                            )
                        }
                    },
                    5 /* terrane-site: src/main.trn:25:12-25:31 */,
                )?,
            ),
            5 /* terrane-site: src/main.trn:25:12-25:31 */,
        )?,
    );
}
fn execute_optional(flag: bool) -> Result<i64, TerraneError> {
    let mut maybe: Option<Connection>;
    let bound: Query;
    maybe = None;
    if flag {
        maybe = Some(
            __terrane_raised_err(
                terrane_static_trn_436f6e6e656374696f6e_new(),
                6 /* terrane-site: src/main.trn:30:17-30:33 */,
            )?,
        );
    }
    if (&maybe).is_some() {
        bound = __terrane_raised_err(
            query(String::from("narrowed")),
            7 /* terrane-site: src/main.trn:32:17-32:34 */,
        )?;
        return Ok(
            __terrane_raised_err(
                terrane_int_support::coerce::<
                    i64,
                >(
                    &__terrane_raised_err(
                        match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| {
                                bound
                                    .fetch(
                                        match &mut maybe {
                                            Some(value) => value,
                                            _ => unreachable!("flow-proven storage refinement"),
                                        },
                                    )
                            }),
                        ) {
                            Ok(value) => {
                                Ok(terrane_int_support::Int::from(i128::from(value)))
                            }
                            Err(payload) => {
                                Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "witness",
                                        "witness::Query::fetch",
                                    ),
                                )
                            }
                        },
                        8 /* terrane-site: src/main.trn:33:16-33:34 */,
                    )?,
                ),
                8 /* terrane-site: src/main.trn:33:16-33:34 */,
            )?,
        );
    }
    return Ok(0);
}
fn execute_deferred(flag: bool) -> Result<i64, TerraneError> {
    let mut late: Option<Connection> = None;
    let bound: Query;
    if flag {
        let _ = late
            .insert(
                __terrane_raised_err(
                    terrane_static_trn_436f6e6e656374696f6e_new(),
                    9 /* terrane-site: src/main.trn:38:27-38:43 */,
                )?,
            );
    }
    bound = __terrane_raised_err(
        query(String::from("deferred")),
        10 /* terrane-site: src/main.trn:39:13-39:30 */,
    )?;
    return Ok(
        __terrane_raised_err(
            terrane_int_support::coerce::<
                i64,
            >(
                &__terrane_raised_err(
                    match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| {
                            bound
                                .fetch(
                                    late
                                        .as_mut()
                                        .unwrap_or_else(|| __terrane_uninitialized_binding(
                                            "late",
                                            "main.trn",
                                            40,
                                            25,
                                        )),
                                )
                        }),
                    ) {
                        Ok(value) => {
                            Ok(terrane_int_support::Int::from(i128::from(value)))
                        }
                        Err(payload) => {
                            Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "witness",
                                    "witness::Query::fetch",
                                ),
                            )
                        }
                    },
                    11 /* terrane-site: src/main.trn:40:12-40:29 */,
                )?,
            ),
            11 /* terrane-site: src/main.trn:40:12-40:29 */,
        )?,
    );
}
fn execute_enum(stage: &mut Stage) -> Result<i64, TerraneError> {
    return Ok(
        __terrane_raised_err(
            terrane_int_support::coerce::<
                i64,
            >(
                &__terrane_raised_err(
                    match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| witness::advance(&mut *stage)),
                    ) {
                        Ok(value) => {
                            Ok(terrane_int_support::Int::from(i128::from(value)))
                        }
                        Err(payload) => {
                            Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "witness",
                                    "witness::advance",
                                ),
                            )
                        }
                    },
                    12 /* terrane-site: src/main.trn:43:12-43:26 */,
                )?,
            ),
            12 /* terrane-site: src/main.trn:43:12-43:26 */,
        )?,
    );
}
fn observe_native(connection: &Connection) -> Result<i64, TerraneError> {
    return Ok(
        __terrane_raised_err(
            terrane_int_support::coerce::<
                i64,
            >(
                &__terrane_raised_err(
                    match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| witness::observed(&*connection)),
                    ) {
                        Ok(value) => {
                            Ok(terrane_int_support::Int::from(i128::from(value)))
                        }
                        Err(payload) => {
                            Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "witness",
                                    "witness::observed",
                                ),
                            )
                        }
                    },
                    13 /* terrane-site: src/main.trn:46:12-46:32 */,
                )?,
            ),
            13 /* terrane-site: src/main.trn:46:12-46:32 */,
        )?,
    );
}
fn observe_captured() -> Result<i64, TerraneError> {
    let connection: Connection;
    let reader: std::sync::Arc<dyn Fn() -> Result<i64, TerraneError> + Send + Sync>;
    let first: i64;
    connection = __terrane_raised_err(
        terrane_static_trn_436f6e6e656374696f6e_new(),
        14 /* terrane-site: src/main.trn:49:18-49:34 */,
    )?;
    reader = {
        let connection = connection;
        std::sync::Arc::new(move || -> Result<i64, TerraneError> {
            return Ok(
                __terrane_raised_err(
                    terrane_int_support::coerce::<
                        i64,
                    >(
                        &__terrane_raised_err(
                            match std::panic::catch_unwind(
                                std::panic::AssertUnwindSafe(|| witness::observed(
                                    &connection,
                                )),
                            ) {
                                Ok(value) => {
                                    Ok(terrane_int_support::Int::from(i128::from(value)))
                                }
                                Err(payload) => {
                                    Err(
                                        crate::__terrane_dependency_panic(
                                            payload,
                                            "witness",
                                            "witness::observed",
                                        ),
                                    )
                                }
                            },
                            15 /* terrane-site: src/main.trn:51:16-51:36 */,
                        )?,
                    ),
                    15 /* terrane-site: src/main.trn:51:16-51:36 */,
                )?,
            );
        })
    };
    first = __terrane_traced_err(
        reader(),
        16 /* terrane-site: src/main.trn:52:13-52:20 */,
    )?;
    return Ok(
        __terrane_raised_err(
            terrane_int_support::fixed_addition(
                first,
                __terrane_traced_err(
                    reader(),
                    17 /* terrane-site: src/main.trn:53:21-53:28 */,
                )?,
            ),
            18 /* terrane-site: src/main.trn:53:12-53:29 */,
        )?,
    );
}
fn execute_outlives() -> Result<i64, TerraneError> {
    let mut connection: Connection;
    let bound: Query;
    let first: terrane_int_support::Int;
    let second: terrane_int_support::Int;
    connection = __terrane_raised_err(
        terrane_static_trn_436f6e6e656374696f6e_new(),
        19 /* terrane-site: src/main.trn:56:18-56:34 */,
    )?;
    bound = __terrane_raised_err(
        query(String::from("region-bound")),
        20 /* terrane-site: src/main.trn:57:13-57:34 */,
    )?;
    first = __terrane_raised_err(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| bound.fetch_outlives(&mut connection)),
        ) {
            Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::Query::fetch_outlives",
                    ),
                )
            }
        },
        21 /* terrane-site: src/main.trn:58:13-58:45 */,
    )?;
    second = {
        let __terrane_native_input_0 = __terrane_raised_err(
            query(String::from("region-direct")),
            22 /* terrane-site: src/main.trn:59:15-59:37 */,
        )?;
        let __terrane_native_input_1 = &mut connection;
        __terrane_raised_err(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    __terrane_native_input_0.fetch_outlives(__terrane_native_input_1)
                }),
            ) {
                Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "witness",
                            "witness::Query::fetch_outlives",
                        ),
                    )
                }
            },
            23 /* terrane-site: src/main.trn:59:14-59:65 */,
        )?
    };
    return Ok(
        __terrane_raised_err(
            terrane_int_support::coerce::<i64>(&(first.clone() + second.clone())),
            24 /* terrane-site: src/main.trn:60:12-60:26 */,
        )?,
    );
}
fn observe_outlives() -> Result<i64, TerraneError> {
    let connection: Connection;
    let observer: &Connection;
    let reader: std::sync::Arc<dyn Fn() -> Result<i64, TerraneError> + Send + Sync>;
    let first: i64;
    connection = __terrane_raised_err(
        terrane_static_trn_436f6e6e656374696f6e_new(),
        25 /* terrane-site: src/main.trn:63:18-63:34 */,
    )?;
    observer = &connection;
    reader = {
        let observer = observer;
        std::sync::Arc::new(move || -> Result<i64, TerraneError> {
            return Ok(
                __terrane_raised_err(
                    terrane_int_support::coerce::<
                        i64,
                    >(
                        &__terrane_raised_err(
                            match std::panic::catch_unwind(
                                std::panic::AssertUnwindSafe(|| witness::outlives(
                                    &*observer,
                                )),
                            ) {
                                Ok(value) => {
                                    Ok(terrane_int_support::Int::from(i128::from(value)))
                                }
                                Err(payload) => {
                                    Err(
                                        crate::__terrane_dependency_panic(
                                            payload,
                                            "witness",
                                            "witness::outlives",
                                        ),
                                    )
                                }
                            },
                            26 /* terrane-site: src/main.trn:66:16-66:34 */,
                        )?,
                    ),
                    26 /* terrane-site: src/main.trn:66:16-66:34 */,
                )?,
            );
        })
    };
    first = __terrane_traced_err(
        reader(),
        27 /* terrane-site: src/main.trn:67:13-67:20 */,
    )?;
    return Ok(
        __terrane_raised_err(
            terrane_int_support::fixed_addition(
                first,
                __terrane_traced_err(
                    reader(),
                    28 /* terrane-site: src/main.trn:68:21-68:28 */,
                )?,
            ),
            29 /* terrane-site: src/main.trn:68:12-68:29 */,
        )?,
    );
}
fn execute_owner(connection: &mut Connection) -> Result<i64, TerraneError> {
    return Ok(
        __terrane_raised_err(
            terrane_int_support::coerce::<
                i64,
            >(
                &{
                    let __terrane_native_input_0 = __terrane_raised_err(
                        owner_query(String::from("owner-forward")),
                        30 /* terrane-site: src/main.trn:71:13-71:41 */,
                    )?;
                    let __terrane_native_input_1 = &mut *connection;
                    __terrane_raised_err(
                        match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| {
                                __terrane_native_input_0.fetch(__terrane_native_input_1)
                            }),
                        ) {
                            Ok(value) => {
                                Ok(terrane_int_support::Int::from(i128::from(value)))
                            }
                            Err(payload) => {
                                Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "witness",
                                        "witness::OwnerQuery::fetch",
                                    ),
                                )
                            }
                        },
                        31 /* terrane-site: src/main.trn:71:12-71:60 */,
                    )?
                },
            ),
            31 /* terrane-site: src/main.trn:71:12-71:60 */,
        )?,
    );
}
fn main() {
    __terrane_run(async move {
        let mut connection: Connection;
        let mut source: SourceCounter;
        let mut stage: Stage;
        let mut holder: EnumHolder;
        let observer: &mut Connection;
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(execute_local(),
            32 /* terrane-site: src/main.trn:74:13-74:27 */))
        );
        connection = __terrane_raised(
            terrane_static_trn_436f6e6e656374696f6e_new(),
            33 /* terrane-site: src/main.trn:75:18-75:34 */,
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(execute_reference(&mut
            connection), 34 /* terrane-site: src/main.trn:76:13-76:42 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
            __terrane_raised(query(String::from("direct")), 35 /* terrane-site: src/main.trn:77:14-77:29 */); let __terrane_native_input_1 = &mut connection;
            __terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
            __terrane_native_input_0.fetch(__terrane_native_input_1))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "witness",
            "witness::Query::fetch")) }, 36 /* terrane-site: src/main.trn:77:13-77:48 */) })
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(execute_formal_reference(&mut
            connection), 37 /* terrane-site: src/main.trn:78:13-78:49 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&connection)
            .executions())) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "witness",
            "witness::Connection::executions")) }, 38 /* terrane-site: src/main.trn:79:13-79:35 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(execute_optional(true),
            39 /* terrane-site: src/main.trn:80:13-80:35 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(execute_optional(false),
            40 /* terrane-site: src/main.trn:81:13-81:36 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(execute_deferred(true),
            41 /* terrane-site: src/main.trn:82:13-82:35 */))
        );
        source = SourceCounter::terrane_construct();
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
            witness::advance(&mut source))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "witness",
            "witness::advance")) }, 42 /* terrane-site: src/main.trn:84:13-84:28 */))
        );
        println!("{}", terrane_scalar_support::scalar_text(&source.count));
        stage = witness::Stage::Idle;
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(execute_enum(&mut
            stage), 43 /* terrane-site: src/main.trn:87:13-87:32 */))
        );
        match &stage {
            witness::Stage::Done => {
                println!("{}", terrane_scalar_support::scalar_text(&8));
            }
            witness::Stage::Idle => {
                println!("{}", terrane_scalar_support::scalar_text(&0));
            }
        }
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(observe_native(&connection),
            44 /* terrane-site: src/main.trn:93:13-93:39 */))
        );
        holder = EnumHolder::terrane_construct();
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
            witness::advance(&mut holder.stage))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "witness",
            "witness::advance")) }, 45 /* terrane-site: src/main.trn:95:13-95:34 */))
        );
        let __terrane_match_value_2746 = holder.stage;
        match __terrane_match_value_2746 {
            witness::Stage::Done => {
                println!("{}", terrane_scalar_support::scalar_text(&9));
            }
            witness::Stage::Idle => {
                println!("{}", terrane_scalar_support::scalar_text(&0));
            }
        }
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(observe_captured(),
            46 /* terrane-site: src/main.trn:101:13-101:30 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(execute_outlives(),
            47 /* terrane-site: src/main.trn:102:13-102:30 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(observe_outlives(),
            48 /* terrane-site: src/main.trn:103:13-103:30 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
            __terrane_raised(owner_query(String::from("owner-direct")),
            49 /* terrane-site: src/main.trn:104:14-104:41 */); let
            __terrane_native_input_1 = &mut connection; __terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
            __terrane_native_input_0.fetch(__terrane_native_input_1))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "witness",
            "witness::OwnerQuery::fetch")) }, 50 /* terrane-site: src/main.trn:104:13-104:60 */) })
        );
        observer = &mut connection;
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(execute_owner(&mut *
            observer), 51 /* terrane-site: src/main.trn:106:13-106:36 */))
        );
    });
}
