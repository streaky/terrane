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
    #[allow(
        dead_code,
        reason = "support-error conversions are selected by each lowered program"
    )]
    fn from_support_source_name(name: &str) -> Self {
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
            self.to_string().trim_start_matches(".decode-error: "),
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
        {
            /* terrane-site-row: site 0: /app::main (case.trn:52:13-52:38) */
            Site {
                function: 0,
                file: 0,
                line: 52,
                column: 13,
                end_line: 52,
                end_column: 38,
            }
        },
        {
            /* terrane-site-row: site 1: /app::main (case.trn:56:13-56:40) */
            Site {
                function: 0,
                file: 0,
                line: 56,
                column: 13,
                end_line: 56,
                end_column: 40,
            }
        },
        {
            /* terrane-site-row: site 2: /app::main (case.trn:65:13-65:40) */
            Site {
                function: 0,
                file: 0,
                line: 65,
                column: 13,
                end_line: 65,
                end_column: 40,
            }
        },
        {
            /* terrane-site-row: site 3: /app::main (case.trn:69:13-69:45) */
            Site {
                function: 0,
                file: 0,
                line: 69,
                column: 13,
                end_line: 69,
                end_column: 45,
            }
        },
        {
            /* terrane-site-row: site 4: /app::main (case.trn:73:13-73:44) */
            Site {
                function: 0,
                file: 0,
                line: 73,
                column: 13,
                end_line: 73,
                end_column: 44,
            }
        },
        {
            /* terrane-site-row: site 5: /app::main (case.trn:78:13-78:45) */
            Site {
                function: 0,
                file: 0,
                line: 78,
                column: 13,
                end_line: 78,
                end_column: 45,
            }
        },
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
type TerranePlatformCapability = terrane_platform_support::Capability;
type TerranePlatformResult = terrane_platform_support::ResultValue;
fn terrane_platform_i128(
    value: &terrane_int_support::Int,
    label: &str,
) -> Result<i128, TerranePlatformResult> {
    terrane_int_support::coerce::<i128>(value)
        .map_err(|_| TerranePlatformResult::error(
            format!("{label} is outside the signed 128-bit platform range"),
        ))
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
fn terrane_platform_secure_random() -> TerranePlatformCapability {
    terrane_platform_support::secure_random()
}
fn terrane_platform_pseudo_random(
    algorithm: String,
    seed: Vec<u8>,
) -> TerranePlatformCapability {
    terrane_platform_support::pseudo_random(&algorithm, &seed)
}
fn terrane_platform_secret_buffer(data: Vec<u8>) -> TerranePlatformCapability {
    terrane_platform_support::secret_buffer(data)
}
fn terrane_platform_destroy_secret(
    secret: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::destroy_secret(secret)
}
fn terrane_platform_random_bytes(
    source: &TerranePlatformCapability,
    count: terrane_int_support::Int,
) -> TerranePlatformResult {
    terrane_platform_support::random_bytes(
        source,
        terrane_platform_i128!(count, "random byte count"),
    )
}
fn terrane_platform_random_bounded(
    source: &TerranePlatformCapability,
    upper: terrane_int_support::Int,
) -> TerranePlatformResult {
    terrane_platform_support::random_bounded(
        source,
        terrane_platform_i128!(upper, "random upper bound"),
    )
}
fn terrane_platform_random_split(
    source: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::random_split(source)
}
fn terrane_platform_digest(algorithm: &String, data: Vec<u8>) -> TerranePlatformResult {
    terrane_platform_support::digest(algorithm, &data)
}
fn terrane_platform_hmac(
    algorithm: &String,
    key: &TerranePlatformCapability,
    data: Vec<u8>,
) -> TerranePlatformResult {
    terrane_platform_support::hmac(algorithm, key, &data)
}
fn terrane_platform_constant_time_equal(left: Vec<u8>, right: Vec<u8>) -> bool {
    terrane_platform_support::constant_time_equal(&left, &right)
}
fn terrane_platform_hex_encode(data: Vec<u8>) -> String {
    terrane_platform_support::hex_encode(&data)
}
fn terrane_platform_hex_decode(text: String) -> TerranePlatformResult {
    terrane_platform_support::hex_decode(&text)
}
fn terrane_platform_base64_encode(
    data: Vec<u8>,
    url_safe: bool,
    padded: bool,
) -> String {
    terrane_platform_support::base64_encode(&data, url_safe, padded)
}
fn terrane_platform_base64_decode(
    text: String,
    url_safe: bool,
    padded: bool,
) -> TerranePlatformResult {
    terrane_platform_support::base64_decode(&text, url_safe, padded)
}
fn terrane_platform_compress(
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
fn terrane_platform_decompress(
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
fn terrane_platform_uuid_parse(text: String) -> TerranePlatformResult {
    terrane_platform_support::uuid_parse(&text)
}
fn terrane_platform_uuid_v4(
    source: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::uuid_v4(source)
}
fn terrane_platform_uuid_v7(
    source: &TerranePlatformCapability,
    unix_milliseconds: terrane_int_support::Int,
) -> TerranePlatformResult {
    terrane_platform_support::uuid_v7(
        source,
        terrane_platform_i128!(unix_milliseconds, "UUID v7 timestamp"),
    )
}
// Source: case.trn
// Namespace: app
fn main() {
    let first: PseudoRandom = PseudoRandom::terrane_construct(
        chacha20(),
        Vec::from([115, 101, 101, 100]),
    );
    let second: PseudoRandom = PseudoRandom::terrane_construct(
        chacha20(),
        Vec::from([115, 101, 101, 100]),
    );
    let hash: HashAlgorithm = sha256();
    let gzip_codec: CompressionCodec = gzip();
    let left: ByteResult = pseudo_bytes(
        first.clone(),
        terrane_int_support::Int::from(32_i128),
    );
    let right: ByteResult = pseudo_bytes(
        second.clone(),
        terrane_int_support::Int::from(32_i128),
    );
    println!("{}", terrane_scalar_support::scalar_text(&(left.value == right.value)));
    let first_child: PseudoRandom = split_pseudo(first.clone());
    let second_child: PseudoRandom = split_pseudo(second.clone());
    println!(
        "{}", terrane_scalar_support::scalar_text(&(pseudo_bytes(first_child,
        terrane_int_support::Int::from(16_i128)).value == pseudo_bytes(second_child,
        terrane_int_support::Int::from(16_i128)).value))
    );
    let bounded: IntResult = pseudo_bounded_int(
        first.clone(),
        terrane_int_support::Int::from(17_i128),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(bounded.value.clone() >=
        terrane_int_support::Int::from(0_i128) &&bounded.value.clone() <
        terrane_int_support::Int::from(17_i128)))
    );
    let secure: SecureRandom = SecureRandom::terrane_construct();
    let secure_data: ByteResult = secure_bytes(
        secure.clone(),
        terrane_int_support::Int::from(16_i128),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(! secure_data.failed
        &&terrane_int_support::Int::from(secure_data.value.len() as i128) ==
        terrane_int_support::Int::from(16_i128)))
    );
    let secure_number: IntResult = secure_bounded_int(
        secure.clone(),
        terrane_int_support::Int::from(17_i128),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(! secure_number.failed
        &&secure_number.value.clone() >= terrane_int_support::Int::from(0_i128)
        &&secure_number.value.clone() < terrane_int_support::Int::from(17_i128)))
    );
    let digest: DigestResult = digest_bytes(hash.clone(), Vec::from([97, 98, 99]));
    println!(
        "{}", terrane_scalar_support::scalar_text(&encode_hex(digest.value.value
        .clone()))
    );
    let wide_hash: HashAlgorithm = sha512();
    let wide_digest: DigestResult = digest_bytes(
        wide_hash.clone(),
        Vec::from([97, 98, 99]),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&encode_hex(wide_digest.value.value
        .clone()))
    );
    let key: SecretBuffer = SecretBuffer::terrane_construct(Vec::from([107, 101, 121]));
    let same_key: SecretBuffer = SecretBuffer::terrane_construct(
        Vec::from([107, 101, 121]),
    );
    let mac: SignatureResult = sign_hmac(
        hash.clone(),
        key.clone(),
        Vec::from([100, 97, 116, 97]),
    );
    let same_mac: SignatureResult = sign_hmac(
        hash.clone(),
        same_key.clone(),
        Vec::from([100, 97, 116, 97]),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&signature_equals(mac.value, same_mac
        .value))
    );
    let wide_mac: SignatureResult = sign_hmac(
        wide_hash.clone(),
        key.clone(),
        Vec::from([100, 97, 116, 97]),
    );
    let same_wide_mac: SignatureResult = sign_hmac(
        wide_hash.clone(),
        same_key.clone(),
        Vec::from([100, 97, 116, 97]),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&signature_equals(wide_mac.value,
        same_wide_mac.value))
    );
    destroy_secret(same_key.clone());
    let destroyed: SignatureResult = sign_hmac(
        hash.clone(),
        same_key.clone(),
        Vec::from([100, 97, 116, 97]),
    );
    println!("{}", terrane_scalar_support::scalar_text(&destroyed.failed));
    let unsupported: HashAlgorithm = HashAlgorithm::terrane_construct(
        String::from("unsupported"),
    );
    let failed_digest: DigestResult = digest_bytes(
        unsupported.clone(),
        Vec::from([100, 97, 116, 97]),
    );
    println!("{}", terrane_scalar_support::scalar_text(&failed_digest.failed));
    let failed_mac: SignatureResult = sign_hmac(
        unsupported.clone(),
        key.clone(),
        Vec::from([100, 97, 116, 97]),
    );
    println!("{}", terrane_scalar_support::scalar_text(&failed_mac.failed));
    println!(
        "{}", terrane_scalar_support::scalar_text(&encode_base64(Vec::from([104, 101,
        108, 108, 111]), false, true))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&encode_base64(Vec::from([104, 101,
        108, 108, 111, 63]), true, false))
    );
    let strict: DecodeResult = decode_base64(String::from("aGVsbG8="), false, true);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::decode(&strict
        .value, terrane_string_support::Encoding::Utf8), 0 /* terrane-site: case.trn:52:13-52:38 */))
    );
    let unpadded_as_padded: DecodeResult = decode_base64(
        String::from("aGVsbG8"),
        false,
        true,
    );
    println!("{}", terrane_scalar_support::scalar_text(&unpadded_as_padded.failed));
    let unpadded: DecodeResult = decode_base64(String::from("aGVsbG8"), false, false);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::decode(&unpadded
        .value, terrane_string_support::Encoding::Utf8), 1 /* terrane-site: case.trn:56:13-56:40 */))
    );
    let wrong_alphabet: DecodeResult = decode_base64(
        String::from("aGVsbG8_"),
        false,
        false,
    );
    println!("{}", terrane_scalar_support::scalar_text(&wrong_alphabet.failed));
    let malformed: DecodeResult = decode_hex(String::from("abc"));
    println!("{}", terrane_scalar_support::scalar_text(&malformed.failed));
    let options: CompressionOptions = CompressionOptions::terrane_construct(
        terrane_int_support::Int::from(6_i128),
        true,
    );
    let packed: CompressionResult = gzip_codec
        .compress(
            Vec::from([99, 111, 109, 112, 114, 101, 115, 115, 32, 109, 101]),
            options.clone(),
        );
    let limits: DecompressionLimits = DecompressionLimits::terrane_construct(
        terrane_int_support::Int::from(1024_i128),
        terrane_int_support::Int::from(100_i128),
        terrane_int_support::Int::from(4096_i128),
    );
    let unpacked: CompressionResult = gzip_codec
        .decompress(packed.value.clone(), limits.clone());
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::decode(&unpacked
        .value, terrane_string_support::Encoding::Utf8), 2 /* terrane-site: case.trn:65:13-65:40 */))
    );
    let zlib_codec: CompressionCodec = zlib();
    let zlib_packed: CompressionResult = zlib_codec
        .compress(
            Vec::from([99, 111, 109, 112, 114, 101, 115, 115, 32, 109, 101]),
            options.clone(),
        );
    let zlib_unpacked: CompressionResult = zlib_codec
        .decompress(zlib_packed.value.clone(), limits.clone());
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::decode(&zlib_unpacked
        .value, terrane_string_support::Encoding::Utf8), 3 /* terrane-site: case.trn:69:13-69:45 */))
    );
    let raw_codec: CompressionCodec = deflate_raw();
    let raw_packed: CompressionResult = raw_codec
        .compress(
            Vec::from([99, 111, 109, 112, 114, 101, 115, 115, 32, 109, 101]),
            options.clone(),
        );
    let raw_unpacked: CompressionResult = raw_codec
        .decompress(raw_packed.value.clone(), limits.clone());
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::decode(&raw_unpacked
        .value, terrane_string_support::Encoding::Utf8), 4 /* terrane-site: case.trn:73:13-73:44 */))
    );
    let zstd_codec: CompressionCodec = zstd();
    let zstd_packed: CompressionResult = zstd_codec
        .compress(
            Vec::from([99, 111, 109, 112, 114, 101, 115, 115, 32, 109, 101]),
            options.clone(),
        );
    let zstd_limits: DecompressionLimits = DecompressionLimits::terrane_construct(
        terrane_int_support::Int::from(1073741824_i128),
        terrane_int_support::Int::from(100_i128),
        terrane_int_support::Int::from(1073741824_i128),
    );
    let zstd_unpacked: CompressionResult = zstd_codec
        .decompress(zstd_packed.value.clone(), zstd_limits);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::decode(&zstd_unpacked
        .value, terrane_string_support::Encoding::Utf8), 5 /* terrane-site: case.trn:78:13-78:45 */))
    );
    let bomb_limits: DecompressionLimits = DecompressionLimits::terrane_construct(
        terrane_int_support::Int::from(4_i128),
        terrane_int_support::Int::from(2_i128),
        terrane_int_support::Int::from(8_i128),
    );
    let refused: CompressionResult = gzip_codec
        .decompress(packed.value.clone(), bomb_limits);
    println!("{}", terrane_scalar_support::scalar_text(&refused.resource_limit));
    let parsed: UuidResult = parse_uuid(
        String::from("01890f3e-7b4d-7cc0-98c8-77e22c318a14"),
    );
    println!("{}", terrane_scalar_support::scalar_text(&parsed.value.string));
    let generated_v4: UuidResult = random_uuid(secure.clone());
    let reparsed_v4: UuidResult = parse_uuid(generated_v4.value.string.clone());
    println!(
        "{}", terrane_scalar_support::scalar_text(&(! generated_v4.failed &&! reparsed_v4
        .failed &&terrane_int_support::Int::from(generated_v4.value.bytes.len() as i128)
        == terrane_int_support::Int::from(16_i128)))
    );
    let generated_v7: UuidResult = time_uuid(
        secure.clone(),
        terrane_int_support::Int::from(1700000000000_i128),
    );
    let reparsed_v7: UuidResult = parse_uuid(generated_v7.value.string.clone());
    println!(
        "{}", terrane_scalar_support::scalar_text(&(! generated_v7.failed &&! reparsed_v7
        .failed &&terrane_int_support::Int::from(generated_v7.value.bytes.len() as i128)
        == terrane_int_support::Int::from(16_i128)))
    );
    let noncanonical: UuidResult = parse_uuid(
        String::from("01890F3E-7B4D-7CC0-98C8-77E22C318A14"),
    );
    println!("{}", terrane_scalar_support::scalar_text(&noncanonical.failed));
    let invalid_time: UuidResult = time_uuid(
        secure.clone(),
        terrane_int_support::Int::from(-1_i128),
    );
    println!("{}", terrane_scalar_support::scalar_text(&invalid_time.failed));
}
// Source: standard/codecs.trn
// Namespace: standard/codecs
#[derive(Clone)]
pub struct DecodeResult {
    pub failed: bool,
    pub message: String,
    pub value: Vec<u8>,
}
impl DecodeResult {
    pub fn terrane_construct(failed: bool, message: String, data: Vec<u8>) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            value: Vec::from([]),
        };
        value.construct(failed, message, data);
        value
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
        let raw: TerranePlatformResult = terrane_platform_hex_decode(text);
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
        let mut value = Self { url_safe: false };
        value.construct(url_safe);
        value
    }
    pub fn construct(&mut self, url_safe: bool) {
        self.url_safe = url_safe;
    }
    pub fn encode(&self, data: Vec<u8>, padded: bool) -> String {
        return terrane_platform_base64_encode(data, self.url_safe, padded);
    }
    pub fn decode(&self, text: String, padded: bool) -> DecodeResult {
        let raw: TerranePlatformResult = terrane_platform_base64_decode(
            text,
            self.url_safe,
            padded,
        );
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
pub fn encode_hex(data: Vec<u8>) -> String {
    return terrane_platform_hex_encode(data);
}
pub fn decode_hex(text: String) -> DecodeResult {
    let raw: TerranePlatformResult = terrane_platform_hex_decode(text);
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
    let raw: TerranePlatformResult = terrane_platform_base64_decode(
        text,
        url_safe,
        padded,
    );
    return DecodeResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        terrane_platform_result_bytes(&raw),
    );
}
// Source: standard/compression.trn
// Namespace: standard/compression
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
        let mut value = Self {
            level: terrane_int_support::Int::from(6_i128),
            deterministic: true,
        };
        value.construct(level, deterministic);
        value
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
        let mut value = Self {
            max_output: terrane_int_support::Int::from(16777216_i128),
            max_ratio: terrane_int_support::Int::from(100_i128),
            max_work: terrane_int_support::Int::from(67108864_i128),
        };
        value.construct(max_output, max_ratio, max_work);
        value
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
        let mut value = Self {
            failed: false,
            resource_limit: false,
            message: String::from(""),
            value: Vec::from([]),
        };
        value.construct(failed, resource_limit, message, data);
        value
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
        let mut value = Self { format: String::from("") };
        value.construct(format);
        value
    }
    pub fn construct(&mut self, format: String) {
        self.format = format;
    }
    pub fn compress(
        &self,
        data: Vec<u8>,
        options: CompressionOptions,
    ) -> CompressionResult {
        let format: String = self.format.clone();
        let raw: TerranePlatformResult = terrane_platform_compress(
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
        let format: String = self.format.clone();
        let raw: TerranePlatformResult = terrane_platform_decompress(
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
// Source: standard/random.trn
// Namespace: standard/random
#[derive(Clone)]
pub struct ByteResult {
    pub failed: bool,
    pub message: String,
    pub value: Vec<u8>,
}
impl ByteResult {
    pub fn terrane_construct(failed: bool, message: String, data: Vec<u8>) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            value: Vec::from([]),
        };
        value.construct(failed, message, data);
        value
    }
    pub fn construct(&mut self, failed: bool, message: String, data: Vec<u8>) {
        self.failed = failed;
        self.message = message;
        self.value = data;
    }
}
#[derive(Clone)]
pub struct IntResult {
    pub failed: bool,
    pub message: String,
    pub value: terrane_int_support::Int,
}
impl IntResult {
    pub fn terrane_construct(
        failed: bool,
        message: String,
        number: terrane_int_support::Int,
    ) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            value: terrane_int_support::Int::from(0_i128),
        };
        value.construct(failed, message, number);
        value
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
        let mut value = Self {
            failed: false,
            message: String::from(""),
        };
        value.construct(failed, message);
        value
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
        let mut value = Self {
            handle: terrane_platform_secret_buffer(Vec::from([])),
        };
        value.construct(data);
        value
    }
    pub fn construct(&mut self, data: Vec<u8>) {
        self.handle = terrane_platform_secret_buffer(data);
    }
}
pub fn destroy_secret(secret: SecretBuffer) -> SecretOperationResult {
    let raw: TerranePlatformResult = terrane_platform_destroy_secret(&secret.handle);
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
        let mut value = Self {
            algorithm: String::from(""),
            value: Vec::from([]),
        };
        value.construct(algorithm, data);
        value
    }
    pub fn construct(&mut self, algorithm: String, data: Vec<u8>) {
        self.algorithm = algorithm;
        self.value = data;
    }
    pub fn constant_time_equals(&self, other: DigestValue) -> bool {
        if self.algorithm != other.algorithm {
            return false;
        }
        let left: Vec<u8> = self.value.clone();
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
        let mut value = Self {
            failed: false,
            message: String::from(""),
            value: DigestValue::terrane_construct(String::from(""), Vec::from([])),
        };
        value.construct(failed, message, digest);
        value
    }
    pub fn construct(&mut self, failed: bool, message: String, digest: DigestValue) {
        self.failed = failed;
        self.message = message;
        self.value = digest.clone();
    }
}
#[derive(Clone)]
pub struct SignatureValue {
    pub algorithm: String,
    pub value: Vec<u8>,
}
impl SignatureValue {
    pub fn terrane_construct(algorithm: String, data: Vec<u8>) -> Self {
        let mut value = Self {
            algorithm: String::from(""),
            value: Vec::from([]),
        };
        value.construct(algorithm, data);
        value
    }
    pub fn construct(&mut self, algorithm: String, data: Vec<u8>) {
        self.algorithm = algorithm;
        self.value = data;
    }
    pub fn constant_time_equals(&self, other: SignatureValue) -> bool {
        if self.algorithm != other.algorithm {
            return false;
        }
        let left: Vec<u8> = self.value.clone();
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
        let mut value = Self {
            failed: false,
            message: String::from(""),
            value: SignatureValue::terrane_construct(String::from(""), Vec::from([])),
        };
        value.construct(failed, message, signature);
        value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        message: String,
        signature: SignatureValue,
    ) {
        self.failed = failed;
        self.message = message;
        self.value = signature.clone();
    }
}
#[derive(Clone)]
pub struct SecureRandom {
    pub handle: TerranePlatformCapability,
}
impl SecureRandom {
    pub fn terrane_construct() -> Self {
        let mut value = Self {
            handle: terrane_platform_secure_random(),
        };
        value.construct();
        value
    }
    pub fn construct(&mut self) {
        self.handle = terrane_platform_secure_random();
    }
    pub fn generate_bytes(&self, count: terrane_int_support::Int) -> ByteResult {
        let raw: TerranePlatformResult = terrane_platform_random_bytes(
            &self.handle,
            count,
        );
        return ByteResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_bytes(&raw),
        );
    }
    pub fn bounded_int(&self, upper_exclusive: terrane_int_support::Int) -> IntResult {
        let raw: TerranePlatformResult = terrane_platform_random_bounded(
            &self.handle,
            upper_exclusive,
        );
        return IntResult::terrane_construct(
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
        let mut value = Self { name: String::from("") };
        value.construct(name);
        value
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
        let mut value = Self {
            handle: terrane_platform_pseudo_random(
                String::from("chacha20"),
                Vec::from([]),
            ),
        };
        value.construct(algorithm, seed);
        value
    }
    pub fn construct(&mut self, algorithm: PseudoRandomAlgorithm, seed: Vec<u8>) {
        self.handle = terrane_platform_pseudo_random(algorithm.name, seed);
    }
    pub fn generate_bytes(&self, count: terrane_int_support::Int) -> ByteResult {
        let raw: TerranePlatformResult = terrane_platform_random_bytes(
            &self.handle,
            count,
        );
        return ByteResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_bytes(&raw),
        );
    }
    pub fn bounded_int(&self, upper_exclusive: terrane_int_support::Int) -> IntResult {
        let raw: TerranePlatformResult = terrane_platform_random_bounded(
            &self.handle,
            upper_exclusive,
        );
        return IntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
    pub fn split(&self) -> PseudoRandom {
        let raw: TerranePlatformResult = terrane_platform_random_split(&self.handle);
        let mut child: PseudoRandom = PseudoRandom::terrane_construct(
            chacha20(),
            Vec::from([]),
        );
        child.handle = terrane_platform_result_capability(&raw);
        return child.clone();
    }
}
pub fn split_pseudo(source: PseudoRandom) -> PseudoRandom {
    let raw: TerranePlatformResult = terrane_platform_random_split(&source.handle);
    let mut child: PseudoRandom = PseudoRandom::terrane_construct(
        chacha20(),
        Vec::from([]),
    );
    child.handle = terrane_platform_result_capability(&raw);
    return child.clone();
}
pub fn secure_bytes(
    source: SecureRandom,
    count: terrane_int_support::Int,
) -> ByteResult {
    let raw: TerranePlatformResult = terrane_platform_random_bytes(
        &source.handle,
        count,
    );
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
    let raw: TerranePlatformResult = terrane_platform_random_bytes(
        &source.handle,
        count,
    );
    return ByteResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        terrane_platform_result_bytes(&raw),
    );
}
pub fn secure_bounded_int(
    source: SecureRandom,
    upper_exclusive: terrane_int_support::Int,
) -> IntResult {
    let raw: TerranePlatformResult = terrane_platform_random_bounded(
        &source.handle,
        upper_exclusive,
    );
    return IntResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        terrane_platform_result_int(&raw),
    );
}
pub fn pseudo_bounded_int(
    source: PseudoRandom,
    upper_exclusive: terrane_int_support::Int,
) -> IntResult {
    let raw: TerranePlatformResult = terrane_platform_random_bounded(
        &source.handle,
        upper_exclusive,
    );
    return IntResult::terrane_construct(
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
        let mut value = Self { name: String::from("") };
        value.construct(name);
        value
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
    let raw: TerranePlatformResult = terrane_platform_digest(&algorithm.name, data);
    let value: DigestValue = DigestValue::terrane_construct(
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
    let raw: TerranePlatformResult = terrane_platform_hmac(
        &algorithm.name,
        &key.handle,
        data,
    );
    let value: SignatureValue = SignatureValue::terrane_construct(
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
    return left.constant_time_equals(right.clone());
}
pub fn signature_equals(left: SignatureValue, right: SignatureValue) -> bool {
    return left.constant_time_equals(right.clone());
}
// Source: standard/uuid.trn
// Namespace: standard/uuid
#[derive(Clone)]
pub struct Uuid {
    pub string: String,
    pub bytes: Vec<u8>,
}
impl Uuid {
    pub fn terrane_construct(text: String, data: Vec<u8>) -> Self {
        let mut value = Self {
            string: String::from(""),
            bytes: Vec::from([]),
        };
        value.construct(text, data);
        value
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
        let mut value = Self {
            failed: false,
            message: String::from(""),
            value: Uuid::terrane_construct(String::from(""), Vec::from([])),
        };
        value.construct(failed, message, identifier);
        value
    }
    pub fn construct(&mut self, failed: bool, message: String, identifier: Uuid) {
        self.failed = failed;
        self.message = message;
        self.value = identifier.clone();
    }
}
pub fn parse_uuid(text: String) -> UuidResult {
    let raw: TerranePlatformResult = terrane_platform_uuid_parse(text);
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
    let raw: TerranePlatformResult = terrane_platform_uuid_v4(&source.handle);
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
    let raw: TerranePlatformResult = terrane_platform_uuid_v7(
        &source.handle,
        unix_milliseconds,
    );
    return UuidResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        Uuid::terrane_construct(
            terrane_platform_result_text(&raw),
            terrane_platform_result_bytes(&raw),
        ),
    );
}
