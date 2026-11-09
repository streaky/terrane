// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: completion-capture-boundaries
#[derive(Clone)]
pub struct Err {
    pub message: String,
}
impl Err {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("failure"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
fn eat(
    value: terrane_collection_support::List<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    return terrane_int_support::Int::from(value.length());
}
fn restored(
    value: terrane_collection_support::List<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    let mut value: Option<terrane_collection_support::List<terrane_int_support::Int>> = Some(
        value,
    );
    let mut __terrane_completion_0: TerraneCompletion<terrane_int_support::Int> = (|| {
        let __terrane_try_0: TerraneCompletion<terrane_int_support::Int> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&eat(value.as_ref()
                .expect("flow-proven available binding").clone()))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_0: TerraneCompletion<terrane_int_support::Int> = (|| {
        let _ = value
            .insert(
                terrane_collection_support::List::<
                    terrane_int_support::Int,
                >::new(vec![terrane_int_support::Int::from(2_i128)]),
            );
        TerraneCompletion::Normal
    })();
    match __terrane_finally_0 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_0 = replacement,
    }
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    return __terrane_raised(
        value
            .as_ref()
            .expect("flow-proven available binding")
            .get_or_error(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(0_i128),
                    ),
                    0 /* terrane-site: case.trn:18:12-18:20 */,
                ),
            ),
        0 /* terrane-site: case.trn:18:12-18:20 */,
    );
}
fn restored_and_read(
    value: terrane_collection_support::List<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    let mut value: Option<terrane_collection_support::List<terrane_int_support::Int>> = Some(
        value,
    );
    let mut __terrane_completion_1: TerraneCompletion<terrane_int_support::Int> = (|| {
        let __terrane_try_1: TerraneCompletion<terrane_int_support::Int> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&eat(value.as_ref()
                .expect("flow-proven available binding").clone()))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_1 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_1) => {
                let mut __terrane_handled_1 = false;
                if !__terrane_handled_1 {
                    return TerraneCompletion::Error(__terrane_error_1);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_1: TerraneCompletion<terrane_int_support::Int> = (|| {
        let _ = value
            .insert(
                terrane_collection_support::List::<
                    terrane_int_support::Int,
                >::new(vec![terrane_int_support::Int::from(6_i128)]),
            );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(value
            .as_ref().expect("flow-proven available binding")
            .get_or_error(__terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            1 /* terrane-site: case.trn:25:16-25:24 */)), 1 /* terrane-site: case.trn:25:16-25:24 */))
        );
        TerraneCompletion::Normal
    })();
    match __terrane_finally_1 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_1 = replacement,
    }
    match __terrane_completion_1 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    return __terrane_raised(
        value
            .as_ref()
            .expect("flow-proven available binding")
            .get_or_error(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(0_i128),
                    ),
                    2 /* terrane-site: case.trn:26:12-26:20 */,
                ),
            ),
        2 /* terrane-site: case.trn:26:12-26:20 */,
    );
}
fn fail() -> Result<terrane_int_support::Int, TerraneError> {
    return Err({
        let value = Err::terrane_construct();
        let rendered = value.render();
        let original = &value.message;
        let (message, rendered) = if &rendered == original {
            (rendered, None)
        } else {
            (original.clone(), Some(rendered))
        };
        TerraneError::raised_with_rendered_message(
            TerraneErrorKind::Custom(DescriptorId(0)),
            message,
            rendered,
            3 /* terrane-site: case.trn:29:5-29:24 */,
        )
    });
}
fn nested_restored(
    value: terrane_collection_support::List<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    let mut value: Option<terrane_collection_support::List<terrane_int_support::Int>> = Some(
        value,
    );
    let __terrane_completion_2: TerraneCompletion<terrane_int_support::Int> = (|| {
        let __terrane_try_2: TerraneCompletion<terrane_int_support::Int> = (|| {
            let mut __terrane_completion_3: TerraneCompletion<
                terrane_int_support::Int,
            > = (|| {
                let __terrane_try_3: TerraneCompletion<terrane_int_support::Int> = (|| {
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&eat(value.as_ref()
                        .expect("flow-proven available binding").clone()))
                    );
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&__terrane_traced_completion!(fail(),
                        4 /* terrane-site: case.trn:35:21-35:26 */))
                    );
                    TerraneCompletion::Normal
                })();
                match __terrane_try_3 {
                    TerraneCompletion::Return(value) => {
                        return TerraneCompletion::Return(value);
                    }
                    TerraneCompletion::Break => return TerraneCompletion::Break,
                    TerraneCompletion::Continue => return TerraneCompletion::Continue,
                    TerraneCompletion::Normal => {}
                    TerraneCompletion::Error(__terrane_error_3) => {
                        let mut __terrane_handled_3 = false;
                        if !__terrane_handled_3 {
                            return TerraneCompletion::Error(__terrane_error_3);
                        }
                    }
                }
                TerraneCompletion::Normal
            })();
            let __terrane_finally_3: TerraneCompletion<terrane_int_support::Int> = (|| {
                let _ = value
                    .insert(
                        terrane_collection_support::List::<
                            terrane_int_support::Int,
                        >::new(vec![terrane_int_support::Int::from(8_i128)]),
                    );
                TerraneCompletion::Normal
            })();
            match __terrane_finally_3 {
                TerraneCompletion::Normal => {}
                replacement => __terrane_completion_3 = replacement,
            }
            match __terrane_completion_3 {
                TerraneCompletion::Normal => {}
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Error(error) => return TerraneCompletion::Error(error),
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
            }
            TerraneCompletion::Normal
        })();
        match __terrane_try_2 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_2) => {
                let mut __terrane_handled_2 = false;
                if !__terrane_handled_2 {
                    __terrane_handled_2 = true;
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&__terrane_raised_completion!(value
                        .as_ref().expect("flow-proven available binding")
                        .get_or_error(__terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
                        5 /* terrane-site: case.trn:39:16-39:24 */)),
                        5 /* terrane-site: case.trn:39:16-39:24 */))
                    );
                }
                if !__terrane_handled_2 {
                    return TerraneCompletion::Error(__terrane_error_2);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_2 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    return __terrane_raised(
        value
            .as_ref()
            .expect("flow-proven available binding")
            .get_or_error(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(0_i128),
                    ),
                    6 /* terrane-site: case.trn:40:12-40:20 */,
                ),
            ),
        6 /* terrane-site: case.trn:40:12-40:20 */,
    );
}
#[derive(Clone)]
pub struct Worker {}
impl Worker {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn restored(
        &self,
        value: terrane_collection_support::List<terrane_int_support::Int>,
    ) -> terrane_int_support::Int {
        let mut value: Option<
            terrane_collection_support::List<terrane_int_support::Int>,
        > = Some(value);
        let mut __terrane_completion_4: TerraneCompletion<terrane_int_support::Int> = (||
        {
            let __terrane_try_4: TerraneCompletion<terrane_int_support::Int> = (|| {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&eat(value.as_ref()
                    .expect("flow-proven available binding").clone()))
                );
                TerraneCompletion::Normal
            })();
            match __terrane_try_4 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_4) => {
                    let mut __terrane_handled_4 = false;
                    if !__terrane_handled_4 {
                        return TerraneCompletion::Error(__terrane_error_4);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        let __terrane_finally_4: TerraneCompletion<terrane_int_support::Int> = (|| {
            let _ = value
                .insert(
                    terrane_collection_support::List::<
                        terrane_int_support::Int,
                    >::new(vec![terrane_int_support::Int::from(3_i128)]),
                );
            TerraneCompletion::Normal
        })();
        match __terrane_finally_4 {
            TerraneCompletion::Normal => {}
            replacement => __terrane_completion_4 = replacement,
        }
        match __terrane_completion_4 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        return __terrane_raised(
            value
                .as_ref()
                .expect("flow-proven available binding")
                .get_or_error(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &terrane_int_support::Int::from(0_i128),
                        ),
                        7 /* terrane-site: case.trn:48:16-48:24 */,
                    ),
                ),
            7 /* terrane-site: case.trn:48:16-48:24 */,
        );
    }
    pub fn terrane_static_restored_static(
        value: terrane_collection_support::List<terrane_int_support::Int>,
    ) -> terrane_int_support::Int {
        let mut value: Option<
            terrane_collection_support::List<terrane_int_support::Int>,
        > = Some(value);
        let mut __terrane_completion_5: TerraneCompletion<terrane_int_support::Int> = (||
        {
            let __terrane_try_5: TerraneCompletion<terrane_int_support::Int> = (|| {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&eat(value.as_ref()
                    .expect("flow-proven available binding").clone()))
                );
                TerraneCompletion::Normal
            })();
            match __terrane_try_5 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_5) => {
                    let mut __terrane_handled_5 = false;
                    if !__terrane_handled_5 {
                        return TerraneCompletion::Error(__terrane_error_5);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        let __terrane_finally_5: TerraneCompletion<terrane_int_support::Int> = (|| {
            let _ = value
                .insert(
                    terrane_collection_support::List::<
                        terrane_int_support::Int,
                    >::new(vec![terrane_int_support::Int::from(4_i128)]),
                );
            TerraneCompletion::Normal
        })();
        match __terrane_finally_5 {
            TerraneCompletion::Normal => {}
            replacement => __terrane_completion_5 = replacement,
        }
        match __terrane_completion_5 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        return __terrane_raised(
            value
                .as_ref()
                .expect("flow-proven available binding")
                .get_or_error(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &terrane_int_support::Int::from(0_i128),
                        ),
                        8 /* terrane-site: case.trn:55:16-55:24 */,
                    ),
                ),
            8 /* terrane-site: case.trn:55:16-55:24 */,
        );
    }
}
fn main() {
    let worker: Worker;
    let callback: std::sync::Arc<
        dyn Fn(
            terrane_collection_support::List<terrane_int_support::Int>,
        ) -> terrane_int_support::Int + Send + Sync,
    >;
    let mut ignored: Option<
        std::sync::Arc<
            dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
        >,
    > = None;
    let mut outer: Option<terrane_collection_support::List<terrane_int_support::Int>> = None;
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&restored(terrane_collection_support::List::
        < terrane_int_support::Int >
        ::new(vec![terrane_int_support::Int::from(1_i128)])))
    );
    worker = Worker::terrane_construct();
    println!(
        "{}", terrane_scalar_support::scalar_text(&worker
        .restored(terrane_collection_support::List:: < terrane_int_support::Int >
        ::new(vec![terrane_int_support::Int::from(1_i128)])))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&Worker::terrane_static_restored_static(terrane_collection_support::List::
        < terrane_int_support::Int >
        ::new(vec![terrane_int_support::Int::from(1_i128)])))
    );
    callback = {
        std::sync::Arc::new(move |
            value: terrane_collection_support::List<terrane_int_support::Int>,
        | -> terrane_int_support::Int {
            let mut value: Option<
                terrane_collection_support::List<terrane_int_support::Int>,
            > = Some(value);
            let mut __terrane_completion_6: TerraneCompletion<
                terrane_int_support::Int,
            > = (|| {
                let __terrane_try_6: TerraneCompletion<terrane_int_support::Int> = (|| {
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&eat(value.as_ref()
                        .expect("flow-proven available binding").clone()))
                    );
                    TerraneCompletion::Normal
                })();
                match __terrane_try_6 {
                    TerraneCompletion::Return(value) => {
                        return TerraneCompletion::Return(value);
                    }
                    TerraneCompletion::Break => return TerraneCompletion::Break,
                    TerraneCompletion::Continue => return TerraneCompletion::Continue,
                    TerraneCompletion::Normal => {}
                    TerraneCompletion::Error(__terrane_error_6) => {
                        let mut __terrane_handled_6 = false;
                        if !__terrane_handled_6 {
                            return TerraneCompletion::Error(__terrane_error_6);
                        }
                    }
                }
                TerraneCompletion::Normal
            })();
            let __terrane_finally_6: TerraneCompletion<terrane_int_support::Int> = (|| {
                let _ = value
                    .insert(
                        terrane_collection_support::List::<
                            terrane_int_support::Int,
                        >::new(vec![terrane_int_support::Int::from(5_i128)]),
                    );
                TerraneCompletion::Normal
            })();
            match __terrane_finally_6 {
                TerraneCompletion::Normal => {}
                replacement => __terrane_completion_6 = replacement,
            }
            match __terrane_completion_6 {
                TerraneCompletion::Normal => {}
                TerraneCompletion::Return(value) => return value,
                TerraneCompletion::Error(error) => __terrane_uncaught(error),
                TerraneCompletion::Break | TerraneCompletion::Continue => {
                    __terrane_generated_defect("loop control escaped a non-loop try")
                }
            }
            return __terrane_raised(
                value
                    .as_ref()
                    .expect("flow-proven available binding")
                    .get_or_error(
                        __terrane_raised(
                            terrane_collection_support::index_from_int(
                                &terrane_int_support::Int::from(0_i128),
                            ),
                            9 /* terrane-site: case.trn:69:16-69:24 */,
                        ),
                    ),
                9 /* terrane-site: case.trn:69:16-69:24 */,
            );
        })
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&callback(terrane_collection_support::List::
        < terrane_int_support::Int >
        ::new(vec![terrane_int_support::Int::from(1_i128)])))
    );
    let __terrane_completion_7: TerraneCompletion<()> = (|| {
        let __terrane_try_7: TerraneCompletion<()> = (|| {
            let _ = ignored
                .insert({
                    std::sync::Arc::new(move |
                        value: terrane_int_support::Int,
                    | -> terrane_int_support::Int {
                        return value.clone() + terrane_int_support::Int::from(1_i128);
                    })
                });
            println!(
                "{}", terrane_scalar_support::scalar_text(&ignored.as_ref()
                .expect("flow-proven available binding").clone()
                (terrane_int_support::Int::from(6_i128)))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_7 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_7) => {
                let mut __terrane_handled_7 = false;
                if !__terrane_handled_7 {
                    __terrane_handled_7 = true;
                    println!("{}", terrane_scalar_support::scalar_text(&0));
                }
                if !__terrane_handled_7 {
                    return TerraneCompletion::Error(__terrane_error_7);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_7 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    let _ = outer
        .insert(
            terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(1_i128)]),
        );
    let __terrane_completion_8: TerraneCompletion<()> = (|| {
        let __terrane_try_8: TerraneCompletion<()> = (|| {
            let mut __terrane_completion_9: TerraneCompletion<()> = (|| {
                let __terrane_try_9: TerraneCompletion<()> = (|| {
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&eat(outer.as_ref()
                        .expect("flow-proven available binding").clone()))
                    );
                    return TerraneCompletion::Error({
                        let value = Err::terrane_construct();
                        let rendered = value.render();
                        let original = &value.message;
                        let (message, rendered) = if &rendered == original {
                            (rendered, None)
                        } else {
                            (original.clone(), Some(rendered))
                        };
                        TerraneError::raised_with_rendered_message(
                            TerraneErrorKind::Custom(DescriptorId(0)),
                            message,
                            rendered,
                            10 /* terrane-site: case.trn:83:13-83:32 */,
                        )
                    });
                })();
                match __terrane_try_9 {
                    TerraneCompletion::Return(value) => {
                        return TerraneCompletion::Return(value);
                    }
                    TerraneCompletion::Break => return TerraneCompletion::Break,
                    TerraneCompletion::Continue => return TerraneCompletion::Continue,
                    TerraneCompletion::Normal => {}
                    TerraneCompletion::Error(__terrane_error_9) => {
                        let mut __terrane_handled_9 = false;
                        if !__terrane_handled_9 {
                            return TerraneCompletion::Error(__terrane_error_9);
                        }
                    }
                }
                TerraneCompletion::Normal
            })();
            let __terrane_finally_9: TerraneCompletion<()> = (|| {
                let _ = outer
                    .insert(
                        terrane_collection_support::List::<
                            terrane_int_support::Int,
                        >::new(vec![terrane_int_support::Int::from(2_i128)]),
                    );
                TerraneCompletion::Normal
            })();
            match __terrane_finally_9 {
                TerraneCompletion::Normal => {}
                replacement => __terrane_completion_9 = replacement,
            }
            match __terrane_completion_9 {
                TerraneCompletion::Normal => {
                    __terrane_generated_defect("non-fallthrough try completed normally")
                }
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Error(error) => return TerraneCompletion::Error(error),
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
            }
        })();
        match __terrane_try_8 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_8) => {
                let mut __terrane_handled_8 = false;
                if !__terrane_handled_8 {
                    __terrane_handled_8 = true;
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&__terrane_raised_completion!(outer
                        .as_ref().expect("flow-proven available binding")
                        .get_or_error(__terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
                        11 /* terrane-site: case.trn:87:16-87:24 */)),
                        11 /* terrane-site: case.trn:87:16-87:24 */))
                    );
                }
                if !__terrane_handled_8 {
                    return TerraneCompletion::Error(__terrane_error_8);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_8 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(outer.as_ref()
        .expect("flow-proven available binding")
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        12 /* terrane-site: case.trn:88:12-88:20 */)), 12 /* terrane-site: case.trn:88:12-88:20 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&restored_and_read(terrane_collection_support::List::
        < terrane_int_support::Int >
        ::new(vec![terrane_int_support::Int::from(1_i128)])))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&nested_restored(terrane_collection_support::List::
        < terrane_int_support::Int >
        ::new(vec![terrane_int_support::Int::from(1_i128)])))
    );
}
