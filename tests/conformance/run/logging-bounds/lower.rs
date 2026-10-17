// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: logging-bounds
fn main() {
    let empty: terrane_collection_support::List<LogField>;
    let dropping_sink: LogSinkResult;
    let dropping: Logger;
    let first: LogOutcome;
    let second: LogOutcome;
    let dropped_records: terrane_collection_support::List<String>;
    let rejecting_sink: LogSinkResult;
    let rejecting: Logger;
    let accepted: LogOutcome;
    let rejected: LogOutcome;
    let fallback: terrane_collection_support::List<String>;
    let one_field: terrane_collection_support::List<LogField>;
    let strict_fields: Logger;
    let too_many: LogOutcome;
    let strict_bytes: Logger;
    let too_large: LogOutcome;
    let invalid_sink: LogSinkResult;
    let negative_sink: LogSinkResult;
    let negative_limit: Logger;
    let invalid_limit: LogOutcome;
    empty = terrane_collection_support::List::<LogField>::new(vec![]);
    dropping_sink = memory_sink(
        terrane_int_support::Int::from(1_i128),
        String::from("drop-oldest"),
        terrane_int_support::Int::from(10_i128),
        terrane_int_support::Int::from(1_i128),
        false,
    );
    dropping = default_logger(dropping_sink.value.clone());
    first = emit_at(
        dropping.clone(),
        info_level(),
        String::from("first"),
        "case.trn:11:13".to_owned(),
        empty.clone(),
    );
    second = emit_at(
        dropping.clone(),
        info_level(),
        String::from("second"),
        "case.trn:12:14".to_owned(),
        empty.clone(),
    );
    dropped_records = drain_memory(dropping.clone());
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&first.failed),
        terrane_scalar_support::scalar_text(&second.failed),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(dropped_records
        .length())), terrane_scalar_support::scalar_text(&discarded_count(dropping))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(dropped_records
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        0 /* terrane-site: case.trn:15:13-15:31 */)), 0 /* terrane-site: case.trn:15:13-15:31 */).contains(&String::from("\"message\":\"second\""))),
        terrane_scalar_support::scalar_text(&__terrane_raised(dropped_records
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        1 /* terrane-site: case.trn:15:66-15:84 */)), 1 /* terrane-site: case.trn:15:66-15:84 */).contains(&String::from("\"message\":\"first\"")))
    );
    rejecting_sink = memory_sink(
        terrane_int_support::Int::from(1_i128),
        String::from("reject"),
        terrane_int_support::Int::from(20_i128),
        terrane_int_support::Int::from(1_i128),
        false,
    );
    rejecting = default_logger(rejecting_sink.value.clone());
    accepted = emit_at(
        rejecting.clone(),
        info_level(),
        String::from("accepted"),
        "case.trn:19:16".to_owned(),
        empty.clone(),
    );
    rejected = emit_at(
        rejecting,
        info_level(),
        String::from("rejected"),
        "case.trn:20:16".to_owned(),
        empty.clone(),
    );
    fallback = drain_fallback();
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&accepted.failed),
        terrane_scalar_support::scalar_text(&rejected.failed),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(fallback
        .length()))
    );
    one_field = terrane_collection_support::List::<
        LogField,
    >::new(
        vec![
            field_at(String::from("value"), log_text(String::from("bounded")), false,
            "case.trn:24:24".to_owned())
        ],
    );
    strict_fields = make_logger(
        memory_sink(
                terrane_int_support::Int::from(2_i128),
                String::from("reject"),
                terrane_int_support::Int::from(30_i128),
                terrane_int_support::Int::from(1_i128),
                false,
            )
            .value,
        make_logger_options(
            info_level(),
            String::from(""),
            String::from("bounded.fields"),
            terrane_int_support::Int::from(0_i128),
            terrane_int_support::Int::from(4096_i128),
        ),
    );
    too_many = emit_at(
        strict_fields,
        info_level(),
        String::from("too-many"),
        "case.trn:26:16".to_owned(),
        one_field,
    );
    strict_bytes = make_logger(
        memory_sink(
                terrane_int_support::Int::from(2_i128),
                String::from("reject"),
                terrane_int_support::Int::from(40_i128),
                terrane_int_support::Int::from(1_i128),
                false,
            )
            .value,
        make_logger_options(
            info_level(),
            String::from(""),
            String::from("bounded.bytes"),
            terrane_int_support::Int::from(4_i128),
            terrane_int_support::Int::from(32_i128),
        ),
    );
    too_large = emit_at(
        strict_bytes,
        info_level(),
        String::from("too-large"),
        "case.trn:28:17".to_owned(),
        empty.clone(),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&too_many.failed),
        terrane_scalar_support::scalar_text(&too_large.failed)
    );
    invalid_sink = memory_sink(
        terrane_int_support::Int::from(-1_i128),
        String::from("reject"),
        terrane_int_support::Int::from(0_i128),
        terrane_int_support::Int::from(1_i128),
        false,
    );
    println!("{}", terrane_scalar_support::scalar_text(&invalid_sink.failed));
    negative_sink = memory_sink(
        terrane_int_support::Int::from(1_i128),
        String::from("reject"),
        terrane_int_support::Int::from(50_i128),
        terrane_int_support::Int::from(1_i128),
        false,
    );
    negative_limit = make_logger(
        negative_sink.value.clone(),
        make_logger_options(
            info_level(),
            String::from(""),
            String::from("bounded.negative"),
            terrane_int_support::Int::from(-1_i128),
            terrane_int_support::Int::from(4096_i128),
        ),
    );
    invalid_limit = emit_at(
        negative_limit,
        info_level(),
        String::from("invalid-limit"),
        "case.trn:36:21".to_owned(),
        empty,
    );
    println!("{}", terrane_scalar_support::scalar_text(&invalid_limit.failed));
}
