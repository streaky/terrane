// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let sink_result: LogSinkResult;
    let writer: Logger;
    let installed: LogOutcome;
    let records: terrane_collection_support::List<String>;
    sink_result = memory_sink(
        terrane_int_support::Int::from(8_i128),
        String::from("reject"),
        terrane_int_support::Int::from(4000_i128),
        terrane_int_support::Int::from(2_i128),
        false,
    );
    writer = default_logger(sink_result.value.clone());
    installed = install_dependency_bridge(writer.clone());
    println!("{}", terrane_scalar_support::scalar_text(&installed.failed));
    __terrane_raised(emit_log_event(), 0 /* terrane-site: src/main.trn:12:5-12:20 */);
    __terrane_raised(
        emit_tracing_event(),
        1 /* terrane-site: src/main.trn:13:5-13:24 */,
    );
    records = drain_memory(writer);
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(records
        .length()))
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_0 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        2 /* terrane-site: src/main.trn:17:13-17:23 */); let __terrane_index_place_1 =
        __terrane_raised(records.get(__terrane_index_0).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_0)),
        2 /* terrane-site: src/main.trn:17:13-17:23 */); let
        __terrane_string_argument_0 = "\"origin\":\"dependency\"";
        __terrane_index_place_1.contains(std::convert::AsRef:: < str >
        ::as_ref(&__terrane_string_argument_0)) }),
        terrane_scalar_support::scalar_text(&{ let __terrane_index_2 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        3 /* terrane-site: src/main.trn:17:61-17:71 */); let __terrane_index_place_3 =
        __terrane_raised(records.get(__terrane_index_2).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_2)),
        3 /* terrane-site: src/main.trn:17:61-17:71 */); let
        __terrane_string_argument_0 = "\"target\":\"dependency.log\"";
        __terrane_index_place_3.contains(std::convert::AsRef:: < str >
        ::as_ref(&__terrane_string_argument_0)) }),
        terrane_scalar_support::scalar_text(&{ let __terrane_index_4 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        4 /* terrane-site: src/main.trn:17:113-17:123 */); let __terrane_index_place_5
        = __terrane_raised(records.get(__terrane_index_4).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_4)),
        4 /* terrane-site: src/main.trn:17:113-17:123 */); let
        __terrane_string_argument_0 = "cache miss"; __terrane_index_place_5
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        })
    );
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_6 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        5 /* terrane-site: src/main.trn:18:13-18:23 */); let __terrane_index_place_7 =
        __terrane_raised(records.get(__terrane_index_6).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_6)),
        5 /* terrane-site: src/main.trn:18:13-18:23 */); let
        __terrane_string_argument_0 = "\"name\":\"item\""; __terrane_index_place_7
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        }), terrane_scalar_support::scalar_text(&{ let __terrane_index_8 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        6 /* terrane-site: src/main.trn:18:53-18:63 */); let __terrane_index_place_9 =
        __terrane_raised(records.get(__terrane_index_8).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_8)),
        6 /* terrane-site: src/main.trn:18:53-18:63 */); let
        __terrane_string_argument_0 = "\"value\":3"; __terrane_index_place_9
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        }), terrane_scalar_support::scalar_text(&{ let __terrane_index_10 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        7 /* terrane-site: src/main.trn:18:89-18:99 */); let __terrane_index_place_11
        = __terrane_raised(records.get(__terrane_index_10).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_10)),
        7 /* terrane-site: src/main.trn:18:89-18:99 */); let
        __terrane_string_argument_0 = "observability_witness"; __terrane_index_place_11
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        }), terrane_scalar_support::scalar_text(&{ let __terrane_index_12 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        8 /* terrane-site: src/main.trn:18:137-18:147 */); let
        __terrane_index_place_13 = __terrane_raised(records.get(__terrane_index_12)
        .ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_12)),
        8 /* terrane-site: src/main.trn:18:137-18:147 */); let
        __terrane_string_argument_0 = "src/lib.rs:2"; __terrane_index_place_13
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        })
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_14 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        9 /* terrane-site: src/main.trn:19:13-19:23 */); let __terrane_index_place_15
        = __terrane_raised(records.get(__terrane_index_14).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_14)),
        9 /* terrane-site: src/main.trn:19:13-19:23 */); let
        __terrane_string_argument_0 = "\"origin\":\"dependency\"";
        __terrane_index_place_15.contains(std::convert::AsRef:: < str >
        ::as_ref(&__terrane_string_argument_0)) }),
        terrane_scalar_support::scalar_text(&{ let __terrane_index_16 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        10 /* terrane-site: src/main.trn:19:61-19:71 */); let __terrane_index_place_17
        = __terrane_raised(records.get(__terrane_index_16).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_16)),
        10 /* terrane-site: src/main.trn:19:61-19:71 */); let
        __terrane_string_argument_0 = "\"target\":\"dependency.trace\"";
        __terrane_index_place_17.contains(std::convert::AsRef:: < str >
        ::as_ref(&__terrane_string_argument_0)) }),
        terrane_scalar_support::scalar_text(&{ let __terrane_index_18 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        11 /* terrane-site: src/main.trn:19:115-19:125 */); let
        __terrane_index_place_19 = __terrane_raised(records.get(__terrane_index_18)
        .ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_18)),
        11 /* terrane-site: src/main.trn:19:115-19:125 */); let
        __terrane_string_argument_0 = "loaded"; __terrane_index_place_19
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        })
    );
    println!(
        "{}{}{}{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_20 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        12 /* terrane-site: src/main.trn:20:13-20:23 */); let __terrane_index_place_21
        = __terrane_raised(records.get(__terrane_index_20).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_20)),
        12 /* terrane-site: src/main.trn:20:13-20:23 */); let
        __terrane_string_argument_0 = "\"name\":\"item\""; __terrane_index_place_21
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        }), terrane_scalar_support::scalar_text(&{ let __terrane_index_22 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        13 /* terrane-site: src/main.trn:20:53-20:63 */); let __terrane_index_place_23
        = __terrane_raised(records.get(__terrane_index_22).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_22)),
        13 /* terrane-site: src/main.trn:20:53-20:63 */); let
        __terrane_string_argument_0 = "\"value\":7"; __terrane_index_place_23
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        }), terrane_scalar_support::scalar_text(&{ let __terrane_index_24 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        14 /* terrane-site: src/main.trn:20:89-20:99 */); let __terrane_index_place_25
        = __terrane_raised(records.get(__terrane_index_24).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_24)),
        14 /* terrane-site: src/main.trn:20:89-20:99 */); let
        __terrane_string_argument_0 = "\"spans\":[\"load_batch\"]";
        __terrane_index_place_25.contains(std::convert::AsRef:: < str >
        ::as_ref(&__terrane_string_argument_0)) }),
        terrane_scalar_support::scalar_text(&{ let __terrane_index_26 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        15 /* terrane-site: src/main.trn:20:138-20:148 */); let
        __terrane_index_place_27 = __terrane_raised(records.get(__terrane_index_26)
        .ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_26)),
        15 /* terrane-site: src/main.trn:20:138-20:148 */); let
        __terrane_string_argument_0 = "\"name\":\"span.load_batch.batch\"";
        __terrane_index_place_27.contains(std::convert::AsRef:: < str >
        ::as_ref(&__terrane_string_argument_0)) }),
        terrane_scalar_support::scalar_text(&{ let __terrane_index_28 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        16 /* terrane-site: src/main.trn:20:195-20:205 */); let
        __terrane_index_place_29 = __terrane_raised(records.get(__terrane_index_28)
        .ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_28)),
        16 /* terrane-site: src/main.trn:20:195-20:205 */); let
        __terrane_string_argument_0 = "\"value\":11"; __terrane_index_place_29
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        }), terrane_scalar_support::scalar_text(&{ let __terrane_index_30 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        17 /* terrane-site: src/main.trn:20:232-20:242 */); let
        __terrane_index_place_31 = __terrane_raised(records.get(__terrane_index_30)
        .ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_30)),
        17 /* terrane-site: src/main.trn:20:232-20:242 */); let
        __terrane_string_argument_0 = "src/lib.rs:8"; __terrane_index_place_31
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        })
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_32 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        18 /* terrane-site: src/main.trn:21:13-21:23 */); let __terrane_index_place_33
        = __terrane_raised(records.get(__terrane_index_32).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_32)),
        18 /* terrane-site: src/main.trn:21:13-21:23 */); let
        __terrane_string_argument_0 = "case.trn"; __terrane_index_place_33
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        }), terrane_scalar_support::scalar_text(&{ let __terrane_index_34 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        19 /* terrane-site: src/main.trn:21:48-21:58 */); let __terrane_index_place_35
        = __terrane_raised(records.get(__terrane_index_34).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_34)),
        19 /* terrane-site: src/main.trn:21:48-21:58 */); let
        __terrane_string_argument_0 = "case.trn"; __terrane_index_place_35
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        })
    );
}
