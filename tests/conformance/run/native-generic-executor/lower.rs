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
                    2 /* terrane-site: src/main.trn:18:12-18:39 */,
                )?,
            ),
            2 /* terrane-site: src/main.trn:18:12-18:39 */,
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
                    4 /* terrane-site: src/main.trn:22:12-22:39 */,
                )?,
            ),
            4 /* terrane-site: src/main.trn:22:12-22:39 */,
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
                        8 /* terrane-site: src/main.trn:33:16-33:38 */,
                    )?,
                ),
                8 /* terrane-site: src/main.trn:33:16-33:38 */,
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
                                            29,
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
                    11 /* terrane-site: src/main.trn:40:12-40:33 */,
                )?,
            ),
            11 /* terrane-site: src/main.trn:40:12-40:33 */,
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
                            15 /* terrane-site: src/main.trn:51:16-51:40 */,
                        )?,
                    ),
                    15 /* terrane-site: src/main.trn:51:16-51:40 */,
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
        21 /* terrane-site: src/main.trn:58:13-58:49 */,
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
            23 /* terrane-site: src/main.trn:59:14-59:69 */,
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
    let reader: std::sync::Arc<dyn Fn() -> Result<i64, TerraneError> + Send + Sync>;
    let first: i64;
    connection = __terrane_raised_err(
        terrane_static_trn_436f6e6e656374696f6e_new(),
        25 /* terrane-site: src/main.trn:63:18-63:34 */,
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
                                std::panic::AssertUnwindSafe(|| witness::outlives(
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
                                            "witness::outlives",
                                        ),
                                    )
                                }
                            },
                            26 /* terrane-site: src/main.trn:65:16-65:40 */,
                        )?,
                    ),
                    26 /* terrane-site: src/main.trn:65:16-65:40 */,
                )?,
            );
        })
    };
    first = __terrane_traced_err(
        reader(),
        27 /* terrane-site: src/main.trn:66:13-66:20 */,
    )?;
    return Ok(
        __terrane_raised_err(
            terrane_int_support::fixed_addition(
                first,
                __terrane_traced_err(
                    reader(),
                    28 /* terrane-site: src/main.trn:67:21-67:28 */,
                )?,
            ),
            29 /* terrane-site: src/main.trn:67:12-67:29 */,
        )?,
    );
}
fn main() {
    __terrane_run(async move {
        let mut connection: Connection;
        let mut source: SourceCounter;
        let mut stage: Stage;
        let mut holder: EnumHolder;
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(execute_local(),
            30 /* terrane-site: src/main.trn:70:13-70:27 */))
        );
        connection = __terrane_raised(
            terrane_static_trn_436f6e6e656374696f6e_new(),
            31 /* terrane-site: src/main.trn:71:18-71:34 */,
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(execute_reference(&mut
            connection), 32 /* terrane-site: src/main.trn:72:13-72:46 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
            __terrane_raised(query(String::from("direct")), 33 /* terrane-site: src/main.trn:73:14-73:29 */); let __terrane_native_input_1 = &mut connection;
            __terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
            __terrane_native_input_0.fetch(__terrane_native_input_1))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "witness",
            "witness::Query::fetch")) }, 34 /* terrane-site: src/main.trn:73:13-73:52 */) })
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(execute_formal_reference(&mut
            connection), 35 /* terrane-site: src/main.trn:74:13-74:53 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&connection)
            .executions())) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "witness",
            "witness::Connection::executions")) }, 36 /* terrane-site: src/main.trn:75:13-75:35 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(execute_optional(true),
            37 /* terrane-site: src/main.trn:76:13-76:35 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(execute_optional(false),
            38 /* terrane-site: src/main.trn:77:13-77:36 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(execute_deferred(true),
            39 /* terrane-site: src/main.trn:78:13-78:35 */))
        );
        source = SourceCounter::terrane_construct();
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
            witness::advance(&mut source))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "witness",
            "witness::advance")) }, 40 /* terrane-site: src/main.trn:80:13-80:32 */))
        );
        println!("{}", terrane_scalar_support::scalar_text(&source.count));
        stage = witness::Stage::Idle;
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(execute_enum(&mut
            stage), 41 /* terrane-site: src/main.trn:83:13-83:36 */))
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
            42 /* terrane-site: src/main.trn:89:13-89:43 */))
        );
        holder = EnumHolder::terrane_construct();
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
            witness::advance(&mut holder.stage))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "witness",
            "witness::advance")) }, 43 /* terrane-site: src/main.trn:91:13-91:38 */))
        );
        let __terrane_match_value_2648 = holder.stage;
        match __terrane_match_value_2648 {
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
            44 /* terrane-site: src/main.trn:97:13-97:30 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(execute_outlives(),
            45 /* terrane-site: src/main.trn:98:13-98:30 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(observe_outlives(),
            46 /* terrane-site: src/main.trn:99:13-99:30 */))
        );
    });
}
