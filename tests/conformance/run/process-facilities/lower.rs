// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: conformance/process-facilities
fn main() {
    let actual: terrane_collection_support::List<NativeString>;
    let ambient: terrane_collection_support::List<EnvironmentEntry>;
    let schema_entries: terrane_collection_support::List<String>;
    let schema: CliSchema;
    let supplied: terrane_collection_support::List<NativeString>;
    let parsed: CommandLine;
    let success_code: i64;
    let invalid_code: i64;
    let success: ExitStatus;
    let invalid: ExitStatus;
    actual = arguments();
    println!(
        "{}{}{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(actual
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(actual
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        0 /* terrane-site: case.trn:9:27-9:36 */)), 0 /* terrane-site: case.trn:9:27-9:36 */).text),
        terrane_scalar_support::scalar_text(&__terrane_raised(actual
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        1 /* terrane-site: case.trn:9:43-9:52 */)), 1 /* terrane-site: case.trn:9:43-9:52 */).text),
        terrane_scalar_support::scalar_text(&__terrane_raised(actual
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        2 /* terrane-site: case.trn:9:59-9:68 */)), 2 /* terrane-site: case.trn:9:59-9:68 */).is_text),
        terrane_scalar_support::scalar_text(&(__terrane_raised(actual
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        3 /* terrane-site: case.trn:9:78-9:87 */)), 3 /* terrane-site: case.trn:9:78-9:87 */).raw.len() as i128))
    );
    ambient = environment();
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(terrane_int_support::Int::from(ambient
        .length())) > terrane_int_support::Int::from(0_i128)))
    );
    schema_entries = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("flag:--verbose"), String::from("value:--output")]);
    schema = CliSchema::terrane_construct(schema_entries);
    supplied = terrane_collection_support::List::<
        NativeString,
    >::new(
        vec![
            NativeString::terrane_construct(String::from("text:--verbose")),
            NativeString::terrane_construct(String::from("text:--output")),
            NativeString::terrane_construct(String::from("text:result.txt")),
            NativeString::terrane_construct(String::from("text:input.trn")),
            NativeString::terrane_construct(String::from("text:--unknown")),
            NativeString::terrane_construct(String::from("text:--output"))
        ],
    );
    parsed = parse_command_line(schema, supplied);
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(parsed.flags
        .length())),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(parsed
        .option_names.length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(parsed.option_values
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        4 /* terrane-site: case.trn:16:61-16:84 */)), 4 /* terrane-site: case.trn:16:61-16:84 */).text)
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(parsed
        .positionals.length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(parsed.positionals
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        5 /* terrane-site: case.trn:17:39-17:60 */)), 5 /* terrane-site: case.trn:17:39-17:60 */).text)
    );
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(parsed
        .diagnostic_arguments.length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(parsed.diagnostic_arguments
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        6 /* terrane-site: case.trn:18:48-18:78 */)), 6 /* terrane-site: case.trn:18:48-18:78 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(parsed.diagnostic_messages
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        7 /* terrane-site: case.trn:18:80-18:109 */)), 7 /* terrane-site: case.trn:18:80-18:109 */))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(parsed
        .diagnostic_arguments
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        8 /* terrane-site: case.trn:19:12-19:42 */)), 8 /* terrane-site: case.trn:19:12-19:42 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(parsed.diagnostic_messages
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        9 /* terrane-site: case.trn:19:44-19:73 */)), 9 /* terrane-site: case.trn:19:44-19:73 */))
    );
    success_code = 0;
    invalid_code = 256;
    success = make_exit_status(terrane_int_support::Int::from(success_code as i128));
    invalid = make_exit_status(terrane_int_support::Int::from(invalid_code as i128));
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&success.valid),
        terrane_scalar_support::scalar_text(&success.code),
        terrane_scalar_support::scalar_text(&invalid.valid),
        terrane_scalar_support::scalar_text(&invalid.code)
    );
}
