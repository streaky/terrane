// Generated deterministically by Terrane <version>.
// Runtime support: platform_data_base.rs, platform_documents.rs, platform_json.rs, platform_yaml.rs
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
    pub static FILES: [&str; 1] = ["core/documents.trn"];
    pub static FUNCTIONS: [&str; 3] = [
        "/core/documents::make-document-list",
        "/core/documents::mapping-required-fields",
        "/core/documents::decode-document",
    ];
    pub static SITES: [Site; 9] = [
        /* terrane-site-row: site 0: /core/documents::make-document-list (core/documents.trn:140:47-140:60) */
        { Site { function: 0, file: 0, line: 140, column: 47, end_line: 140, end_column: 60 } },
        /* terrane-site-row: site 1: /core/documents::mapping-required-fields (core/documents.trn:153:17-153:30) */
        { Site { function: 1, file: 0, line: 153, column: 17, end_line: 153, end_column: 30 } },
        /* terrane-site-row: site 2: /core/documents::mapping-required-fields (core/documents.trn:157:16-157:47) */
        { Site { function: 1, file: 0, line: 157, column: 16, end_line: 157, end_column: 47 } },
        /* terrane-site-row: site 3: /core/documents::mapping-required-fields (core/documents.trn:163:16-163:45) */
        { Site { function: 1, file: 0, line: 163, column: 16, end_line: 163, end_column: 45 } },
        /* terrane-site-row: site 4: /core/documents::decode-document (core/documents.trn:176:12-176:44) */
        { Site { function: 2, file: 0, line: 176, column: 12, end_line: 176, end_column: 44 } },
        /* terrane-site-row: site 5: /core/documents::decode-document (core/documents.trn:177:37-177:69) */
        { Site { function: 2, file: 0, line: 177, column: 37, end_line: 177, end_column: 69 } },
        /* terrane-site-row: site 6: /core/documents::decode-document (core/documents.trn:183:12-183:49) */
        { Site { function: 2, file: 0, line: 183, column: 12, end_line: 183, end_column: 49 } },
        /* terrane-site-row: site 7: /core/documents::decode-document (core/documents.trn:184:36-184:73) */
        { Site { function: 2, file: 0, line: 184, column: 36, end_line: 184, end_column: 73 } },
        /* terrane-site-row: site 8: /core/documents::decode-document (core/documents.trn:185:36-185:73) */
        { Site { function: 2, file: 0, line: 185, column: 36, end_line: 185, end_column: 73 } },
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
// Source: case.trn
// Namespace: conformance/document-json-yaml
#[derive(Clone)]
pub struct Note {
    pub text: String,
}
impl Note {
    pub fn terrane_construct(text: String) -> Self {
        let mut __terrane_constructed_value = Self { text: String::from("") };
        __terrane_constructed_value.construct(text);
        __terrane_constructed_value
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
        Note::to_document(&*self)
    }
}
impl From<Note> for Serializable {
    fn from(value: Note) -> Self {
        Self(Box::new(value))
    }
}
fn main() {
    let options_terrane_f0_s1004: JsonOptions;
    let parsed_terrane_f0_s1040: DocumentResult;
    let integer_terrane_f0_s1168: DocumentResult;
    let decimal_terrane_f0_s1206: DocumentResult;
    let canonical_terrane_f0_s1463: DocumentResult;
    let reparsed_terrane_f0_s1543: DocumentResult;
    let equivalent_terrane_f0_s1603: DocumentResult;
    let stringify_input_terrane_f0_s1775: DocumentResult;
    let stringified_terrane_f0_s1837: DocumentResult;
    let yaml_input_terrane_f0_s1902: DocumentResult;
    let yaml_written_terrane_f0_s1959: DocumentResult;
    let list_value_terrane_f0_s2062: DocumentResult;
    let list_document_terrane_f0_s2128: DocumentValue;
    let first_terrane_f0_s2165: DocumentResult;
    let second_terrane_f0_s2199: DocumentResult;
    let nested_terrane_f0_s2234: DocumentResult;
    let negative_item_terrane_f0_s2376: DocumentResult;
    let duplicate_terrane_f0_s2452: DocumentResult;
    let fields_terrane_f0_s2585: terrane_collection_support::List<String>;
    let optional_fields_terrane_f0_s2648: terrane_collection_support::List<String>;
    let default_fields_terrane_f0_s2702: terrane_collection_support::List<String>;
    let default_values_terrane_f0_s2753: terrane_collection_support::List<String>;
    let mut mapping_terrane_f0_s2802: DocumentMapping;
    let mut missing_terrane_f0_s3033: DocumentResult;
    let mut unknown_terrane_f0_s3185: DocumentResult;
    let mut mapped_terrane_f0_s3359: DocumentResult;
    let active_terrane_f0_s3477: DocumentResult;
    let constructor_mapping_terrane_f0_s3565: DocumentMapping;
    let constructor_input_terrane_f0_s3642: DocumentResult;
    let constructor_result_terrane_f0_s3697: DocumentResult;
    let encoded_note_terrane_f0_s3856: DocumentResult;
    let encoded_yaml_note_terrane_f0_s3922: DocumentResult;
    let exact_integer_terrane_f0_s4055: DocumentResult;
    let exact_decimal_terrane_f0_s4131: DocumentResult;
    let values_terrane_f0_s4183: terrane_collection_support::List<DocumentValue>;
    let built_list_terrane_f0_s4289: DocumentResult;
    let mut entries_terrane_f0_s4388: DocumentMapEntries;
    let count_value_terrane_f0_s4526: DocumentResult;
    let built_map_terrane_f0_s4677: DocumentResult;
    let mut duplicate_entries_terrane_f0_s4773: DocumentMapEntries;
    let duplicate_map_terrane_f0_s5049: DocumentResult;
    let canonical_integer_terrane_f0_s5106: DocumentResult;
    let decoded_through_interface_terrane_f0_s5268: DocumentResult;
    let negative_limits_terrane_f0_s5393: JsonOptions;
    let limited_terrane_f0_s5445: DocumentResult;
    let excessive_depth_terrane_f0_s5561: DocumentResult;
    let trailing_json_terrane_f0_s5727: DocumentResult;
    let yaml_limits_terrane_f0_s5876: YamlOptions;
    let yaml_value_terrane_f0_s5926: DocumentResult;
    let yaml_integer_terrane_f0_s6053: DocumentResult;
    let yaml_decimal_terrane_f0_s6106: DocumentResult;
    let yaml_decoded_terrane_f0_s6308: DocumentResult;
    let ordinary_star_terrane_f0_s6430: DocumentResult;
    let bomb_terrane_f0_s6548: DocumentResult;
    let yaml_depth_terrane_f0_s6736: DocumentResult;
    let excessive_yaml_depth_terrane_f0_s6891: DocumentResult;
    options_terrane_f0_s1004 = default_json_options();
    parsed_terrane_f0_s1040 = parse_json(
        String::from("{\"a\":123456789012345678901234567890,\"z\":1.2300}"),
        options_terrane_f0_s1004.clone(),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&parsed_terrane_f0_s1040.failed),
        terrane_scalar_support::scalar_text(&parsed_terrane_f0_s1040.value.kind)
    );
    integer_terrane_f0_s1168 = parsed_terrane_f0_s1040.value.field(String::from("a"));
    decimal_terrane_f0_s1206 = parsed_terrane_f0_s1040.value.field(String::from("z"));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&integer_terrane_f0_s1168.value
        .kind), terrane_scalar_support::scalar_text(&integer_terrane_f0_s1168.value
        .scalar)
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&decimal_terrane_f0_s1206.value
        .kind), terrane_scalar_support::scalar_text(&decimal_terrane_f0_s1206.value
        .scalar)
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&integer_terrane_f0_s1168.value.integer
        .text)
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&decimal_terrane_f0_s1206.value
        .decimal.coefficient),
        terrane_scalar_support::scalar_text(&decimal_terrane_f0_s1206.value.decimal
        .exponent)
    );
    canonical_terrane_f0_s1463 = canonical_json(parsed_terrane_f0_s1040.value);
    println!(
        "{}", terrane_scalar_support::scalar_text(&canonical_terrane_f0_s1463.value
        .encoded)
    );
    reparsed_terrane_f0_s1543 = parse_json(
        canonical_terrane_f0_s1463.value.encoded.clone(),
        options_terrane_f0_s1004.clone(),
    );
    equivalent_terrane_f0_s1603 = parse_json(
        String::from("{\"a\":1.2345678901234567890123456789e+29,\"z\":1.23}"),
        options_terrane_f0_s1004.clone(),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&reparsed_terrane_f0_s1543.failed),
        terrane_scalar_support::scalar_text(&(canonical_terrane_f0_s1463.value.encoded
        .as_str() == equivalent_terrane_f0_s1603.value.encoded.as_str()))
    );
    stringify_input_terrane_f0_s1775 = parse_json(
        String::from("{\"a\":1,\"z\":1.23}"),
        options_terrane_f0_s1004.clone(),
    );
    stringified_terrane_f0_s1837 = stringify_json(
        stringify_input_terrane_f0_s1775.value,
        options_terrane_f0_s1004.clone(),
    );
    yaml_input_terrane_f0_s1902 = parse_json(
        String::from("{\"a\":1,\"z\":1.23}"),
        options_terrane_f0_s1004.clone(),
    );
    yaml_written_terrane_f0_s1959 = stringify_yaml(yaml_input_terrane_f0_s1902.value);
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&stringified_terrane_f0_s1837
        .failed), terrane_scalar_support::scalar_text(&yaml_written_terrane_f0_s1959
        .failed)
    );
    list_value_terrane_f0_s2062 = parse_json(
        String::from("[\"first\",{\"nested\":true}]"),
        options_terrane_f0_s1004.clone(),
    );
    list_document_terrane_f0_s2128 = list_value_terrane_f0_s2062.value;
    first_terrane_f0_s2165 = list_document_terrane_f0_s2128
        .item(terrane_int_support::Int::from(0_i128));
    second_terrane_f0_s2199 = list_document_terrane_f0_s2128
        .item(terrane_int_support::Int::from(1_i128));
    nested_terrane_f0_s2234 = second_terrane_f0_s2199
        .value
        .field(String::from("nested"));
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&list_document_terrane_f0_s2128
        .length()), terrane_scalar_support::scalar_text(&list_document_terrane_f0_s2128
        .key(terrane_int_support::Int::from(0_i128))),
        terrane_scalar_support::scalar_text(&first_terrane_f0_s2165.value.scalar),
        terrane_scalar_support::scalar_text(&nested_terrane_f0_s2234.value.scalar)
    );
    negative_item_terrane_f0_s2376 = list_document_terrane_f0_s2128
        .item(terrane_int_support::Int::from(-1_i128));
    println!(
        "{}", terrane_scalar_support::scalar_text(&negative_item_terrane_f0_s2376.failed)
    );
    duplicate_terrane_f0_s2452 = parse_json(
        String::from("{\"key\":1,\"key\":2}"),
        options_terrane_f0_s1004.clone(),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&duplicate_terrane_f0_s2452.failed),
        terrane_scalar_support::scalar_text(&duplicate_terrane_f0_s2452.message
        .contains(&String::from("duplicate key")))
    );
    fields_terrane_f0_s2585 = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("name"), String::from("nickname"), String::from("active")]);
    optional_fields_terrane_f0_s2648 = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("nickname")]);
    default_fields_terrane_f0_s2702 = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("active")]);
    default_values_terrane_f0_s2753 = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("true")]);
    mapping_terrane_f0_s2802 = DocumentMapping::terrane_construct(
        String::from("person"),
        String::from("map"),
        false,
    );
    mapping_terrane_f0_s2802.field_names = fields_terrane_f0_s2585;
    mapping_terrane_f0_s2802.optional_fields = optional_fields_terrane_f0_s2648;
    mapping_terrane_f0_s2802.default_fields = default_fields_terrane_f0_s2702;
    mapping_terrane_f0_s2802.default_values = default_values_terrane_f0_s2753;
    missing_terrane_f0_s3033 = parse_json(
        String::from("{}"),
        options_terrane_f0_s1004.clone(),
    );
    missing_terrane_f0_s3033 = decode_document(
        missing_terrane_f0_s3033.value,
        mapping_terrane_f0_s2802.clone(),
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&missing_terrane_f0_s3033.failed),
        terrane_scalar_support::scalar_text(&missing_terrane_f0_s3033.path),
        terrane_scalar_support::scalar_text(&missing_terrane_f0_s3033.expected)
    );
    unknown_terrane_f0_s3185 = parse_json(
        String::from("{\"name\":\"Ada\",\"extra\":1}"),
        options_terrane_f0_s1004.clone(),
    );
    unknown_terrane_f0_s3185 = decode_document(
        unknown_terrane_f0_s3185.value,
        mapping_terrane_f0_s2802.clone(),
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&unknown_terrane_f0_s3185.failed),
        terrane_scalar_support::scalar_text(&unknown_terrane_f0_s3185.path),
        terrane_scalar_support::scalar_text(&unknown_terrane_f0_s3185.expected)
    );
    mapped_terrane_f0_s3359 = parse_json(
        String::from("{\"name\":\"Ada\",\"nickname\":\"A\"}"),
        options_terrane_f0_s1004.clone(),
    );
    mapped_terrane_f0_s3359 = decode_document(
        mapped_terrane_f0_s3359.value,
        mapping_terrane_f0_s2802.clone(),
    );
    active_terrane_f0_s3477 = mapped_terrane_f0_s3359
        .value
        .field(String::from("active"));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&mapped_terrane_f0_s3359.failed),
        terrane_scalar_support::scalar_text(&active_terrane_f0_s3477.value.scalar)
    );
    constructor_mapping_terrane_f0_s3565 = DocumentMapping::terrane_construct(
        String::from("open-map"),
        String::from("map"),
        true,
    );
    constructor_input_terrane_f0_s3642 = parse_json(
        String::from("{\"a\":1}"),
        options_terrane_f0_s1004.clone(),
    );
    constructor_result_terrane_f0_s3697 = decode_document(
        constructor_input_terrane_f0_s3642.value,
        constructor_mapping_terrane_f0_s3565,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&constructor_result_terrane_f0_s3697
        .failed),
        terrane_scalar_support::scalar_text(&constructor_result_terrane_f0_s3697.value
        .encoded)
    );
    encoded_note_terrane_f0_s3856 = encode_json(
        <Serializable>::from(Note::terrane_construct(String::from("hello"))),
        options_terrane_f0_s1004.clone(),
    );
    encoded_yaml_note_terrane_f0_s3922 = encode_yaml(
        <Serializable>::from(Note::terrane_construct(String::from("hello"))),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&encoded_note_terrane_f0_s3856.value
        .encoded),
        terrane_scalar_support::scalar_text(&encoded_yaml_note_terrane_f0_s3922.value
        .encoded)
    );
    exact_integer_terrane_f0_s4055 = make_document_integer(
        String::from("123456789012345678901234567890"),
    );
    exact_decimal_terrane_f0_s4131 = make_document_decimal(String::from("1.2300"));
    values_terrane_f0_s4183 = terrane_collection_support::List::<
        DocumentValue,
    >::new(
        vec![
            exact_integer_terrane_f0_s4055.value, exact_decimal_terrane_f0_s4131.value,
            make_document_none()
        ],
    );
    built_list_terrane_f0_s4289 = make_document_list(values_terrane_f0_s4183);
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&built_list_terrane_f0_s4289.failed),
        terrane_scalar_support::scalar_text(&built_list_terrane_f0_s4289.value.encoded)
    );
    entries_terrane_f0_s4388 = DocumentMapEntries::terrane_construct();
    entries_terrane_f0_s4388 = append_document_map_entry(
        entries_terrane_f0_s4388,
        String::from("message"),
        make_document_string(String::from("hello")),
    );
    count_value_terrane_f0_s4526 = make_document_integer(
        String::from("123456789012345678901234567890"),
    );
    entries_terrane_f0_s4388 = append_document_map_entry(
        entries_terrane_f0_s4388,
        String::from("count"),
        count_value_terrane_f0_s4526.value,
    );
    built_map_terrane_f0_s4677 = make_document_map(entries_terrane_f0_s4388);
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&built_map_terrane_f0_s4677.failed),
        terrane_scalar_support::scalar_text(&built_map_terrane_f0_s4677.value.encoded)
    );
    duplicate_entries_terrane_f0_s4773 = DocumentMapEntries::terrane_construct();
    duplicate_entries_terrane_f0_s4773 = append_document_map_entry(
        duplicate_entries_terrane_f0_s4773,
        String::from("same"),
        make_document_string(String::from("first")),
    );
    duplicate_entries_terrane_f0_s4773 = append_document_map_entry(
        duplicate_entries_terrane_f0_s4773,
        String::from("same"),
        make_document_string(String::from("second")),
    );
    duplicate_map_terrane_f0_s5049 = make_document_map(
        duplicate_entries_terrane_f0_s4773,
    );
    canonical_integer_terrane_f0_s5106 = make_document_integer(
        String::from("1.2345678901234567890123456789e+29"),
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&duplicate_map_terrane_f0_s5049
        .failed), terrane_scalar_support::scalar_text(&duplicate_map_terrane_f0_s5049
        .path), terrane_scalar_support::scalar_text(&canonical_integer_terrane_f0_s5106
        .failed)
    );
    decoded_through_interface_terrane_f0_s5268 = decode_json(
        String::from("{\"name\":\"Ada\"}"),
        <Deserializable>::from(mapping_terrane_f0_s2802.clone()),
        options_terrane_f0_s1004.clone(),
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&decoded_through_interface_terrane_f0_s5268
        .failed)
    );
    negative_limits_terrane_f0_s5393 = JsonOptions::terrane_construct(
        terrane_int_support::Int::from(-1_i128),
        terrane_int_support::Int::from(-1_i128),
    );
    limited_terrane_f0_s5445 = parse_json(
        String::from("{}"),
        negative_limits_terrane_f0_s5393,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&limited_terrane_f0_s5445.failed),
        terrane_scalar_support::scalar_text(&limited_terrane_f0_s5445.message
        .contains(&String::from("byte limit")))
    );
    excessive_depth_terrane_f0_s5561 = parse_json(
        String::from("[]"),
        JsonOptions::terrane_construct(
            terrane_int_support::Int::from(1000000_i128),
            terrane_int_support::Int::from(1024_i128),
        ),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&excessive_depth_terrane_f0_s5561
        .failed), terrane_scalar_support::scalar_text(&excessive_depth_terrane_f0_s5561
        .message.contains(&String::from("cannot exceed")))
    );
    trailing_json_terrane_f0_s5727 = parse_json(
        String::from("{\"a\":1} garbage"),
        options_terrane_f0_s1004,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&trailing_json_terrane_f0_s5727
        .failed), terrane_scalar_support::scalar_text(&trailing_json_terrane_f0_s5727
        .message.contains(&String::from("trailing characters")))
    );
    yaml_limits_terrane_f0_s5876 = make_yaml_options(
        terrane_int_support::Int::from(32_i128),
        terrane_int_support::Int::from(2048_i128),
        terrane_int_support::Int::from(20_i128),
    );
    yaml_value_terrane_f0_s5926 = parse_yaml(
        String::from(
            "integer: 123456789012345678901234567890\ndecimal: 3.141592653589793238462643383279",
        ),
        yaml_limits_terrane_f0_s5876.clone(),
    );
    yaml_integer_terrane_f0_s6053 = yaml_value_terrane_f0_s5926
        .value
        .field(String::from("integer"));
    yaml_decimal_terrane_f0_s6106 = yaml_value_terrane_f0_s5926
        .value
        .field(String::from("decimal"));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&yaml_value_terrane_f0_s5926.failed),
        terrane_scalar_support::scalar_text(&yaml_integer_terrane_f0_s6053.value.integer
        .text)
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&yaml_decimal_terrane_f0_s6106.value
        .decimal.coefficient),
        terrane_scalar_support::scalar_text(&yaml_decimal_terrane_f0_s6106.value.decimal
        .exponent)
    );
    yaml_decoded_terrane_f0_s6308 = decode_yaml(
        String::from("name: Ada"),
        <Deserializable>::from(mapping_terrane_f0_s2802),
        make_yaml_options(
            terrane_int_support::Int::from(32_i128),
            terrane_int_support::Int::from(1024_i128),
            terrane_int_support::Int::from(65536_i128),
        ),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&yaml_decoded_terrane_f0_s6308.failed)
    );
    ordinary_star_terrane_f0_s6430 = parse_yaml(
        String::from("glob: \"a * b * c\""),
        make_yaml_options(
            terrane_int_support::Int::from(32_i128),
            terrane_int_support::Int::from(1024_i128),
            terrane_int_support::Int::from(0_i128),
        ),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&ordinary_star_terrane_f0_s6430.failed)
    );
    bomb_terrane_f0_s6548 = parse_yaml(
        String::from(
            "leaf: &leaf [1, 2, 3, 4]\na: &a [*leaf, *leaf, *leaf, *leaf]\nb: [*a, *a, *a, *a]",
        ),
        yaml_limits_terrane_f0_s5876,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&bomb_terrane_f0_s6548.failed),
        terrane_scalar_support::scalar_text(&bomb_terrane_f0_s6548.message
        .contains(&String::from("alias node limit")))
    );
    yaml_depth_terrane_f0_s6736 = parse_yaml(
        String::from("a: [[[[]]]]"),
        make_yaml_options(
            terrane_int_support::Int::from(2_i128),
            terrane_int_support::Int::from(1024_i128),
            terrane_int_support::Int::from(65536_i128),
        ),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&yaml_depth_terrane_f0_s6736.failed),
        terrane_scalar_support::scalar_text(&yaml_depth_terrane_f0_s6736.message
        .contains(&String::from("depth limit")))
    );
    excessive_yaml_depth_terrane_f0_s6891 = parse_yaml(
        String::from("[]"),
        make_yaml_options(
            terrane_int_support::Int::from(256_i128),
            terrane_int_support::Int::from(1024_i128),
            terrane_int_support::Int::from(65536_i128),
        ),
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&excessive_yaml_depth_terrane_f0_s6891
        .failed),
        terrane_scalar_support::scalar_text(&excessive_yaml_depth_terrane_f0_s6891
        .message.contains(&String::from("cannot exceed 255")))
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
        let mut __terrane_constructed_value = Self { text: String::from("0") };
        __terrane_constructed_value.construct(text);
        __terrane_constructed_value
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
        self.coefficient = coefficient;
        self.exponent = exponent.clone();
        self.text = text;
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
        self.kind = terrane_document_kind(&raw);
        self.scalar = terrane_document_text(&raw);
        self.encoded = terrane_data_encoded(&raw);
        if self.kind.as_str() == "integer" {
            self.integer = DocumentInteger::terrane_construct(self.scalar.clone());
        }
        if self.kind.as_str() == "decimal" {
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
        let raw_terrane_f1_s1690: terrane_document_support::DataResult;
        raw_terrane_f1_s1690 = terrane_document_item(&self.raw, index.clone());
        return make_document_result(raw_terrane_f1_s1690);
    }
    pub fn key(&self, index: terrane_int_support::Int) -> String {
        return terrane_document_key(&self.raw, index.clone());
    }
    pub fn field(&self, name: String) -> DocumentResult {
        let raw_terrane_f1_s1916: terrane_document_support::DataResult;
        raw_terrane_f1_s1916 = terrane_document_field(&self.raw, name);
        return make_document_result(raw_terrane_f1_s1916);
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
        self.descriptor_name = descriptor_name;
        self.expected_kind = expected_kind;
        self.allow_unknown = allow_unknown;
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
    entries.append(key, value);
    return entries;
}
pub fn make_document_list(
    values: terrane_collection_support::List<DocumentValue>,
) -> DocumentResult {
    let mut raw_terrane_f1_s4793: terrane_document_support::DataResult;
    let mut index_terrane_f1_s4828: terrane_int_support::Int;
    raw_terrane_f1_s4793 = terrane_make_document_list();
    index_terrane_f1_s4828 = terrane_int_support::Int::from(0_i128);
    while index_terrane_f1_s4828.clone()
        < terrane_int_support::Int::from(terrane_int_support::Int::from(values.length()))
    {
        raw_terrane_f1_s4793 = terrane_document_list_append(
            &raw_terrane_f1_s4793,
            &__terrane_raised(
                    values
                        .get_or_error(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(
                                    &index_terrane_f1_s4828.clone(),
                                ),
                                0 /* terrane-site: core/documents.trn:140:47-140:60 */,
                            ),
                        ),
                    0 /* terrane-site: core/documents.trn:140:47-140:60 */,
                )
                .raw,
        );
        index_terrane_f1_s4828 = index_terrane_f1_s4828.clone()
            + terrane_int_support::Int::from(1_i128);
    }
    return make_document_result(raw_terrane_f1_s4793);
}
pub fn make_document_map(entries: DocumentMapEntries) -> DocumentResult {
    return make_document_result(entries.raw);
}
pub fn mapping_required_fields(
    mapping: DocumentMapping,
) -> terrane_collection_support::List<String> {
    let fields_terrane_f1_s5198: terrane_collection_support::List<String>;
    let optional_fields_terrane_f1_s5231: terrane_collection_support::List<String>;
    let default_fields_terrane_f1_s5277: terrane_collection_support::List<String>;
    let mut required_terrane_f1_s5321: terrane_collection_support::List<String>;
    let mut index_terrane_f1_s5357: terrane_int_support::Int;
    let mut field_terrane_f1_s5411: String;
    let mut optional_terrane_f1_s5441: bool;
    let mut optional_index_terrane_f1_s5471: terrane_int_support::Int;
    let mut defaulted_terrane_f1_s5692: bool;
    let mut default_index_terrane_f1_s5723: terrane_int_support::Int;
    fields_terrane_f1_s5198 = mapping.field_names.clone();
    optional_fields_terrane_f1_s5231 = mapping.optional_fields.clone();
    default_fields_terrane_f1_s5277 = mapping.default_fields.clone();
    required_terrane_f1_s5321 = terrane_collection_support::List::<String>::new(vec![]);
    index_terrane_f1_s5357 = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_0 = required_terrane_f1_s5321.make_unique();
        while index_terrane_f1_s5357.clone()
            < terrane_int_support::Int::from(
                terrane_int_support::Int::from(fields_terrane_f1_s5198.length()),
            )
        {
            field_terrane_f1_s5411 = __terrane_raised(
                fields_terrane_f1_s5198
                    .get_or_error(
                        __terrane_raised(
                            terrane_collection_support::index_from_int(
                                &index_terrane_f1_s5357.clone(),
                            ),
                            1 /* terrane-site: core/documents.trn:153:17-153:30 */,
                        ),
                    ),
                1 /* terrane-site: core/documents.trn:153:17-153:30 */,
            );
            optional_terrane_f1_s5441 = false;
            optional_index_terrane_f1_s5471 = terrane_int_support::Int::from(0_i128);
            while optional_index_terrane_f1_s5471.clone()
                < terrane_int_support::Int::from(
                    terrane_int_support::Int::from(
                        optional_fields_terrane_f1_s5231.length(),
                    ),
                )
            {
                if __terrane_raised(
                        optional_fields_terrane_f1_s5231
                            .get_or_error(
                                __terrane_raised(
                                    terrane_collection_support::index_from_int(
                                        &optional_index_terrane_f1_s5471.clone(),
                                    ),
                                    2 /* terrane-site: core/documents.trn:157:16-157:47 */,
                                ),
                            ),
                        2 /* terrane-site: core/documents.trn:157:16-157:47 */,
                    )
                    .as_str() == field_terrane_f1_s5411.as_str()
                {
                    optional_terrane_f1_s5441 = true;
                }
                optional_index_terrane_f1_s5471 = optional_index_terrane_f1_s5471.clone()
                    + terrane_int_support::Int::from(1_i128);
            }
            defaulted_terrane_f1_s5692 = false;
            default_index_terrane_f1_s5723 = terrane_int_support::Int::from(0_i128);
            while default_index_terrane_f1_s5723.clone()
                < terrane_int_support::Int::from(
                    terrane_int_support::Int::from(
                        default_fields_terrane_f1_s5277.length(),
                    ),
                )
            {
                if __terrane_raised(
                        default_fields_terrane_f1_s5277
                            .get_or_error(
                                __terrane_raised(
                                    terrane_collection_support::index_from_int(
                                        &default_index_terrane_f1_s5723.clone(),
                                    ),
                                    3 /* terrane-site: core/documents.trn:163:16-163:45 */,
                                ),
                            ),
                        3 /* terrane-site: core/documents.trn:163:16-163:45 */,
                    )
                    .as_str() == field_terrane_f1_s5411.as_str()
                {
                    defaulted_terrane_f1_s5692 = true;
                }
                default_index_terrane_f1_s5723 = default_index_terrane_f1_s5723.clone()
                    + terrane_int_support::Int::from(1_i128);
            }
            if field_terrane_f1_s5411.as_str() != "" && !optional_terrane_f1_s5441
                && !defaulted_terrane_f1_s5692
            {
                __terrane_list_append_0.push(field_terrane_f1_s5411);
            }
            index_terrane_f1_s5357 = index_terrane_f1_s5357.clone()
                + terrane_int_support::Int::from(1_i128);
        }
    }
    return required_terrane_f1_s5321;
}
pub fn decode_document(
    value: DocumentValue,
    mapping: DocumentMapping,
) -> DocumentResult {
    let required_terrane_f1_s6163: terrane_collection_support::List<String>;
    let mut declared_fields_terrane_f1_s6211: terrane_collection_support::List<String>;
    let mut field_index_terrane_f1_s6254: terrane_int_support::Int;
    let mut default_fields_terrane_f1_s6486: terrane_collection_support::List<String>;
    let mut default_values_terrane_f1_s6528: terrane_collection_support::List<String>;
    let mut default_index_terrane_f1_s6570: terrane_int_support::Int;
    let raw_terrane_f1_s6945: terrane_document_support::DataResult;
    let mut result_terrane_f1_s7093: DocumentResult;
    required_terrane_f1_s6163 = mapping_required_fields(mapping.clone());
    declared_fields_terrane_f1_s6211 = terrane_collection_support::List::<
        String,
    >::new(vec![]);
    field_index_terrane_f1_s6254 = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_1 = declared_fields_terrane_f1_s6211.make_unique();
        while field_index_terrane_f1_s6254.clone()
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
                                    &field_index_terrane_f1_s6254.clone(),
                                ),
                                4 /* terrane-site: core/documents.trn:176:12-176:44 */,
                            ),
                        ),
                    4 /* terrane-site: core/documents.trn:176:12-176:44 */,
                )
                .as_str() != ""
            {
                __terrane_list_append_1
                    .push(
                        __terrane_raised(
                            mapping
                                .field_names
                                .get_or_error(
                                    __terrane_raised(
                                        terrane_collection_support::index_from_int(
                                            &field_index_terrane_f1_s6254.clone(),
                                        ),
                                        5 /* terrane-site: core/documents.trn:177:37-177:69 */,
                                    ),
                                ),
                            5 /* terrane-site: core/documents.trn:177:37-177:69 */,
                        ),
                    );
            }
            field_index_terrane_f1_s6254 = field_index_terrane_f1_s6254.clone()
                + terrane_int_support::Int::from(1_i128);
        }
    }
    default_fields_terrane_f1_s6486 = terrane_collection_support::List::<
        String,
    >::new(vec![]);
    default_values_terrane_f1_s6528 = terrane_collection_support::List::<
        String,
    >::new(vec![]);
    default_index_terrane_f1_s6570 = terrane_int_support::Int::from(0_i128);
    while default_index_terrane_f1_s6570.clone()
        < terrane_int_support::Int::from(
            terrane_int_support::Int::from(mapping.default_fields.length()),
        )
        && default_index_terrane_f1_s6570.clone()
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
                                &default_index_terrane_f1_s6570.clone(),
                            ),
                            6 /* terrane-site: core/documents.trn:183:12-183:49 */,
                        ),
                    ),
                6 /* terrane-site: core/documents.trn:183:12-183:49 */,
            )
            .as_str() != ""
        {
            default_fields_terrane_f1_s6486
                .append(
                    __terrane_raised(
                        mapping
                            .default_fields
                            .get_or_error(
                                __terrane_raised(
                                    terrane_collection_support::index_from_int(
                                        &default_index_terrane_f1_s6570.clone(),
                                    ),
                                    7 /* terrane-site: core/documents.trn:184:36-184:73 */,
                                ),
                            ),
                        7 /* terrane-site: core/documents.trn:184:36-184:73 */,
                    ),
                );
            default_values_terrane_f1_s6528
                .append(
                    __terrane_raised(
                        mapping
                            .default_values
                            .get_or_error(
                                __terrane_raised(
                                    terrane_collection_support::index_from_int(
                                        &default_index_terrane_f1_s6570.clone(),
                                    ),
                                    8 /* terrane-site: core/documents.trn:185:36-185:73 */,
                                ),
                            ),
                        8 /* terrane-site: core/documents.trn:185:36-185:73 */,
                    ),
                );
        }
        default_index_terrane_f1_s6570 = default_index_terrane_f1_s6570.clone()
            + terrane_int_support::Int::from(1_i128);
    }
    raw_terrane_f1_s6945 = terrane_validate_mapping(
        &value.raw,
        mapping.expected_kind,
        required_terrane_f1_s6163,
        declared_fields_terrane_f1_s6211,
        default_fields_terrane_f1_s6486,
        default_values_terrane_f1_s6528,
        mapping.allow_unknown,
    );
    result_terrane_f1_s7093 = make_document_result(raw_terrane_f1_s6945);
    if result_terrane_f1_s7093.failed {
        result_terrane_f1_s7093.expected = mapping.descriptor_name.clone();
    }
    return result_terrane_f1_s7093;
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
    let raw_terrane_f2_s641: terrane_document_support::DataResult;
    raw_terrane_f2_s641 = terrane_json_parse(
        input,
        options.max_depth.clone(),
        options.max_bytes.clone(),
    );
    return make_document_result(raw_terrane_f2_s641);
}
pub fn stringify_json(value: DocumentValue, options: JsonOptions) -> DocumentResult {
    let raw_terrane_f2_s834: terrane_document_support::DataResult;
    let _ = &options;
    raw_terrane_f2_s834 = terrane_json_canonical(&value.raw);
    return make_document_result(raw_terrane_f2_s834);
}
pub fn canonical_json(value: DocumentValue) -> DocumentResult {
    let raw_terrane_f2_s975: terrane_document_support::DataResult;
    raw_terrane_f2_s975 = terrane_json_canonical(&value.raw);
    return make_document_result(raw_terrane_f2_s975);
}
pub fn decode_json(
    input: String,
    mapping: Deserializable,
    options: JsonOptions,
) -> DocumentResult {
    let parsed_terrane_f2_s1151: DocumentResult;
    parsed_terrane_f2_s1151 = parse_json(input, options);
    if parsed_terrane_f2_s1151.failed {
        return parsed_terrane_f2_s1151.clone();
    }
    return deserialize_document(parsed_terrane_f2_s1151.value, mapping);
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
    let raw_terrane_f3_s912: terrane_document_support::DataResult;
    raw_terrane_f3_s912 = terrane_yaml_parse(
        input,
        options.max_depth.clone(),
        options.max_bytes.clone(),
        options.max_alias_nodes.clone(),
    );
    return make_document_result(raw_terrane_f3_s912);
}
pub fn stringify_yaml(value: DocumentValue) -> DocumentResult {
    let raw_terrane_f3_s1196: terrane_document_support::DataResult;
    raw_terrane_f3_s1196 = terrane_json_canonical(&value.raw);
    return make_document_result(raw_terrane_f3_s1196);
}
pub fn decode_yaml(
    input: String,
    mapping: Deserializable,
    options: YamlOptions,
) -> DocumentResult {
    let parsed_terrane_f3_s1372: DocumentResult;
    parsed_terrane_f3_s1372 = parse_yaml(input, options);
    if parsed_terrane_f3_s1372.failed {
        return parsed_terrane_f3_s1372.clone();
    }
    return deserialize_document(parsed_terrane_f3_s1372.value, mapping);
}
pub fn encode_yaml(value: Serializable) -> DocumentResult {
    return stringify_yaml(serialize_document(value));
}
