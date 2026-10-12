// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support, terrane-platform-support
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
    pub static FILES: [&str; 1] = ["case.trn"];
    pub static FUNCTIONS: [&str; 1] = ["/app::main"];
    pub static SITES: [Site; 6] = [
        /* terrane-site-row: site 0: /app::main (case.trn:51:13-51:38) */
        { Site { function: 0, file: 0, line: 51, column: 13, end_line: 51, end_column: 38 } },
        /* terrane-site-row: site 1: /app::main (case.trn:55:13-55:40) */
        { Site { function: 0, file: 0, line: 55, column: 13, end_line: 55, end_column: 40 } },
        /* terrane-site-row: site 2: /app::main (case.trn:64:13-64:40) */
        { Site { function: 0, file: 0, line: 64, column: 13, end_line: 64, end_column: 40 } },
        /* terrane-site-row: site 3: /app::main (case.trn:68:13-68:45) */
        { Site { function: 0, file: 0, line: 68, column: 13, end_line: 68, end_column: 45 } },
        /* terrane-site-row: site 4: /app::main (case.trn:72:13-72:44) */
        { Site { function: 0, file: 0, line: 72, column: 13, end_line: 72, end_column: 44 } },
        /* terrane-site-row: site 5: /app::main (case.trn:77:13-77:45) */
        { Site { function: 0, file: 0, line: 77, column: 13, end_line: 77, end_column: 45 } },
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
pub type TerranePlatformCapability = terrane_platform_support::Capability;
pub type TerranePlatformResult = terrane_platform_support::ResultValue;
pub fn terrane_platform_i128(
    value: &terrane_int_support::Int,
    label: &str,
) -> Result<i128, TerranePlatformResult> {
    terrane_int_support::coerce::<i128>(value)
        .map_err(|_| TerranePlatformResult::error(
            format!("{label} is outside the signed 128-bit platform range"),
        ))
}
pub fn terrane_platform_timeout_nanos(value: terrane_int_support::Int) -> i128 {
    if value < terrane_int_support::Int::from(0_i128) {
        return -1;
    }
    i128::from(terrane_int_support::coerce::<u64>(&value).unwrap_or(u64::MAX))
}
macro_rules! terrane_platform_i128 {
    ($value:expr, $label:literal) => {
        match terrane_platform_i128(&$value, $label) { Ok(value) => value, Err(error) =>
        return error, }
    };
}
#[allow(dead_code)]
fn terrane_platform_cancellation_token() -> TerranePlatformCapability {
    terrane_platform_support::cancellation_token()
}
#[allow(dead_code)]
fn terrane_platform_no_resource() -> TerranePlatformCapability {
    TerranePlatformCapability::default()
}
#[allow(dead_code)]
fn terrane_platform_failed_result() -> TerranePlatformResult {
    TerranePlatformResult::error("uninitialized platform value")
}
#[allow(dead_code)]
fn terrane_platform_cancel(token: &TerranePlatformCapability) -> TerranePlatformResult {
    terrane_platform_support::cancel(token)
}
#[allow(dead_code)]
fn terrane_platform_result_failed(result: &TerranePlatformResult) -> bool {
    result.failed
}
#[allow(dead_code)]
fn terrane_platform_result_resource_limit(result: &TerranePlatformResult) -> bool {
    result.resource_limit
}
#[allow(dead_code)]
fn terrane_platform_result_truncated(result: &TerranePlatformResult) -> bool {
    result.truncated
}
#[allow(dead_code)]
fn terrane_platform_result_deadline_exceeded(result: &TerranePlatformResult) -> bool {
    result.deadline_exceeded
}
#[allow(dead_code)]
fn terrane_platform_result_message(result: &TerranePlatformResult) -> String {
    result.message.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_text(result: &TerranePlatformResult) -> String {
    result.text.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_detail(result: &TerranePlatformResult) -> String {
    result.detail.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_bytes(result: &TerranePlatformResult) -> Vec<u8> {
    result.data.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_int(
    result: &TerranePlatformResult,
) -> terrane_int_support::Int {
    terrane_int_support::Int::from(result.number)
}
#[allow(dead_code)]
fn terrane_platform_result_bool(result: &TerranePlatformResult) -> bool {
    result.flag
}
#[allow(dead_code)]
fn terrane_platform_result_entries(result: &TerranePlatformResult) -> Vec<String> {
    result.entries.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_capability(
    result: &TerranePlatformResult,
) -> TerranePlatformCapability {
    result.capability.clone().unwrap_or_default()
}
pub fn terrane_platform_secure_random() -> TerranePlatformCapability {
    terrane_platform_support::secure_random()
}
pub fn terrane_platform_pseudo_random(
    algorithm: String,
    seed: Vec<u8>,
) -> TerranePlatformCapability {
    terrane_platform_support::pseudo_random(&algorithm, &seed)
}
pub fn terrane_platform_secret_buffer(data: Vec<u8>) -> TerranePlatformCapability {
    terrane_platform_support::secret_buffer(data)
}
pub fn terrane_platform_destroy_secret(
    secret: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::destroy_secret(secret)
}
pub fn terrane_platform_random_bytes(
    source: &TerranePlatformCapability,
    count: terrane_int_support::Int,
) -> TerranePlatformResult {
    terrane_platform_support::random_bytes(
        source,
        terrane_platform_i128!(count, "random byte count"),
    )
}
pub fn terrane_platform_random_bounded(
    source: &TerranePlatformCapability,
    upper: terrane_int_support::Int,
) -> TerranePlatformResult {
    terrane_platform_support::random_bounded(
        source,
        terrane_platform_i128!(upper, "random upper bound"),
    )
}
pub fn terrane_platform_random_split(
    source: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::random_split(source)
}
pub fn terrane_platform_digest(
    algorithm: &String,
    data: Vec<u8>,
) -> TerranePlatformResult {
    terrane_platform_support::digest(algorithm, &data)
}
pub fn terrane_platform_hmac(
    algorithm: &String,
    key: &TerranePlatformCapability,
    data: Vec<u8>,
) -> TerranePlatformResult {
    terrane_platform_support::hmac(algorithm, key, &data)
}
pub fn terrane_platform_constant_time_equal(left: Vec<u8>, right: Vec<u8>) -> bool {
    terrane_platform_support::constant_time_equal(&left, &right)
}
pub fn terrane_platform_hex_encode(data: Vec<u8>) -> String {
    terrane_platform_support::hex_encode(&data)
}
pub fn terrane_platform_hex_decode(text: String) -> TerranePlatformResult {
    terrane_platform_support::hex_decode(&text)
}
pub fn terrane_platform_base64_encode(
    data: Vec<u8>,
    url_safe: bool,
    padded: bool,
) -> String {
    terrane_platform_support::base64_encode(&data, url_safe, padded)
}
pub fn terrane_platform_base64_decode(
    text: String,
    url_safe: bool,
    padded: bool,
) -> TerranePlatformResult {
    terrane_platform_support::base64_decode(&text, url_safe, padded)
}
pub fn terrane_platform_compress(
    format: String,
    data: Vec<u8>,
    level: terrane_int_support::Int,
    deterministic: bool,
) -> TerranePlatformResult {
    terrane_platform_support::compress(
        &format,
        &data,
        terrane_platform_i128!(level, "compression level"),
        deterministic,
    )
}
pub fn terrane_platform_decompress(
    format: String,
    data: Vec<u8>,
    output: terrane_int_support::Int,
    ratio: terrane_int_support::Int,
    work: terrane_int_support::Int,
) -> TerranePlatformResult {
    terrane_platform_support::decompress(
        &format,
        &data,
        terrane_platform_i128!(output, "decompression output limit"),
        terrane_platform_i128!(ratio, "decompression ratio limit"),
        terrane_platform_i128!(work, "decompression work limit"),
    )
}
pub fn terrane_platform_uuid_parse(text: String) -> TerranePlatformResult {
    terrane_platform_support::uuid_parse(&text)
}
pub fn terrane_platform_uuid_v4(
    source: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::uuid_v4(source)
}
pub fn terrane_platform_uuid_v7(
    source: &TerranePlatformCapability,
    unix_milliseconds: terrane_int_support::Int,
) -> TerranePlatformResult {
    terrane_platform_support::uuid_v7(
        source,
        terrane_platform_i128!(unix_milliseconds, "UUID v7 timestamp"),
    )
}
// Source: core/codecs.trn
// Namespace: core/codecs
#[derive(Clone)]
pub struct DecodeResult {
    pub failed: bool,
    pub message: String,
    pub value: Vec<u8>,
}
impl DecodeResult {
    pub fn terrane_construct(failed: bool, message: String, data: Vec<u8>) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            value: Vec::from([]),
        };
        __terrane_constructed_value.construct(failed, message, data);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, failed: bool, message: String, data: Vec<u8>) {
        self.failed = failed;
        self.message = message;
        self.value = data;
    }
}
#[derive(Clone)]
pub struct HexCodec {}
impl HexCodec {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn encode(&self, data: Vec<u8>) -> String {
        return terrane_platform_hex_encode(data);
    }
    pub fn decode(&self, text: String) -> DecodeResult {
        let raw: TerranePlatformResult;
        raw = terrane_platform_hex_decode(text);
        return DecodeResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_bytes(&raw),
        );
    }
}
#[derive(Clone)]
pub struct Base64Codec {
    pub url_safe: bool,
}
impl Base64Codec {
    pub fn terrane_construct(url_safe: bool) -> Self {
        let mut __terrane_constructed_value = Self { url_safe: false };
        __terrane_constructed_value.construct(url_safe);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, url_safe: bool) {
        self.url_safe = url_safe;
    }
    pub fn encode(&self, data: Vec<u8>, padded: bool) -> String {
        return terrane_platform_base64_encode(data, self.url_safe, padded);
    }
    pub fn decode(&self, text: String, padded: bool) -> DecodeResult {
        let raw: TerranePlatformResult;
        raw = terrane_platform_base64_decode(text, self.url_safe, padded);
        return DecodeResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_bytes(&raw),
        );
    }
}
pub fn hex() -> HexCodec {
    return HexCodec::terrane_construct();
}
pub fn base64() -> Base64Codec {
    return Base64Codec::terrane_construct(false);
}
pub fn base64_url() -> Base64Codec {
    return Base64Codec::terrane_construct(true);
}
pub fn bytes_from_octets(octets: terrane_collection_support::List<u8>) -> Vec<u8> {
    return octets.into_vec();
}
pub fn encode_hex(data: Vec<u8>) -> String {
    return terrane_platform_hex_encode(data);
}
pub fn decode_hex(text: String) -> DecodeResult {
    let raw: TerranePlatformResult;
    raw = terrane_platform_hex_decode(text);
    return DecodeResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        terrane_platform_result_bytes(&raw),
    );
}
pub fn encode_base64(data: Vec<u8>, url_safe: bool, padded: bool) -> String {
    return terrane_platform_base64_encode(data, url_safe, padded);
}
pub fn decode_base64(text: String, url_safe: bool, padded: bool) -> DecodeResult {
    let raw: TerranePlatformResult;
    raw = terrane_platform_base64_decode(text, url_safe, padded);
    return DecodeResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        terrane_platform_result_bytes(&raw),
    );
}
// Source: core/compression.trn
// Namespace: core/compression
#[derive(Clone)]
pub struct CompressionOptions {
    pub level: terrane_int_support::Int,
    pub deterministic: bool,
}
impl CompressionOptions {
    pub fn terrane_construct(
        level: terrane_int_support::Int,
        deterministic: bool,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            level: terrane_int_support::Int::from(6_i128),
            deterministic: true,
        };
        __terrane_constructed_value.construct(level, deterministic);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, level: terrane_int_support::Int, deterministic: bool) {
        self.level = level.clone();
        self.deterministic = deterministic;
    }
}
#[derive(Clone)]
pub struct DecompressionLimits {
    pub max_output: terrane_int_support::Int,
    pub max_ratio: terrane_int_support::Int,
    pub max_work: terrane_int_support::Int,
}
impl DecompressionLimits {
    pub fn terrane_construct(
        max_output: terrane_int_support::Int,
        max_ratio: terrane_int_support::Int,
        max_work: terrane_int_support::Int,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            max_output: terrane_int_support::Int::from(16777216_i128),
            max_ratio: terrane_int_support::Int::from(100_i128),
            max_work: terrane_int_support::Int::from(67108864_i128),
        };
        __terrane_constructed_value.construct(max_output, max_ratio, max_work);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        max_output: terrane_int_support::Int,
        max_ratio: terrane_int_support::Int,
        max_work: terrane_int_support::Int,
    ) {
        self.max_output = max_output.clone();
        self.max_ratio = max_ratio.clone();
        self.max_work = max_work.clone();
    }
}
#[derive(Clone)]
pub struct CompressionResult {
    pub failed: bool,
    pub resource_limit: bool,
    pub message: String,
    pub value: Vec<u8>,
}
impl CompressionResult {
    pub fn terrane_construct(
        failed: bool,
        resource_limit: bool,
        message: String,
        data: Vec<u8>,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            resource_limit: false,
            message: String::from(""),
            value: Vec::from([]),
        };
        __terrane_constructed_value.construct(failed, resource_limit, message, data);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        resource_limit: bool,
        message: String,
        data: Vec<u8>,
    ) {
        self.failed = failed;
        self.resource_limit = resource_limit;
        self.message = message;
        self.value = data;
    }
}
#[derive(Clone)]
pub struct CompressionCodec {
    pub format: String,
}
impl CompressionCodec {
    pub fn terrane_construct(format: String) -> Self {
        let mut __terrane_constructed_value = Self { format: String::from("") };
        __terrane_constructed_value.construct(format);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, format: String) {
        self.format = format;
    }
    pub fn compress(
        &self,
        data: Vec<u8>,
        options: CompressionOptions,
    ) -> CompressionResult {
        let format: String;
        let raw: TerranePlatformResult;
        format = self.format.clone();
        raw = terrane_platform_compress(
            format,
            data,
            options.level,
            options.deterministic,
        );
        return CompressionResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_resource_limit(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_bytes(&raw),
        );
    }
    pub fn decompress(
        &self,
        data: Vec<u8>,
        limits: DecompressionLimits,
    ) -> CompressionResult {
        let format: String;
        let raw: TerranePlatformResult;
        format = self.format.clone();
        raw = terrane_platform_decompress(
            format,
            data,
            limits.max_output,
            limits.max_ratio,
            limits.max_work,
        );
        return CompressionResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_resource_limit(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_bytes(&raw),
        );
    }
}
pub fn gzip() -> CompressionCodec {
    return CompressionCodec::terrane_construct(String::from("gzip"));
}
pub fn zlib() -> CompressionCodec {
    return CompressionCodec::terrane_construct(String::from("zlib"));
}
pub fn deflate_raw() -> CompressionCodec {
    return CompressionCodec::terrane_construct(String::from("deflate-raw"));
}
pub fn zstd() -> CompressionCodec {
    return CompressionCodec::terrane_construct(String::from("zstd"));
}
// Source: core/random.trn
// Namespace: core/random
#[derive(Clone)]
pub struct ByteResult {
    pub failed: bool,
    pub message: String,
    pub value: Vec<u8>,
}
impl ByteResult {
    pub fn terrane_construct(failed: bool, message: String, data: Vec<u8>) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            value: Vec::from([]),
        };
        __terrane_constructed_value.construct(failed, message, data);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, failed: bool, message: String, data: Vec<u8>) {
        self.failed = failed;
        self.message = message;
        self.value = data;
    }
}
#[derive(Clone)]
pub struct RandomIntResult {
    pub failed: bool,
    pub message: String,
    pub value: terrane_int_support::Int,
}
impl RandomIntResult {
    pub fn terrane_construct(
        failed: bool,
        message: String,
        number: terrane_int_support::Int,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            value: terrane_int_support::Int::from(0_i128),
        };
        __terrane_constructed_value.construct(failed, message, number);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        message: String,
        number: terrane_int_support::Int,
    ) {
        self.failed = failed;
        self.message = message;
        self.value = number.clone();
    }
}
#[derive(Clone)]
pub struct SecretOperationResult {
    pub failed: bool,
    pub message: String,
}
impl SecretOperationResult {
    pub fn terrane_construct(failed: bool, message: String) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
        };
        __terrane_constructed_value.construct(failed, message);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, failed: bool, message: String) {
        self.failed = failed;
        self.message = message;
    }
}
#[derive(Clone)]
pub struct SecretBuffer {
    pub handle: TerranePlatformCapability,
}
impl SecretBuffer {
    pub fn terrane_construct(data: Vec<u8>) -> Self {
        let mut __terrane_constructed_value = Self {
            handle: terrane_platform_secret_buffer(Vec::from([])),
        };
        __terrane_constructed_value.construct(data);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, data: Vec<u8>) {
        self.handle = terrane_platform_secret_buffer(data);
    }
}
pub fn destroy_secret(secret: SecretBuffer) -> SecretOperationResult {
    let raw: TerranePlatformResult;
    raw = terrane_platform_destroy_secret(&secret.handle);
    return SecretOperationResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
    );
}
#[derive(Clone)]
pub struct DigestValue {
    pub algorithm: String,
    pub value: Vec<u8>,
}
impl DigestValue {
    pub fn terrane_construct(algorithm: String, data: Vec<u8>) -> Self {
        let mut __terrane_constructed_value = Self {
            algorithm: String::from(""),
            value: Vec::from([]),
        };
        __terrane_constructed_value.construct(algorithm, data);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, algorithm: String, data: Vec<u8>) {
        self.algorithm = algorithm;
        self.value = data;
    }
    pub fn constant_time_equals(&self, other: DigestValue) -> bool {
        let left: Vec<u8>;
        if self.algorithm.as_str() != other.algorithm.as_str() {
            return false;
        }
        left = self.value.clone();
        return terrane_platform_constant_time_equal(left, other.value);
    }
}
#[derive(Clone)]
pub struct DigestResult {
    pub failed: bool,
    pub message: String,
    pub value: DigestValue,
}
impl DigestResult {
    pub fn terrane_construct(
        failed: bool,
        message: String,
        digest: DigestValue,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            value: DigestValue::terrane_construct(String::from(""), Vec::from([])),
        };
        __terrane_constructed_value.construct(failed, message, digest);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, failed: bool, message: String, digest: DigestValue) {
        self.failed = failed;
        self.message = message;
        self.value = digest;
    }
}
#[derive(Clone)]
pub struct SignatureValue {
    pub algorithm: String,
    pub value: Vec<u8>,
}
impl SignatureValue {
    pub fn terrane_construct(algorithm: String, data: Vec<u8>) -> Self {
        let mut __terrane_constructed_value = Self {
            algorithm: String::from(""),
            value: Vec::from([]),
        };
        __terrane_constructed_value.construct(algorithm, data);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, algorithm: String, data: Vec<u8>) {
        self.algorithm = algorithm;
        self.value = data;
    }
    pub fn constant_time_equals(&self, other: SignatureValue) -> bool {
        let left: Vec<u8>;
        if self.algorithm.as_str() != other.algorithm.as_str() {
            return false;
        }
        left = self.value.clone();
        return terrane_platform_constant_time_equal(left, other.value);
    }
}
#[derive(Clone)]
pub struct SignatureResult {
    pub failed: bool,
    pub message: String,
    pub value: SignatureValue,
}
impl SignatureResult {
    pub fn terrane_construct(
        failed: bool,
        message: String,
        signature: SignatureValue,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            value: SignatureValue::terrane_construct(String::from(""), Vec::from([])),
        };
        __terrane_constructed_value.construct(failed, message, signature);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        message: String,
        signature: SignatureValue,
    ) {
        self.failed = failed;
        self.message = message;
        self.value = signature;
    }
}
#[derive(Clone)]
pub struct SecureRandom {
    pub handle: TerranePlatformCapability,
}
impl SecureRandom {
    pub fn terrane_construct() -> Self {
        let mut __terrane_constructed_value = Self {
            handle: terrane_platform_secure_random(),
        };
        __terrane_constructed_value.construct();
        __terrane_constructed_value
    }
    pub fn construct(&mut self) {
        self.handle = terrane_platform_secure_random();
    }
    pub fn generate_bytes(&self, count: terrane_int_support::Int) -> ByteResult {
        let raw: TerranePlatformResult;
        raw = terrane_platform_random_bytes(&self.handle, count);
        return ByteResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_bytes(&raw),
        );
    }
    pub fn bounded_int(
        &self,
        upper_exclusive: terrane_int_support::Int,
    ) -> RandomIntResult {
        let raw: TerranePlatformResult;
        raw = terrane_platform_random_bounded(&self.handle, upper_exclusive);
        return RandomIntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
}
#[derive(Clone)]
pub struct PseudoRandomAlgorithm {
    pub name: String,
}
impl PseudoRandomAlgorithm {
    pub fn terrane_construct(name: String) -> Self {
        let mut __terrane_constructed_value = Self { name: String::from("") };
        __terrane_constructed_value.construct(name);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, name: String) {
        self.name = name;
    }
}
pub fn chacha20() -> PseudoRandomAlgorithm {
    return PseudoRandomAlgorithm::terrane_construct(String::from("chacha20"));
}
#[derive(Clone)]
pub struct PseudoRandom {
    pub handle: TerranePlatformCapability,
}
impl PseudoRandom {
    pub fn terrane_construct(algorithm: PseudoRandomAlgorithm, seed: Vec<u8>) -> Self {
        let mut __terrane_constructed_value = Self {
            handle: terrane_platform_pseudo_random(
                String::from("chacha20"),
                Vec::from([]),
            ),
        };
        __terrane_constructed_value.construct(algorithm, seed);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, algorithm: PseudoRandomAlgorithm, seed: Vec<u8>) {
        self.handle = terrane_platform_pseudo_random(algorithm.name, seed);
    }
    pub fn generate_bytes(&self, count: terrane_int_support::Int) -> ByteResult {
        let raw: TerranePlatformResult;
        raw = terrane_platform_random_bytes(&self.handle, count);
        return ByteResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_bytes(&raw),
        );
    }
    pub fn bounded_int(
        &self,
        upper_exclusive: terrane_int_support::Int,
    ) -> RandomIntResult {
        let raw: TerranePlatformResult;
        raw = terrane_platform_random_bounded(&self.handle, upper_exclusive);
        return RandomIntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
    pub fn split(self) -> PseudoRandom {
        let raw: TerranePlatformResult;
        let mut child: PseudoRandom;
        raw = terrane_platform_random_split(&self.handle);
        child = PseudoRandom::terrane_construct(chacha20(), Vec::from([]));
        child.handle = terrane_platform_result_capability(&raw);
        return child;
    }
}
pub fn split_pseudo(source: PseudoRandom) -> PseudoRandom {
    let raw: TerranePlatformResult;
    let mut child: PseudoRandom;
    raw = terrane_platform_random_split(&source.handle);
    child = PseudoRandom::terrane_construct(chacha20(), Vec::from([]));
    child.handle = terrane_platform_result_capability(&raw);
    return child;
}
pub fn secure_bytes(
    source: SecureRandom,
    count: terrane_int_support::Int,
) -> ByteResult {
    let raw: TerranePlatformResult;
    raw = terrane_platform_random_bytes(&source.handle, count);
    return ByteResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        terrane_platform_result_bytes(&raw),
    );
}
pub fn pseudo_bytes(
    source: PseudoRandom,
    count: terrane_int_support::Int,
) -> ByteResult {
    let raw: TerranePlatformResult;
    raw = terrane_platform_random_bytes(&source.handle, count);
    return ByteResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        terrane_platform_result_bytes(&raw),
    );
}
pub fn secure_bounded_int(
    source: SecureRandom,
    upper_exclusive: terrane_int_support::Int,
) -> RandomIntResult {
    let raw: TerranePlatformResult;
    raw = terrane_platform_random_bounded(&source.handle, upper_exclusive);
    return RandomIntResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        terrane_platform_result_int(&raw),
    );
}
pub fn pseudo_bounded_int(
    source: PseudoRandom,
    upper_exclusive: terrane_int_support::Int,
) -> RandomIntResult {
    let raw: TerranePlatformResult;
    raw = terrane_platform_random_bounded(&source.handle, upper_exclusive);
    return RandomIntResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        terrane_platform_result_int(&raw),
    );
}
#[derive(Clone)]
pub struct HashAlgorithm {
    pub name: String,
}
impl HashAlgorithm {
    pub fn terrane_construct(name: String) -> Self {
        let mut __terrane_constructed_value = Self { name: String::from("") };
        __terrane_constructed_value.construct(name);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, name: String) {
        self.name = name;
    }
}
pub fn sha256() -> HashAlgorithm {
    return HashAlgorithm::terrane_construct(String::from("sha-256"));
}
pub fn sha512() -> HashAlgorithm {
    return HashAlgorithm::terrane_construct(String::from("sha-512"));
}
pub fn digest_bytes(algorithm: HashAlgorithm, data: Vec<u8>) -> DigestResult {
    let raw: TerranePlatformResult;
    let value: DigestValue;
    raw = terrane_platform_digest(&algorithm.name, data);
    value = DigestValue::terrane_construct(
        algorithm.name.clone(),
        terrane_platform_result_bytes(&raw),
    );
    return DigestResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        value,
    );
}
pub fn sign_hmac(
    algorithm: HashAlgorithm,
    key: SecretBuffer,
    data: Vec<u8>,
) -> SignatureResult {
    let raw: TerranePlatformResult;
    let value: SignatureValue;
    raw = terrane_platform_hmac(&algorithm.name, &key.handle, data);
    value = SignatureValue::terrane_construct(
        algorithm.name.clone(),
        terrane_platform_result_bytes(&raw),
    );
    return SignatureResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        value,
    );
}
pub fn digest_equals(left: DigestValue, right: DigestValue) -> bool {
    return left.constant_time_equals(right);
}
pub fn signature_equals(left: SignatureValue, right: SignatureValue) -> bool {
    return left.constant_time_equals(right);
}
// Source: core/uuid.trn
// Namespace: core/random/uuid
#[derive(Clone)]
pub struct Uuid {
    pub string: String,
    pub bytes: Vec<u8>,
}
impl Uuid {
    pub fn terrane_construct(text: String, data: Vec<u8>) -> Self {
        let mut __terrane_constructed_value = Self {
            string: String::from(""),
            bytes: Vec::from([]),
        };
        __terrane_constructed_value.construct(text, data);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, text: String, data: Vec<u8>) {
        self.string = text;
        self.bytes = data;
    }
}
#[derive(Clone)]
pub struct UuidResult {
    pub failed: bool,
    pub message: String,
    pub value: Uuid,
}
impl UuidResult {
    pub fn terrane_construct(failed: bool, message: String, identifier: Uuid) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            value: Uuid::terrane_construct(String::from(""), Vec::from([])),
        };
        __terrane_constructed_value.construct(failed, message, identifier);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, failed: bool, message: String, identifier: Uuid) {
        self.failed = failed;
        self.message = message;
        self.value = identifier;
    }
}
pub fn parse_uuid(text: String) -> UuidResult {
    let raw: TerranePlatformResult;
    raw = terrane_platform_uuid_parse(text);
    return UuidResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        Uuid::terrane_construct(
            terrane_platform_result_text(&raw),
            terrane_platform_result_bytes(&raw),
        ),
    );
}
pub fn random_uuid(source: SecureRandom) -> UuidResult {
    let raw: TerranePlatformResult;
    raw = terrane_platform_uuid_v4(&source.handle);
    return UuidResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        Uuid::terrane_construct(
            terrane_platform_result_text(&raw),
            terrane_platform_result_bytes(&raw),
        ),
    );
}
pub fn time_uuid(
    source: SecureRandom,
    unix_milliseconds: terrane_int_support::Int,
) -> UuidResult {
    let raw: TerranePlatformResult;
    raw = terrane_platform_uuid_v7(&source.handle, unix_milliseconds);
    return UuidResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        Uuid::terrane_construct(
            terrane_platform_result_text(&raw),
            terrane_platform_result_bytes(&raw),
        ),
    );
}
