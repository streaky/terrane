// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/app/main.trn
// Namespace: annotation-cli-consumer
fn greet(name: String, limit: terrane_int_support::Int) -> terrane_int_support::Int {
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&name),
        terrane_scalar_support::scalar_text(&(limit.clone() *
        terrane_int_support::Int::from(2_i128)))
    );
    return terrane_int_support::Int::from(0_i128);
}
fn main() {
    let status: terrane_int_support::Int;
    status = run_command();
    if status.clone() != terrane_int_support::Int::from(0_i128) {
        __terrane_uncaught(
            TerraneError::raised(
                TerraneErrorKind::CoercionError,
                0 /* terrane-site: src/app/main.trn:21:9-21:29 */,
            ),
        );
    }
}
// Source: src/cli-schema/main.trn
// Namespace: annotation-cli
#[derive(Clone)]
pub struct Command {
    pub name: Option<String>,
    pub summary: Option<String>,
}
impl Command {
    pub fn terrane_construct() -> Self {
        Self { name: None, summary: None }
    }
}
#[derive(Clone)]
pub struct CommandOption {
    pub long_name: String,
    pub help: Option<String>,
}
impl CommandOption {
    pub fn terrane_construct() -> Self {
        Self {
            long_name: String::new(),
            help: None,
        }
    }
}
// Source: .trn/generated/cli/cli-binder.trn
// Namespace: generated/annotation-cli
fn run_command() -> terrane_int_support::Int {
    let user_arguments: terrane_collection_support::List<NativeString>;
    let mut schema_entries: terrane_collection_support::List<String>;
    let schema: CliSchema;
    let parsed: CommandLine;
    let mut index: terrane_int_support::Int;
    let mut name: String;
    let mut seen_name: bool;
    let mut limit: terrane_int_support::Int;
    let mut option_index: terrane_int_support::Int;
    let mut option_name: String;
    let mut option_value: NativeString;
    user_arguments = arguments();
    schema_entries = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("flag:--help")]);
    {
        let __terrane_collection_argument_0 = String::from("value:--name");
        schema_entries.append(__terrane_collection_argument_0);
    };
    {
        let __terrane_collection_argument_0 = String::from("value:--limit");
        schema_entries.append(__terrane_collection_argument_0);
    };
    schema = CliSchema::terrane_construct(schema_entries);
    parsed = parse_command_line(schema, user_arguments);
    if terrane_int_support::Int::from(
        terrane_int_support::Int::from(parsed.diagnostic_messages.length()),
    ) > terrane_int_support::Int::from(0_i128)
    {
        index = terrane_int_support::Int::from(0_i128);
        while index.clone()
            < terrane_int_support::Int::from(
                terrane_int_support::Int::from(parsed.diagnostic_messages.length()),
            )
        {
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised(parsed
                .diagnostic_messages
                .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&index
                .clone()), 1 /* terrane-site: .trn/generated/cli/cli-binder.trn:18:20-18:53 */)), 1 /* terrane-site: .trn/generated/cli/cli-binder.trn:18:20-18:53 */))
            );
            index = index.clone() + terrane_int_support::Int::from(1_i128);
        }
        return terrane_int_support::Int::from(2_i128);
    }
    if terrane_int_support::Int::from(
        terrane_int_support::Int::from(parsed.flags.length()),
    ) > terrane_int_support::Int::from(0_i128)
    {
        println!("{}", String::from("Usage: greet [options]"));
        println!("{}", String::from("Configured command summary"));
        println!("{}", String::from("  --name <string>  Name to greet"));
        println!("{}", String::from("  --limit <int> (default: 10)  Greeting limit"));
        println!("{}", String::from("  --help  Show this help"));
        return terrane_int_support::Int::from(0_i128);
    }
    name = String::from("");
    seen_name = false;
    limit = terrane_int_support::Int::from(10_i128);
    option_index = terrane_int_support::Int::from(0_i128);
    while option_index.clone()
        < terrane_int_support::Int::from(
            terrane_int_support::Int::from(parsed.option_names.length()),
        )
    {
        option_name = __terrane_raised(
            parsed
                .option_names
                .get_or_error(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &option_index.clone(),
                        ),

                            2 /* terrane-site: .trn/generated/cli/cli-binder.trn:33:23-33:56 */,
                    ),
                ),
            2 /* terrane-site: .trn/generated/cli/cli-binder.trn:33:23-33:56 */,
        );
        option_value = __terrane_raised(
            parsed
                .option_values
                .get_or_error(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &option_index.clone(),
                        ),

                            3 /* terrane-site: .trn/generated/cli/cli-binder.trn:34:24-34:58 */,
                    ),
                ),
            3 /* terrane-site: .trn/generated/cli/cli-binder.trn:34:24-34:58 */,
        );
        if option_name.as_str() == "--name" {
            name = option_value.text.clone();
            seen_name = true;
        }
        if option_name.as_str() == "--limit" {
            let __terrane_completion_0: TerraneCompletion<terrane_int_support::Int> = (||
            {
                let __terrane_try_0: TerraneCompletion<terrane_int_support::Int> = (|| {
                    limit = __terrane_raised_completion!(
                        terrane_int_support::parse_radix(&option_value.text, &10),
                        4 /* terrane-site: .trn/generated/cli/cli-binder.trn:40:25-40:52 */
                    );
                    TerraneCompletion::Normal
                })();
                match __terrane_try_0 {
                    TerraneCompletion::Return(value) => {
                        return TerraneCompletion::Return(value);
                    }
                    TerraneCompletion::Break => return TerraneCompletion::Break,
                    TerraneCompletion::Continue => return TerraneCompletion::Continue,
                    TerraneCompletion::Normal => {}
                    TerraneCompletion::Error(__terrane_error_0) => {
                        let mut __terrane_handled_0 = false;
                        if !__terrane_handled_0
                            && __terrane_error_0.kind == TerraneErrorKind::CoercionError
                        {
                            __terrane_handled_0 = true;
                            println!(
                                "{}", String::from("invalid integer value for --limit")
                            );
                            return TerraneCompletion::Return(
                                terrane_int_support::Int::from(2_i128),
                            );
                        }
                        if !__terrane_handled_0 {
                            return TerraneCompletion::Error(__terrane_error_0);
                        }
                    }
                }
                TerraneCompletion::Normal
            })();
            match __terrane_completion_0 {
                TerraneCompletion::Normal => {}
                TerraneCompletion::Return(value) => return value,
                TerraneCompletion::Error(error) => __terrane_uncaught(error),
                TerraneCompletion::Break => break,
                TerraneCompletion::Continue => continue,
            }
        }
        option_index = option_index.clone() + terrane_int_support::Int::from(1_i128);
    }
    if !seen_name {
        println!("{}", String::from("missing required option --name"));
        return terrane_int_support::Int::from(2_i128);
    }
    return greet(name, limit.clone());
}
