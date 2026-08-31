// Generated deterministically by Terrane 0.1.0.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TerraneErrorKind {
    ArithmeticOverflow,
    DivisionByZero,
    IntegerConversionOverflow,
    NegativeShiftCount,
    CoercionError,
    DecodeError,
    IndexError,
    MissingKey,
    ResourceError,
    SourceError,
}
impl TerraneErrorKind {
    fn from_source_name(name: &str) -> Self {
        match name {
            "arithmetic-overflow" => Self::ArithmeticOverflow,
            "division-by-zero" => Self::DivisionByZero,
            "integer-conversion-overflow" => Self::IntegerConversionOverflow,
            "negative-shift-count" => Self::NegativeShiftCount,
            "coercion-error" => Self::CoercionError,
            "decode-error" => Self::DecodeError,
            "index-error" => Self::IndexError,
            "missing-key" => Self::MissingKey,
            "resource-error" => Self::ResourceError,
            _ => Self::SourceError,
        }
    }
    fn display_name(self) -> &'static str {
        match self {
            Self::ArithmeticOverflow => "arithmetic-overflow",
            Self::DivisionByZero => "division-by-zero",
            Self::IntegerConversionOverflow => "integer-conversion-overflow",
            Self::NegativeShiftCount => "negative-shift-count",
            Self::CoercionError => "coercion-error",
            Self::DecodeError => "decode-error",
            Self::IndexError => "index-error",
            Self::MissingKey => "missing-key",
            Self::ResourceError => "resource-error",
            Self::SourceError => "error",
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerraneError {
    kind: TerraneErrorKind,
    message: String,
    cause: Option<Box<TerraneError>>,
    context: Vec<&'static str>,
}
impl TerraneError {
    fn new(kind: TerraneErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            cause: None,
            context: Vec::new(),
        }
    }
    #[allow(dead_code)]
    fn at(mut self, frame: &'static str) -> Self {
        self.context.push(frame);
        self
    }
    fn render(&self) -> String {
        let mut rendered = format!("{}: {}", self.kind.display_name(), self.message);
        if let Some(cause) = &self.cause {
            rendered.push_str("\ncaused by: ");
            rendered.push_str(&cause.render());
        }
        for frame in &self.context {
            rendered.push_str("\nat ");
            rendered.push_str(frame);
        }
        rendered
    }
}
impl std::fmt::Display for TerraneError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.render())
    }
}
impl From<terrane_int_support::ArithmeticError> for TerraneError {
    fn from(error: terrane_int_support::ArithmeticError) -> Self {
        Self::new(
            TerraneErrorKind::from_source_name(error.source_name()),
            error.to_string(),
        )
    }
}
impl From<terrane_string_support::DecodeError> for TerraneError {
    fn from(error: terrane_string_support::DecodeError) -> Self {
        Self::new(
            TerraneErrorKind::DecodeError,
            error.to_string(),
        )
    }
}
impl From<terrane_collection_support::IndexError> for TerraneError {
    fn from(error: terrane_collection_support::IndexError) -> Self {
        Self::new(TerraneErrorKind::IndexError, error.to_string())
    }
}
impl From<terrane_collection_support::MissingKey> for TerraneError {
    fn from(error: terrane_collection_support::MissingKey) -> Self {
        Self::new(TerraneErrorKind::MissingKey, error.to_string())
    }
}
impl From<terrane_collection_support::RangeStepError> for TerraneError {
    fn from(error: terrane_collection_support::RangeStepError) -> Self {
        Self::new(TerraneErrorKind::SourceError, error.to_string())
    }
}
fn __terrane_uncaught(error: TerraneError) -> ! {
    eprintln!("{}", error.render());
    std::process::exit(1);
}
fn __terrane_generated_defect(message: &str) -> ! {
    eprintln!(
        "internal compiler defect: generated program reached an impossible completion: {message}"
    );
    std::process::exit(5);
}
#[allow(dead_code)]
enum TerraneCompletion<T> {
    Normal,
    Return(T),
    Error(TerraneError),
    Break,
    Continue,
}
type TerranePlatformResult = terrane_platform_support::ResultValue;
fn terrane_unhex(text: &str) -> Vec<u8> {
    fn digit(byte: u8) -> Option<u8> {
        match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'a'..=b'f' => Some(byte - b'a' + 10),
            b'A'..=b'F' => Some(byte - b'A' + 10),
            _ => None,
        }
    }
    text.as_bytes()
        .chunks_exact(2)
        .filter_map(|pair| Some(digit(pair[0])? << 4 | digit(pair[1])?))
        .collect()
}
fn terrane_platform_value(value: std::ffi::OsString) -> String {
    terrane_platform_support::platform_value(value)
}
fn terrane_platform_value_is_text(value: &str) -> bool {
    value.starts_with("text:")
}
fn terrane_platform_value_text(value: &str) -> String {
    value.strip_prefix("text:").unwrap_or("").to_owned()
}
fn terrane_platform_value_bytes(value: &str) -> Vec<u8> {
    value.strip_prefix("raw:").map(terrane_unhex).unwrap_or_default()
}
fn terrane_process_arguments() -> Vec<String> {
    std::env::args_os().skip(1).map(terrane_platform_value).collect()
}
fn terrane_environment_entries() -> Vec<String> {
    std::env::vars_os()
        .flat_map(|(name, value)| [
            terrane_platform_value(name),
            terrane_platform_value(value),
        ])
        .collect()
}
fn terrane_process_exit(code: terrane_int_support::Int) {
    let code = terrane_int_support::checked_coerce::<i32>(&code).unwrap_or(255);
    std::process::exit(code)
}
// Source: src/main.trn
// Namespace: benchmark-elementwise-materialized
fn benchmark_size() -> i64 {
    let supplied: terrane_collection_support::List<PlatformString> = arguments();
    if terrane_int_support::Int::from(terrane_int_support::Int::from(supplied.length()))
        != terrane_int_support::Int::from(1_i128)
    {
        exit(make_exit_status(terrane_int_support::Int::from(2_i128)));
    }
    let count: i64 = terrane_int_support::coerce::<
        i64,
    >(
            &terrane_int_support::parse_radix(
                    &supplied
                        .get_or_error(
                            terrane_collection_support::index_from_int(
                                    &terrane_int_support::Int::from(0_i128),
                                )
                                .unwrap_or_else(|error| __terrane_uncaught(
                                    TerraneError::from(error)
                                        .at(
                                            "/benchmark-elementwise-materialized::benchmark-size (main.trn:11:18)",
                                        ),
                                )),
                        )
                        .unwrap_or_else(|error| __terrane_uncaught(
                            TerraneError::from(error)
                                .at(
                                    "/benchmark-elementwise-materialized::benchmark-size (main.trn:11:18)",
                                ),
                        ))
                        .text,
                    &10,
                )
                .unwrap_or_else(|error| __terrane_uncaught(
                    TerraneError::from(error)
                        .at(
                            "/benchmark-elementwise-materialized::benchmark-size (main.trn:11:18)",
                        ),
                )),
        )
        .unwrap_or_else(|error| __terrane_uncaught(
            TerraneError::from(error)
                .at(
                    "/benchmark-elementwise-materialized::benchmark-size (main.trn:11:17)",
                ),
        ));
    if count <= 0 {
        exit(make_exit_status(terrane_int_support::Int::from(2_i128)));
    }
    return count;
}
fn main() {
    let count: i64 = benchmark_size();
    let mut transformed: terrane_collection_support::List<f64> = terrane_collection_support::List::<
        f64,
    >::new(Vec::new());
    let mut index: i64 = 0;
    while index < count {
        let raw: f64 = terrane_int_support::exact_f64(
                &terrane_int_support::fixed_remainder(index, 1000)
                    .unwrap_or_else(|error| __terrane_uncaught(
                        TerraneError::from(error)
                            .at(
                                "/benchmark-elementwise-materialized::main (main.trn:22:19)",
                            ),
                    )),
            )
            .unwrap_or_else(|error| __terrane_uncaught(
                TerraneError::from(error)
                    .at("/benchmark-elementwise-materialized::main (main.trn:22:19)"),
            ));
        let x: f64 = raw / 100.0_f64;
        transformed.append(x * x + 3.0_f64 * x - 7.0_f64);
        index = terrane_int_support::fixed_addition(index, 1)
            .unwrap_or_else(|error| __terrane_uncaught(
                TerraneError::from(error)
                    .at("/benchmark-elementwise-materialized::main (main.trn:25:5)"),
            ));
    }
    let mut total: f64 = 0.0_f64;
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &transformed,
    );
    loop {
        let value = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        total = total + value;
    }
    println!("{}", terrane_scalar_support::scalar_text(&total));
}
// Source: standard/process.trn
// Namespace: standard/process
#[derive(Clone)]
pub struct PlatformString {
    pub is_text: bool,
    pub text: String,
    pub raw: Vec<u8>,
}
impl PlatformString {
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
    pub name: PlatformString,
    pub value: PlatformString,
}
impl EnvironmentEntry {
    pub fn terrane_construct(name: PlatformString, entry_value: PlatformString) -> Self {
        let mut value = Self {
            name: PlatformString::terrane_construct(String::from("text:")),
            value: PlatformString::terrane_construct(String::from("text:")),
        };
        value.construct(name, entry_value);
        value
    }
    pub fn construct(&mut self, name: PlatformString, entry_value: PlatformString) {
        self.name = name.clone();
        self.value = entry_value.clone();
    }
}
#[derive(Clone)]
pub struct HostNameResult {
    pub failed: bool,
    pub available: bool,
    pub message: String,
    pub value: PlatformString,
}
impl HostNameResult {
    pub fn terrane_construct(
        did_fail: bool,
        is_available: bool,
        detail: String,
        result_value: PlatformString,
    ) -> Self {
        let mut value = Self {
            failed: false,
            available: false,
            message: String::from(""),
            value: PlatformString::terrane_construct(String::from("text:")),
        };
        value.construct(did_fail, is_available, detail, result_value);
        value
    }
    pub fn construct(
        &mut self,
        did_fail: bool,
        is_available: bool,
        detail: String,
        result_value: PlatformString,
    ) {
        self.failed = did_fail;
        self.available = is_available;
        self.message = detail;
        self.value = result_value.clone();
    }
}
pub fn host_name() -> HostNameResult {
    let raw: TerranePlatformResult = terrane_platform_support::system_host_name();
    return HostNameResult::terrane_construct(
        raw.failed,
        raw.flag,
        raw.message.clone(),
        PlatformString::terrane_construct(raw.text.clone()),
    );
}
pub fn arguments() -> terrane_collection_support::List<PlatformString> {
    let encoded: Vec<String> = terrane_process_arguments();
    let mut values: terrane_collection_support::List<PlatformString> = terrane_collection_support::List::<
        PlatformString,
    >::new(Vec::new());
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone() < terrane_int_support::Int::from(encoded.len() as i128) {
        values
            .append(
                PlatformString::terrane_construct(
                    encoded
                        .get(
                            terrane_collection_support::index_from_int(&index.clone())
                                .unwrap_or_else(|error| __terrane_uncaught(
                                    TerraneError::from(error)
                                        .at("/standard/process::arguments (process.trn:51:42)"),
                                )),
                        )
                        .cloned()
                        .ok_or(terrane_collection_support::IndexError {
                            index: terrane_collection_support::index_from_int(
                                    &index.clone(),
                                )
                                .unwrap_or_else(|error| __terrane_uncaught(
                                    TerraneError::from(error)
                                        .at("/standard/process::arguments (process.trn:51:42)"),
                                )),
                        })
                        .unwrap_or_else(|error| __terrane_uncaught(
                            TerraneError::from(error)
                                .at("/standard/process::arguments (process.trn:51:42)"),
                        )),
                ),
            );
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    return values.clone();
}
pub fn environment() -> terrane_collection_support::List<EnvironmentEntry> {
    let encoded: Vec<String> = terrane_environment_entries();
    let mut values: terrane_collection_support::List<EnvironmentEntry> = terrane_collection_support::List::<
        EnvironmentEntry,
    >::new(Vec::new());
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone() + terrane_int_support::Int::from(1_i128)
        < terrane_int_support::Int::from(encoded.len() as i128)
    {
        let name: PlatformString = PlatformString::terrane_construct(
            encoded
                .get(
                    terrane_collection_support::index_from_int(&index.clone())
                        .unwrap_or_else(|error| __terrane_uncaught(
                            TerraneError::from(error)
                                .at("/standard/process::environment (process.trn:60:33)"),
                        )),
                )
                .cloned()
                .ok_or(terrane_collection_support::IndexError {
                    index: terrane_collection_support::index_from_int(&index.clone())
                        .unwrap_or_else(|error| __terrane_uncaught(
                            TerraneError::from(error)
                                .at("/standard/process::environment (process.trn:60:33)"),
                        )),
                })
                .unwrap_or_else(|error| __terrane_uncaught(
                    TerraneError::from(error)
                        .at("/standard/process::environment (process.trn:60:33)"),
                )),
        );
        let value: PlatformString = PlatformString::terrane_construct(
            encoded
                .get(
                    terrane_collection_support::index_from_int(
                            &(index.clone() + terrane_int_support::Int::from(1_i128)),
                        )
                        .unwrap_or_else(|error| __terrane_uncaught(
                            TerraneError::from(error)
                                .at("/standard/process::environment (process.trn:61:34)"),
                        )),
                )
                .cloned()
                .ok_or(terrane_collection_support::IndexError {
                    index: terrane_collection_support::index_from_int(
                            &(index.clone() + terrane_int_support::Int::from(1_i128)),
                        )
                        .unwrap_or_else(|error| __terrane_uncaught(
                            TerraneError::from(error)
                                .at("/standard/process::environment (process.trn:61:34)"),
                        )),
                })
                .unwrap_or_else(|error| __terrane_uncaught(
                    TerraneError::from(error)
                        .at("/standard/process::environment (process.trn:61:34)"),
                )),
        );
        values.append(EnvironmentEntry::terrane_construct(name, value));
        index = index.clone() + terrane_int_support::Int::from(2_i128);
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
    pub option_values: terrane_collection_support::List<PlatformString>,
    pub positionals: terrane_collection_support::List<PlatformString>,
    pub diagnostic_arguments: terrane_collection_support::List<terrane_int_support::Int>,
    pub diagnostic_messages: terrane_collection_support::List<String>,
}
impl CommandLine {
    pub fn terrane_construct() -> Self {
        Self {
            flags: terrane_collection_support::List::<String>::new(Vec::new()),
            option_names: terrane_collection_support::List::<String>::new(Vec::new()),
            option_values: terrane_collection_support::List::<
                PlatformString,
            >::new(Vec::new()),
            positionals: terrane_collection_support::List::<
                PlatformString,
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
    supplied: terrane_collection_support::List<PlatformString>,
) -> CommandLine {
    let mut flags: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    let mut option_names: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    let mut option_values: terrane_collection_support::List<PlatformString> = terrane_collection_support::List::<
        PlatformString,
    >::new(Vec::new());
    let mut positionals: terrane_collection_support::List<PlatformString> = terrane_collection_support::List::<
        PlatformString,
    >::new(Vec::new());
    let mut diagnostic_arguments: terrane_collection_support::List<
        terrane_int_support::Int,
    > = terrane_collection_support::List::<terrane_int_support::Int>::new(Vec::new());
    let mut diagnostic_messages: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone()
        < terrane_int_support::Int::from(
            terrane_int_support::Int::from(supplied.length()),
        )
    {
        let argument: PlatformString = supplied
            .get_or_error(
                terrane_collection_support::index_from_int(&index.clone())
                    .unwrap_or_else(|error| __terrane_uncaught(
                        TerraneError::from(error)
                            .at(
                                "/standard/process::parse-command-line (process.trn:96:20)",
                            ),
                    )),
            )
            .unwrap_or_else(|error| __terrane_uncaught(
                TerraneError::from(error)
                    .at("/standard/process::parse-command-line (process.trn:96:20)"),
            ));
        if !argument.is_text {
            diagnostic_arguments.append(index.clone());
            diagnostic_messages
                .append(String::from("command-line option is not Unicode text"));
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
                flags.append(argument.text.clone());
            } else if schema_has(schema.clone(), value_entry) {
                if index.clone() + terrane_int_support::Int::from(1_i128)
                    >= terrane_int_support::Int::from(
                        terrane_int_support::Int::from(supplied.length()),
                    )
                {
                    diagnostic_arguments.append(index.clone());
                    diagnostic_messages.append(String::from("option requires a value"));
                } else {
                    option_names.append(argument.text.clone());
                    option_values
                        .append(
                            supplied
                                .get_or_error(
                                    terrane_collection_support::index_from_int(
                                            &(index.clone() + terrane_int_support::Int::from(1_i128)),
                                        )
                                        .unwrap_or_else(|error| __terrane_uncaught(
                                            TerraneError::from(error)
                                                .at(
                                                    "/standard/process::parse-command-line (process.trn:111:43)",
                                                ),
                                        )),
                                )
                                .unwrap_or_else(|error| __terrane_uncaught(
                                    TerraneError::from(error)
                                        .at(
                                            "/standard/process::parse-command-line (process.trn:111:43)",
                                        ),
                                )),
                        );
                    index = index.clone() + terrane_int_support::Int::from(1_i128);
                }
            } else if argument.text.starts_with(&String::from("--")) {
                diagnostic_arguments.append(index.clone());
                diagnostic_messages.append(String::from("unknown option"));
            } else {
                positionals.append(argument.clone());
            }
        }
        index = index.clone() + terrane_int_support::Int::from(1_i128);
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
// Generated Rust files: src/runtime/errors.rs, src/runtime/platform_system.rs, src/authored/src/main.trn.rs, src/authored/standard/process.trn.rs, src/main.rs
// Vendored support crates: terrane-int-support, terrane-scalar-support, terrane-string-support, terrane-stream-abi
