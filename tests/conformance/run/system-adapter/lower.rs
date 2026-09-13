// Generated deterministically by Terrane <version>.
// Runtime support: platform_result_type.rs, platform_process.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support, terrane-platform-support
// Source: src/main.trn
// Namespace: system-adapter
fn main() {
    let name: ProcessHostNameResult = process_host_name();
    println!(
        "{}", terrane_scalar_support::scalar_text(&(name.failed || name.available))
    );
}
// Source: core/process.trn
// Namespace: core/process
#[derive(Clone)]
pub struct NativeString {
    pub is_text: bool,
    pub text: String,
    pub raw: Vec<u8>,
}
impl NativeString {
    pub fn terrane_construct(encoded: String) -> Self {
        let mut value = Self {
            is_text: true,
            text: String::from(""),
            raw: Vec::from([]),
        };
        value.construct(encoded);
        value
    }
    pub fn construct(&mut self, encoded: String) {
        self.is_text = terrane_platform_value_is_text(&encoded);
        self.text = terrane_platform_value_text(&encoded);
        self.raw = terrane_platform_value_bytes(&encoded);
    }
}
#[derive(Clone)]
pub struct EnvironmentEntry {
    pub name: NativeString,
    pub value: NativeString,
}
impl EnvironmentEntry {
    pub fn terrane_construct(name: NativeString, entry_value: NativeString) -> Self {
        let mut value = Self {
            name: NativeString::terrane_construct(String::from("text:")),
            value: NativeString::terrane_construct(String::from("text:")),
        };
        value.construct(name, entry_value);
        value
    }
    pub fn construct(&mut self, name: NativeString, entry_value: NativeString) {
        self.name = name.clone();
        self.value = entry_value.clone();
    }
}
#[derive(Clone)]
pub struct ProcessHostNameResult {
    pub failed: bool,
    pub available: bool,
    pub message: String,
    pub value: NativeString,
}
impl ProcessHostNameResult {
    pub fn terrane_construct(
        did_fail: bool,
        is_available: bool,
        detail: String,
        result_value: NativeString,
    ) -> Self {
        let mut value = Self {
            failed: false,
            available: false,
            message: String::from(""),
            value: NativeString::terrane_construct(String::from("text:")),
        };
        value.construct(did_fail, is_available, detail, result_value);
        value
    }
    pub fn construct(
        &mut self,
        did_fail: bool,
        is_available: bool,
        detail: String,
        result_value: NativeString,
    ) {
        self.failed = did_fail;
        self.available = is_available;
        self.message = detail;
        self.value = result_value.clone();
    }
}
pub fn process_host_name() -> ProcessHostNameResult {
    let raw: TerranePlatformResult = terrane_platform_support::system_host_name();
    return ProcessHostNameResult::terrane_construct(
        raw.failed,
        raw.flag,
        raw.message.clone(),
        NativeString::terrane_construct(raw.text.clone()),
    );
}
pub fn arguments() -> terrane_collection_support::List<NativeString> {
    let encoded: Vec<String> = terrane_process_arguments();
    let mut values: terrane_collection_support::List<NativeString> = terrane_collection_support::List::<
        NativeString,
    >::new(Vec::new());
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_0 = values.make_unique();
        while index.clone() < terrane_int_support::Int::from(encoded.len() as i128) {
            __terrane_list_append_0
                .push(
                    NativeString::terrane_construct(
                        __terrane_raised(
                            encoded
                                .get(
                                    __terrane_raised(
                                        terrane_collection_support::index_from_int(&index.clone()),
                                        0 /* terrane-site: core/process.trn:44:49-44:63 */,
                                    ),
                                )
                                .cloned()
                                .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                                    __terrane_raised(
                                        terrane_collection_support::index_from_int(&index.clone()),
                                        0 /* terrane-site: core/process.trn:44:49-44:63 */,
                                    ),
                                )),
                            0 /* terrane-site: core/process.trn:44:49-44:63 */,
                        ),
                    ),
                );
            index = index.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    return values.clone();
}
pub fn environment() -> terrane_collection_support::List<EnvironmentEntry> {
    let encoded: Vec<String> = terrane_environment_entries();
    let mut values: terrane_collection_support::List<EnvironmentEntry> = terrane_collection_support::List::<
        EnvironmentEntry,
    >::new(Vec::new());
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_1 = values.make_unique();
        while index.clone() + terrane_int_support::Int::from(1_i128)
            < terrane_int_support::Int::from(encoded.len() as i128)
        {
            let name: NativeString = NativeString::terrane_construct(
                __terrane_raised(
                    encoded
                        .get(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(&index.clone()),
                                1 /* terrane-site: core/process.trn:53:40-53:54 */,
                            ),
                        )
                        .cloned()
                        .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(&index.clone()),
                                1 /* terrane-site: core/process.trn:53:40-53:54 */,
                            ),
                        )),
                    1 /* terrane-site: core/process.trn:53:40-53:54 */,
                ),
            );
            let value: NativeString = NativeString::terrane_construct(
                __terrane_raised(
                    encoded
                        .get(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(
                                    &(index.clone() + terrane_int_support::Int::from(1_i128)),
                                ),
                                2 /* terrane-site: core/process.trn:54:41-54:59 */,
                            ),
                        )
                        .cloned()
                        .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(
                                    &(index.clone() + terrane_int_support::Int::from(1_i128)),
                                ),
                                2 /* terrane-site: core/process.trn:54:41-54:59 */,
                            ),
                        )),
                    2 /* terrane-site: core/process.trn:54:41-54:59 */,
                ),
            );
            __terrane_list_append_1
                .push(EnvironmentEntry::terrane_construct(name, value));
            index = index.clone() + terrane_int_support::Int::from(2_i128);
        }
    }
    return values.clone();
}
#[derive(Clone)]
pub struct CliSchema {
    pub entries: terrane_collection_support::List<String>,
}
impl CliSchema {
    pub fn terrane_construct(
        declared: terrane_collection_support::List<String>,
    ) -> Self {
        let mut value = Self {
            entries: terrane_collection_support::List::<String>::new(Vec::new()),
        };
        value.construct(declared);
        value
    }
    pub fn construct(&mut self, declared: terrane_collection_support::List<String>) {
        self.entries = declared.clone();
    }
}
#[derive(Clone)]
pub struct CommandLine {
    pub flags: terrane_collection_support::List<String>,
    pub option_names: terrane_collection_support::List<String>,
    pub option_values: terrane_collection_support::List<NativeString>,
    pub positionals: terrane_collection_support::List<NativeString>,
    pub diagnostic_arguments: terrane_collection_support::List<terrane_int_support::Int>,
    pub diagnostic_messages: terrane_collection_support::List<String>,
}
impl CommandLine {
    pub fn terrane_construct() -> Self {
        Self {
            flags: terrane_collection_support::List::<String>::new(Vec::new()),
            option_names: terrane_collection_support::List::<String>::new(Vec::new()),
            option_values: terrane_collection_support::List::<
                NativeString,
            >::new(Vec::new()),
            positionals: terrane_collection_support::List::<
                NativeString,
            >::new(Vec::new()),
            diagnostic_arguments: terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(Vec::new()),
            diagnostic_messages: terrane_collection_support::List::<
                String,
            >::new(Vec::new()),
        }
    }
}
pub fn schema_has(schema: CliSchema, sought: String) -> bool {
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &schema.entries,
    );
    loop {
        let entry = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        if entry == sought {
            return true;
        }
    }
    return false;
}
pub fn parse_command_line(
    schema: CliSchema,
    supplied: terrane_collection_support::List<NativeString>,
) -> CommandLine {
    let mut flags: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    let mut option_names: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    let mut option_values: terrane_collection_support::List<NativeString> = terrane_collection_support::List::<
        NativeString,
    >::new(Vec::new());
    let mut positionals: terrane_collection_support::List<NativeString> = terrane_collection_support::List::<
        NativeString,
    >::new(Vec::new());
    let mut diagnostic_arguments: terrane_collection_support::List<
        terrane_int_support::Int,
    > = terrane_collection_support::List::<terrane_int_support::Int>::new(Vec::new());
    let mut diagnostic_messages: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_2 = diagnostic_arguments.make_unique();
        let __terrane_list_append_3 = diagnostic_messages.make_unique();
        let __terrane_list_append_4 = flags.make_unique();
        let __terrane_list_append_5 = option_names.make_unique();
        let __terrane_list_append_6 = option_values.make_unique();
        let __terrane_list_append_7 = positionals.make_unique();
        while index.clone()
            < terrane_int_support::Int::from(
                terrane_int_support::Int::from(supplied.length()),
            )
        {
            let argument: NativeString = __terrane_raised(
                supplied
                    .get_or_error(
                        __terrane_raised(
                            terrane_collection_support::index_from_int(&index.clone()),
                            3 /* terrane-site: core/process.trn:89:20-89:35 */,
                        ),
                    ),
                3 /* terrane-site: core/process.trn:89:20-89:35 */,
            );
            if !argument.is_text {
                __terrane_list_append_2.push(index.clone());
                __terrane_list_append_3
                    .push(String::from("command-line option is not Unicode text"));
            } else {
                let flag_entry: String = format!(
                    "{}{}", terrane_scalar_support::scalar_text(&String::from("flag:")),
                    terrane_scalar_support::scalar_text(&argument.text)
                );
                let value_entry: String = format!(
                    "{}{}", terrane_scalar_support::scalar_text(&String::from("value:")),
                    terrane_scalar_support::scalar_text(&argument.text)
                );
                if schema_has(schema.clone(), flag_entry) {
                    __terrane_list_append_4.push(argument.text.clone());
                } else if schema_has(schema.clone(), value_entry) {
                    if index.clone() + terrane_int_support::Int::from(1_i128)
                        >= terrane_int_support::Int::from(
                            terrane_int_support::Int::from(supplied.length()),
                        )
                    {
                        __terrane_list_append_2.push(index.clone());
                        __terrane_list_append_3
                            .push(String::from("option requires a value"));
                    } else {
                        __terrane_list_append_5.push(argument.text.clone());
                        __terrane_list_append_6
                            .push(
                                __terrane_raised(
                                    supplied
                                        .get_or_error(
                                            __terrane_raised(
                                                terrane_collection_support::index_from_int(
                                                    &(index.clone() + terrane_int_support::Int::from(1_i128)),
                                                ),
                                                4 /* terrane-site: core/process.trn:104:43-104:62 */,
                                            ),
                                        ),
                                    4 /* terrane-site: core/process.trn:104:43-104:62 */,
                                ),
                            );
                        index = index.clone() + terrane_int_support::Int::from(1_i128);
                    }
                } else if argument.text.starts_with(&String::from("--")) {
                    __terrane_list_append_2.push(index.clone());
                    __terrane_list_append_3.push(String::from("unknown option"));
                } else {
                    __terrane_list_append_7.push(argument.clone());
                }
            }
            index = index.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    let mut result: CommandLine = CommandLine::terrane_construct();
    result.flags = flags.clone();
    result.option_names = option_names.clone();
    result.option_values = option_values.clone();
    result.positionals = positionals.clone();
    result.diagnostic_arguments = diagnostic_arguments.clone();
    result.diagnostic_messages = diagnostic_messages.clone();
    return result.clone();
}
#[derive(Clone)]
pub struct ExitStatus {
    pub code: terrane_int_support::Int,
    pub valid: bool,
}
impl ExitStatus {
    pub fn terrane_construct() -> Self {
        Self {
            code: terrane_int_support::Int::from(0_i128),
            valid: true,
        }
    }
}
pub fn make_exit_status(requested: terrane_int_support::Int) -> ExitStatus {
    let mut result: ExitStatus = ExitStatus::terrane_construct();
    if requested.clone() < terrane_int_support::Int::from(0_i128)
        || requested.clone() > terrane_int_support::Int::from(255_i128)
    {
        result.code = terrane_int_support::Int::from(255_i128);
        result.valid = false;
    } else {
        result.code = requested.clone();
    }
    return result.clone();
}
pub fn exit(status: ExitStatus) {
    terrane_process_exit(status.code.clone());
}
