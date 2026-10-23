// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn contextual() -> terrane_int_support::Int {
    return __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let __terrane_macro_result: i64 = core::convert::Into::into(
                    witness::sum!(20_i64, 22_i64),
                );
                __terrane_macro_result
            }),
        ) {
            Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "dependency::ForeignType::sum",
                    ),
                )
            }
        },
        0 /* terrane-site: src/main.trn:8:12-8:23 */,
    );
}
fn main() {
    let empty: terrane_int_support::Int;
    let total: terrane_int_support::Int;
    let nested: terrane_int_support::Int;
    let mut record: Packet;
    let observed: terrane_int_support::Int;
    let text: String;
    let sequence: terrane_collection_support::List<terrane_int_support::Int>;
    let mut value: terrane_int_support::Int;
    let repeated: terrane_int_support::Int;
    let chosen: terrane_int_support::Int;
    let combined: terrane_int_support::Int;
    let empty_values: terrane_collection_support::List<terrane_int_support::Int>;
    empty = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let __terrane_macro_result: i64 = core::convert::Into::into(
                    witness::sum!(),
                );
                __terrane_macro_result
            }),
        ) {
            Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "dependency::ForeignType::sum",
                    ),
                )
            }
        },
        1 /* terrane-site: src/main.trn:11:17-11:21 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&empty));
    total = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let __terrane_macro_result: i64 = core::convert::Into::into(
                    witness::sum!(1_i64, 2_i64, 3_i64),
                );
                __terrane_macro_result
            }),
        ) {
            Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "dependency::ForeignType::sum",
                    ),
                )
            }
        },
        2 /* terrane-site: src/main.trn:13:17-13:29 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&total));
    nested = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let __terrane_macro_result: i64 = core::convert::Into::into(
                    witness::sum!(witness::sum!(2_i64, 3_i64), 4_i64),
                );
                __terrane_macro_result
            }),
        ) {
            Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "dependency::ForeignType::sum",
                    ),
                )
            }
        },
        3 /* terrane-site: src/main.trn:15:18-15:37 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&nested));
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(sum_terrane_deps_witness(),
        4 /* terrane-site: src/main.trn:17:13-17:26 */))
    );
    record = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let __terrane_macro_result: witness::Packet = core::convert::Into::into(
                    witness::packet!(7_i64, true),
                );
                __terrane_macro_result
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "dependency::ForeignType::packet",
                    ),
                )
            }
        },
        5 /* terrane-site: src/main.trn:18:21-18:36 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&record.value));
    println!("{}", terrane_scalar_support::scalar_text(&record.enabled));
    observed = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let __terrane_macro_result: i64 = core::convert::Into::into(
                    witness::disable!(&mut record),
                );
                __terrane_macro_result
            }),
        ) {
            Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "dependency::ForeignType::disable",
                    ),
                )
            }
        },
        6 /* terrane-site: src/main.trn:21:20-21:39 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&observed));
    println!("{}", terrane_scalar_support::scalar_text(&record.enabled));
    text = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let __terrane_macro_result: String = core::convert::Into::into(
                    witness::message!("{}:{}", 7_i64, true),
                );
                __terrane_macro_result
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "dependency::ForeignType::message",
                    ),
                )
            }
        },
        7 /* terrane-site: src/main.trn:24:19-24:44 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&text));
    sequence = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let __terrane_macro_result: std::vec::Vec<i64> = core::convert::Into::into(
                    witness::values!(3_i64, 5_i64, 8_i64),
                );
                __terrane_macro_result
            }),
        ) {
            Ok(value) => {
                Ok(
                    terrane_collection_support::List::new(
                        value
                            .into_iter()
                            .map(|item| terrane_int_support::Int::from(i128::from(item)))
                            .collect(),
                    ),
                )
            }
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "dependency::ForeignType::values",
                    ),
                )
            }
        },
        8 /* terrane-site: src/main.trn:26:28-26:43 */,
    );
    let __terrane_iterable_0 = sequence;
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        value = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&value));
    }
    repeated = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let __terrane_macro_result: i64 = core::convert::Into::into(
                    witness::twice!(
                        match | | -> Result < _, crate ::TerraneForeignError > {
                        Ok(terrane_int_support::coerce:: < i64 >
                        (&__terrane_raised(next(), 9 /* terrane-site: src/main.trn:29:28-29:33 */)).map_err(| error | crate
                        ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
                        ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
                        std::panic::panic_any(error) }
                    ),
                );
                __terrane_macro_result
            }),
        ) {
            Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "dependency::ForeignType::twice",
                    ),
                )
            }
        },
        10 /* terrane-site: src/main.trn:29:20-29:34 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&repeated));
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(evaluations(),
        11 /* terrane-site: src/main.trn:31:13-31:25 */))
    );
    chosen = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let __terrane_macro_result: i64 = core::convert::Into::into(
                    witness::selected!(
                        false, match | | -> Result < _, crate ::TerraneForeignError > {
                        Ok(terrane_int_support::coerce:: < i64 >
                        (&__terrane_raised(next(), 12 /* terrane-site: src/main.trn:32:36-32:41 */)).map_err(| error | crate
                        ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
                        ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
                        std::panic::panic_any(error) }, 17_i64
                    ),
                );
                __terrane_macro_result
            }),
        ) {
            Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "dependency::ForeignType::selected",
                    ),
                )
            }
        },
        13 /* terrane-site: src/main.trn:32:18-32:46 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&chosen));
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(evaluations(),
        14 /* terrane-site: src/main.trn:34:13-34:25 */))
    );
    println!("{}", terrane_scalar_support::scalar_text(&contextual()));
    combined = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let __terrane_macro_result: i64 = core::convert::Into::into(
                    witness::layout::combine!(4_i64, 6_i64),
                );
                __terrane_macro_result
            }),
        ) {
            Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "dependency::ForeignType::combine",
                    ),
                )
            }
        },
        15 /* terrane-site: src/main.trn:36:20-36:33 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&combined));
    empty_values = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let __terrane_macro_result: std::vec::Vec<i64> = core::convert::Into::into(
                    witness::values!(),
                );
                __terrane_macro_result
            }),
        ) {
            Ok(value) => {
                Ok(
                    terrane_collection_support::List::new(
                        value
                            .into_iter()
                            .map(|item| terrane_int_support::Int::from(i128::from(item)))
                            .collect(),
                    ),
                )
            }
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "dependency::ForeignType::values",
                    ),
                )
            }
        },
        16 /* terrane-site: src/main.trn:38:32-38:39 */,
    );
    let __terrane_iterable_1 = empty_values;
    let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_1,
    );
    loop {
        value = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &value;
        println!("{}", terrane_scalar_support::scalar_text(&value));
    }
    println!("{}", String::from("empty collection selected"));
}
