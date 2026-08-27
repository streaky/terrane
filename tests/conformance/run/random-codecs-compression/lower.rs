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
type TerranePlatformCapability = terrane_platform_support::Capability;
type TerranePlatformResult = terrane_platform_support::ResultValue;
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
    result.capability.clone().expect("successful platform result carries capability")
}
fn terrane_platform_secure_random() -> TerranePlatformCapability {
    terrane_platform_support::secure_random()
}
fn terrane_platform_pseudo_random(seed: Vec<u8>) -> TerranePlatformCapability {
    terrane_platform_support::pseudo_random(&seed)
}
fn terrane_platform_secret_buffer(data: Vec<u8>) -> TerranePlatformCapability {
    terrane_platform_support::secret_buffer(data)
}
fn terrane_platform_random_bytes(
    source: &TerranePlatformCapability,
    count: terrane_int_support::Int,
) -> TerranePlatformResult {
    terrane_platform_support::random_bytes(
        source,
        count.as_big().to_string().parse::<i128>().unwrap_or(-1),
    )
}
fn terrane_platform_random_bounded(
    source: &TerranePlatformCapability,
    upper: terrane_int_support::Int,
) -> TerranePlatformResult {
    terrane_platform_support::random_bounded(
        source,
        upper.as_big().to_string().parse::<i128>().unwrap_or(-1),
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
        level.as_big().to_string().parse::<i128>().unwrap_or(6),
        deterministic,
    )
}
fn terrane_platform_decompress(
    format: String,
    data: Vec<u8>,
    output: terrane_int_support::Int,
    ratio: terrane_int_support::Int,
    nesting: terrane_int_support::Int,
    work: terrane_int_support::Int,
) -> TerranePlatformResult {
    terrane_platform_support::decompress(
        &format,
        &data,
        output.as_big().to_string().parse::<i128>().unwrap_or(-1),
        ratio.as_big().to_string().parse::<i128>().unwrap_or(-1),
        nesting.as_big().to_string().parse::<i128>().unwrap_or(-1),
        work.as_big().to_string().parse::<i128>().unwrap_or(-1),
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
fn terrane_platform_uuid_v7() -> TerranePlatformResult {
    terrane_platform_support::uuid_v7()
}
// Source: case.trn
// Namespace: app
fn main() {
    let first: PseudoRandom = PseudoRandom::terrane_construct(
        Vec::from([115, 101, 101, 100]),
    );
    let second: PseudoRandom = PseudoRandom::terrane_construct(
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
    let digest: DigestValue = digest_bytes(hash.clone(), Vec::from([97, 98, 99]));
    println!(
        "{}", terrane_scalar_support::scalar_text(&encode_hex(digest.value.clone()))
    );
    let key: SecretBuffer = SecretBuffer::terrane_construct(Vec::from([107, 101, 121]));
    let mac: DigestValue = sign_hmac(
        hash.clone(),
        key.clone(),
        Vec::from([100, 97, 116, 97]),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&digest_equals(mac.clone(), mac
        .clone()))
    );
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
        "{}", terrane_scalar_support::scalar_text(&terrane_string_support::decode(&strict
        .value, terrane_string_support::Encoding::Utf8).unwrap_or_else(| error |
        __terrane_uncaught(TerraneError::from(error).at("/app::main (case.trn:26:13)"))))
    );
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
        terrane_int_support::Int::from(1_i128),
        terrane_int_support::Int::from(4096_i128),
    );
    let unpacked: CompressionResult = gzip_codec
        .decompress(packed.value.clone(), limits.clone());
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_string_support::decode(&unpacked
        .value, terrane_string_support::Encoding::Utf8).unwrap_or_else(| error |
        __terrane_uncaught(TerraneError::from(error).at("/app::main (case.trn:33:13)"))))
    );
    let bomb_limits: DecompressionLimits = DecompressionLimits::terrane_construct(
        terrane_int_support::Int::from(4_i128),
        terrane_int_support::Int::from(2_i128),
        terrane_int_support::Int::from(1_i128),
        terrane_int_support::Int::from(8_i128),
    );
    let refused: CompressionResult = gzip_codec
        .decompress(packed.value.clone(), bomb_limits.clone());
    println!("{}", terrane_scalar_support::scalar_text(&refused.resource_limit));
    let parsed: UuidResult = parse_uuid(
        String::from("01890f3e-7b4d-7cc0-98c8-77e22c318a14"),
    );
    println!("{}", terrane_scalar_support::scalar_text(&parsed.value.string));
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
    pub max_nesting: terrane_int_support::Int,
    pub max_work: terrane_int_support::Int,
}
impl DecompressionLimits {
    pub fn terrane_construct(
        max_output: terrane_int_support::Int,
        max_ratio: terrane_int_support::Int,
        max_nesting: terrane_int_support::Int,
        max_work: terrane_int_support::Int,
    ) -> Self {
        let mut value = Self {
            max_output: terrane_int_support::Int::from(16777216_i128),
            max_ratio: terrane_int_support::Int::from(100_i128),
            max_nesting: terrane_int_support::Int::from(1_i128),
            max_work: terrane_int_support::Int::from(67108864_i128),
        };
        value.construct(max_output, max_ratio, max_nesting, max_work);
        value
    }
    pub fn construct(
        &mut self,
        max_output: terrane_int_support::Int,
        max_ratio: terrane_int_support::Int,
        max_nesting: terrane_int_support::Int,
        max_work: terrane_int_support::Int,
    ) {
        self.max_output = max_output.clone();
        self.max_ratio = max_ratio.clone();
        self.max_nesting = max_nesting.clone();
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
            limits.max_nesting,
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
pub struct PseudoRandom {
    pub handle: TerranePlatformCapability,
}
impl PseudoRandom {
    pub fn terrane_construct(seed: Vec<u8>) -> Self {
        let mut value = Self {
            handle: terrane_platform_pseudo_random(Vec::from([])),
        };
        value.construct(seed);
        value
    }
    pub fn construct(&mut self, seed: Vec<u8>) {
        self.handle = terrane_platform_pseudo_random(seed);
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
        let mut child: PseudoRandom = PseudoRandom::terrane_construct(Vec::from([]));
        child.handle = terrane_platform_result_capability(&raw);
        return child.clone();
    }
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
pub fn digest_bytes(algorithm: HashAlgorithm, data: Vec<u8>) -> DigestValue {
    let raw: TerranePlatformResult = terrane_platform_digest(&algorithm.name, data);
    return DigestValue::terrane_construct(
        algorithm.name.clone(),
        terrane_platform_result_bytes(&raw),
    );
}
pub fn sign_hmac(
    algorithm: HashAlgorithm,
    key: SecretBuffer,
    data: Vec<u8>,
) -> DigestValue {
    let raw: TerranePlatformResult = terrane_platform_hmac(
        &algorithm.name,
        &key.handle,
        data,
    );
    return DigestValue::terrane_construct(
        algorithm.name.clone(),
        terrane_platform_result_bytes(&raw),
    );
}
pub fn digest_equals(left: DigestValue, right: DigestValue) -> bool {
    if left.algorithm != right.algorithm {
        return false;
    }
    return terrane_platform_constant_time_equal(left.value, right.value);
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
pub fn time_uuid() -> UuidResult {
    let raw: TerranePlatformResult = terrane_platform_uuid_v7();
    return UuidResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        Uuid::terrane_construct(
            terrane_platform_result_text(&raw),
            terrane_platform_result_bytes(&raw),
        ),
    );
}
