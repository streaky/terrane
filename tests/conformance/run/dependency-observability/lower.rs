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
    writer = default_logger(sink_result.value);
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
        "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(records
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        2 /* terrane-site: src/main.trn:17:13-17:23 */)), 2 /* terrane-site: src/main.trn:17:13-17:23 */)
        .contains(&String::from("\"origin\":\"dependency\""))),
        terrane_scalar_support::scalar_text(&__terrane_raised(records
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        3 /* terrane-site: src/main.trn:17:61-17:71 */)), 3 /* terrane-site: src/main.trn:17:61-17:71 */)
        .contains(&String::from("\"target\":\"dependency.log\""))),
        terrane_scalar_support::scalar_text(&__terrane_raised(records
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        4 /* terrane-site: src/main.trn:17:113-17:123 */)), 4 /* terrane-site: src/main.trn:17:113-17:123 */).contains(&String::from("cache miss")))
    );
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(records
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        5 /* terrane-site: src/main.trn:18:13-18:23 */)), 5 /* terrane-site: src/main.trn:18:13-18:23 */).contains(&String::from("\"name\":\"item\""))),
        terrane_scalar_support::scalar_text(&__terrane_raised(records
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        6 /* terrane-site: src/main.trn:18:53-18:63 */)), 6 /* terrane-site: src/main.trn:18:53-18:63 */).contains(&String::from("\"value\":3"))),
        terrane_scalar_support::scalar_text(&__terrane_raised(records
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        7 /* terrane-site: src/main.trn:18:89-18:99 */)), 7 /* terrane-site: src/main.trn:18:89-18:99 */).contains(&String::from("observability_witness"))),
        terrane_scalar_support::scalar_text(&__terrane_raised(records
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        8 /* terrane-site: src/main.trn:18:137-18:147 */)), 8 /* terrane-site: src/main.trn:18:137-18:147 */).contains(&String::from("src/lib.rs:2")))
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(records
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        9 /* terrane-site: src/main.trn:19:13-19:23 */)), 9 /* terrane-site: src/main.trn:19:13-19:23 */)
        .contains(&String::from("\"origin\":\"dependency\""))),
        terrane_scalar_support::scalar_text(&__terrane_raised(records
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        10 /* terrane-site: src/main.trn:19:61-19:71 */)), 10 /* terrane-site: src/main.trn:19:61-19:71 */)
        .contains(&String::from("\"target\":\"dependency.trace\""))),
        terrane_scalar_support::scalar_text(&__terrane_raised(records
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        11 /* terrane-site: src/main.trn:19:115-19:125 */)), 11 /* terrane-site: src/main.trn:19:115-19:125 */).contains(&String::from("loaded")))
    );
    println!(
        "{}{}{}{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(records
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        12 /* terrane-site: src/main.trn:20:13-20:23 */)), 12 /* terrane-site: src/main.trn:20:13-20:23 */).contains(&String::from("\"name\":\"item\""))),
        terrane_scalar_support::scalar_text(&__terrane_raised(records
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        13 /* terrane-site: src/main.trn:20:53-20:63 */)), 13 /* terrane-site: src/main.trn:20:53-20:63 */).contains(&String::from("\"value\":7"))),
        terrane_scalar_support::scalar_text(&__terrane_raised(records
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        14 /* terrane-site: src/main.trn:20:89-20:99 */)), 14 /* terrane-site: src/main.trn:20:89-20:99 */)
        .contains(&String::from("\"spans\":[\"load_batch\"]"))),
        terrane_scalar_support::scalar_text(&__terrane_raised(records
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        15 /* terrane-site: src/main.trn:20:138-20:148 */)), 15 /* terrane-site: src/main.trn:20:138-20:148 */)
        .contains(&String::from("\"name\":\"span.load_batch.batch\""))),
        terrane_scalar_support::scalar_text(&__terrane_raised(records
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        16 /* terrane-site: src/main.trn:20:195-20:205 */)), 16 /* terrane-site: src/main.trn:20:195-20:205 */).contains(&String::from("\"value\":11"))),
        terrane_scalar_support::scalar_text(&__terrane_raised(records
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        17 /* terrane-site: src/main.trn:20:232-20:242 */)), 17 /* terrane-site: src/main.trn:20:232-20:242 */).contains(&String::from("src/lib.rs:8")))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(records
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        18 /* terrane-site: src/main.trn:21:13-21:23 */)), 18 /* terrane-site: src/main.trn:21:13-21:23 */).contains(&String::from("case.trn"))),
        terrane_scalar_support::scalar_text(&__terrane_raised(records
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        19 /* terrane-site: src/main.trn:21:48-21:58 */)), 19 /* terrane-site: src/main.trn:21:48-21:58 */).contains(&String::from("case.trn")))
    );
}
