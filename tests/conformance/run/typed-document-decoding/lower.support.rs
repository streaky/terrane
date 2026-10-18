// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support, terrane-document-support
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
    ReplaceError,
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
            Self::ReplaceError => "replace-error",
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
            Self::ReplaceError => {
                "replacement search and replacement lists have different lengths"
            }
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
    rendered_message: Option<String>,
    cause: Option<std::boxed::Box<TerraneError>>,
    frames: Vec<TerraneSite>,
    structured: Vec<String>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerraneError {
    kind: TerraneErrorKind,
    origin: TerraneSite,
    detail: Option<std::boxed::Box<TerraneErrorDetail>>,
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
                std::boxed::Box::new(TerraneErrorDetail {
                    message: Some(message.into()),
                    rendered_message: None,
                    cause: None,
                    frames: Vec::new(),
                    structured: Vec::new(),
                }),
            ),
        }
    }
    #[cold]
    #[inline(never)]
    fn raised_with_rendered_message(
        kind: TerraneErrorKind,
        message: impl Into<String>,
        rendered_message: Option<String>,
        origin: TerraneSite,
    ) -> Self {
        Self {
            kind,
            origin,
            detail: Some(
                std::boxed::Box::new(TerraneErrorDetail {
                    message: Some(message.into()),
                    rendered_message,
                    cause: None,
                    frames: Vec::new(),
                    structured: Vec::new(),
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
                std::boxed::Box::new(TerraneErrorDetail {
                    message: None,
                    rendered_message: None,
                    cause: None,
                    frames: Vec::new(),
                    structured: Vec::new(),
                })
            })
            .cause = Some(std::boxed::Box::new(cause));
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
                std::boxed::Box::new(TerraneErrorDetail {
                    message: None,
                    rendered_message: None,
                    cause: None,
                    frames: Vec::new(),
                    structured: Vec::new(),
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
    fn rendered_message(&self) -> &str {
        self.detail
            .as_ref()
            .and_then(|detail| detail.rendered_message.as_deref())
            .unwrap_or_else(|| self.message())
    }
    fn descriptor_name(&self) -> &str {
        self.kind.display_name()
    }
    fn source_frames(&self) -> Vec<String> {
        let mut frames = Vec::new();
        if self.origin != TERRANE_NO_SITE {
            frames.push(__terrane_trace::render(self.origin));
        }
        if let Some(detail) = &self.detail {
            frames
                .extend(
                    detail.frames.iter().map(|frame| __terrane_trace::render(*frame)),
                );
        }
        frames
    }
    fn with_structured_details(mut self, structured: Vec<String>) -> Self {
        self
            .detail
            .get_or_insert_with(|| {
                std::boxed::Box::new(TerraneErrorDetail {
                    message: None,
                    rendered_message: None,
                    cause: None,
                    frames: Vec::new(),
                    structured: Vec::new(),
                })
            })
            .structured = structured;
        self
    }
    fn structured_details(&self) -> &[String] {
        self.detail.as_deref().map_or(&[], |detail| detail.structured.as_slice())
    }
    #[cold]
    #[inline(never)]
    fn render(&self) -> String {
        let mut rendered = format!(
            "{}: {}", self.kind.display_name(), self.rendered_message()
        );
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
impl TerraneRaised for terrane_string_support::ReplaceError {
    fn raised(self, origin: TerraneSite) -> TerraneError {
        TerraneError::raised_with_message(
            TerraneErrorKind::ReplaceError,
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
        /* terrane-site-row: site 0: /typed-document-decoding::main (case.trn:38:12-38:34) */
        { Site { function: 0, file: 0, line: 38, column: 12, end_line: 38, end_column: 34 } },
        /* terrane-site-row: site 1: /typed-document-decoding::main (case.trn:38:36-38:64) */
        { Site { function: 0, file: 0, line: 38, column: 36, end_line: 38, end_column: 64 } },
        /* terrane-site-row: site 2: /typed-document-decoding::main (case.trn:38:66-38:94) */
        { Site { function: 0, file: 0, line: 38, column: 66, end_line: 38, end_column: 94 } },
        /* terrane-site-row: site 3: /typed-document-decoding::main (case.trn:44:13-44:37) */
        { Site { function: 0, file: 0, line: 44, column: 13, end_line: 44, end_column: 37 } },
        /* terrane-site-row: site 4: /typed-document-decoding::main (case.trn:44:69-44:93) */
        { Site { function: 0, file: 0, line: 44, column: 69, end_line: 44, end_column: 93 } },
        /* terrane-site-row: site 5: /typed-document-decoding::main (case.trn:47:28-47:50) */
        { Site { function: 0, file: 0, line: 47, column: 28, end_line: 47, end_column: 50 } },
        /* terrane-site-row: site 6: /typed-document-decoding::main (case.trn:47:59-47:81) */
        { Site { function: 0, file: 0, line: 47, column: 59, end_line: 47, end_column: 81 } },
        /* terrane-site-row: site 7: /typed-document-decoding::main (case.trn:56:48-56:71) */
        { Site { function: 0, file: 0, line: 56, column: 48, end_line: 56, end_column: 71 } },
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
enum TerraneDocumentPath<'a> {
    Root(&'a str),
    Field(&'a TerraneDocumentPath<'a>, &'a str),
    MapKey(&'a TerraneDocumentPath<'a>, &'a str),
    Index(&'a TerraneDocumentPath<'a>, usize),
}
impl std::fmt::Display for TerraneDocumentPath<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Root(path) => formatter.write_str(path),
            Self::Field(parent, key) => {
                if !key.is_empty()
                    && key
                        .chars()
                        .all(|character| {
                            character.is_ascii_alphanumeric() || character == '_'
                                || character == '-'
                        })
                {
                    write!(formatter, "{parent}.{key}")
                } else {
                    write!(formatter, "{parent}[{key:?}]")
                }
            }
            Self::MapKey(parent, key) => write!(formatter, "{parent}.{key}"),
            Self::Index(parent, index) => write!(formatter, "{parent}[{index}]"),
        }
    }
}
trait TerraneDocumentDecode: Sized {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>>;
}
fn __terrane_document_diagnostic(
    path: &TerraneDocumentPath<'_>,
    expected: impl Into<String>,
    actual_kind: impl Into<String>,
    reason: &'static str,
    message: impl Into<String>,
    source: &str,
    field_source: &str,
) -> TerraneDocumentDiagnostic {
    TerraneDocumentDiagnostic {
        path: path.to_string(),
        expected: expected.into(),
        actual_kind: actual_kind.into(),
        reason,
        message: message.into(),
        source: source.to_owned(),
        field_source: field_source.to_owned(),
    }
}
fn __terrane_document_kind(input: &terrane_document_support::Document) -> &'static str {
    use terrane_document_support::Document;
    match input {
        Document::None => "none",
        Document::Bool(_) => "bool",
        Document::Integer(_) => "integer",
        Document::Decimal(_) => "decimal",
        Document::String(_) => "string",
        Document::List(_) => "list",
        Document::Map(_) => "map",
    }
}
fn __terrane_document_child_path<'a>(
    path: &'a TerraneDocumentPath<'a>,
    key: &'a str,
) -> TerraneDocumentPath<'a> {
    TerraneDocumentPath::Field(path, key)
}
fn __terrane_document_type_error<T>(
    input: &terrane_document_support::Document,
    path: &TerraneDocumentPath<'_>,
    expected: &str,
    source: &str,
    field_source: &str,
) -> Result<T, Vec<TerraneDocumentDiagnostic>> {
    Err(
        vec![
            __terrane_document_diagnostic(path, expected, __terrane_document_kind(input),
            "type-mismatch", format!("expected {expected}"), source, field_source,)
        ],
    )
}
impl TerraneDocumentDecode for String {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        _: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if let terrane_document_support::Document::String(value) = input {
            Ok(std::mem::take(value))
        } else {
            __terrane_document_type_error(input, path, "string", source, field_source)
        }
    }
}
impl TerraneDocumentDecode for bool {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        _: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if let terrane_document_support::Document::Bool(value) = input {
            Ok(*value)
        } else {
            __terrane_document_type_error(input, path, "bool", source, field_source)
        }
    }
}
macro_rules! __terrane_document_float {
    ($type:ty) => {
        impl TerraneDocumentDecode for $type { fn terrane_decode_document(input : & mut
        terrane_document_support::Document, path : & TerraneDocumentPath <'_ >, _ : bool,
        source : & str, field_source : & str,) -> Result < Self, Vec <
        TerraneDocumentDiagnostic >> { use terrane_document_support::Document; let text =
        match input { Document::Integer(text) | Document::Decimal(text) => text, _ => {
        return __terrane_document_type_error(input, path, stringify!($type), source,
        field_source,); } }; text.parse::<$type > ().ok().filter(| value | value
        .is_finite()).ok_or_else(|| { vec![__terrane_document_diagnostic(path,
        stringify!($type), __terrane_document_kind(input), "numeric-conversion",
        "document number is outside the finite range of the destination float", source,
        field_source,)] }) } }
    };
}
__terrane_document_float!(f32);
__terrane_document_float!(f64);
fn __terrane_document_integer_parts(
    text: &str,
) -> Option<(std::borrow::Cow<'_, str>, u32)> {
    let (coefficient, exponent) = text.split_once('e')?;
    let fraction_length = coefficient
        .split_once('.')
        .map_or(0, |(_, fraction)| fraction.len());
    let zeros = exponent
        .parse::<i64>()
        .ok()?
        .checked_sub(i64::try_from(fraction_length).ok()?)?;
    let coefficient = if fraction_length == 0 {
        std::borrow::Cow::Borrowed(coefficient)
    } else {
        std::borrow::Cow::Owned(coefficient.replace('.', ""))
    };
    Some((coefficient, u32::try_from(zeros).ok()?))
}
impl TerraneDocumentDecode for terrane_int_support::Int {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        _: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let terrane_document_support::Document::Integer(text) = input else {
            return __terrane_document_type_error(
                input,
                path,
                "integer",
                source,
                field_source,
            );
        };
        let decoded = text
            .parse::<i128>()
            .ok()
            .map(terrane_int_support::Int::from)
            .or_else(|| {
                let radix = terrane_int_support::Int::from(10_i128);
                if let Some((coefficient, mut zeros)) = __terrane_document_integer_parts(
                    text,
                ) {
                    let mut value = terrane_int_support::parse_radix(
                            &coefficient,
                            &radix,
                        )
                        .ok()?;
                    let mut factor = radix;
                    while zeros != 0 {
                        if zeros & 1 != 0 {
                            value = value * factor.clone();
                        }
                        zeros >>= 1;
                        if zeros != 0 {
                            factor = factor.clone() * factor;
                        }
                    }
                    Some(value)
                } else {
                    terrane_int_support::parse_radix(text, &radix).ok()
                }
            });
        decoded
            .ok_or_else(|| {
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
        $(impl TerraneDocumentDecode for $type { fn terrane_decode_document(input : & mut
        terrane_document_support::Document, path : & TerraneDocumentPath <'_ >, _ : bool,
        source : & str, field_source : & str,) -> Result < Self, Vec <
        TerraneDocumentDiagnostic >> { let
        terrane_document_support::Document::Integer(text) = input else { return
        __terrane_document_type_error(input, path, stringify!($type), source,
        field_source); }; let decoded = text.parse::<$type > ().ok().or_else(|| { let
        (coefficient, zeros) = __terrane_document_integer_parts(text) ?; let value =
        coefficient.parse::<$type > ().ok() ?; let ten : $type = 10; value
        .checked_mul(ten.checked_pow(zeros) ?) }); decoded.ok_or_else(||
        vec![__terrane_document_diagnostic(path, stringify!($type), "integer",
        "numeric-conversion", "integer is outside the destination range", source,
        field_source,)]) } })+
    };
}
__terrane_fixed_document_integer!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
impl<T: TerraneDocumentDecode> TerraneDocumentDecode for Option<T> {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if matches!(input, terrane_document_support::Document::None) {
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
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let terrane_document_support::Document::List(items) = input else {
            return __terrane_document_type_error(
                input,
                path,
                "list",
                source,
                field_source,
            );
        };
        let mut values = Vec::with_capacity(items.len());
        let mut diagnostics = Vec::new();
        for (index, item) in items.iter_mut().enumerate() {
            let item_path = TerraneDocumentPath::Index(path, index);
            match T::terrane_decode_document(
                item,
                &item_path,
                allow_unknown,
                source,
                field_source,
            ) {
                Ok(value) => values.push(value),
                Err(mut errors) => diagnostics.append(&mut errors),
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
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let values = terrane_collection_support::List::<
            T,
        >::terrane_decode_document(input, path, allow_unknown, source, field_source)?;
        Ok(terrane_collection_support::Tuple::new(values.into_vec()))
    }
}
impl<T: Clone + TerraneDocumentDecode> TerraneDocumentDecode
for terrane_collection_support::Map<String, T> {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let terrane_document_support::Document::Map(items) = input else {
            return __terrane_document_type_error(
                input,
                path,
                "map",
                source,
                field_source,
            );
        };
        let mut entries = Vec::with_capacity(items.len());
        let mut diagnostics = Vec::new();
        for (key, mut item) in std::mem::take(items) {
            let item_path = TerraneDocumentPath::MapKey(path, &key);
            match T::terrane_decode_document(
                &mut item,
                &item_path,
                allow_unknown,
                source,
                field_source,
            ) {
                Ok(value) => {
                    entries.push(terrane_collection_support::Entry::new(key, value))
                }
                Err(mut errors) => diagnostics.append(&mut errors),
            }
        }
        if diagnostics.is_empty() {
            Ok(terrane_collection_support::Map::new(entries))
        } else {
            Err(diagnostics)
        }
    }
}
// Source: core/documents.trn
// Namespace: core/documents
#[derive(Clone)]
pub struct DocumentInteger {
    pub text: String,
}
impl DocumentInteger {
    pub fn terrane_construct(text: String) -> Self {
        let mut __terrane_constructed_value = Self { text: String::from("0") };
        __terrane_constructed_value.construct(text);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, text: String) {
        {
            let __terrane_value = text;
            self.text = __terrane_value;
        }
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
        let mut __terrane_constructed_value = Self {
            coefficient: String::from("0"),
            exponent: terrane_int_support::Int::from(0_i128),
            text: String::from("0"),
        };
        __terrane_constructed_value.construct(coefficient, exponent, text);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        coefficient: String,
        exponent: terrane_int_support::Int,
        text: String,
    ) {
        {
            let __terrane_value = coefficient;
            self.coefficient = __terrane_value;
        }
        {
            let __terrane_value = exponent.clone();
            self.exponent = __terrane_value;
        }
        {
            let __terrane_value = text;
            self.text = __terrane_value;
        }
    }
}
pub trait SerializableProtocol: Send + Sync {
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
pub trait DeserializableProtocol: Send + Sync {
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
pub trait DocumentDecodableProtocol: Send + Sync {
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
pub trait DocumentValidatableProtocol: Send + Sync {
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
        let mut __terrane_constructed_value = Self {
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
        __terrane_constructed_value.construct(raw);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, raw: terrane_document_support::DataResult) {
        {
            let __terrane_value = terrane_document_kind(&raw);
            self.kind = __terrane_value;
        }
        {
            let __terrane_value = terrane_document_text(&raw);
            self.scalar = __terrane_value;
        }
        {
            let __terrane_value = terrane_data_encoded(&raw);
            self.encoded = __terrane_value;
        }
        if self.kind.as_str() == "integer" {
            {
                let __terrane_value = DocumentInteger::terrane_construct(
                    self.scalar.clone(),
                );
                self.integer = __terrane_value;
            }
        }
        if self.kind.as_str() == "decimal" {
            {
                let __terrane_value = DocumentDecimal::terrane_construct(
                    terrane_document_coefficient(&raw),
                    terrane_document_exponent(&raw),
                    self.scalar.clone(),
                );
                self.decimal = __terrane_value;
            }
        }
        {
            let __terrane_value = raw;
            self.raw = __terrane_value;
        }
    }
    pub fn length(&self) -> terrane_int_support::Int {
        return terrane_document_length(&self.raw);
    }
    pub fn to_document(&self) -> DocumentValue {
        return self.clone();
    }
    pub fn item(&self, index: terrane_int_support::Int) -> DocumentResult {
        let raw: terrane_document_support::DataResult;
        raw = terrane_document_item(&self.raw, index.clone());
        return make_document_result(raw);
    }
    pub fn key(&self, index: terrane_int_support::Int) -> String {
        return terrane_document_key(&self.raw, index.clone());
    }
    pub fn field(&self, name: String) -> DocumentResult {
        let raw: terrane_document_support::DataResult;
        raw = terrane_document_field(&self.raw, name);
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
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            path: String::from("$"),
            expected: String::from(""),
            value: DocumentValue::terrane_construct(terrane_empty_document()),
        };
        __terrane_constructed_value.construct(failed, message, path, expected, raw);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        message: String,
        path: String,
        expected: String,
        raw: terrane_document_support::DataResult,
    ) {
        {
            let __terrane_value = failed;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value = message;
            self.message = __terrane_value;
        }
        {
            let __terrane_value = path;
            self.path = __terrane_value;
        }
        {
            let __terrane_value = expected;
            self.expected = __terrane_value;
        }
        {
            let __terrane_value = DocumentValue::terrane_construct(raw);
            self.value = __terrane_value;
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
    pub default_values: terrane_collection_support::List<String>,
    pub allow_unknown: bool,
}
impl DocumentMapping {
    pub fn terrane_construct(
        descriptor_name: String,
        expected_kind: String,
        allow_unknown: bool,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
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
        __terrane_constructed_value
            .construct(descriptor_name, expected_kind, allow_unknown);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        descriptor_name: String,
        expected_kind: String,
        allow_unknown: bool,
    ) {
        {
            let __terrane_value = descriptor_name;
            self.descriptor_name = __terrane_value;
        }
        {
            let __terrane_value = expected_kind;
            self.expected_kind = __terrane_value;
        }
        {
            let __terrane_value = allow_unknown;
            self.allow_unknown = __terrane_value;
        }
    }
    pub fn from_document(&self, value: DocumentValue) -> DocumentResult {
        return decode_document(value, self.clone());
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
    return destination.from_document(value);
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
        let mut __terrane_constructed_value = Self {
            raw: terrane_make_document_map(),
        };
        __terrane_constructed_value.construct();
        __terrane_constructed_value
    }
    pub fn construct(&mut self) {
        {
            let __terrane_value = terrane_make_document_map();
            self.raw = __terrane_value;
        }
    }
    pub fn append(&mut self, key: String, value: DocumentValue) {
        {
            let __terrane_value = terrane_document_map_insert(
                &self.raw,
                key,
                &value.raw,
            );
            self.raw = __terrane_value;
        }
    }
}
pub fn append_document_map_entry(
    mut entries: DocumentMapEntries,
    key: String,
    value: DocumentValue,
) -> DocumentMapEntries {
    entries.append(key, value);
    return entries;
}
pub fn make_document_list(
    values: terrane_collection_support::List<DocumentValue>,
) -> DocumentResult {
    let mut raw: terrane_document_support::DataResult;
    let mut index: terrane_int_support::Int;
    raw = terrane_make_document_list();
    index = terrane_int_support::Int::from(0_i128);
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
    let fields: terrane_collection_support::List<String>;
    let optional_fields: terrane_collection_support::List<String>;
    let default_fields: terrane_collection_support::List<String>;
    let mut required: terrane_collection_support::List<String>;
    let mut index: terrane_int_support::Int;
    let mut field: String;
    let mut optional: bool;
    let mut optional_index: terrane_int_support::Int;
    let mut defaulted: bool;
    let mut default_index: terrane_int_support::Int;
    fields = mapping.field_names.clone();
    optional_fields = mapping.optional_fields.clone();
    default_fields = mapping.default_fields.clone();
    required = terrane_collection_support::List::<String>::new(vec![]);
    index = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_0 = required.make_unique();
        while index.clone()
            < terrane_int_support::Int::from(
                terrane_int_support::Int::from(fields.length()),
            )
        {
            field = __terrane_raised(
                fields
                    .get_or_error(
                        __terrane_raised(
                            terrane_collection_support::index_from_int(&index.clone()),
                            9 /* terrane-site: core/documents.trn:153:17-153:30 */,
                        ),
                    ),
                9 /* terrane-site: core/documents.trn:153:17-153:30 */,
            );
            optional = false;
            optional_index = terrane_int_support::Int::from(0_i128);
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
                    )
                    .as_str() == field.as_str()
                {
                    optional = true;
                }
                optional_index = optional_index.clone()
                    + terrane_int_support::Int::from(1_i128);
            }
            defaulted = false;
            default_index = terrane_int_support::Int::from(0_i128);
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
                    )
                    .as_str() == field.as_str()
                {
                    defaulted = true;
                }
                default_index = default_index.clone()
                    + terrane_int_support::Int::from(1_i128);
            }
            if field.as_str() != "" && !optional && !defaulted {
                {
                    let __terrane_collection_argument_0 = field;
                    __terrane_list_append_0.push(__terrane_collection_argument_0);
                };
            }
            index = index.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    return required;
}
pub fn decode_document(
    value: DocumentValue,
    mapping: DocumentMapping,
) -> DocumentResult {
    let required: terrane_collection_support::List<String>;
    let mut declared_fields: terrane_collection_support::List<String>;
    let mut field_index: terrane_int_support::Int;
    let mut default_fields: terrane_collection_support::List<String>;
    let mut default_values: terrane_collection_support::List<String>;
    let mut default_index: terrane_int_support::Int;
    let raw: terrane_document_support::DataResult;
    let mut result: DocumentResult;
    required = mapping_required_fields(mapping.clone());
    declared_fields = terrane_collection_support::List::<String>::new(vec![]);
    field_index = terrane_int_support::Int::from(0_i128);
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
                )
                .as_str() != ""
            {
                {
                    let __terrane_collection_argument_0 = __terrane_raised(
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
                    );
                    __terrane_list_append_1.push(__terrane_collection_argument_0);
                };
            }
            field_index = field_index.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    default_fields = terrane_collection_support::List::<String>::new(vec![]);
    default_values = terrane_collection_support::List::<String>::new(vec![]);
    default_index = terrane_int_support::Int::from(0_i128);
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
            )
            .as_str() != ""
        {
            {
                let __terrane_collection_argument_0 = __terrane_raised(
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
                );
                default_fields.append(__terrane_collection_argument_0);
            };
            {
                let __terrane_collection_argument_0 = __terrane_raised(
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
                );
                default_values.append(__terrane_collection_argument_0);
            };
        }
        default_index = default_index.clone() + terrane_int_support::Int::from(1_i128);
    }
    raw = terrane_validate_mapping(
        &value.raw,
        mapping.expected_kind,
        required,
        declared_fields,
        default_fields,
        default_values,
        mapping.allow_unknown,
    );
    result = make_document_result(raw);
    if result.failed {
        {
            let __terrane_value = mapping.descriptor_name.clone();
            result.expected = __terrane_value;
        }
    }
    return result;
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
        let mut __terrane_constructed_value = Self {
            max_depth: terrane_int_support::Int::from(256_i128),
            max_bytes: terrane_int_support::Int::from(16777216_i128),
        };
        __terrane_constructed_value.construct(max_depth, max_bytes);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        max_depth: terrane_int_support::Int,
        max_bytes: terrane_int_support::Int,
    ) {
        {
            let __terrane_value = max_depth.clone();
            self.max_depth = __terrane_value;
        }
        {
            let __terrane_value = max_bytes.clone();
            self.max_bytes = __terrane_value;
        }
    }
}
pub fn default_json_options() -> JsonOptions {
    return JsonOptions::terrane_construct(
        terrane_int_support::Int::from(256_i128),
        terrane_int_support::Int::from(16777216_i128),
    );
}
pub fn parse_json(input: String, options: JsonOptions) -> DocumentResult {
    let raw: terrane_document_support::DataResult;
    raw = terrane_json_parse(
        input,
        options.max_depth.clone(),
        options.max_bytes.clone(),
    );
    return make_document_result(raw);
}
pub fn stringify_json(value: DocumentValue, options: JsonOptions) -> DocumentResult {
    let raw: terrane_document_support::DataResult;
    let _ = &options;
    raw = terrane_json_canonical(&value.raw);
    return make_document_result(raw);
}
pub fn canonical_json(value: DocumentValue) -> DocumentResult {
    let raw: terrane_document_support::DataResult;
    raw = terrane_json_canonical(&value.raw);
    return make_document_result(raw);
}
pub fn decode_json(
    input: String,
    mapping: Deserializable,
    options: JsonOptions,
) -> DocumentResult {
    let parsed: DocumentResult;
    parsed = parse_json(input, options);
    if parsed.failed {
        return parsed.clone();
    }
    return deserialize_document(parsed.value.clone(), mapping);
}
pub fn encode_json(value: Serializable, options: JsonOptions) -> DocumentResult {
    return stringify_json(serialize_document(value), options);
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
        let mut __terrane_constructed_value = Self {
            max_depth: terrane_int_support::Int::from(128_i128),
            max_bytes: terrane_int_support::Int::from(16777216_i128),
            max_alias_nodes: terrane_int_support::Int::from(65536_i128),
        };
        __terrane_constructed_value.construct(max_depth, max_bytes, max_alias_nodes);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        max_depth: terrane_int_support::Int,
        max_bytes: terrane_int_support::Int,
        max_alias_nodes: terrane_int_support::Int,
    ) {
        {
            let __terrane_value = max_depth.clone();
            self.max_depth = __terrane_value;
        }
        {
            let __terrane_value = max_bytes.clone();
            self.max_bytes = __terrane_value;
        }
        {
            let __terrane_value = max_alias_nodes.clone();
            self.max_alias_nodes = __terrane_value;
        }
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
    let raw: terrane_document_support::DataResult;
    raw = terrane_yaml_parse(
        input,
        options.max_depth.clone(),
        options.max_bytes.clone(),
        options.max_alias_nodes.clone(),
    );
    return make_document_result(raw);
}
pub fn stringify_yaml(value: DocumentValue) -> DocumentResult {
    let raw: terrane_document_support::DataResult;
    raw = terrane_json_canonical(&value.raw);
    return make_document_result(raw);
}
pub fn decode_yaml(
    input: String,
    mapping: Deserializable,
    options: YamlOptions,
) -> DocumentResult {
    let parsed: DocumentResult;
    parsed = parse_yaml(input, options);
    if parsed.failed {
        return parsed.clone();
    }
    return deserialize_document(parsed.value.clone(), mapping);
}
pub fn encode_yaml(value: Serializable) -> DocumentResult {
    return stringify_yaml(serialize_document(value));
}
