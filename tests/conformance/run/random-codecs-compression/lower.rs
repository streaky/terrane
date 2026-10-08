// Generated deterministically by Terrane <version>.
// Runtime support: platform_capability_types.rs, platform_result_type.rs, platform_int_conversion.rs, platform_capability_base.rs, platform_random.rs, platform_codecs.rs, platform_compression.rs, platform_uuid.rs
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
    pub static FILES: [&str; 1] = ["case.trn"];
    pub static FUNCTIONS: [&str; 1] = ["/app::main"];
    pub static SITES: [Site; 6] = [
        /* terrane-site-row: site 0: /app::main (case.trn:52:13-52:38) */
        { Site { function: 0, file: 0, line: 52, column: 13, end_line: 52, end_column: 38 } },
        /* terrane-site-row: site 1: /app::main (case.trn:56:13-56:40) */
        { Site { function: 0, file: 0, line: 56, column: 13, end_line: 56, end_column: 40 } },
        /* terrane-site-row: site 2: /app::main (case.trn:65:13-65:40) */
        { Site { function: 0, file: 0, line: 65, column: 13, end_line: 65, end_column: 40 } },
        /* terrane-site-row: site 3: /app::main (case.trn:69:13-69:45) */
        { Site { function: 0, file: 0, line: 69, column: 13, end_line: 69, end_column: 45 } },
        /* terrane-site-row: site 4: /app::main (case.trn:73:13-73:44) */
        { Site { function: 0, file: 0, line: 73, column: 13, end_line: 73, end_column: 44 } },
        /* terrane-site-row: site 5: /app::main (case.trn:78:13-78:45) */
        { Site { function: 0, file: 0, line: 78, column: 13, end_line: 78, end_column: 45 } },
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
// Namespace: app
fn main() {
    let first_terrane_f0_s597: PseudoRandom;
    let second_terrane_f0_s654: PseudoRandom;
    let hash_terrane_f0_s712: HashAlgorithm;
    let gzip_codec_terrane_f0_s731: CompressionCodec;
    let left_terrane_f0_s754: ByteResult;
    let right_terrane_f0_s789: ByteResult;
    let first_child_terrane_f0_s863: PseudoRandom;
    let second_child_terrane_f0_s901: PseudoRandom;
    let bounded_terrane_f0_s1032: RandomIntResult;
    let secure_terrane_f0_s1129: SecureRandom;
    let secure_data_terrane_f0_s1166: ByteResult;
    let secure_number_terrane_f0_s1278: RandomIntResult;
    let digest_terrane_f0_s1423: DigestResult;
    let wide_hash_terrane_f0_s1507: HashAlgorithm;
    let wide_digest_terrane_f0_s1531: DigestResult;
    let key_terrane_f0_s1630: SecretBuffer;
    let same_key_terrane_f0_s1671: SecretBuffer;
    let mac_terrane_f0_s1717: SignatureResult;
    let same_mac_terrane_f0_s1757: SignatureResult;
    let wide_mac_terrane_f0_s1864: SignatureResult;
    let same_wide_mac_terrane_f0_s1914: SignatureResult;
    let destroyed_terrane_f0_s2070: SignatureResult;
    let unsupported_terrane_f0_s2149: HashAlgorithm;
    let failed_digest_terrane_f0_s2206: DigestResult;
    let failed_mac_terrane_f0_s2293: SignatureResult;
    let strict_terrane_f0_s2477: DecodeResult;
    let unpadded_as_padded_terrane_f0_s2568: DecodeResult;
    let unpadded_terrane_f0_s2668: DecodeResult;
    let wrong_alphabet_terrane_f0_s2763: DecodeResult;
    let malformed_terrane_f0_s2857: DecodeResult;
    let options_terrane_f0_s2919: CompressionOptions;
    let packed_terrane_f0_s2971: CompressionResult;
    let limits_terrane_f0_s3029: DecompressionLimits;
    let unpacked_terrane_f0_s3089: CompressionResult;
    let zlib_codec_terrane_f0_s3189: CompressionCodec;
    let zlib_packed_terrane_f0_s3212: CompressionResult;
    let zlib_unpacked_terrane_f0_s3275: CompressionResult;
    let raw_codec_terrane_f0_s3390: CompressionCodec;
    let raw_packed_terrane_f0_s3419: CompressionResult;
    let raw_unpacked_terrane_f0_s3480: CompressionResult;
    let zstd_codec_terrane_f0_s3591: CompressionCodec;
    let zstd_packed_terrane_f0_s3614: CompressionResult;
    let zstd_limits_terrane_f0_s3677: DecompressionLimits;
    let zstd_unpacked_terrane_f0_s3754: CompressionResult;
    let bomb_limits_terrane_f0_s3874: DecompressionLimits;
    let refused_terrane_f0_s3931: CompressionResult;
    let parsed_terrane_f0_s4028: UuidResult;
    let generated_v4_terrane_f0_s4123: UuidResult;
    let reparsed_v4_terrane_f0_s4162: UuidResult;
    let generated_v7_terrane_f0_s4322: UuidResult;
    let reparsed_v7_terrane_f0_s4374: UuidResult;
    let noncanonical_terrane_f0_s4534: UuidResult;
    let invalid_time_terrane_f0_s4635: UuidResult;
    first_terrane_f0_s597 = PseudoRandom::terrane_construct(
        chacha20(),
        Vec::from([115, 101, 101, 100]),
    );
    second_terrane_f0_s654 = PseudoRandom::terrane_construct(
        chacha20(),
        Vec::from([115, 101, 101, 100]),
    );
    hash_terrane_f0_s712 = sha256();
    gzip_codec_terrane_f0_s731 = gzip();
    left_terrane_f0_s754 = pseudo_bytes(
        first_terrane_f0_s597.clone(),
        terrane_int_support::Int::from(32_i128),
    );
    right_terrane_f0_s789 = pseudo_bytes(
        second_terrane_f0_s654.clone(),
        terrane_int_support::Int::from(32_i128),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(left_terrane_f0_s754.value ==
        right_terrane_f0_s789.value))
    );
    first_child_terrane_f0_s863 = split_pseudo(first_terrane_f0_s597.clone());
    second_child_terrane_f0_s901 = split_pseudo(second_terrane_f0_s654);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(pseudo_bytes(first_child_terrane_f0_s863,
        terrane_int_support::Int::from(16_i128)).value ==
        pseudo_bytes(second_child_terrane_f0_s901,
        terrane_int_support::Int::from(16_i128)).value))
    );
    bounded_terrane_f0_s1032 = pseudo_bounded_int(
        first_terrane_f0_s597,
        terrane_int_support::Int::from(17_i128),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(bounded_terrane_f0_s1032.value
        .clone() >= terrane_int_support::Int::from(0_i128) &&bounded_terrane_f0_s1032
        .value.clone() < terrane_int_support::Int::from(17_i128)))
    );
    secure_terrane_f0_s1129 = SecureRandom::terrane_construct();
    secure_data_terrane_f0_s1166 = secure_bytes(
        secure_terrane_f0_s1129.clone(),
        terrane_int_support::Int::from(16_i128),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(! secure_data_terrane_f0_s1166.failed
        &&terrane_int_support::Int::from(secure_data_terrane_f0_s1166.value.len() as
        i128) == terrane_int_support::Int::from(16_i128)))
    );
    secure_number_terrane_f0_s1278 = secure_bounded_int(
        secure_terrane_f0_s1129.clone(),
        terrane_int_support::Int::from(17_i128),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(! secure_number_terrane_f0_s1278
        .failed &&secure_number_terrane_f0_s1278.value.clone() >=
        terrane_int_support::Int::from(0_i128) &&secure_number_terrane_f0_s1278.value
        .clone() < terrane_int_support::Int::from(17_i128)))
    );
    digest_terrane_f0_s1423 = digest_bytes(
        hash_terrane_f0_s712.clone(),
        Vec::from([97, 98, 99]),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&encode_hex(digest_terrane_f0_s1423
        .value.value.clone()))
    );
    wide_hash_terrane_f0_s1507 = sha512();
    wide_digest_terrane_f0_s1531 = digest_bytes(
        wide_hash_terrane_f0_s1507.clone(),
        Vec::from([97, 98, 99]),
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&encode_hex(wide_digest_terrane_f0_s1531
        .value.value.clone()))
    );
    key_terrane_f0_s1630 = SecretBuffer::terrane_construct(Vec::from([107, 101, 121]));
    same_key_terrane_f0_s1671 = SecretBuffer::terrane_construct(
        Vec::from([107, 101, 121]),
    );
    mac_terrane_f0_s1717 = sign_hmac(
        hash_terrane_f0_s712.clone(),
        key_terrane_f0_s1630.clone(),
        Vec::from([100, 97, 116, 97]),
    );
    same_mac_terrane_f0_s1757 = sign_hmac(
        hash_terrane_f0_s712.clone(),
        same_key_terrane_f0_s1671.clone(),
        Vec::from([100, 97, 116, 97]),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&signature_equals(mac_terrane_f0_s1717
        .value, same_mac_terrane_f0_s1757.value))
    );
    wide_mac_terrane_f0_s1864 = sign_hmac(
        wide_hash_terrane_f0_s1507.clone(),
        key_terrane_f0_s1630.clone(),
        Vec::from([100, 97, 116, 97]),
    );
    same_wide_mac_terrane_f0_s1914 = sign_hmac(
        wide_hash_terrane_f0_s1507,
        same_key_terrane_f0_s1671.clone(),
        Vec::from([100, 97, 116, 97]),
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&signature_equals(wide_mac_terrane_f0_s1864
        .value, same_wide_mac_terrane_f0_s1914.value))
    );
    destroy_secret(same_key_terrane_f0_s1671.clone());
    destroyed_terrane_f0_s2070 = sign_hmac(
        hash_terrane_f0_s712,
        same_key_terrane_f0_s1671,
        Vec::from([100, 97, 116, 97]),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&destroyed_terrane_f0_s2070.failed)
    );
    unsupported_terrane_f0_s2149 = HashAlgorithm::terrane_construct(
        String::from("unsupported"),
    );
    failed_digest_terrane_f0_s2206 = digest_bytes(
        unsupported_terrane_f0_s2149.clone(),
        Vec::from([100, 97, 116, 97]),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&failed_digest_terrane_f0_s2206.failed)
    );
    failed_mac_terrane_f0_s2293 = sign_hmac(
        unsupported_terrane_f0_s2149,
        key_terrane_f0_s1630,
        Vec::from([100, 97, 116, 97]),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&failed_mac_terrane_f0_s2293.failed)
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&encode_base64(Vec::from([104, 101,
        108, 108, 111]), false, true))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&encode_base64(Vec::from([104, 101,
        108, 108, 111, 63]), true, false))
    );
    strict_terrane_f0_s2477 = decode_base64(String::from("aGVsbG8="), false, true);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::decode(&strict_terrane_f0_s2477
        .value, terrane_string_support::Encoding::Utf8), 0 /* terrane-site: case.trn:52:13-52:38 */))
    );
    unpadded_as_padded_terrane_f0_s2568 = decode_base64(
        String::from("aGVsbG8"),
        false,
        true,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&unpadded_as_padded_terrane_f0_s2568
        .failed)
    );
    unpadded_terrane_f0_s2668 = decode_base64(String::from("aGVsbG8"), false, false);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::decode(&unpadded_terrane_f0_s2668
        .value, terrane_string_support::Encoding::Utf8), 1 /* terrane-site: case.trn:56:13-56:40 */))
    );
    wrong_alphabet_terrane_f0_s2763 = decode_base64(
        String::from("aGVsbG8_"),
        false,
        false,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&wrong_alphabet_terrane_f0_s2763
        .failed)
    );
    malformed_terrane_f0_s2857 = decode_hex(String::from("abc"));
    println!(
        "{}", terrane_scalar_support::scalar_text(&malformed_terrane_f0_s2857.failed)
    );
    options_terrane_f0_s2919 = CompressionOptions::terrane_construct(
        terrane_int_support::Int::from(6_i128),
        true,
    );
    packed_terrane_f0_s2971 = gzip_codec_terrane_f0_s731
        .compress(
            Vec::from([99, 111, 109, 112, 114, 101, 115, 115, 32, 109, 101]),
            options_terrane_f0_s2919.clone(),
        );
    limits_terrane_f0_s3029 = DecompressionLimits::terrane_construct(
        terrane_int_support::Int::from(1024_i128),
        terrane_int_support::Int::from(100_i128),
        terrane_int_support::Int::from(4096_i128),
    );
    unpacked_terrane_f0_s3089 = gzip_codec_terrane_f0_s731
        .decompress(
            packed_terrane_f0_s2971.value.clone(),
            limits_terrane_f0_s3029.clone(),
        );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::decode(&unpacked_terrane_f0_s3089
        .value, terrane_string_support::Encoding::Utf8), 2 /* terrane-site: case.trn:65:13-65:40 */))
    );
    zlib_codec_terrane_f0_s3189 = zlib();
    zlib_packed_terrane_f0_s3212 = zlib_codec_terrane_f0_s3189
        .compress(
            Vec::from([99, 111, 109, 112, 114, 101, 115, 115, 32, 109, 101]),
            options_terrane_f0_s2919.clone(),
        );
    zlib_unpacked_terrane_f0_s3275 = zlib_codec_terrane_f0_s3189
        .decompress(
            zlib_packed_terrane_f0_s3212.value.clone(),
            limits_terrane_f0_s3029.clone(),
        );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::decode(&zlib_unpacked_terrane_f0_s3275
        .value, terrane_string_support::Encoding::Utf8), 3 /* terrane-site: case.trn:69:13-69:45 */))
    );
    raw_codec_terrane_f0_s3390 = deflate_raw();
    raw_packed_terrane_f0_s3419 = raw_codec_terrane_f0_s3390
        .compress(
            Vec::from([99, 111, 109, 112, 114, 101, 115, 115, 32, 109, 101]),
            options_terrane_f0_s2919.clone(),
        );
    raw_unpacked_terrane_f0_s3480 = raw_codec_terrane_f0_s3390
        .decompress(raw_packed_terrane_f0_s3419.value.clone(), limits_terrane_f0_s3029);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::decode(&raw_unpacked_terrane_f0_s3480
        .value, terrane_string_support::Encoding::Utf8), 4 /* terrane-site: case.trn:73:13-73:44 */))
    );
    zstd_codec_terrane_f0_s3591 = zstd();
    zstd_packed_terrane_f0_s3614 = zstd_codec_terrane_f0_s3591
        .compress(
            Vec::from([99, 111, 109, 112, 114, 101, 115, 115, 32, 109, 101]),
            options_terrane_f0_s2919,
        );
    zstd_limits_terrane_f0_s3677 = DecompressionLimits::terrane_construct(
        terrane_int_support::Int::from(1073741824_i128),
        terrane_int_support::Int::from(100_i128),
        terrane_int_support::Int::from(1073741824_i128),
    );
    zstd_unpacked_terrane_f0_s3754 = zstd_codec_terrane_f0_s3591
        .decompress(
            zstd_packed_terrane_f0_s3614.value.clone(),
            zstd_limits_terrane_f0_s3677,
        );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::decode(&zstd_unpacked_terrane_f0_s3754
        .value, terrane_string_support::Encoding::Utf8), 5 /* terrane-site: case.trn:78:13-78:45 */))
    );
    bomb_limits_terrane_f0_s3874 = DecompressionLimits::terrane_construct(
        terrane_int_support::Int::from(4_i128),
        terrane_int_support::Int::from(2_i128),
        terrane_int_support::Int::from(8_i128),
    );
    refused_terrane_f0_s3931 = gzip_codec_terrane_f0_s731
        .decompress(packed_terrane_f0_s2971.value.clone(), bomb_limits_terrane_f0_s3874);
    println!(
        "{}", terrane_scalar_support::scalar_text(&refused_terrane_f0_s3931
        .resource_limit)
    );
    parsed_terrane_f0_s4028 = parse_uuid(
        String::from("01890f3e-7b4d-7cc0-98c8-77e22c318a14"),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&parsed_terrane_f0_s4028.value.string)
    );
    generated_v4_terrane_f0_s4123 = random_uuid(secure_terrane_f0_s1129.clone());
    reparsed_v4_terrane_f0_s4162 = parse_uuid(
        generated_v4_terrane_f0_s4123.value.string.clone(),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(! generated_v4_terrane_f0_s4123
        .failed &&! reparsed_v4_terrane_f0_s4162.failed
        &&terrane_int_support::Int::from(generated_v4_terrane_f0_s4123.value.bytes.len()
        as i128) == terrane_int_support::Int::from(16_i128)))
    );
    generated_v7_terrane_f0_s4322 = time_uuid(
        secure_terrane_f0_s1129.clone(),
        terrane_int_support::Int::from(1700000000000_i128),
    );
    reparsed_v7_terrane_f0_s4374 = parse_uuid(
        generated_v7_terrane_f0_s4322.value.string.clone(),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(! generated_v7_terrane_f0_s4322
        .failed &&! reparsed_v7_terrane_f0_s4374.failed
        &&terrane_int_support::Int::from(generated_v7_terrane_f0_s4322.value.bytes.len()
        as i128) == terrane_int_support::Int::from(16_i128)))
    );
    noncanonical_terrane_f0_s4534 = parse_uuid(
        String::from("01890F3E-7B4D-7CC0-98C8-77E22C318A14"),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&noncanonical_terrane_f0_s4534.failed)
    );
    invalid_time_terrane_f0_s4635 = time_uuid(
        secure_terrane_f0_s1129,
        terrane_int_support::Int::from(-1_i128),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&invalid_time_terrane_f0_s4635.failed)
    );
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
        let raw_terrane_f1_s414: TerranePlatformResult;
        raw_terrane_f1_s414 = terrane_platform_hex_decode(text);
        return DecodeResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s414),
            terrane_platform_result_message(&raw_terrane_f1_s414),
            terrane_platform_result_bytes(&raw_terrane_f1_s414),
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
        let raw_terrane_f1_s864: TerranePlatformResult;
        raw_terrane_f1_s864 = terrane_platform_base64_decode(
            text,
            self.url_safe,
            padded,
        );
        return DecodeResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s864),
            terrane_platform_result_message(&raw_terrane_f1_s864),
            terrane_platform_result_bytes(&raw_terrane_f1_s864),
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
    let raw_terrane_f1_s1463: TerranePlatformResult;
    raw_terrane_f1_s1463 = terrane_platform_hex_decode(text);
    return DecodeResult::terrane_construct(
        terrane_platform_result_failed(&raw_terrane_f1_s1463),
        terrane_platform_result_message(&raw_terrane_f1_s1463),
        terrane_platform_result_bytes(&raw_terrane_f1_s1463),
    );
}
pub fn encode_base64(data: Vec<u8>, url_safe: bool, padded: bool) -> String {
    return terrane_platform_base64_encode(data, url_safe, padded);
}
pub fn decode_base64(text: String, url_safe: bool, padded: bool) -> DecodeResult {
    let raw_terrane_f1_s1814: TerranePlatformResult;
    raw_terrane_f1_s1814 = terrane_platform_base64_decode(text, url_safe, padded);
    return DecodeResult::terrane_construct(
        terrane_platform_result_failed(&raw_terrane_f1_s1814),
        terrane_platform_result_message(&raw_terrane_f1_s1814),
        terrane_platform_result_bytes(&raw_terrane_f1_s1814),
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
        let format_terrane_f2_s1064: String;
        let raw_terrane_f2_s1093: TerranePlatformResult;
        format_terrane_f2_s1064 = self.format.clone();
        raw_terrane_f2_s1093 = terrane_platform_compress(
            format_terrane_f2_s1064,
            data,
            options.level,
            options.deterministic,
        );
        return CompressionResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f2_s1093),
            terrane_platform_result_resource_limit(&raw_terrane_f2_s1093),
            terrane_platform_result_message(&raw_terrane_f2_s1093),
            terrane_platform_result_bytes(&raw_terrane_f2_s1093),
        );
    }
    pub fn decompress(
        &self,
        data: Vec<u8>,
        limits: DecompressionLimits,
    ) -> CompressionResult {
        let format_terrane_f2_s1417: String;
        let raw_terrane_f2_s1446: TerranePlatformResult;
        format_terrane_f2_s1417 = self.format.clone();
        raw_terrane_f2_s1446 = terrane_platform_decompress(
            format_terrane_f2_s1417,
            data,
            limits.max_output,
            limits.max_ratio,
            limits.max_work,
        );
        return CompressionResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f2_s1446),
            terrane_platform_result_resource_limit(&raw_terrane_f2_s1446),
            terrane_platform_result_message(&raw_terrane_f2_s1446),
            terrane_platform_result_bytes(&raw_terrane_f2_s1446),
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
    let raw_terrane_f3_s948: TerranePlatformResult;
    raw_terrane_f3_s948 = terrane_platform_destroy_secret(&secret.handle);
    return SecretOperationResult::terrane_construct(
        terrane_platform_result_failed(&raw_terrane_f3_s948),
        terrane_platform_result_message(&raw_terrane_f3_s948),
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
        let left_terrane_f3_s1409: Vec<u8>;
        if self.algorithm.as_str() != other.algorithm.as_str() {
            return false;
        }
        left_terrane_f3_s1409 = self.value.clone();
        return terrane_platform_constant_time_equal(left_terrane_f3_s1409, other.value);
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
        let left_terrane_f3_s2100: Vec<u8>;
        if self.algorithm.as_str() != other.algorithm.as_str() {
            return false;
        }
        left_terrane_f3_s2100 = self.value.clone();
        return terrane_platform_constant_time_equal(left_terrane_f3_s2100, other.value);
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
        let raw_terrane_f3_s2698: TerranePlatformResult;
        raw_terrane_f3_s2698 = terrane_platform_random_bytes(&self.handle, count);
        return ByteResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f3_s2698),
            terrane_platform_result_message(&raw_terrane_f3_s2698),
            terrane_platform_result_bytes(&raw_terrane_f3_s2698),
        );
    }
    pub fn bounded_int(
        &self,
        upper_exclusive: terrane_int_support::Int,
    ) -> RandomIntResult {
        let raw_terrane_f3_s2932: TerranePlatformResult;
        raw_terrane_f3_s2932 = terrane_platform_random_bounded(
            &self.handle,
            upper_exclusive,
        );
        return RandomIntResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f3_s2932),
            terrane_platform_result_message(&raw_terrane_f3_s2932),
            terrane_platform_result_int(&raw_terrane_f3_s2932),
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
        let raw_terrane_f3_s3620: TerranePlatformResult;
        raw_terrane_f3_s3620 = terrane_platform_random_bytes(&self.handle, count);
        return ByteResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f3_s3620),
            terrane_platform_result_message(&raw_terrane_f3_s3620),
            terrane_platform_result_bytes(&raw_terrane_f3_s3620),
        );
    }
    pub fn bounded_int(
        &self,
        upper_exclusive: terrane_int_support::Int,
    ) -> RandomIntResult {
        let raw_terrane_f3_s3854: TerranePlatformResult;
        raw_terrane_f3_s3854 = terrane_platform_random_bounded(
            &self.handle,
            upper_exclusive,
        );
        return RandomIntResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f3_s3854),
            terrane_platform_result_message(&raw_terrane_f3_s3854),
            terrane_platform_result_int(&raw_terrane_f3_s3854),
        );
    }
    pub fn split(self) -> PseudoRandom {
        let raw_terrane_f3_s4084: TerranePlatformResult;
        let mut child_terrane_f3_s4129: PseudoRandom;
        raw_terrane_f3_s4084 = terrane_platform_random_split(&self.handle);
        child_terrane_f3_s4129 = PseudoRandom::terrane_construct(
            chacha20(),
            Vec::from([]),
        );
        child_terrane_f3_s4129.handle = terrane_platform_result_capability(
            &raw_terrane_f3_s4084,
        );
        return child_terrane_f3_s4129;
    }
}
pub fn split_pseudo(source: PseudoRandom) -> PseudoRandom {
    let raw_terrane_f3_s4313: TerranePlatformResult;
    let mut child_terrane_f3_s4356: PseudoRandom;
    raw_terrane_f3_s4313 = terrane_platform_random_split(&source.handle);
    child_terrane_f3_s4356 = PseudoRandom::terrane_construct(chacha20(), Vec::from([]));
    child_terrane_f3_s4356.handle = terrane_platform_result_capability(
        &raw_terrane_f3_s4313,
    );
    return child_terrane_f3_s4356;
}
pub fn secure_bytes(
    source: SecureRandom,
    count: terrane_int_support::Int,
) -> ByteResult {
    let raw_terrane_f3_s4540: TerranePlatformResult;
    raw_terrane_f3_s4540 = terrane_platform_random_bytes(&source.handle, count);
    return ByteResult::terrane_construct(
        terrane_platform_result_failed(&raw_terrane_f3_s4540),
        terrane_platform_result_message(&raw_terrane_f3_s4540),
        terrane_platform_result_bytes(&raw_terrane_f3_s4540),
    );
}
pub fn pseudo_bytes(
    source: PseudoRandom,
    count: terrane_int_support::Int,
) -> ByteResult {
    let raw_terrane_f3_s4771: TerranePlatformResult;
    raw_terrane_f3_s4771 = terrane_platform_random_bytes(&source.handle, count);
    return ByteResult::terrane_construct(
        terrane_platform_result_failed(&raw_terrane_f3_s4771),
        terrane_platform_result_message(&raw_terrane_f3_s4771),
        terrane_platform_result_bytes(&raw_terrane_f3_s4771),
    );
}
pub fn secure_bounded_int(
    source: SecureRandom,
    upper_exclusive: terrane_int_support::Int,
) -> RandomIntResult {
    let raw_terrane_f3_s5024: TerranePlatformResult;
    raw_terrane_f3_s5024 = terrane_platform_random_bounded(
        &source.handle,
        upper_exclusive,
    );
    return RandomIntResult::terrane_construct(
        terrane_platform_result_failed(&raw_terrane_f3_s5024),
        terrane_platform_result_message(&raw_terrane_f3_s5024),
        terrane_platform_result_int(&raw_terrane_f3_s5024),
    );
}
pub fn pseudo_bounded_int(
    source: PseudoRandom,
    upper_exclusive: terrane_int_support::Int,
) -> RandomIntResult {
    let raw_terrane_f3_s5293: TerranePlatformResult;
    raw_terrane_f3_s5293 = terrane_platform_random_bounded(
        &source.handle,
        upper_exclusive,
    );
    return RandomIntResult::terrane_construct(
        terrane_platform_result_failed(&raw_terrane_f3_s5293),
        terrane_platform_result_message(&raw_terrane_f3_s5293),
        terrane_platform_result_int(&raw_terrane_f3_s5293),
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
    let raw_terrane_f3_s5811: TerranePlatformResult;
    let value_terrane_f3_s5855: DigestValue;
    raw_terrane_f3_s5811 = terrane_platform_digest(&algorithm.name, data);
    value_terrane_f3_s5855 = DigestValue::terrane_construct(
        algorithm.name.clone(),
        terrane_platform_result_bytes(&raw_terrane_f3_s5811),
    );
    return DigestResult::terrane_construct(
        terrane_platform_result_failed(&raw_terrane_f3_s5811),
        terrane_platform_result_message(&raw_terrane_f3_s5811),
        value_terrane_f3_s5855,
    );
}
pub fn sign_hmac(
    algorithm: HashAlgorithm,
    key: SecretBuffer,
    data: Vec<u8>,
) -> SignatureResult {
    let raw_terrane_f3_s6121: TerranePlatformResult;
    let value_terrane_f3_s6175: SignatureValue;
    raw_terrane_f3_s6121 = terrane_platform_hmac(&algorithm.name, &key.handle, data);
    value_terrane_f3_s6175 = SignatureValue::terrane_construct(
        algorithm.name.clone(),
        terrane_platform_result_bytes(&raw_terrane_f3_s6121),
    );
    return SignatureResult::terrane_construct(
        terrane_platform_result_failed(&raw_terrane_f3_s6121),
        terrane_platform_result_message(&raw_terrane_f3_s6121),
        value_terrane_f3_s6175,
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
    let raw_terrane_f4_s545: TerranePlatformResult;
    raw_terrane_f4_s545 = terrane_platform_uuid_parse(text);
    return UuidResult::terrane_construct(
        terrane_platform_result_failed(&raw_terrane_f4_s545),
        terrane_platform_result_message(&raw_terrane_f4_s545),
        Uuid::terrane_construct(
            terrane_platform_result_text(&raw_terrane_f4_s545),
            terrane_platform_result_bytes(&raw_terrane_f4_s545),
        ),
    );
}
pub fn random_uuid(source: SecureRandom) -> UuidResult {
    let raw_terrane_f4_s788: TerranePlatformResult;
    raw_terrane_f4_s788 = terrane_platform_uuid_v4(&source.handle);
    return UuidResult::terrane_construct(
        terrane_platform_result_failed(&raw_terrane_f4_s788),
        terrane_platform_result_message(&raw_terrane_f4_s788),
        Uuid::terrane_construct(
            terrane_platform_result_text(&raw_terrane_f4_s788),
            terrane_platform_result_bytes(&raw_terrane_f4_s788),
        ),
    );
}
pub fn time_uuid(
    source: SecureRandom,
    unix_milliseconds: terrane_int_support::Int,
) -> UuidResult {
    let raw_terrane_f4_s1058: TerranePlatformResult;
    raw_terrane_f4_s1058 = terrane_platform_uuid_v7(&source.handle, unix_milliseconds);
    return UuidResult::terrane_construct(
        terrane_platform_result_failed(&raw_terrane_f4_s1058),
        terrane_platform_result_message(&raw_terrane_f4_s1058),
        Uuid::terrane_construct(
            terrane_platform_result_text(&raw_terrane_f4_s1058),
            terrane_platform_result_bytes(&raw_terrane_f4_s1058),
        ),
    );
}
