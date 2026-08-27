// Generated deterministically by Terrane <version>.
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
            ".arithmetic-overflow" => Self::ArithmeticOverflow,
            ".division-by-zero" => Self::DivisionByZero,
            ".integer-conversion-overflow" => Self::IntegerConversionOverflow,
            ".negative-shift-count" => Self::NegativeShiftCount,
            ".coercion-error" => Self::CoercionError,
            ".decode-error" => Self::DecodeError,
            ".index-error" => Self::IndexError,
            ".missing-key" => Self::MissingKey,
            ".resource-error" => Self::ResourceError,
            _ => Self::SourceError,
        }
    }
    fn source_name(self) -> &'static str {
        match self {
            Self::ArithmeticOverflow => ".arithmetic-overflow",
            Self::DivisionByZero => ".division-by-zero",
            Self::IntegerConversionOverflow => ".integer-conversion-overflow",
            Self::NegativeShiftCount => ".negative-shift-count",
            Self::CoercionError => ".coercion-error",
            Self::DecodeError => ".decode-error",
            Self::IndexError => ".index-error",
            Self::MissingKey => ".missing-key",
            Self::ResourceError => ".resource-error",
            Self::SourceError => ".error",
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
        let mut rendered = format!("{}: {}", self.kind.source_name(), self.message);
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
            error.to_string().trim_start_matches(".decode-error: "),
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
fn terrane_limit(value: &terrane_int_support::Int) -> usize {
    value.as_usize().unwrap_or(0)
}
fn terrane_index(value: &terrane_int_support::Int) -> Option<usize> {
    value.as_usize()
}
fn terrane_empty_document() -> terrane_document_support::DataResult {
    terrane_document_support::parse_json("null", 0, 4)
}
fn terrane_make_document_none() -> terrane_document_support::DataResult {
    terrane_document_support::document_none()
}
fn terrane_make_document_bool(value: bool) -> terrane_document_support::DataResult {
    terrane_document_support::document_bool(value)
}
fn terrane_make_document_string(value: String) -> terrane_document_support::DataResult {
    terrane_document_support::document_string(value)
}
fn terrane_make_document_integer(value: String) -> terrane_document_support::DataResult {
    terrane_document_support::document_integer(&value)
}
fn terrane_make_document_decimal(value: String) -> terrane_document_support::DataResult {
    terrane_document_support::document_decimal(&value)
}
fn terrane_make_document_list() -> terrane_document_support::DataResult {
    terrane_document_support::document_list()
}
fn terrane_document_list_append(
    list: &terrane_document_support::DataResult,
    value: &terrane_document_support::DataResult,
) -> terrane_document_support::DataResult {
    terrane_document_support::document_list_append(list, value)
}
fn terrane_make_document_map() -> terrane_document_support::DataResult {
    terrane_document_support::document_map()
}
fn terrane_document_map_insert(
    map: &terrane_document_support::DataResult,
    key: String,
    value: &terrane_document_support::DataResult,
) -> terrane_document_support::DataResult {
    terrane_document_support::document_map_insert(map, key, value)
}
fn terrane_data_failed(result: &terrane_document_support::DataResult) -> bool {
    result.failed
}
fn terrane_data_message(result: &terrane_document_support::DataResult) -> String {
    result.message.clone()
}
fn terrane_data_path(result: &terrane_document_support::DataResult) -> String {
    result.path.clone()
}
fn terrane_data_expected(result: &terrane_document_support::DataResult) -> String {
    result.expected.clone()
}
fn terrane_data_encoded(result: &terrane_document_support::DataResult) -> String {
    result.encoded.clone()
}
fn terrane_document_kind(result: &terrane_document_support::DataResult) -> String {
    terrane_document_support::document_kind(result)
}
fn terrane_document_text(result: &terrane_document_support::DataResult) -> String {
    terrane_document_support::document_text(result)
}
fn terrane_document_coefficient(
    result: &terrane_document_support::DataResult,
) -> String {
    terrane_document_support::document_coefficient(result)
}
fn terrane_document_exponent(
    result: &terrane_document_support::DataResult,
) -> terrane_int_support::Int {
    terrane_int_support::Int::from(terrane_document_support::document_exponent(result))
}
fn terrane_document_length(
    result: &terrane_document_support::DataResult,
) -> terrane_int_support::Int {
    terrane_int_support::Int::from(
        i128::try_from(terrane_document_support::document_length(result))
            .expect("document length fits in i128"),
    )
}
fn terrane_document_item(
    result: &terrane_document_support::DataResult,
    index: terrane_int_support::Int,
) -> terrane_document_support::DataResult {
    terrane_index(&index)
        .map_or_else(
            || terrane_document_support::invalid_document_index(),
            |index| terrane_document_support::document_item(result, index),
        )
}
fn terrane_document_key(
    result: &terrane_document_support::DataResult,
    index: terrane_int_support::Int,
) -> String {
    terrane_index(&index)
        .map_or_else(
            String::new,
            |index| terrane_document_support::document_key(result, index),
        )
}
fn terrane_document_field(
    result: &terrane_document_support::DataResult,
    key: String,
) -> terrane_document_support::DataResult {
    terrane_document_support::document_field(result, &key)
}
fn terrane_string_list(value: terrane_collection_support::List<String>) -> Vec<String> {
    value.into_iter().collect()
}
fn terrane_validate_mapping(
    result: &terrane_document_support::DataResult,
    expected_kind: String,
    required_fields: terrane_collection_support::List<String>,
    declared_fields: terrane_collection_support::List<String>,
    default_fields: terrane_collection_support::List<String>,
    default_values: terrane_collection_support::List<String>,
    allow_unknown: bool,
) -> terrane_document_support::DataResult {
    let required_fields = terrane_string_list(required_fields);
    let declared_fields = terrane_string_list(declared_fields);
    let default_fields = terrane_string_list(default_fields);
    let default_values = terrane_string_list(default_values);
    terrane_document_support::validate_mapping(
        result,
        &expected_kind,
        &required_fields,
        &declared_fields,
        &default_fields,
        &default_values,
        allow_unknown,
    )
}
fn terrane_json_parse(
    input: String,
    max_depth: terrane_int_support::Int,
    max_bytes: terrane_int_support::Int,
) -> terrane_document_support::DataResult {
    terrane_document_support::parse_json(
        &input,
        terrane_limit(&max_depth),
        terrane_limit(&max_bytes),
    )
}
fn terrane_json_canonical(
    value: &terrane_document_support::DataResult,
) -> terrane_document_support::DataResult {
    terrane_document_support::canonical_json(value)
}
fn terrane_yaml_parse(
    input: String,
    max_depth: terrane_int_support::Int,
    max_bytes: terrane_int_support::Int,
    max_aliases: terrane_int_support::Int,
) -> terrane_document_support::DataResult {
    terrane_document_support::parse_yaml(
        &input,
        terrane_limit(&max_depth),
        terrane_limit(&max_bytes),
        terrane_limit(&max_aliases),
    )
}
// Source: case.trn
// Namespace: conformance/document-json-yaml
#[derive(Clone)]
pub struct Note {
    pub text: String,
}
impl Note {
    pub fn terrane_construct(text: String) -> Self {
        let mut value = Self { text: String::from("") };
        value.construct(text);
        value
    }
    pub fn construct(&mut self, text: String) {
        self.text = text;
    }
    pub fn to_document(&self) -> DocumentValue {
        return make_document_string(self.text.clone());
    }
}
impl SerializableProtocol for Note {
    fn clone_box(&self) -> Box<dyn SerializableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn SerializableProtocol> {
        Box::new(self.clone())
    }
    fn to_document(&self) -> DocumentValue {
        Note::to_document(self)
    }
}
impl From<Note> for Serializable {
    fn from(value: Note) -> Self {
        Self(Box::new(value))
    }
}
fn main() {
    let options: JsonOptions = default_json_options();
    let parsed: DocumentResult = parse_json(
        String::from("{\"a\":123456789012345678901234567890,\"z\":1.2300}"),
        options.clone(),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&parsed.failed),
        terrane_scalar_support::scalar_text(&parsed.value.kind)
    );
    let integer: DocumentResult = parsed.value.field(String::from("a"));
    let decimal: DocumentResult = parsed.value.field(String::from("z"));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&integer.value.kind),
        terrane_scalar_support::scalar_text(&integer.value.scalar)
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&decimal.value.kind),
        terrane_scalar_support::scalar_text(&decimal.value.scalar)
    );
    println!("{}", terrane_scalar_support::scalar_text(&integer.value.integer.text));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&decimal.value.decimal.coefficient),
        terrane_scalar_support::scalar_text(&decimal.value.decimal.exponent)
    );
    let canonical: DocumentResult = canonical_json(parsed.value);
    println!("{}", terrane_scalar_support::scalar_text(&canonical.value.encoded));
    let reparsed: DocumentResult = parse_json(
        canonical.value.encoded.clone(),
        options.clone(),
    );
    let equivalent: DocumentResult = parse_json(
        String::from("{\"a\":1.2345678901234567890123456789e+29,\"z\":1.23}"),
        options.clone(),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&reparsed.failed),
        terrane_scalar_support::scalar_text(&(canonical.value.encoded == equivalent.value
        .encoded))
    );
    let stringify_input: DocumentResult = parse_json(
        String::from("{\"a\":1,\"z\":1.23}"),
        options.clone(),
    );
    let stringified: DocumentResult = stringify_json(
        stringify_input.value,
        options.clone(),
    );
    let yaml_input: DocumentResult = parse_json(
        String::from("{\"a\":1,\"z\":1.23}"),
        options.clone(),
    );
    let yaml_written: DocumentResult = stringify_yaml(yaml_input.value);
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&stringified.failed),
        terrane_scalar_support::scalar_text(&yaml_written.failed)
    );
    let list_value: DocumentResult = parse_json(
        String::from("[\"first\",{\"nested\":true}]"),
        options.clone(),
    );
    let list_document: DocumentValue = list_value.value;
    let first: DocumentResult = list_document
        .item(terrane_int_support::Int::from(0_i128));
    let second: DocumentResult = list_document
        .item(terrane_int_support::Int::from(1_i128));
    let nested: DocumentResult = second.value.field(String::from("nested"));
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&list_document.length()),
        terrane_scalar_support::scalar_text(&list_document
        .key(terrane_int_support::Int::from(0_i128))),
        terrane_scalar_support::scalar_text(&first.value.scalar),
        terrane_scalar_support::scalar_text(&nested.value.scalar)
    );
    let negative_item: DocumentResult = list_document
        .item(terrane_int_support::Int::from(-1_i128));
    println!("{}", terrane_scalar_support::scalar_text(&negative_item.failed));
    let duplicate: DocumentResult = parse_json(
        String::from("{\"key\":1,\"key\":2}"),
        options.clone(),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&duplicate.failed),
        terrane_scalar_support::scalar_text(&duplicate.message
        .contains(&String::from("duplicate key")))
    );
    let fields: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("name"), String::from("nickname"), String::from("active")]);
    let optional_fields: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("nickname")]);
    let default_fields: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("active")]);
    let default_values: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("true")]);
    let mut mapping: DocumentMapping = DocumentMapping::terrane_construct(
        String::from("person"),
        String::from("map"),
        false,
    );
    mapping.field_names = fields.clone();
    mapping.optional_fields = optional_fields.clone();
    mapping.default_fields = default_fields.clone();
    mapping.default_values = default_values.clone();
    let mut missing: DocumentResult = parse_json(String::from("{}"), options.clone());
    missing = decode_document(missing.value, mapping.clone());
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&missing.failed),
        terrane_scalar_support::scalar_text(&missing.path),
        terrane_scalar_support::scalar_text(&missing.expected)
    );
    let mut unknown: DocumentResult = parse_json(
        String::from("{\"name\":\"Ada\",\"extra\":1}"),
        options.clone(),
    );
    unknown = decode_document(unknown.value, mapping.clone());
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&unknown.failed),
        terrane_scalar_support::scalar_text(&unknown.path),
        terrane_scalar_support::scalar_text(&unknown.expected)
    );
    let mut mapped: DocumentResult = parse_json(
        String::from("{\"name\":\"Ada\",\"nickname\":\"A\"}"),
        options.clone(),
    );
    mapped = decode_document(mapped.value, mapping.clone());
    let active: DocumentResult = mapped.value.field(String::from("active"));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&mapped.failed),
        terrane_scalar_support::scalar_text(&active.value.scalar)
    );
    let constructor_mapping: DocumentMapping = DocumentMapping::terrane_construct(
        String::from("open-map"),
        String::from("map"),
        true,
    );
    let constructor_input: DocumentResult = parse_json(
        String::from("{\"a\":1}"),
        options.clone(),
    );
    let constructor_result: DocumentResult = decode_document(
        constructor_input.value,
        constructor_mapping.clone(),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&constructor_result.failed),
        terrane_scalar_support::scalar_text(&constructor_result.value.encoded)
    );
    let encoded_note: DocumentResult = encode_json(
        Serializable::from(Note::terrane_construct(String::from("hello"))),
        options.clone(),
    );
    let encoded_yaml_note: DocumentResult = encode_yaml(
        Serializable::from(Note::terrane_construct(String::from("hello"))),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&encoded_note.value.encoded),
        terrane_scalar_support::scalar_text(&encoded_yaml_note.value.encoded)
    );
    let exact_integer: DocumentResult = make_document_integer(
        String::from("123456789012345678901234567890"),
    );
    let exact_decimal: DocumentResult = make_document_decimal(String::from("1.2300"));
    let values: terrane_collection_support::List<DocumentValue> = terrane_collection_support::List::<
        DocumentValue,
    >::new(vec![exact_integer.value, exact_decimal.value, make_document_none()]);
    let built_list: DocumentResult = make_document_list(values.clone());
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&built_list.failed),
        terrane_scalar_support::scalar_text(&built_list.value.encoded)
    );
    let mut entries: DocumentMapEntries = DocumentMapEntries::terrane_construct();
    entries = append_document_map_entry(
        entries.clone(),
        String::from("message"),
        make_document_string(String::from("hello")),
    );
    let count_value: DocumentResult = make_document_integer(
        String::from("123456789012345678901234567890"),
    );
    entries = append_document_map_entry(
        entries.clone(),
        String::from("count"),
        count_value.value,
    );
    let built_map: DocumentResult = make_document_map(entries.clone());
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&built_map.failed),
        terrane_scalar_support::scalar_text(&built_map.value.encoded)
    );
    let mut duplicate_entries: DocumentMapEntries = DocumentMapEntries::terrane_construct();
    duplicate_entries = append_document_map_entry(
        duplicate_entries.clone(),
        String::from("same"),
        make_document_string(String::from("first")),
    );
    duplicate_entries = append_document_map_entry(
        duplicate_entries.clone(),
        String::from("same"),
        make_document_string(String::from("second")),
    );
    let duplicate_map: DocumentResult = make_document_map(duplicate_entries.clone());
    let canonical_integer: DocumentResult = make_document_integer(
        String::from("1.2345678901234567890123456789e+29"),
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&duplicate_map.failed),
        terrane_scalar_support::scalar_text(&duplicate_map.path),
        terrane_scalar_support::scalar_text(&canonical_integer.failed)
    );
    let decoded_through_interface: DocumentResult = decode_json(
        String::from("{\"name\":\"Ada\"}"),
        Deserializable::from(mapping.clone()),
        options.clone(),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&decoded_through_interface.failed)
    );
    let negative_limits: JsonOptions = JsonOptions::terrane_construct(
        terrane_int_support::Int::from(-1_i128),
        terrane_int_support::Int::from(-1_i128),
    );
    let limited: DocumentResult = parse_json(
        String::from("{}"),
        negative_limits.clone(),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&limited.failed),
        terrane_scalar_support::scalar_text(&limited.message
        .contains(&String::from("byte limit")))
    );
    let excessive_depth: DocumentResult = parse_json(
        String::from("[]"),
        JsonOptions::terrane_construct(
            terrane_int_support::Int::from(1000000_i128),
            terrane_int_support::Int::from(1024_i128),
        ),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&excessive_depth.failed),
        terrane_scalar_support::scalar_text(&excessive_depth.message
        .contains(&String::from("cannot exceed")))
    );
    let yaml_limits: YamlOptions = make_yaml_options(
        terrane_int_support::Int::from(32_i128),
        terrane_int_support::Int::from(2048_i128),
        terrane_int_support::Int::from(20_i128),
    );
    let yaml_value: DocumentResult = parse_yaml(
        String::from(
            "integer: 123456789012345678901234567890\ndecimal: 3.141592653589793238462643383279",
        ),
        yaml_limits.clone(),
    );
    let yaml_integer: DocumentResult = yaml_value.value.field(String::from("integer"));
    let yaml_decimal: DocumentResult = yaml_value.value.field(String::from("decimal"));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&yaml_value.failed),
        terrane_scalar_support::scalar_text(&yaml_integer.value.integer.text)
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&yaml_decimal.value.decimal
        .coefficient), terrane_scalar_support::scalar_text(&yaml_decimal.value.decimal
        .exponent)
    );
    let yaml_decoded: DocumentResult = decode_yaml(
        String::from("name: Ada"),
        Deserializable::from(mapping.clone()),
        make_yaml_options(
            terrane_int_support::Int::from(32_i128),
            terrane_int_support::Int::from(1024_i128),
            terrane_int_support::Int::from(65536_i128),
        ),
    );
    println!("{}", terrane_scalar_support::scalar_text(&yaml_decoded.failed));
    let ordinary_star: DocumentResult = parse_yaml(
        String::from("glob: \"a * b * c\""),
        make_yaml_options(
            terrane_int_support::Int::from(32_i128),
            terrane_int_support::Int::from(1024_i128),
            terrane_int_support::Int::from(0_i128),
        ),
    );
    println!("{}", terrane_scalar_support::scalar_text(&ordinary_star.failed));
    let bomb: DocumentResult = parse_yaml(
        String::from(
            "leaf: &leaf [1, 2, 3, 4]\na: &a [*leaf, *leaf, *leaf, *leaf]\nb: [*a, *a, *a, *a]",
        ),
        yaml_limits.clone(),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&bomb.failed),
        terrane_scalar_support::scalar_text(&bomb.message
        .contains(&String::from("alias node limit")))
    );
    let yaml_depth: DocumentResult = parse_yaml(
        String::from("a: [[[[]]]]"),
        make_yaml_options(
            terrane_int_support::Int::from(2_i128),
            terrane_int_support::Int::from(1024_i128),
            terrane_int_support::Int::from(65536_i128),
        ),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&yaml_depth.failed),
        terrane_scalar_support::scalar_text(&yaml_depth.message
        .contains(&String::from("depth limit")))
    );
    let excessive_yaml_depth: DocumentResult = parse_yaml(
        String::from("[]"),
        make_yaml_options(
            terrane_int_support::Int::from(256_i128),
            terrane_int_support::Int::from(1024_i128),
            terrane_int_support::Int::from(65536_i128),
        ),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&excessive_yaml_depth.failed),
        terrane_scalar_support::scalar_text(&excessive_yaml_depth.message
        .contains(&String::from("cannot exceed 255")))
    );
}
// Source: standard/documents.trn
// Namespace: standard/documents
#[derive(Clone)]
pub struct DocumentInteger {
    pub text: String,
}
impl DocumentInteger {
    pub fn terrane_construct(text: String) -> Self {
        let mut value = Self { text: String::from("0") };
        value.construct(text);
        value
    }
    pub fn construct(&mut self, text: String) {
        self.text = text;
    }
}
#[derive(Clone)]
pub struct DocumentDecimal {
    pub coefficient: String,
    pub exponent: terrane_int_support::Int,
    pub text: String,
}
impl DocumentDecimal {
    pub fn terrane_construct(
        coefficient: String,
        exponent: terrane_int_support::Int,
        text: String,
    ) -> Self {
        let mut value = Self {
            coefficient: String::from("0"),
            exponent: terrane_int_support::Int::from(0_i128),
            text: String::from("0"),
        };
        value.construct(coefficient, exponent, text);
        value
    }
    pub fn construct(
        &mut self,
        coefficient: String,
        exponent: terrane_int_support::Int,
        text: String,
    ) {
        self.coefficient = coefficient;
        self.exponent = exponent.clone();
        self.text = text;
    }
}
pub trait SerializableProtocol {
    fn clone_box(&self) -> Box<dyn SerializableProtocol>;
    fn separate_box(&self) -> Box<dyn SerializableProtocol>;
    fn to_document(&self) -> DocumentValue;
}
impl Clone for Box<dyn SerializableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
#[derive(Clone)]
pub struct Serializable(Box<dyn SerializableProtocol>);
impl Serializable {
    pub fn to_document(&self) -> DocumentValue {
        self.0.to_document()
    }
}
pub trait DeserializableProtocol {
    fn clone_box(&self) -> Box<dyn DeserializableProtocol>;
    fn separate_box(&self) -> Box<dyn DeserializableProtocol>;
    fn from_document(&self, value: DocumentValue) -> DocumentResult;
}
impl Clone for Box<dyn DeserializableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
#[derive(Clone)]
pub struct Deserializable(Box<dyn DeserializableProtocol>);
impl Deserializable {
    pub fn from_document(&self, value: DocumentValue) -> DocumentResult {
        self.0.from_document(value)
    }
}
#[derive(Clone)]
pub struct DocumentValue {
    pub raw: terrane_document_support::DataResult,
    pub encoded: String,
    pub kind: String,
    pub scalar: String,
    pub integer: DocumentInteger,
    pub decimal: DocumentDecimal,
}
impl DocumentValue {
    pub fn terrane_construct(raw: terrane_document_support::DataResult) -> Self {
        let mut value = Self {
            raw: terrane_empty_document(),
            encoded: String::from(""),
            kind: String::from("invalid"),
            scalar: String::from(""),
            integer: DocumentInteger::terrane_construct(String::from("0")),
            decimal: DocumentDecimal::terrane_construct(
                String::from("0"),
                terrane_int_support::Int::from(0_i128),
                String::from("0"),
            ),
        };
        value.construct(raw);
        value
    }
    pub fn construct(&mut self, raw: terrane_document_support::DataResult) {
        self.kind = terrane_document_kind(&raw);
        self.scalar = terrane_document_text(&raw);
        self.encoded = terrane_data_encoded(&raw);
        if self.kind == String::from("integer") {
            self.integer = DocumentInteger::terrane_construct(self.scalar.clone());
        }
        if self.kind == String::from("decimal") {
            self.decimal = DocumentDecimal::terrane_construct(
                terrane_document_coefficient(&raw),
                terrane_document_exponent(&raw),
                self.scalar.clone(),
            );
        }
        self.raw = raw;
    }
    pub fn length(&self) -> terrane_int_support::Int {
        return terrane_document_length(&self.raw);
    }
    pub fn to_document(&self) -> DocumentValue {
        return self.clone();
    }
    pub fn item(&self, index: terrane_int_support::Int) -> DocumentResult {
        let raw: terrane_document_support::DataResult = terrane_document_item(
            &self.raw,
            index.clone(),
        );
        return make_document_result(raw);
    }
    pub fn key(&self, index: terrane_int_support::Int) -> String {
        return terrane_document_key(&self.raw, index.clone());
    }
    pub fn field(&self, name: String) -> DocumentResult {
        let raw: terrane_document_support::DataResult = terrane_document_field(
            &self.raw,
            name,
        );
        return make_document_result(raw);
    }
}
impl SerializableProtocol for DocumentValue {
    fn clone_box(&self) -> Box<dyn SerializableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn SerializableProtocol> {
        Box::new(self.clone())
    }
    fn to_document(&self) -> DocumentValue {
        DocumentValue::to_document(self)
    }
}
impl From<DocumentValue> for Serializable {
    fn from(value: DocumentValue) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct DocumentResult {
    pub failed: bool,
    pub message: String,
    pub path: String,
    pub expected: String,
    pub value: DocumentValue,
}
impl DocumentResult {
    pub fn terrane_construct(
        failed: bool,
        message: String,
        path: String,
        expected: String,
        raw: terrane_document_support::DataResult,
    ) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            path: String::from("$"),
            expected: String::from(""),
            value: DocumentValue::terrane_construct(terrane_empty_document()),
        };
        value.construct(failed, message, path, expected, raw);
        value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        message: String,
        path: String,
        expected: String,
        raw: terrane_document_support::DataResult,
    ) {
        self.failed = failed;
        self.message = message;
        self.path = path;
        self.expected = expected;
        self.value = DocumentValue::terrane_construct(raw);
    }
}
#[derive(Clone)]
pub struct DocumentMapping {
    pub descriptor_name: String,
    pub expected_kind: String,
    pub field_names: terrane_collection_support::List<String>,
    pub optional_fields: terrane_collection_support::List<String>,
    pub default_fields: terrane_collection_support::List<String>,
    pub default_values: terrane_collection_support::List<String>,
    pub allow_unknown: bool,
}
impl DocumentMapping {
    pub fn terrane_construct(
        descriptor_name: String,
        expected_kind: String,
        allow_unknown: bool,
    ) -> Self {
        let mut value = Self {
            descriptor_name: String::from("document-value"),
            expected_kind: String::from("map"),
            field_names: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("")]),
            optional_fields: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("")]),
            default_fields: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("")]),
            default_values: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("")]),
            allow_unknown: false,
        };
        value.construct(descriptor_name, expected_kind, allow_unknown);
        value
    }
    pub fn construct(
        &mut self,
        descriptor_name: String,
        expected_kind: String,
        allow_unknown: bool,
    ) {
        self.descriptor_name = descriptor_name;
        self.expected_kind = expected_kind;
        self.allow_unknown = allow_unknown;
    }
    pub fn from_document(&self, value: DocumentValue) -> DocumentResult {
        return decode_document(value.clone(), self.clone());
    }
}
impl DeserializableProtocol for DocumentMapping {
    fn clone_box(&self) -> Box<dyn DeserializableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DeserializableProtocol> {
        Box::new(self.clone())
    }
    fn from_document(&self, value: DocumentValue) -> DocumentResult {
        DocumentMapping::from_document(self, value)
    }
}
impl From<DocumentMapping> for Deserializable {
    fn from(value: DocumentMapping) -> Self {
        Self(Box::new(value))
    }
}
pub fn serialize_document(value: Serializable) -> DocumentValue {
    return value.to_document();
}
pub fn deserialize_document(
    value: DocumentValue,
    destination: Deserializable,
) -> DocumentResult {
    return destination.from_document(value.clone());
}
pub fn make_document_result(
    raw: terrane_document_support::DataResult,
) -> DocumentResult {
    return DocumentResult::terrane_construct(
        terrane_data_failed(&raw),
        terrane_data_message(&raw),
        terrane_data_path(&raw),
        terrane_data_expected(&raw),
        raw,
    );
}
pub fn make_document_none() -> DocumentValue {
    return DocumentValue::terrane_construct(terrane_make_document_none());
}
pub fn make_document_bool(value: bool) -> DocumentValue {
    return DocumentValue::terrane_construct(terrane_make_document_bool(value));
}
pub fn make_document_string(value: String) -> DocumentValue {
    return DocumentValue::terrane_construct(terrane_make_document_string(value));
}
pub fn make_document_integer(value: String) -> DocumentResult {
    return make_document_result(terrane_make_document_integer(value));
}
pub fn make_document_decimal(value: String) -> DocumentResult {
    return make_document_result(terrane_make_document_decimal(value));
}
#[derive(Clone)]
pub struct DocumentMapEntries {
    pub raw: terrane_document_support::DataResult,
}
impl DocumentMapEntries {
    pub fn terrane_construct() -> Self {
        let mut value = Self {
            raw: terrane_make_document_map(),
        };
        value.construct();
        value
    }
    pub fn construct(&mut self) {
        self.raw = terrane_make_document_map();
    }
    pub fn append(&mut self, key: String, value: DocumentValue) {
        self.raw = terrane_document_map_insert(&self.raw, key, &value.raw);
    }
}
pub fn append_document_map_entry(
    mut entries: DocumentMapEntries,
    key: String,
    value: DocumentValue,
) -> DocumentMapEntries {
    entries.append(key, value.clone());
    return entries.clone();
}
pub fn make_document_list(
    values: terrane_collection_support::List<DocumentValue>,
) -> DocumentResult {
    let mut raw: terrane_document_support::DataResult = terrane_make_document_list();
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone()
        < terrane_int_support::Int::from(terrane_int_support::Int::from(values.length()))
    {
        raw = terrane_document_list_append(
            &raw,
            &values
                .get_or_error(
                    terrane_collection_support::index_from_int(&index.clone())
                        .unwrap_or_else(|error| __terrane_uncaught(
                            TerraneError::from(error)
                                .at(
                                    "/standard/documents::make-document-list (documents.trn:149:42)",
                                ),
                        )),
                )
                .unwrap_or_else(|error| __terrane_uncaught(
                    TerraneError::from(error)
                        .at(
                            "/standard/documents::make-document-list (documents.trn:149:42)",
                        ),
                ))
                .raw,
        );
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    return make_document_result(raw);
}
pub fn make_document_map(entries: DocumentMapEntries) -> DocumentResult {
    return make_document_result(entries.raw);
}
pub fn mapping_required_fields(
    mapping: DocumentMapping,
) -> terrane_collection_support::List<String> {
    let fields: terrane_collection_support::List<String> = mapping.field_names;
    let optional_fields: terrane_collection_support::List<String> = mapping
        .optional_fields;
    let default_fields: terrane_collection_support::List<String> = mapping
        .default_fields;
    let mut required: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![]);
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone()
        < terrane_int_support::Int::from(terrane_int_support::Int::from(fields.length()))
    {
        let field: String = fields
            .get_or_error(
                terrane_collection_support::index_from_int(&index.clone())
                    .unwrap_or_else(|error| __terrane_uncaught(
                        TerraneError::from(error)
                            .at(
                                "/standard/documents::mapping-required-fields (documents.trn:162:17)",
                            ),
                    )),
            )
            .unwrap_or_else(|error| __terrane_uncaught(
                TerraneError::from(error)
                    .at(
                        "/standard/documents::mapping-required-fields (documents.trn:162:17)",
                    ),
            ));
        let mut optional: bool = false;
        let mut optional_index: terrane_int_support::Int = terrane_int_support::Int::from(
            0_i128,
        );
        while optional_index.clone()
            < terrane_int_support::Int::from(
                terrane_int_support::Int::from(optional_fields.length()),
            )
        {
            if optional_fields
                .get_or_error(
                    terrane_collection_support::index_from_int(&optional_index.clone())
                        .unwrap_or_else(|error| __terrane_uncaught(
                            TerraneError::from(error)
                                .at(
                                    "/standard/documents::mapping-required-fields (documents.trn:166:16)",
                                ),
                        )),
                )
                .unwrap_or_else(|error| __terrane_uncaught(
                    TerraneError::from(error)
                        .at(
                            "/standard/documents::mapping-required-fields (documents.trn:166:16)",
                        ),
                )) == field
            {
                optional = true;
            }
            optional_index = optional_index.clone()
                + terrane_int_support::Int::from(1_i128);
        }
        let mut defaulted: bool = false;
        let mut default_index: terrane_int_support::Int = terrane_int_support::Int::from(
            0_i128,
        );
        while default_index.clone()
            < terrane_int_support::Int::from(
                terrane_int_support::Int::from(default_fields.length()),
            )
        {
            if default_fields
                .get_or_error(
                    terrane_collection_support::index_from_int(&default_index.clone())
                        .unwrap_or_else(|error| __terrane_uncaught(
                            TerraneError::from(error)
                                .at(
                                    "/standard/documents::mapping-required-fields (documents.trn:172:16)",
                                ),
                        )),
                )
                .unwrap_or_else(|error| __terrane_uncaught(
                    TerraneError::from(error)
                        .at(
                            "/standard/documents::mapping-required-fields (documents.trn:172:16)",
                        ),
                )) == field
            {
                defaulted = true;
            }
            default_index = default_index.clone()
                + terrane_int_support::Int::from(1_i128);
        }
        if field != String::from("") && !optional && !defaulted {
            required.append(field);
        }
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    return required.clone();
}
pub fn decode_document(
    value: DocumentValue,
    mapping: DocumentMapping,
) -> DocumentResult {
    let required: terrane_collection_support::List<String> = mapping_required_fields(
        mapping.clone(),
    );
    let mut declared_fields: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![]);
    let mut field_index: terrane_int_support::Int = terrane_int_support::Int::from(
        0_i128,
    );
    while field_index.clone()
        < terrane_int_support::Int::from(
            terrane_int_support::Int::from(mapping.field_names.length()),
        )
    {
        if mapping
            .field_names
            .get_or_error(
                terrane_collection_support::index_from_int(&field_index.clone())
                    .unwrap_or_else(|error| __terrane_uncaught(
                        TerraneError::from(error)
                            .at(
                                "/standard/documents::decode-document (documents.trn:185:12)",
                            ),
                    )),
            )
            .unwrap_or_else(|error| __terrane_uncaught(
                TerraneError::from(error)
                    .at("/standard/documents::decode-document (documents.trn:185:12)"),
            )) != String::from("")
        {
            declared_fields
                .append(
                    mapping
                        .field_names
                        .get_or_error(
                            terrane_collection_support::index_from_int(
                                    &field_index.clone(),
                                )
                                .unwrap_or_else(|error| __terrane_uncaught(
                                    TerraneError::from(error)
                                        .at(
                                            "/standard/documents::decode-document (documents.trn:186:37)",
                                        ),
                                )),
                        )
                        .unwrap_or_else(|error| __terrane_uncaught(
                            TerraneError::from(error)
                                .at(
                                    "/standard/documents::decode-document (documents.trn:186:37)",
                                ),
                        )),
                );
        }
        field_index = field_index.clone() + terrane_int_support::Int::from(1_i128);
    }
    let mut default_fields: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![]);
    let mut default_values: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![]);
    let mut default_index: terrane_int_support::Int = terrane_int_support::Int::from(
        0_i128,
    );
    while default_index.clone()
        < terrane_int_support::Int::from(
            terrane_int_support::Int::from(mapping.default_fields.length()),
        )
        && default_index.clone()
            < terrane_int_support::Int::from(
                terrane_int_support::Int::from(mapping.default_values.length()),
            )
    {
        if mapping
            .default_fields
            .get_or_error(
                terrane_collection_support::index_from_int(&default_index.clone())
                    .unwrap_or_else(|error| __terrane_uncaught(
                        TerraneError::from(error)
                            .at(
                                "/standard/documents::decode-document (documents.trn:192:12)",
                            ),
                    )),
            )
            .unwrap_or_else(|error| __terrane_uncaught(
                TerraneError::from(error)
                    .at("/standard/documents::decode-document (documents.trn:192:12)"),
            )) != String::from("")
        {
            default_fields
                .append(
                    mapping
                        .default_fields
                        .get_or_error(
                            terrane_collection_support::index_from_int(
                                    &default_index.clone(),
                                )
                                .unwrap_or_else(|error| __terrane_uncaught(
                                    TerraneError::from(error)
                                        .at(
                                            "/standard/documents::decode-document (documents.trn:193:36)",
                                        ),
                                )),
                        )
                        .unwrap_or_else(|error| __terrane_uncaught(
                            TerraneError::from(error)
                                .at(
                                    "/standard/documents::decode-document (documents.trn:193:36)",
                                ),
                        )),
                );
            default_values
                .append(
                    mapping
                        .default_values
                        .get_or_error(
                            terrane_collection_support::index_from_int(
                                    &default_index.clone(),
                                )
                                .unwrap_or_else(|error| __terrane_uncaught(
                                    TerraneError::from(error)
                                        .at(
                                            "/standard/documents::decode-document (documents.trn:194:36)",
                                        ),
                                )),
                        )
                        .unwrap_or_else(|error| __terrane_uncaught(
                            TerraneError::from(error)
                                .at(
                                    "/standard/documents::decode-document (documents.trn:194:36)",
                                ),
                        )),
                );
        }
        default_index = default_index.clone() + terrane_int_support::Int::from(1_i128);
    }
    let raw: terrane_document_support::DataResult = terrane_validate_mapping(
        &value.raw,
        mapping.expected_kind,
        required,
        declared_fields,
        default_fields,
        default_values,
        mapping.allow_unknown,
    );
    let mut result: DocumentResult = make_document_result(raw);
    if result.failed {
        result.expected = mapping.descriptor_name.clone();
    }
    return result.clone();
}
// Source: standard/json.trn
// Namespace: standard/json
#[derive(Clone)]
pub struct JsonOptions {
    pub max_depth: terrane_int_support::Int,
    pub max_bytes: terrane_int_support::Int,
}
impl JsonOptions {
    pub fn terrane_construct(
        max_depth: terrane_int_support::Int,
        max_bytes: terrane_int_support::Int,
    ) -> Self {
        let mut value = Self {
            max_depth: terrane_int_support::Int::from(256_i128),
            max_bytes: terrane_int_support::Int::from(16777216_i128),
        };
        value.construct(max_depth, max_bytes);
        value
    }
    pub fn construct(
        &mut self,
        max_depth: terrane_int_support::Int,
        max_bytes: terrane_int_support::Int,
    ) {
        self.max_depth = max_depth.clone();
        self.max_bytes = max_bytes.clone();
    }
}
pub fn default_json_options() -> JsonOptions {
    return JsonOptions::terrane_construct(
        terrane_int_support::Int::from(256_i128),
        terrane_int_support::Int::from(16777216_i128),
    );
}
pub fn parse_json(input: String, options: JsonOptions) -> DocumentResult {
    let raw: terrane_document_support::DataResult = terrane_json_parse(
        input,
        options.max_depth.clone(),
        options.max_bytes.clone(),
    );
    return make_document_result(raw);
}
pub fn stringify_json(value: DocumentValue, options: JsonOptions) -> DocumentResult {
    let _ = &options;
    let raw: terrane_document_support::DataResult = terrane_json_canonical(&value.raw);
    return make_document_result(raw);
}
pub fn canonical_json(value: DocumentValue) -> DocumentResult {
    let raw: terrane_document_support::DataResult = terrane_json_canonical(&value.raw);
    return make_document_result(raw);
}
pub fn decode_json(
    input: String,
    mapping: Deserializable,
    options: JsonOptions,
) -> DocumentResult {
    let parsed: DocumentResult = parse_json(input, options.clone());
    if parsed.failed {
        return parsed.clone();
    }
    return deserialize_document(parsed.value, mapping.clone());
}
pub fn encode_json(value: Serializable, options: JsonOptions) -> DocumentResult {
    return stringify_json(serialize_document(value.clone()), options.clone());
}
// Source: standard/yaml.trn
// Namespace: standard/yaml
#[derive(Clone)]
pub struct YamlOptions {
    pub max_depth: terrane_int_support::Int,
    pub max_bytes: terrane_int_support::Int,
    pub max_alias_nodes: terrane_int_support::Int,
}
impl YamlOptions {
    pub fn terrane_construct(
        max_depth: terrane_int_support::Int,
        max_bytes: terrane_int_support::Int,
        max_alias_nodes: terrane_int_support::Int,
    ) -> Self {
        let mut value = Self {
            max_depth: terrane_int_support::Int::from(128_i128),
            max_bytes: terrane_int_support::Int::from(16777216_i128),
            max_alias_nodes: terrane_int_support::Int::from(65536_i128),
        };
        value.construct(max_depth, max_bytes, max_alias_nodes);
        value
    }
    pub fn construct(
        &mut self,
        max_depth: terrane_int_support::Int,
        max_bytes: terrane_int_support::Int,
        max_alias_nodes: terrane_int_support::Int,
    ) {
        self.max_depth = max_depth.clone();
        self.max_bytes = max_bytes.clone();
        self.max_alias_nodes = max_alias_nodes.clone();
    }
}
pub fn default_yaml_options() -> YamlOptions {
    return YamlOptions::terrane_construct(
        terrane_int_support::Int::from(128_i128),
        terrane_int_support::Int::from(16777216_i128),
        terrane_int_support::Int::from(65536_i128),
    );
}
pub fn make_yaml_options(
    max_depth: terrane_int_support::Int,
    max_bytes: terrane_int_support::Int,
    max_alias_nodes: terrane_int_support::Int,
) -> YamlOptions {
    return YamlOptions::terrane_construct(
        max_depth.clone(),
        max_bytes.clone(),
        max_alias_nodes.clone(),
    );
}
pub fn parse_yaml(input: String, options: YamlOptions) -> DocumentResult {
    let raw: terrane_document_support::DataResult = terrane_yaml_parse(
        input,
        options.max_depth.clone(),
        options.max_bytes.clone(),
        options.max_alias_nodes.clone(),
    );
    return make_document_result(raw);
}
pub fn stringify_yaml(value: DocumentValue) -> DocumentResult {
    let raw: terrane_document_support::DataResult = terrane_json_canonical(&value.raw);
    return make_document_result(raw);
}
pub fn decode_yaml(
    input: String,
    mapping: Deserializable,
    options: YamlOptions,
) -> DocumentResult {
    let parsed: DocumentResult = parse_yaml(input, options.clone());
    if parsed.failed {
        return parsed.clone();
    }
    return deserialize_document(parsed.value, mapping.clone());
}
pub fn encode_yaml(value: Serializable) -> DocumentResult {
    return stringify_yaml(serialize_document(value.clone()));
}
