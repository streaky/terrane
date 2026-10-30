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
        "{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_index =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        2 /* terrane-site: src/main.trn:17:13-17:23 */); let
        __terrane_string_argument_0 = &String::from("\"origin\":\"dependency\"");
        __terrane_raised(records.get(__terrane_index).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index)),
        2 /* terrane-site: src/main.trn:17:13-17:23 */).contains(std::convert::AsRef::
        < str > ::as_ref(&__terrane_string_argument_0)) }),
        terrane_scalar_support::scalar_text(&{ let __terrane_index =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        3 /* terrane-site: src/main.trn:17:61-17:71 */); let
        __terrane_string_argument_0 = &String::from("\"target\":\"dependency.log\"");
        __terrane_raised(records.get(__terrane_index).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index)),
        3 /* terrane-site: src/main.trn:17:61-17:71 */).contains(std::convert::AsRef::
        < str > ::as_ref(&__terrane_string_argument_0)) }),
        terrane_scalar_support::scalar_text(&{ let __terrane_index =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        4 /* terrane-site: src/main.trn:17:113-17:123 */); let
        __terrane_string_argument_0 = &String::from("cache miss");
        __terrane_raised(records.get(__terrane_index).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index)),
        4 /* terrane-site: src/main.trn:17:113-17:123 */)
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        })
    );
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_index =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        5 /* terrane-site: src/main.trn:18:13-18:23 */); let
        __terrane_string_argument_0 = &String::from("\"name\":\"item\"");
        __terrane_raised(records.get(__terrane_index).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index)),
        5 /* terrane-site: src/main.trn:18:13-18:23 */).contains(std::convert::AsRef::
        < str > ::as_ref(&__terrane_string_argument_0)) }),
        terrane_scalar_support::scalar_text(&{ let __terrane_index =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        6 /* terrane-site: src/main.trn:18:53-18:63 */); let
        __terrane_string_argument_0 = &String::from("\"value\":3");
        __terrane_raised(records.get(__terrane_index).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index)),
        6 /* terrane-site: src/main.trn:18:53-18:63 */).contains(std::convert::AsRef::
        < str > ::as_ref(&__terrane_string_argument_0)) }),
        terrane_scalar_support::scalar_text(&{ let __terrane_index =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        7 /* terrane-site: src/main.trn:18:89-18:99 */); let
        __terrane_string_argument_0 = &String::from("observability_witness");
        __terrane_raised(records.get(__terrane_index).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index)),
        7 /* terrane-site: src/main.trn:18:89-18:99 */).contains(std::convert::AsRef::
        < str > ::as_ref(&__terrane_string_argument_0)) }),
        terrane_scalar_support::scalar_text(&{ let __terrane_index =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        8 /* terrane-site: src/main.trn:18:137-18:147 */); let
        __terrane_string_argument_0 = &String::from("src/lib.rs:2");
        __terrane_raised(records.get(__terrane_index).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index)),
        8 /* terrane-site: src/main.trn:18:137-18:147 */)
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        })
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_index =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        9 /* terrane-site: src/main.trn:19:13-19:23 */); let
        __terrane_string_argument_0 = &String::from("\"origin\":\"dependency\"");
        __terrane_raised(records.get(__terrane_index).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index)),
        9 /* terrane-site: src/main.trn:19:13-19:23 */).contains(std::convert::AsRef::
        < str > ::as_ref(&__terrane_string_argument_0)) }),
        terrane_scalar_support::scalar_text(&{ let __terrane_index =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        10 /* terrane-site: src/main.trn:19:61-19:71 */); let
        __terrane_string_argument_0 = &String::from("\"target\":\"dependency.trace\"");
        __terrane_raised(records.get(__terrane_index).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index)),
        10 /* terrane-site: src/main.trn:19:61-19:71 */)
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        }), terrane_scalar_support::scalar_text(&{ let __terrane_index =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        11 /* terrane-site: src/main.trn:19:115-19:125 */); let
        __terrane_string_argument_0 = &String::from("loaded"); __terrane_raised(records
        .get(__terrane_index).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index)),
        11 /* terrane-site: src/main.trn:19:115-19:125 */)
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        })
    );
    println!(
        "{}{}{}{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_index =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        12 /* terrane-site: src/main.trn:20:13-20:23 */); let
        __terrane_string_argument_0 = &String::from("\"name\":\"item\"");
        __terrane_raised(records.get(__terrane_index).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index)),
        12 /* terrane-site: src/main.trn:20:13-20:23 */)
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        }), terrane_scalar_support::scalar_text(&{ let __terrane_index =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        13 /* terrane-site: src/main.trn:20:53-20:63 */); let
        __terrane_string_argument_0 = &String::from("\"value\":7");
        __terrane_raised(records.get(__terrane_index).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index)),
        13 /* terrane-site: src/main.trn:20:53-20:63 */)
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        }), terrane_scalar_support::scalar_text(&{ let __terrane_index =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        14 /* terrane-site: src/main.trn:20:89-20:99 */); let
        __terrane_string_argument_0 = &String::from("\"spans\":[\"load_batch\"]");
        __terrane_raised(records.get(__terrane_index).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index)),
        14 /* terrane-site: src/main.trn:20:89-20:99 */)
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        }), terrane_scalar_support::scalar_text(&{ let __terrane_index =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        15 /* terrane-site: src/main.trn:20:138-20:148 */); let
        __terrane_string_argument_0 =
        &String::from("\"name\":\"span.load_batch.batch\""); __terrane_raised(records
        .get(__terrane_index).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index)),
        15 /* terrane-site: src/main.trn:20:138-20:148 */)
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        }), terrane_scalar_support::scalar_text(&{ let __terrane_index =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        16 /* terrane-site: src/main.trn:20:195-20:205 */); let
        __terrane_string_argument_0 = &String::from("\"value\":11");
        __terrane_raised(records.get(__terrane_index).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index)),
        16 /* terrane-site: src/main.trn:20:195-20:205 */)
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        }), terrane_scalar_support::scalar_text(&{ let __terrane_index =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        17 /* terrane-site: src/main.trn:20:232-20:242 */); let
        __terrane_string_argument_0 = &String::from("src/lib.rs:8");
        __terrane_raised(records.get(__terrane_index).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index)),
        17 /* terrane-site: src/main.trn:20:232-20:242 */)
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        })
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_index =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        18 /* terrane-site: src/main.trn:21:13-21:23 */); let
        __terrane_string_argument_0 = &String::from("case.trn"); __terrane_raised(records
        .get(__terrane_index).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index)),
        18 /* terrane-site: src/main.trn:21:13-21:23 */)
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        }), terrane_scalar_support::scalar_text(&{ let __terrane_index =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        19 /* terrane-site: src/main.trn:21:48-21:58 */); let
        __terrane_string_argument_0 = &String::from("case.trn"); __terrane_raised(records
        .get(__terrane_index).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index)),
        19 /* terrane-site: src/main.trn:21:48-21:58 */)
        .contains(std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0))
        })
    );
}
