// Generated deterministically by Terrane <version>.
type TerraneSite = u32;
const TERRANE_NO_SITE: TerraneSite = u32::MAX;
#[allow(dead_code, reason = "custom descriptors are absent from some lowered programs")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct DescriptorId(u16);
#[allow(
    dead_code,
    reason = "one canonical runtime enum covers every compiler-owned throwable kind"
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u16)]
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
    fn default_message(self) -> &'static str {
        match self {
            Self::ArithmeticOverflow => "fixed-width integer arithmetic overflow",
            Self::DivisionByZero => "integer division by zero",
            Self::IntegerConversionOverflow => "integer conversion overflow",
            Self::NegativeShiftCount => "negative integer shift count",
            Self::CoercionError => "coercion has no compatible result",
            Self::DecodeError => "invalid byte sequence for selected encoding",
            Self::IndexError => "collection index is out of range",
            Self::MissingKey => "collection key is absent",
            Self::ResourceError => {
                "integer shift count cannot be represented on this target"
            }
            Self::SourceError => "source error",
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct TerraneErrorDetail {
    message: Option<String>,
    cause: Option<Box<TerraneError>>,
    frames: Vec<TerraneSite>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerraneError {
    kind: TerraneErrorKind,
    origin: TerraneSite,
    detail: Option<Box<TerraneErrorDetail>>,
}
#[cfg(target_pointer_width = "64")]
const _: () = assert!(std::mem::size_of::< TerraneError > () == 16);
#[cfg(target_pointer_width = "64")]
const _: () = assert!(std::mem::size_of::< Result < i64, TerraneError >> () == 16);
#[allow(
    dead_code,
    reason = "one canonical runtime implementation serves every lowered error shape"
)]
impl TerraneError {
    #[cold]
    #[inline(never)]
    fn raised(kind: TerraneErrorKind, origin: TerraneSite) -> Self {
        Self { kind, origin, detail: None }
    }
    #[cold]
    #[inline(never)]
    fn raised_with_message(
        kind: TerraneErrorKind,
        message: impl Into<String>,
        origin: TerraneSite,
    ) -> Self {
        Self {
            kind,
            origin,
            detail: Some(
                Box::new(TerraneErrorDetail {
                    message: Some(message.into()),
                    cause: None,
                    frames: Vec::new(),
                }),
            ),
        }
    }
    #[cold]
    #[inline(never)]
    fn with_cause(mut self, cause: TerraneError) -> Self {
        self
            .detail
            .get_or_insert_with(|| {
                Box::new(TerraneErrorDetail {
                    message: None,
                    cause: None,
                    frames: Vec::new(),
                })
            })
            .cause = Some(Box::new(cause));
        self
    }
    #[cold]
    #[inline(never)]
    fn attributed(mut self, origin: TerraneSite) -> Self {
        debug_assert_eq!(self.origin, TERRANE_NO_SITE);
        self.origin = origin;
        self
    }
    #[cold]
    #[inline(never)]
    fn at(mut self, frame: TerraneSite) -> Self {
        self.detail
            .get_or_insert_with(|| {
                Box::new(TerraneErrorDetail {
                    message: None,
                    cause: None,
                    frames: Vec::new(),
                })
            })
            .frames
            .push(frame);
        self
    }
    fn message(&self) -> &str {
        self.detail
            .as_ref()
            .and_then(|detail| detail.message.as_deref())
            .unwrap_or_else(|| self.kind.default_message())
    }
    #[cold]
    #[inline(never)]
    fn render(&self) -> String {
        let mut rendered = format!("{}: {}", self.kind.display_name(), self.message());
        if let Some(cause) = self
            .detail
            .as_ref()
            .and_then(|detail| detail.cause.as_ref())
        {
            rendered.push_str("\ncaused by: ");
            rendered.push_str(&cause.render());
        }
        if self.origin != TERRANE_NO_SITE {
            rendered.push_str("\nat ");
            rendered.push_str(&__terrane_trace::render(self.origin));
        }
        if let Some(detail) = &self.detail {
            for frame in &detail.frames {
                rendered.push_str("\nat ");
                rendered.push_str(&__terrane_trace::render(*frame));
            }
        }
        rendered
    }
}
impl std::fmt::Display for TerraneError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.render())
    }
}
#[allow(
    dead_code,
    reason = "fresh support failures are absent from some lowered programs"
)]
trait TerraneRaised {
    fn raised(self, origin: TerraneSite) -> TerraneError;
}
pub struct TerraneForeignError(TerraneError);
impl TerraneForeignError {
    pub fn render(&self) -> String {
        self.0.render()
    }
}
impl TerraneRaised for TerraneForeignError {
    fn raised(self, origin: TerraneSite) -> TerraneError {
        self.0.attributed(origin)
    }
}
impl TerraneRaised for terrane_int_support::ArithmeticError {
    fn raised(self, origin: TerraneSite) -> TerraneError {
        use terrane_int_support::ArithmeticError;
        match self {
            ArithmeticError::DivisionByZero => {
                TerraneError::raised(TerraneErrorKind::DivisionByZero, origin)
            }
            ArithmeticError::ArithmeticOverflow => {
                TerraneError::raised(TerraneErrorKind::ArithmeticOverflow, origin)
            }
            ArithmeticError::NegativeShiftCount => {
                TerraneError::raised(TerraneErrorKind::NegativeShiftCount, origin)
            }
            ArithmeticError::ShiftCountTooLarge => {
                TerraneError::raised(TerraneErrorKind::ResourceError, origin)
            }
            error @ (ArithmeticError::IntegerConversionOverflow
            | ArithmeticError::IntegerConversionOverflowDetail { .. }) => {
                TerraneError::raised_with_message(
                    TerraneErrorKind::IntegerConversionOverflow,
                    error.to_string(),
                    origin,
                )
            }
            error @ (ArithmeticError::InvalidRadix
            | ArithmeticError::InvalidRadixText) => {
                TerraneError::raised_with_message(
                    TerraneErrorKind::CoercionError,
                    error.to_string(),
                    origin,
                )
            }
        }
    }
}
impl TerraneRaised for terrane_string_support::DecodeError {
    fn raised(self, origin: TerraneSite) -> TerraneError {
        TerraneError::raised_with_message(
            TerraneErrorKind::DecodeError,
            self.to_string(),
            origin,
        )
    }
}
impl TerraneRaised for terrane_collection_support::IndexError {
    fn raised(self, origin: TerraneSite) -> TerraneError {
        TerraneError::raised_with_message(
            TerraneErrorKind::IndexError,
            self.to_string(),
            origin,
        )
    }
}
impl TerraneRaised for terrane_collection_support::MissingKey {
    fn raised(self, origin: TerraneSite) -> TerraneError {
        TerraneError::raised_with_message(
            TerraneErrorKind::MissingKey,
            self.to_string(),
            origin,
        )
    }
}
impl TerraneRaised for terrane_collection_support::RangeStepError {
    fn raised(self, origin: TerraneSite) -> TerraneError {
        TerraneError::raised_with_message(
            TerraneErrorKind::SourceError,
            self.to_string(),
            origin,
        )
    }
}
#[allow(
    dead_code,
    reason = "terminating fresh failures are absent from some lowered programs"
)]
#[cold]
#[inline(never)]
fn __terrane_raise<E: TerraneRaised>(error: E, origin: TerraneSite) -> ! {
    __terrane_uncaught(error.raised(origin))
}
#[allow(
    dead_code,
    reason = "propagating failures are absent from some lowered programs"
)]
#[cold]
#[inline(never)]
fn __terrane_trace_error(error: TerraneError, frame: TerraneSite) -> TerraneError {
    error.at(frame)
}
#[allow(
    dead_code,
    reason = "terminating fresh failures are absent from some lowered programs"
)]
#[inline]
fn __terrane_raised<T, E: TerraneRaised>(
    result: Result<T, E>,
    origin: TerraneSite,
) -> T {
    result.unwrap_or_else(|error| __terrane_raise(error, origin))
}
#[allow(
    dead_code,
    reason = "fresh failure propagation is absent from some lowered programs"
)]
#[cold]
#[inline(never)]
fn __terrane_fresh_error<E: TerraneRaised>(
    error: E,
    origin: TerraneSite,
) -> TerraneError {
    error.raised(origin)
}
#[allow(
    dead_code,
    reason = "returning fresh failures are absent from some lowered programs"
)]
#[inline]
fn __terrane_raised_err<T, E: TerraneRaised>(
    result: Result<T, E>,
    origin: TerraneSite,
) -> Result<T, TerraneError> {
    result.map_err(|error| __terrane_fresh_error(error, origin))
}
macro_rules! __terrane_raised_completion {
    ($result:expr, $origin:expr) => {
        match $result { Ok(value) => value, Err(error) => { return
        TerraneCompletion::Error(__terrane_fresh_error(error, $origin)); } }
    };
}
#[allow(
    dead_code,
    reason = "terminating propagation is absent from some lowered programs"
)]
#[inline]
fn __terrane_traced<T>(result: Result<T, TerraneError>, frame: TerraneSite) -> T {
    result
        .unwrap_or_else(|error| __terrane_uncaught(__terrane_trace_error(error, frame)))
}
#[allow(
    dead_code,
    reason = "returning propagation is absent from some lowered programs"
)]
#[inline]
fn __terrane_traced_err<T>(
    result: Result<T, TerraneError>,
    frame: TerraneSite,
) -> Result<T, TerraneError> {
    result.map_err(|error| __terrane_trace_error(error, frame))
}
macro_rules! __terrane_traced_completion {
    ($result:expr, $frame:expr) => {
        match $result { Ok(value) => value, Err(error) => { return
        TerraneCompletion::Error(__terrane_trace_error(error, $frame)); } }
    };
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
mod __terrane_error_registry {
    #[allow(dead_code, reason = "custom descriptors are absent from some programs")]
    pub static DESCRIPTORS: [&str; 0] = [];
}
mod __terrane_trace {
    pub struct Site {
        pub function: u32,
        pub file: u32,
        pub line: u32,
        pub column: u32,
        pub end_line: u32,
        pub end_column: u32,
    }
    pub static FILES: [&str; 2] = ["case.trn", "core/documents.trn"];
    pub static FUNCTIONS: [&str; 4] = [
        "/typed-document-decoding::main",
        "/core/documents::make-document-list",
        "/core/documents::mapping-required-fields",
        "/core/documents::decode-document",
    ];
    pub static SITES: [Site; 17] = [
        /* terrane-site-row: site 0: /typed-document-decoding::main (case.trn:39:12-39:34) */
        { Site { function: 0, file: 0, line: 39, column: 12, end_line: 39, end_column: 34 } },
        /* terrane-site-row: site 1: /typed-document-decoding::main (case.trn:39:36-39:64) */
        { Site { function: 0, file: 0, line: 39, column: 36, end_line: 39, end_column: 64 } },
        /* terrane-site-row: site 2: /typed-document-decoding::main (case.trn:39:66-39:94) */
        { Site { function: 0, file: 0, line: 39, column: 66, end_line: 39, end_column: 94 } },
        /* terrane-site-row: site 3: /typed-document-decoding::main (case.trn:45:13-45:37) */
        { Site { function: 0, file: 0, line: 45, column: 13, end_line: 45, end_column: 37 } },
        /* terrane-site-row: site 4: /typed-document-decoding::main (case.trn:45:69-45:93) */
        { Site { function: 0, file: 0, line: 45, column: 69, end_line: 45, end_column: 93 } },
        /* terrane-site-row: site 5: /typed-document-decoding::main (case.trn:48:28-48:50) */
        { Site { function: 0, file: 0, line: 48, column: 28, end_line: 48, end_column: 50 } },
        /* terrane-site-row: site 6: /typed-document-decoding::main (case.trn:48:59-48:81) */
        { Site { function: 0, file: 0, line: 48, column: 59, end_line: 48, end_column: 81 } },
        /* terrane-site-row: site 7: /typed-document-decoding::main (case.trn:57:48-57:71) */
        { Site { function: 0, file: 0, line: 57, column: 48, end_line: 57, end_column: 71 } },
        /* terrane-site-row: site 8: /core/documents::make-document-list (core/documents.trn:140:47-140:60) */
        { Site { function: 1, file: 1, line: 140, column: 47, end_line: 140, end_column: 60 } },
        /* terrane-site-row: site 9: /core/documents::mapping-required-fields (core/documents.trn:153:17-153:30) */
        { Site { function: 2, file: 1, line: 153, column: 17, end_line: 153, end_column: 30 } },
        /* terrane-site-row: site 10: /core/documents::mapping-required-fields (core/documents.trn:157:16-157:47) */
        { Site { function: 2, file: 1, line: 157, column: 16, end_line: 157, end_column: 47 } },
        /* terrane-site-row: site 11: /core/documents::mapping-required-fields (core/documents.trn:163:16-163:45) */
        { Site { function: 2, file: 1, line: 163, column: 16, end_line: 163, end_column: 45 } },
        /* terrane-site-row: site 12: /core/documents::decode-document (core/documents.trn:176:12-176:44) */
        { Site { function: 3, file: 1, line: 176, column: 12, end_line: 176, end_column: 44 } },
        /* terrane-site-row: site 13: /core/documents::decode-document (core/documents.trn:177:37-177:69) */
        { Site { function: 3, file: 1, line: 177, column: 37, end_line: 177, end_column: 69 } },
        /* terrane-site-row: site 14: /core/documents::decode-document (core/documents.trn:183:12-183:49) */
        { Site { function: 3, file: 1, line: 183, column: 12, end_line: 183, end_column: 49 } },
        /* terrane-site-row: site 15: /core/documents::decode-document (core/documents.trn:184:36-184:73) */
        { Site { function: 3, file: 1, line: 184, column: 36, end_line: 184, end_column: 73 } },
        /* terrane-site-row: site 16: /core/documents::decode-document (core/documents.trn:185:36-185:73) */
        { Site { function: 3, file: 1, line: 185, column: 36, end_line: 185, end_column: 73 } },
    ];
    #[cold]
    #[inline(never)]
    pub fn render(site: u32) -> String {
        let site = &SITES[usize::try_from(site).expect("site id must fit usize")];
        format!(
            "{} ({}:{}:{}-{}:{})", FUNCTIONS[usize::try_from(site.function)
            .expect("function id must fit usize")], FILES[usize::try_from(site.file)
            .expect("file id must fit usize")], site.line, site.column, site.end_line,
            site.end_column,
        )
    }
}
pub fn terrane_limit(value: &terrane_int_support::Int) -> usize {
    value.as_usize().unwrap_or(0)
}
pub fn terrane_index(value: &terrane_int_support::Int) -> Option<usize> {
    value.as_usize()
}
pub fn terrane_empty_document() -> terrane_document_support::DataResult {
    terrane_document_support::parse_json("null", 0, 4)
}
pub fn terrane_make_document_none() -> terrane_document_support::DataResult {
    terrane_document_support::document_none()
}
pub fn terrane_make_document_bool(value: bool) -> terrane_document_support::DataResult {
    terrane_document_support::document_bool(value)
}
pub fn terrane_make_document_string(
    value: String,
) -> terrane_document_support::DataResult {
    terrane_document_support::document_string(value)
}
pub fn terrane_make_document_integer(
    value: String,
) -> terrane_document_support::DataResult {
    terrane_document_support::document_integer(&value)
}
pub fn terrane_make_document_decimal(
    value: String,
) -> terrane_document_support::DataResult {
    terrane_document_support::document_decimal(&value)
}
pub fn terrane_make_document_list() -> terrane_document_support::DataResult {
    terrane_document_support::document_list()
}
pub fn terrane_document_list_append(
    list: &terrane_document_support::DataResult,
    value: &terrane_document_support::DataResult,
) -> terrane_document_support::DataResult {
    terrane_document_support::document_list_append(list, value)
}
pub fn terrane_make_document_map() -> terrane_document_support::DataResult {
    terrane_document_support::document_map()
}
pub fn terrane_document_map_insert(
    map: &terrane_document_support::DataResult,
    key: String,
    value: &terrane_document_support::DataResult,
) -> terrane_document_support::DataResult {
    terrane_document_support::document_map_insert(map, key, value)
}
pub fn terrane_data_failed(result: &terrane_document_support::DataResult) -> bool {
    result.failed
}
pub fn terrane_data_message(result: &terrane_document_support::DataResult) -> String {
    result.message.clone()
}
pub fn terrane_data_path(result: &terrane_document_support::DataResult) -> String {
    result.path.clone()
}
pub fn terrane_data_expected(result: &terrane_document_support::DataResult) -> String {
    result.expected.clone()
}
pub fn terrane_data_encoded(result: &terrane_document_support::DataResult) -> String {
    result.encoded.clone()
}
pub fn terrane_document_kind(result: &terrane_document_support::DataResult) -> String {
    terrane_document_support::document_kind(result)
}
pub fn terrane_document_text(result: &terrane_document_support::DataResult) -> String {
    terrane_document_support::document_text(result)
}
pub fn terrane_document_coefficient(
    result: &terrane_document_support::DataResult,
) -> String {
    terrane_document_support::document_coefficient(result)
}
pub fn terrane_document_exponent(
    result: &terrane_document_support::DataResult,
) -> terrane_int_support::Int {
    terrane_int_support::Int::from(terrane_document_support::document_exponent(result))
}
pub fn terrane_document_length(
    result: &terrane_document_support::DataResult,
) -> terrane_int_support::Int {
    terrane_int_support::Int::from(
        i128::try_from(terrane_document_support::document_length(result))
            .expect("document length fits in i128"),
    )
}
pub fn terrane_document_item(
    result: &terrane_document_support::DataResult,
    index: terrane_int_support::Int,
) -> terrane_document_support::DataResult {
    terrane_index(&index)
        .map_or_else(
            || terrane_document_support::invalid_document_index(),
            |index| terrane_document_support::document_item(result, index),
        )
}
pub fn terrane_document_key(
    result: &terrane_document_support::DataResult,
    index: terrane_int_support::Int,
) -> String {
    terrane_index(&index)
        .map_or_else(
            String::new,
            |index| terrane_document_support::document_key(result, index),
        )
}
pub fn terrane_document_field(
    result: &terrane_document_support::DataResult,
    key: String,
) -> terrane_document_support::DataResult {
    terrane_document_support::document_field(result, &key)
}
pub fn terrane_string_list(
    value: terrane_collection_support::List<String>,
) -> Vec<String> {
    value.into_iter().collect()
}
pub fn terrane_validate_mapping(
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
pub fn terrane_json_parse(
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
pub fn terrane_json_canonical(
    value: &terrane_document_support::DataResult,
) -> terrane_document_support::DataResult {
    terrane_document_support::canonical_json(value)
}
pub fn terrane_yaml_parse(
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
#[allow(
    dead_code,
    reason = "each diagnostic member is materialized only when Terrane source observes it"
)]
#[derive(Clone)]
struct TerraneDocumentDiagnostic {
    path: String,
    expected: String,
    actual_kind: String,
    reason: &'static str,
    message: String,
    source: String,
    field_source: String,
}
#[derive(Clone)]
struct TerraneDocumentDecodeOutcome<T: Clone> {
    value: T,
    diagnostics: terrane_collection_support::List<TerraneDocumentDiagnostic>,
}
trait TerraneDocumentDecode: Sized {
    fn terrane_decode_document(
        input: &terrane_document_support::DataResult,
        path: &str,
        allow_unknown: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>>;
}
fn __terrane_document_diagnostic(
    path: impl Into<String>,
    expected: impl Into<String>,
    actual_kind: impl Into<String>,
    reason: &'static str,
    message: impl Into<String>,
    source: &str,
    field_source: &str,
) -> TerraneDocumentDiagnostic {
    TerraneDocumentDiagnostic {
        path: path.into(),
        expected: expected.into(),
        actual_kind: actual_kind.into(),
        reason,
        message: message.into(),
        source: source.to_owned(),
        field_source: field_source.to_owned(),
    }
}
fn __terrane_document_child_path(path: &str, key: &str) -> String {
    if !key.is_empty()
        && key
            .chars()
            .all(|character| {
                character.is_ascii_alphanumeric() || character == '_' || character == '-'
            })
    {
        format!("{path}.{key}")
    } else {
        format!("{path}[{key:?}]")
    }
}
fn __terrane_document_type_error<T>(
    input: &terrane_document_support::DataResult,
    path: &str,
    expected: &str,
    source: &str,
    field_source: &str,
) -> Result<T, Vec<TerraneDocumentDiagnostic>> {
    Err(
        vec![
            __terrane_document_diagnostic(path, expected,
            terrane_document_support::document_kind(input), "type-mismatch",
            format!("expected {expected}"), source, field_source,)
        ],
    )
}
impl TerraneDocumentDecode for String {
    fn terrane_decode_document(
        input: &terrane_document_support::DataResult,
        path: &str,
        _allow_unknown: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if terrane_document_support::document_kind(input) == "string" {
            Ok(terrane_document_support::document_text(input))
        } else {
            __terrane_document_type_error(input, path, "string", source, field_source)
        }
    }
}
impl TerraneDocumentDecode for bool {
    fn terrane_decode_document(
        input: &terrane_document_support::DataResult,
        path: &str,
        _allow_unknown: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if terrane_document_support::document_kind(input) == "bool" {
            Ok(terrane_document_support::document_text(input) == "true")
        } else {
            __terrane_document_type_error(input, path, "bool", source, field_source)
        }
    }
}
macro_rules! __terrane_document_float {
    ($type:ty) => {
        impl TerraneDocumentDecode for $type { fn terrane_decode_document(input : &
        terrane_document_support::DataResult, path : & str, _allow_unknown : bool, source
        : & str, field_source : & str,) -> Result < Self, Vec < TerraneDocumentDiagnostic
        >> { let kind = terrane_document_support::document_kind(input); if kind !=
        "integer" && kind != "decimal" { return __terrane_document_type_error(input,
        path, stringify!($type), source, field_source,); } let text =
        terrane_document_support::document_text(input); text.parse::<$type > ().ok()
        .filter(| value | value.is_finite()).ok_or_else(|| {
        vec![__terrane_document_diagnostic(path, stringify!($type), kind,
        "numeric-conversion",
        "document number is outside the finite range of the destination float", source,
        field_source,)] }) } }
    };
}
__terrane_document_float!(f32);
__terrane_document_float!(f64);
impl TerraneDocumentDecode for terrane_int_support::Int {
    fn terrane_decode_document(
        input: &terrane_document_support::DataResult,
        path: &str,
        _allow_unknown: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if terrane_document_support::document_kind(input) != "integer" {
            return __terrane_document_type_error(
                input,
                path,
                "integer",
                source,
                field_source,
            );
        }
        let text = terrane_document_support::document_text(input);
        terrane_int_support::parse_radix(&text, &terrane_int_support::Int::from(10_i128))
            .map_err(|_| {
                vec![
                    __terrane_document_diagnostic(path, "int", "integer",
                    "numeric-conversion", "integer is outside the supported exact range",
                    source, field_source,)
                ]
            })
    }
}
macro_rules! __terrane_fixed_document_integer {
    ($($type:ty),+ $(,)?) => {
        $(impl TerraneDocumentDecode for $type { fn terrane_decode_document(input : &
        terrane_document_support::DataResult, path : & str, _allow_unknown : bool, source
        : & str, field_source : & str,) -> Result < Self, Vec < TerraneDocumentDiagnostic
        >> { if terrane_document_support::document_kind(input) != "integer" { return
        __terrane_document_type_error(input, path, stringify!($type), source,
        field_source,); } terrane_document_support::document_text(input).parse::<$type >
        ().map_err(| _ | { vec![__terrane_document_diagnostic(path, stringify!($type),
        "integer", "numeric-conversion", "integer is outside the destination range",
        source, field_source,)] }) } })+
    };
}
__terrane_fixed_document_integer!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
impl<T: TerraneDocumentDecode> TerraneDocumentDecode for Option<T> {
    fn terrane_decode_document(
        input: &terrane_document_support::DataResult,
        path: &str,
        allow_unknown: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if terrane_document_support::document_kind(input) == "none" {
            Ok(None)
        } else {
            T::terrane_decode_document(input, path, allow_unknown, source, field_source)
                .map(Some)
        }
    }
}
impl<T: TerraneDocumentDecode> TerraneDocumentDecode
for terrane_collection_support::List<T> {
    fn terrane_decode_document(
        input: &terrane_document_support::DataResult,
        path: &str,
        allow_unknown: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if terrane_document_support::document_kind(input) != "list" {
            return __terrane_document_type_error(
                input,
                path,
                "list",
                source,
                field_source,
            );
        }
        let mut values = Vec::new();
        let mut diagnostics = Vec::new();
        for index in 0..terrane_document_support::document_length(input) {
            let item = terrane_document_support::document_item(input, index);
            let item_path = format!("{path}[{index}]");
            match T::terrane_decode_document(
                &item,
                &item_path,
                allow_unknown,
                source,
                field_source,
            ) {
                Ok(value) => values.push(value),
                Err(mut item_diagnostics) => diagnostics.append(&mut item_diagnostics),
            }
        }
        if diagnostics.is_empty() {
            Ok(terrane_collection_support::List::new(values))
        } else {
            Err(diagnostics)
        }
    }
}
impl<T: Clone + TerraneDocumentDecode> TerraneDocumentDecode
for terrane_collection_support::Tuple<T> {
    fn terrane_decode_document(
        input: &terrane_document_support::DataResult,
        path: &str,
        allow_unknown: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let list = terrane_collection_support::List::<
            T,
        >::terrane_decode_document(input, path, allow_unknown, source, field_source)?;
        Ok(terrane_collection_support::Tuple::new(list.into_vec()))
    }
}
impl<T: Clone + TerraneDocumentDecode> TerraneDocumentDecode
for terrane_collection_support::Map<String, T> {
    fn terrane_decode_document(
        input: &terrane_document_support::DataResult,
        path: &str,
        allow_unknown: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if terrane_document_support::document_kind(input) != "map" {
            return __terrane_document_type_error(
                input,
                path,
                "map",
                source,
                field_source,
            );
        }
        let mut entries = Vec::new();
        let mut diagnostics = Vec::new();
        for index in 0..terrane_document_support::document_length(input) {
            let key = terrane_document_support::document_key(input, index);
            let item = terrane_document_support::document_field(input, &key);
            let item_path = format!("{path}.{}", key);
            match T::terrane_decode_document(
                &item,
                &item_path,
                allow_unknown,
                source,
                field_source,
            ) {
                Ok(value) => {
                    entries.push(terrane_collection_support::Entry::new(key, value))
                }
                Err(mut item_diagnostics) => diagnostics.append(&mut item_diagnostics),
            }
        }
        if diagnostics.is_empty() {
            Ok(terrane_collection_support::Map::new(entries))
        } else {
            Err(diagnostics)
        }
    }
}
// Source: case.trn
// Namespace: typed-document-decoding
#[derive(Clone)]
pub struct Endpoint {
    pub host: String,
    pub port: u16,
}
impl Endpoint {
    pub fn terrane_construct() -> Self {
        Self {
            host: String::from(""),
            port: 443,
        }
    }
}
impl TerraneDocumentDecode for Endpoint {
    fn terrane_decode_document(
        input: &terrane_document_support::DataResult,
        path: &str,
        allow_unknown: bool,
        source: &str,
        _field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if input.failed {
            return Err(
                vec![
                    __terrane_document_diagnostic(path, "endpoint", "invalid", "parse",
                    input.message.clone(), source, "case.trn:9:1")
                ],
            );
        }
        if terrane_document_support::document_kind(input) != "map" {
            return __terrane_document_type_error(
                input,
                path,
                "endpoint",
                source,
                "case.trn:9:1",
            );
        }
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &["host", "port"];
        if !allow_unknown {
            for index in 0..terrane_document_support::document_length(input) {
                let key = terrane_document_support::document_key(input, index);
                if !declared_fields.contains(&key.as_str()) {
                    diagnostics
                        .push(
                            __terrane_document_diagnostic(
                                __terrane_document_child_path(path, &key),
                                "endpoint",
                                "present",
                                "unknown-field",
                                format!("unknown field `{key}`"),
                                source,
                                "case.trn:9:1",
                            ),
                        );
                }
            }
        }
        {
            let field = terrane_document_support::document_field(input, "host");
            let field_path = __terrane_document_child_path(path, "host");
            if field.failed {} else {
                match <String as TerraneDocumentDecode>::terrane_decode_document(
                    &field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:10:5",
                ) {
                    Ok(decoded) => value.host = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = terrane_document_support::document_field(input, "port");
            let field_path = __terrane_document_child_path(path, "port");
            if field.failed {} else {
                match <u16 as TerraneDocumentDecode>::terrane_decode_document(
                    &field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:11:5",
                ) {
                    Ok(decoded) => value.port = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl DocumentDecodableProtocol for Endpoint {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<Endpoint> for DocumentDecodable {
    fn from(value: Endpoint) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct ServiceConfig {
    pub service_name: String,
    pub endpoint: Endpoint,
    pub note: Option<String>,
    pub ports: terrane_collection_support::List<u16>,
    pub labels: terrane_collection_support::Map<String, String>,
    pub coordinates: terrane_collection_support::Tuple<terrane_int_support::Int>,
    pub ratio: f64,
}
impl ServiceConfig {
    pub fn terrane_construct() -> Self {
        Self {
            service_name: String::from(""),
            endpoint: Endpoint::terrane_construct(),
            note: None,
            ports: terrane_collection_support::List::<u16>::new(vec![0]),
            labels: terrane_collection_support::Map::<
                String,
                String,
            >::new(
                vec![
                    terrane_collection_support::Entry::< String, String
                    >::new(String::from(""), String::from(""))
                ],
            ),
            coordinates: terrane_collection_support::Tuple::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(0_i128)]),
            ratio: 0.1,
        }
    }
    pub fn validate_document(&self) -> Option<String> {
        if self.service_name == String::from("reserved") {
            return Some(String::from("service name is reserved"));
        }
        return None;
    }
}
impl TerraneDocumentDecode for ServiceConfig {
    fn terrane_decode_document(
        input: &terrane_document_support::DataResult,
        path: &str,
        allow_unknown: bool,
        source: &str,
        _field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if input.failed {
            return Err(
                vec![
                    __terrane_document_diagnostic(path, "service-config", "invalid",
                    "parse", input.message.clone(), source, "case.trn:13:1")
                ],
            );
        }
        if terrane_document_support::document_kind(input) != "map" {
            return __terrane_document_type_error(
                input,
                path,
                "service-config",
                source,
                "case.trn:13:1",
            );
        }
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &[
            "serviceName",
            "endpoint",
            "note",
            "ports",
            "labels",
            "coordinates",
            "ratio",
        ];
        if !allow_unknown {
            for index in 0..terrane_document_support::document_length(input) {
                let key = terrane_document_support::document_key(input, index);
                if !declared_fields.contains(&key.as_str()) {
                    diagnostics
                        .push(
                            __terrane_document_diagnostic(
                                __terrane_document_child_path(path, &key),
                                "service-config",
                                "present",
                                "unknown-field",
                                format!("unknown field `{key}`"),
                                source,
                                "case.trn:13:1",
                            ),
                        );
                }
            }
        }
        {
            let field = terrane_document_support::document_field(input, "serviceName");
            let field_path = __terrane_document_child_path(path, "serviceName");
            if field.failed {} else {
                match <String as TerraneDocumentDecode>::terrane_decode_document(
                    &field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:14:5",
                ) {
                    Ok(decoded) => value.service_name = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = terrane_document_support::document_field(input, "endpoint");
            let field_path = __terrane_document_child_path(path, "endpoint");
            if field.failed {} else {
                match <Endpoint as TerraneDocumentDecode>::terrane_decode_document(
                    &field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:15:5",
                ) {
                    Ok(decoded) => value.endpoint = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = terrane_document_support::document_field(input, "note");
            let field_path = __terrane_document_child_path(path, "note");
            if field.failed {} else {
                match <Option<
                    String,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    &field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:16:5",
                ) {
                    Ok(decoded) => value.note = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = terrane_document_support::document_field(input, "ports");
            let field_path = __terrane_document_child_path(path, "ports");
            if field.failed {} else {
                match <terrane_collection_support::List<
                    u16,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    &field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:17:5",
                ) {
                    Ok(decoded) => value.ports = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = terrane_document_support::document_field(input, "labels");
            let field_path = __terrane_document_child_path(path, "labels");
            if field.failed {} else {
                match <terrane_collection_support::Map<
                    String,
                    String,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    &field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:18:5",
                ) {
                    Ok(decoded) => value.labels = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = terrane_document_support::document_field(input, "coordinates");
            let field_path = __terrane_document_child_path(path, "coordinates");
            if field.failed {} else {
                match <terrane_collection_support::Tuple<
                    terrane_int_support::Int,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    &field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:19:5",
                ) {
                    Ok(decoded) => value.coordinates = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = terrane_document_support::document_field(input, "ratio");
            let field_path = __terrane_document_child_path(path, "ratio");
            if field.failed {} else {
                match <f64 as TerraneDocumentDecode>::terrane_decode_document(
                    &field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:20:5",
                ) {
                    Ok(decoded) => value.ratio = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        if diagnostics.is_empty() {
            if let Some(message) = value.validate_document() {
                diagnostics
                    .push(
                        __terrane_document_diagnostic(
                            path,
                            "service-config",
                            "map",
                            "validation",
                            message,
                            source,
                            "case.trn:13:1",
                        ),
                    );
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl DocumentDecodableProtocol for ServiceConfig {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<ServiceConfig> for DocumentDecodable {
    fn from(value: ServiceConfig) -> Self {
        Self(Box::new(value))
    }
}
impl DocumentValidatableProtocol for ServiceConfig {
    fn clone_box(&self) -> Box<dyn DocumentValidatableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentValidatableProtocol> {
        Box::new(self.clone())
    }
    fn validate_document(&self) -> Option<String> {
        ServiceConfig::validate_document(&*self)
    }
}
impl From<ServiceConfig> for DocumentValidatable {
    fn from(value: ServiceConfig) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct Waypoint {
    pub x: f64,
    pub y: f64,
}
impl Waypoint {
    pub fn terrane_construct() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}
impl TerraneDocumentDecode for Waypoint {
    fn terrane_decode_document(
        input: &terrane_document_support::DataResult,
        path: &str,
        allow_unknown: bool,
        source: &str,
        _field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if input.failed {
            return Err(
                vec![
                    __terrane_document_diagnostic(path, "waypoint", "invalid", "parse",
                    input.message.clone(), source, "case.trn:27:1")
                ],
            );
        }
        if terrane_document_support::document_kind(input) != "map" {
            return __terrane_document_type_error(
                input,
                path,
                "waypoint",
                source,
                "case.trn:27:1",
            );
        }
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &["x", "y"];
        if !allow_unknown {
            for index in 0..terrane_document_support::document_length(input) {
                let key = terrane_document_support::document_key(input, index);
                if !declared_fields.contains(&key.as_str()) {
                    diagnostics
                        .push(
                            __terrane_document_diagnostic(
                                __terrane_document_child_path(path, &key),
                                "waypoint",
                                "present",
                                "unknown-field",
                                format!("unknown field `{key}`"),
                                source,
                                "case.trn:27:1",
                            ),
                        );
                }
            }
        }
        {
            let field = terrane_document_support::document_field(input, "x");
            let field_path = __terrane_document_child_path(path, "x");
            if field.failed {} else {
                match <f64 as TerraneDocumentDecode>::terrane_decode_document(
                    &field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:28:5",
                ) {
                    Ok(decoded) => value.x = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = terrane_document_support::document_field(input, "y");
            let field_path = __terrane_document_child_path(path, "y");
            if field.failed {} else {
                match <f64 as TerraneDocumentDecode>::terrane_decode_document(
                    &field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:29:5",
                ) {
                    Ok(decoded) => value.y = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl DocumentDecodableProtocol for Waypoint {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<Waypoint> for DocumentDecodable {
    fn from(value: Waypoint) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct Route {
    pub name: String,
    pub points: terrane_collection_support::List<Waypoint>,
    pub active: bool,
}
impl Route {
    pub fn terrane_construct() -> Self {
        Self {
            name: String::from(""),
            points: terrane_collection_support::List::<
                Waypoint,
            >::new(vec![Waypoint::terrane_construct()]),
            active: false,
        }
    }
}
impl TerraneDocumentDecode for Route {
    fn terrane_decode_document(
        input: &terrane_document_support::DataResult,
        path: &str,
        allow_unknown: bool,
        source: &str,
        _field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if input.failed {
            return Err(
                vec![
                    __terrane_document_diagnostic(path, "route", "invalid", "parse",
                    input.message.clone(), source, "case.trn:31:1")
                ],
            );
        }
        if terrane_document_support::document_kind(input) != "map" {
            return __terrane_document_type_error(
                input,
                path,
                "route",
                source,
                "case.trn:31:1",
            );
        }
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &["name", "points", "active"];
        if !allow_unknown {
            for index in 0..terrane_document_support::document_length(input) {
                let key = terrane_document_support::document_key(input, index);
                if !declared_fields.contains(&key.as_str()) {
                    diagnostics
                        .push(
                            __terrane_document_diagnostic(
                                __terrane_document_child_path(path, &key),
                                "route",
                                "present",
                                "unknown-field",
                                format!("unknown field `{key}`"),
                                source,
                                "case.trn:31:1",
                            ),
                        );
                }
            }
        }
        {
            let field = terrane_document_support::document_field(input, "name");
            let field_path = __terrane_document_child_path(path, "name");
            if field.failed {} else {
                match <String as TerraneDocumentDecode>::terrane_decode_document(
                    &field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:32:5",
                ) {
                    Ok(decoded) => value.name = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = terrane_document_support::document_field(input, "points");
            let field_path = __terrane_document_child_path(path, "points");
            if field.failed {} else {
                match <terrane_collection_support::List<
                    Waypoint,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    &field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:33:5",
                ) {
                    Ok(decoded) => value.points = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = terrane_document_support::document_field(input, "active");
            let field_path = __terrane_document_child_path(path, "active");
            if field.failed {} else {
                match <bool as TerraneDocumentDecode>::terrane_decode_document(
                    &field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:34:5",
                ) {
                    Ok(decoded) => value.active = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl DocumentDecodableProtocol for Route {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<Route> for DocumentDecodable {
    fn from(value: Route) -> Self {
        Self(Box::new(value))
    }
}
fn main() {
    let decoded: TerraneDocumentDecodeOutcome<ServiceConfig> = {
        let source = String::from(
            "{\"serviceName\":\"api\",\"endpoint\":{\"host\":\"localhost\",\"port\":8443},\"ports\":[80,443],\"labels\":{\"tier\":\"edge\"},\"coordinates\":[4,9],\"ratio\":0.1}",
        );
        let options = default_json_options();
        let input = terrane_document_support::parse_json(
            &source,
            terrane_limit(&options.max_depth),
            terrane_limit(&options.max_bytes),
        );
        match <ServiceConfig as TerraneDocumentDecode>::terrane_decode_document(
            &input,
            "$",
            false,
            "case.trn:37:15",
            "case.trn:37:15",
        ) {
            Ok(value) => {
                TerraneDocumentDecodeOutcome {
                    value,
                    diagnostics: terrane_collection_support::List::new(Vec::new()),
                }
            }
            Err(diagnostics) => {
                TerraneDocumentDecodeOutcome {
                    value: ServiceConfig::terrane_construct(),
                    diagnostics: terrane_collection_support::List::new(diagnostics),
                }
            }
        }
    };
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&(decoded.diagnostics.length() !=
        0)), terrane_scalar_support::scalar_text(&decoded.value.clone().service_name),
        terrane_scalar_support::scalar_text(&decoded.value.clone().endpoint.host),
        terrane_scalar_support::scalar_text(&decoded.value.clone().endpoint.port)
    );
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(decoded.value
        .clone().ports
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        0 /* terrane-site: case.trn:39:12-39:34 */)), 0 /* terrane-site: case.trn:39:12-39:34 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(decoded.value.clone()
        .labels.get_or_error(&String::from("tier")), 1 /* terrane-site: case.trn:39:36-39:64 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(decoded.value.clone()
        .coordinates
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        2 /* terrane-site: case.trn:39:66-39:94 */)), 2 /* terrane-site: case.trn:39:66-39:94 */)), terrane_scalar_support::scalar_text(&decoded.value
        .clone().ratio)
    );
    let malformed: TerraneDocumentDecodeOutcome<ServiceConfig> = {
        let source = String::from(
            "{\"serviceName\":3,\"endpoint\":{\"host\":false},\"ports\":[1,70000],\"ratio\":0.1,\"extra\":true}",
        );
        let options = default_json_options();
        let input = terrane_document_support::parse_json(
            &source,
            terrane_limit(&options.max_depth),
            terrane_limit(&options.max_bytes),
        );
        match <ServiceConfig as TerraneDocumentDecode>::terrane_decode_document(
            &input,
            "$",
            false,
            "case.trn:41:17",
            "case.trn:41:17",
        ) {
            Ok(value) => {
                TerraneDocumentDecodeOutcome {
                    value,
                    diagnostics: terrane_collection_support::List::new(Vec::new()),
                }
            }
            Err(diagnostics) => {
                TerraneDocumentDecodeOutcome {
                    value: ServiceConfig::terrane_construct(),
                    diagnostics: terrane_collection_support::List::new(diagnostics),
                }
            }
        }
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(malformed.diagnostics.length() !=
        0)),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(malformed
        .diagnostics.clone().length()))
    );
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &malformed.diagnostics.clone(),
    );
    loop {
        let diagnostic = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&diagnostic.path.clone()),
            terrane_scalar_support::scalar_text(&String::from(":")),
            terrane_scalar_support::scalar_text(&diagnostic.reason.to_owned())
        );
    }
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(malformed
        .diagnostics.clone()
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        3 /* terrane-site: case.trn:45:13-45:37 */)), 3 /* terrane-site: case.trn:45:13-45:37 */).source.clone().contains(&String::from("case.trn"))),
        terrane_scalar_support::scalar_text(&__terrane_raised(malformed.diagnostics
        .clone()
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        4 /* terrane-site: case.trn:45:69-45:93 */)), 4 /* terrane-site: case.trn:45:69-45:93 */).field_source.clone()
        .contains(&String::from("case.trn")))
    );
    let invalid: TerraneDocumentDecodeOutcome<ServiceConfig> = {
        let source = String::from(
            "{\"serviceName\":\"reserved\",\"endpoint\":{\"host\":\"ok\"}}",
        );
        let options = default_json_options();
        let input = terrane_document_support::parse_json(
            &source,
            terrane_limit(&options.max_depth),
            terrane_limit(&options.max_bytes),
        );
        match <ServiceConfig as TerraneDocumentDecode>::terrane_decode_document(
            &input,
            "$",
            false,
            "case.trn:47:15",
            "case.trn:47:15",
        ) {
            Ok(value) => {
                TerraneDocumentDecodeOutcome {
                    value,
                    diagnostics: terrane_collection_support::List::new(Vec::new()),
                }
            }
            Err(diagnostics) => {
                TerraneDocumentDecodeOutcome {
                    value: ServiceConfig::terrane_construct(),
                    diagnostics: terrane_collection_support::List::new(diagnostics),
                }
            }
        }
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(invalid.diagnostics.length() !=
        0)), terrane_scalar_support::scalar_text(&__terrane_raised(invalid.diagnostics
        .clone()
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        5 /* terrane-site: case.trn:48:28-48:50 */)), 5 /* terrane-site: case.trn:48:28-48:50 */).reason.to_owned()),
        terrane_scalar_support::scalar_text(&__terrane_raised(invalid.diagnostics.clone()
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        6 /* terrane-site: case.trn:48:59-48:81 */)), 6 /* terrane-site: case.trn:48:59-48:81 */).message.clone())
    );
    let permissive: TerraneDocumentDecodeOutcome<ServiceConfig> = {
        let source = String::from(
            "{\"serviceName\":\"extra\",\"endpoint\":{\"host\":\"ok\"},\"extra\":true}",
        );
        let options = default_json_options();
        let input = terrane_document_support::parse_json(
            &source,
            terrane_limit(&options.max_depth),
            terrane_limit(&options.max_bytes),
        );
        match <ServiceConfig as TerraneDocumentDecode>::terrane_decode_document(
            &input,
            "$",
            true,
            "case.trn:50:18",
            "case.trn:50:18",
        ) {
            Ok(value) => {
                TerraneDocumentDecodeOutcome {
                    value,
                    diagnostics: terrane_collection_support::List::new(Vec::new()),
                }
            }
            Err(diagnostics) => {
                TerraneDocumentDecodeOutcome {
                    value: ServiceConfig::terrane_construct(),
                    diagnostics: terrane_collection_support::List::new(diagnostics),
                }
            }
        }
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(permissive.diagnostics.length() !=
        0)), terrane_scalar_support::scalar_text(&permissive.value.clone().endpoint.port)
    );
    let yaml: TerraneDocumentDecodeOutcome<ServiceConfig> = {
        let source = String::from("serviceName: yaml\nendpoint:\n  host: local");
        let options = default_yaml_options();
        let input = terrane_document_support::parse_yaml(
            &source,
            terrane_limit(&options.max_depth),
            terrane_limit(&options.max_bytes),
            terrane_limit(&options.max_alias_nodes),
        );
        match <ServiceConfig as TerraneDocumentDecode>::terrane_decode_document(
            &input,
            "$",
            false,
            "case.trn:53:12",
            "case.trn:53:12",
        ) {
            Ok(value) => {
                TerraneDocumentDecodeOutcome {
                    value,
                    diagnostics: terrane_collection_support::List::new(Vec::new()),
                }
            }
            Err(diagnostics) => {
                TerraneDocumentDecodeOutcome {
                    value: ServiceConfig::terrane_construct(),
                    diagnostics: terrane_collection_support::List::new(diagnostics),
                }
            }
        }
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(yaml.diagnostics.length() != 0)),
        terrane_scalar_support::scalar_text(&yaml.value.clone().service_name),
        terrane_scalar_support::scalar_text(&yaml.value.clone().endpoint.host)
    );
    let journey: TerraneDocumentDecodeOutcome<Route> = {
        let source = String::from(
            "{\"name\":\"coast\",\"points\":[{\"x\":1.25,\"y\":2.75},{\"x\":3.5,\"y\":4.5}],\"active\":true}",
        );
        let options = default_json_options();
        let input = terrane_document_support::parse_json(
            &source,
            terrane_limit(&options.max_depth),
            terrane_limit(&options.max_bytes),
        );
        match <Route as TerraneDocumentDecode>::terrane_decode_document(
            &input,
            "$",
            false,
            "case.trn:56:15",
            "case.trn:56:15",
        ) {
            Ok(value) => {
                TerraneDocumentDecodeOutcome {
                    value,
                    diagnostics: terrane_collection_support::List::new(Vec::new()),
                }
            }
            Err(diagnostics) => {
                TerraneDocumentDecodeOutcome {
                    value: Route::terrane_construct(),
                    diagnostics: terrane_collection_support::List::new(diagnostics),
                }
            }
        }
    };
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&(journey.diagnostics.length() !=
        0)), terrane_scalar_support::scalar_text(&journey.value.clone().name),
        terrane_scalar_support::scalar_text(&__terrane_raised(journey.value.clone()
        .points
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        7 /* terrane-site: case.trn:57:48-57:71 */)), 7 /* terrane-site: case.trn:57:48-57:71 */).y), terrane_scalar_support::scalar_text(&journey.value
        .clone().active)
    );
}
// Source: core/documents.trn
// Namespace: core/documents
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
pub struct Serializable(Box<dyn SerializableProtocol>);
impl Clone for Serializable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
pub struct Deserializable(Box<dyn DeserializableProtocol>);
impl Clone for Deserializable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Deserializable {
    pub fn from_document(&self, value: DocumentValue) -> DocumentResult {
        self.0.from_document(value)
    }
}
pub trait DocumentDecodableProtocol {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol>;
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol>;
}
impl Clone for Box<dyn DocumentDecodableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
#[allow(
    dead_code,
    reason = "marker interface storage is materialized only when a value is erased to that marker"
)]
pub struct DocumentDecodable(Box<dyn DocumentDecodableProtocol>);
impl Clone for DocumentDecodable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl DocumentDecodable {}
pub trait DocumentValidatableProtocol {
    fn clone_box(&self) -> Box<dyn DocumentValidatableProtocol>;
    fn separate_box(&self) -> Box<dyn DocumentValidatableProtocol>;
    fn validate_document(&self) -> Option<String>;
}
impl Clone for Box<dyn DocumentValidatableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct DocumentValidatable(Box<dyn DocumentValidatableProtocol>);
impl Clone for DocumentValidatable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl DocumentValidatable {
    pub fn validate_document(&self) -> Option<String> {
        self.0.validate_document()
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
        DocumentValue::to_document(&*self)
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
        DocumentMapping::from_document(&*self, value)
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
            &__terrane_raised(
                    values
                        .get_or_error(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(&index.clone()),
                                8 /* terrane-site: core/documents.trn:140:47-140:60 */,
                            ),
                        ),
                    8 /* terrane-site: core/documents.trn:140:47-140:60 */,
                )
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
    {
        let __terrane_list_append_0 = required.make_unique();
        while index.clone()
            < terrane_int_support::Int::from(
                terrane_int_support::Int::from(fields.length()),
            )
        {
            let field: String = __terrane_raised(
                fields
                    .get_or_error(
                        __terrane_raised(
                            terrane_collection_support::index_from_int(&index.clone()),
                            9 /* terrane-site: core/documents.trn:153:17-153:30 */,
                        ),
                    ),
                9 /* terrane-site: core/documents.trn:153:17-153:30 */,
            );
            let mut optional: bool = false;
            let mut optional_index: terrane_int_support::Int = terrane_int_support::Int::from(
                0_i128,
            );
            while optional_index.clone()
                < terrane_int_support::Int::from(
                    terrane_int_support::Int::from(optional_fields.length()),
                )
            {
                if __terrane_raised(
                    optional_fields
                        .get_or_error(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(
                                    &optional_index.clone(),
                                ),
                                10 /* terrane-site: core/documents.trn:157:16-157:47 */,
                            ),
                        ),
                    10 /* terrane-site: core/documents.trn:157:16-157:47 */,
                ) == field
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
                if __terrane_raised(
                    default_fields
                        .get_or_error(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(
                                    &default_index.clone(),
                                ),
                                11 /* terrane-site: core/documents.trn:163:16-163:45 */,
                            ),
                        ),
                    11 /* terrane-site: core/documents.trn:163:16-163:45 */,
                ) == field
                {
                    defaulted = true;
                }
                default_index = default_index.clone()
                    + terrane_int_support::Int::from(1_i128);
            }
            if field != String::from("") && !optional && !defaulted {
                __terrane_list_append_0.push(field);
            }
            index = index.clone() + terrane_int_support::Int::from(1_i128);
        }
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
    {
        let __terrane_list_append_1 = declared_fields.make_unique();
        while field_index.clone()
            < terrane_int_support::Int::from(
                terrane_int_support::Int::from(mapping.field_names.length()),
            )
        {
            if __terrane_raised(
                mapping
                    .field_names
                    .get_or_error(
                        __terrane_raised(
                            terrane_collection_support::index_from_int(
                                &field_index.clone(),
                            ),
                            12 /* terrane-site: core/documents.trn:176:12-176:44 */,
                        ),
                    ),
                12 /* terrane-site: core/documents.trn:176:12-176:44 */,
            ) != String::from("")
            {
                __terrane_list_append_1
                    .push(
                        __terrane_raised(
                            mapping
                                .field_names
                                .get_or_error(
                                    __terrane_raised(
                                        terrane_collection_support::index_from_int(
                                            &field_index.clone(),
                                        ),
                                        13 /* terrane-site: core/documents.trn:177:37-177:69 */,
                                    ),
                                ),
                            13 /* terrane-site: core/documents.trn:177:37-177:69 */,
                        ),
                    );
            }
            field_index = field_index.clone() + terrane_int_support::Int::from(1_i128);
        }
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
        if __terrane_raised(
            mapping
                .default_fields
                .get_or_error(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &default_index.clone(),
                        ),
                        14 /* terrane-site: core/documents.trn:183:12-183:49 */,
                    ),
                ),
            14 /* terrane-site: core/documents.trn:183:12-183:49 */,
        ) != String::from("")
        {
            default_fields
                .append(
                    __terrane_raised(
                        mapping
                            .default_fields
                            .get_or_error(
                                __terrane_raised(
                                    terrane_collection_support::index_from_int(
                                        &default_index.clone(),
                                    ),
                                    15 /* terrane-site: core/documents.trn:184:36-184:73 */,
                                ),
                            ),
                        15 /* terrane-site: core/documents.trn:184:36-184:73 */,
                    ),
                );
            default_values
                .append(
                    __terrane_raised(
                        mapping
                            .default_values
                            .get_or_error(
                                __terrane_raised(
                                    terrane_collection_support::index_from_int(
                                        &default_index.clone(),
                                    ),
                                    16 /* terrane-site: core/documents.trn:185:36-185:73 */,
                                ),
                            ),
                        16 /* terrane-site: core/documents.trn:185:36-185:73 */,
                    ),
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
// Source: core/json.trn
// Namespace: core/documents/json
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
// Source: core/yaml.trn
// Namespace: core/documents/yaml
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
