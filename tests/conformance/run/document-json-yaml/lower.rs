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
    value.as_usize().unwrap_or(usize::MAX)
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
fn terrane_document_kind(encoded: &String) -> String {
    terrane_document_support::document_kind(encoded)
}
fn terrane_document_text(encoded: &String) -> String {
    terrane_document_support::document_text(encoded)
}
fn terrane_document_length(encoded: &String) -> terrane_int_support::Int {
    terrane_int_support::Int::from(
        i128::try_from(terrane_document_support::document_length(encoded))
            .expect("document length fits in i128"),
    )
}
fn terrane_document_item(
    encoded: &String,
    index: terrane_int_support::Int,
) -> terrane_document_support::DataResult {
    terrane_document_support::document_item(encoded, terrane_limit(&index))
}
fn terrane_document_key(encoded: &String, index: terrane_int_support::Int) -> String {
    terrane_document_support::document_key(encoded, terrane_limit(&index))
}
fn terrane_document_field(
    encoded: &String,
    key: String,
) -> terrane_document_support::DataResult {
    terrane_document_support::document_field(encoded, &key)
}
fn terrane_string_list(value: terrane_collection_support::List<String>) -> Vec<String> {
    (0..usize::try_from(value.length()).expect("list length fits in usize"))
        .filter_map(|index| value.get(index).cloned())
        .collect()
}
fn terrane_validate_mapping(
    encoded: &String,
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
        encoded,
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
    reject_duplicates: bool,
    max_depth: terrane_int_support::Int,
    max_bytes: terrane_int_support::Int,
) -> terrane_document_support::DataResult {
    terrane_document_support::parse_json(
        &input,
        reject_duplicates,
        terrane_limit(&max_depth),
        terrane_limit(&max_bytes),
    )
}
fn terrane_json_canonical(input: String) -> terrane_document_support::DataResult {
    terrane_document_support::canonical_json(&input)
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
    let canonical: DocumentResult = canonical_json(parsed.value);
    println!("{}", terrane_scalar_support::scalar_text(&canonical.value.encoded));
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
    let active_default: DocumentValue = DocumentValue::terrane_construct(
        String::from("true"),
    );
    let default_values: terrane_collection_support::List<DocumentValue> = terrane_collection_support::List::<
        DocumentValue,
    >::new(vec![active_default.clone()]);
    let mut mapping: DocumentMapping = DocumentMapping::terrane_construct(
        String::from("person"),
        String::from("map"),
        false,
    );
    mapping.field_names = fields.clone();
    mapping.optional_fields = optional_fields.clone();
    mapping.default_fields = default_fields.clone();
    mapping.default_values = default_values.clone();
    let missing_value: DocumentValue = DocumentValue::terrane_construct(
        String::from("{}"),
    );
    let missing: DocumentResult = decode_document(
        missing_value.clone(),
        mapping.clone(),
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&missing.failed),
        terrane_scalar_support::scalar_text(&missing.path),
        terrane_scalar_support::scalar_text(&missing.expected)
    );
    let unknown_value: DocumentValue = DocumentValue::terrane_construct(
        String::from("{\"name\":\"Ada\",\"extra\":1}"),
    );
    let unknown: DocumentResult = decode_document(
        unknown_value.clone(),
        mapping.clone(),
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&unknown.failed),
        terrane_scalar_support::scalar_text(&unknown.path),
        terrane_scalar_support::scalar_text(&unknown.expected)
    );
    let mapped_value: DocumentValue = DocumentValue::terrane_construct(
        String::from("{\"name\":\"Ada\",\"nickname\":\"A\"}"),
    );
    let mapped: DocumentResult = decode_document(mapped_value.clone(), mapping.clone());
    let active: DocumentResult = mapped.value.field(String::from("active"));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&mapped.failed),
        terrane_scalar_support::scalar_text(&active.value.scalar)
    );
    let yaml_limits: YamlOptions = make_yaml_options(
        terrane_int_support::Int::from(32_i128),
        terrane_int_support::Int::from(1024_i128),
        terrane_int_support::Int::from(1_i128),
    );
    let bomb: DocumentResult = parse_yaml(
        String::from("root: &root [1]\na: *root\nb: *root"),
        yaml_limits.clone(),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&bomb.failed),
        terrane_scalar_support::scalar_text(&bomb.message
        .contains(&String::from("alias expansion limit")))
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
#[derive(Clone)]
pub struct DocumentValue {
    pub encoded: String,
    pub kind: String,
    pub scalar: String,
}
impl DocumentValue {
    pub fn terrane_construct(encoded: String) -> Self {
        let mut value = Self {
            encoded: String::from("null"),
            kind: String::from("none"),
            scalar: String::from(""),
        };
        value.construct(encoded);
        value
    }
    pub fn construct(&mut self, encoded: String) {
        self.kind = terrane_document_kind(&encoded);
        self.scalar = terrane_document_text(&encoded);
        self.encoded = encoded;
    }
    pub fn length(&self) -> terrane_int_support::Int {
        return terrane_document_length(&self.encoded);
    }
    pub fn item(&self, index: terrane_int_support::Int) -> DocumentResult {
        let raw: terrane_document_support::DataResult = terrane_document_item(
            &self.encoded,
            index.clone(),
        );
        return make_document_result(raw);
    }
    pub fn key(&self, index: terrane_int_support::Int) -> String {
        return terrane_document_key(&self.encoded, index.clone());
    }
    pub fn field(&self, name: String) -> DocumentResult {
        let raw: terrane_document_support::DataResult = terrane_document_field(
            &self.encoded,
            name,
        );
        return make_document_result(raw);
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
        encoded: String,
    ) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            path: String::from("$"),
            expected: String::from(""),
            value: DocumentValue::terrane_construct(String::from("null")),
        };
        value.construct(failed, message, path, expected, encoded);
        value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        message: String,
        path: String,
        expected: String,
        encoded: String,
    ) {
        self.failed = failed;
        self.message = message;
        self.path = path;
        self.expected = expected;
        if !failed {
            self.value = DocumentValue::terrane_construct(encoded);
        }
    }
}
#[derive(Clone)]
pub struct DocumentMapping {
    pub descriptor_name: String,
    pub expected_kind: String,
    pub field_names: terrane_collection_support::List<String>,
    pub optional_fields: terrane_collection_support::List<String>,
    pub default_fields: terrane_collection_support::List<String>,
    pub default_values: terrane_collection_support::List<DocumentValue>,
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
                DocumentValue,
            >::new(vec![DocumentValue::terrane_construct(String::from("null"))]),
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
}
pub fn make_document_result(
    raw: terrane_document_support::DataResult,
) -> DocumentResult {
    return DocumentResult::terrane_construct(
        terrane_data_failed(&raw),
        terrane_data_message(&raw),
        terrane_data_path(&raw),
        terrane_data_expected(&raw),
        terrane_data_encoded(&raw),
    );
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
                                "/standard/documents::mapping-required-fields (documents.trn:91:17)",
                            ),
                    )),
            )
            .unwrap_or_else(|error| __terrane_uncaught(
                TerraneError::from(error)
                    .at(
                        "/standard/documents::mapping-required-fields (documents.trn:91:17)",
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
                                    "/standard/documents::mapping-required-fields (documents.trn:95:16)",
                                ),
                        )),
                )
                .unwrap_or_else(|error| __terrane_uncaught(
                    TerraneError::from(error)
                        .at(
                            "/standard/documents::mapping-required-fields (documents.trn:95:16)",
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
                                    "/standard/documents::mapping-required-fields (documents.trn:101:16)",
                                ),
                        )),
                )
                .unwrap_or_else(|error| __terrane_uncaught(
                    TerraneError::from(error)
                        .at(
                            "/standard/documents::mapping-required-fields (documents.trn:101:16)",
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
    let mut default_encodings: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![]);
    let mut default_index: terrane_int_support::Int = terrane_int_support::Int::from(
        0_i128,
    );
    while default_index.clone()
        < terrane_int_support::Int::from(
            terrane_int_support::Int::from(mapping.default_values.length()),
        )
    {
        default_encodings
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
                                        "/standard/documents::decode-document (documents.trn:114:35)",
                                    ),
                            )),
                    )
                    .unwrap_or_else(|error| __terrane_uncaught(
                        TerraneError::from(error)
                            .at(
                                "/standard/documents::decode-document (documents.trn:114:35)",
                            ),
                    ))
                    .encoded
                    .clone(),
            );
        default_index = default_index.clone() + terrane_int_support::Int::from(1_i128);
    }
    let raw: terrane_document_support::DataResult = terrane_validate_mapping(
        &value.encoded,
        mapping.expected_kind,
        required,
        mapping.field_names,
        mapping.default_fields,
        default_encodings,
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
    pub reject_duplicate_keys: bool,
    pub canonical: bool,
    pub max_depth: terrane_int_support::Int,
    pub max_bytes: terrane_int_support::Int,
}
impl JsonOptions {
    pub fn terrane_construct(
        reject_duplicate_keys: bool,
        canonical: bool,
        max_depth: terrane_int_support::Int,
        max_bytes: terrane_int_support::Int,
    ) -> Self {
        let mut value = Self {
            reject_duplicate_keys: true,
            canonical: false,
            max_depth: terrane_int_support::Int::from(256_i128),
            max_bytes: terrane_int_support::Int::from(16777216_i128),
        };
        value.construct(reject_duplicate_keys, canonical, max_depth, max_bytes);
        value
    }
    pub fn construct(
        &mut self,
        reject_duplicate_keys: bool,
        canonical: bool,
        max_depth: terrane_int_support::Int,
        max_bytes: terrane_int_support::Int,
    ) {
        self.reject_duplicate_keys = reject_duplicate_keys;
        self.canonical = canonical;
        self.max_depth = max_depth.clone();
        self.max_bytes = max_bytes.clone();
    }
}
pub fn default_json_options() -> JsonOptions {
    return JsonOptions::terrane_construct(
        true,
        false,
        terrane_int_support::Int::from(256_i128),
        terrane_int_support::Int::from(16777216_i128),
    );
}
pub fn parse_json(input: String, options: JsonOptions) -> DocumentResult {
    let raw: terrane_document_support::DataResult = terrane_json_parse(
        input,
        options.reject_duplicate_keys,
        options.max_depth.clone(),
        options.max_bytes.clone(),
    );
    return make_document_result(raw);
}
pub fn stringify_json(value: DocumentValue, options: JsonOptions) -> DocumentResult {
    if options.canonical {
        let raw: terrane_document_support::DataResult = terrane_json_canonical(
            value.encoded,
        );
        return make_document_result(raw);
    }
    return DocumentResult::terrane_construct(
        false,
        String::from(""),
        String::from("$"),
        String::from(""),
        value.encoded.clone(),
    );
}
pub fn canonical_json(value: DocumentValue) -> DocumentResult {
    let raw: terrane_document_support::DataResult = terrane_json_canonical(
        value.encoded,
    );
    return make_document_result(raw);
}
pub fn decode_json(
    input: String,
    mapping: DocumentMapping,
    options: JsonOptions,
) -> DocumentResult {
    let parsed: DocumentResult = parse_json(input, options.clone());
    if parsed.failed {
        return parsed.clone();
    }
    return decode_document(parsed.value, mapping.clone());
}
pub fn encode_json(value: DocumentValue, options: JsonOptions) -> DocumentResult {
    return stringify_json(value.clone(), options.clone());
}
// Source: standard/yaml.trn
// Namespace: standard/yaml
#[derive(Clone)]
pub struct YamlOptions {
    pub max_depth: terrane_int_support::Int,
    pub max_bytes: terrane_int_support::Int,
    pub max_aliases: terrane_int_support::Int,
}
impl YamlOptions {
    pub fn terrane_construct(
        max_depth: terrane_int_support::Int,
        max_bytes: terrane_int_support::Int,
        max_aliases: terrane_int_support::Int,
    ) -> Self {
        let mut value = Self {
            max_depth: terrane_int_support::Int::from(128_i128),
            max_bytes: terrane_int_support::Int::from(16777216_i128),
            max_aliases: terrane_int_support::Int::from(64_i128),
        };
        value.construct(max_depth, max_bytes, max_aliases);
        value
    }
    pub fn construct(
        &mut self,
        max_depth: terrane_int_support::Int,
        max_bytes: terrane_int_support::Int,
        max_aliases: terrane_int_support::Int,
    ) {
        self.max_depth = max_depth.clone();
        self.max_bytes = max_bytes.clone();
        self.max_aliases = max_aliases.clone();
    }
}
pub fn default_yaml_options() -> YamlOptions {
    return YamlOptions::terrane_construct(
        terrane_int_support::Int::from(128_i128),
        terrane_int_support::Int::from(16777216_i128),
        terrane_int_support::Int::from(64_i128),
    );
}
pub fn make_yaml_options(
    max_depth: terrane_int_support::Int,
    max_bytes: terrane_int_support::Int,
    max_aliases: terrane_int_support::Int,
) -> YamlOptions {
    return YamlOptions::terrane_construct(
        max_depth.clone(),
        max_bytes.clone(),
        max_aliases.clone(),
    );
}
pub fn parse_yaml(input: String, options: YamlOptions) -> DocumentResult {
    let raw: terrane_document_support::DataResult = terrane_yaml_parse(
        input,
        options.max_depth.clone(),
        options.max_bytes.clone(),
        options.max_aliases.clone(),
    );
    return make_document_result(raw);
}
pub fn stringify_yaml(value: DocumentValue) -> DocumentResult {
    let raw: terrane_document_support::DataResult = terrane_json_canonical(
        value.encoded,
    );
    return make_document_result(raw);
}
pub fn decode_yaml(
    input: String,
    mapping: DocumentMapping,
    options: YamlOptions,
) -> DocumentResult {
    let parsed: DocumentResult = parse_yaml(input, options.clone());
    if parsed.failed {
        return parsed.clone();
    }
    return decode_document(parsed.value, mapping.clone());
}
pub fn encode_yaml(value: DocumentValue) -> DocumentResult {
    return stringify_yaml(value.clone());
}
