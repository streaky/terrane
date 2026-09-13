// Generated deterministically by Terrane <version>.
// Runtime support: platform_capability_types.rs, platform_result_type.rs, platform_capability_base.rs, platform_codecs.rs
// Vendored support crates: terrane-int-support, terrane-scalar-support, terrane-platform-support
// Source: app/main.trn
// Namespace: app
fn local_choice() -> terrane_int_support::Int {
    println!(
        "{}", terrane_scalar_support::scalar_text(&local_value_terrane_local_one())
    );
    return local_value_terrane_local_two();
}
fn main() {
    println!("{}", terrane_scalar_support::scalar_text(&root_value_terrane_root_one()));
    println!("{}", terrane_scalar_support::scalar_text(&root_value_terrane_root_two()));
    println!("{}", terrane_scalar_support::scalar_text(&local_choice()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&encode_base64_terrane_codec_shadow())
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&encode_base64_terrane_core_codecs(Vec::from([97]),
        false, true))
    );
}
// Source: codec-shadow/value.trn
// Namespace: codec-shadow
fn encode_base64_terrane_codec_shadow() -> String {
    return String::from("shadow");
}
// Source: local-one/value.trn
// Namespace: local-one
fn local_value_terrane_local_one() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(3_i128);
}
// Source: local-two/value.trn
// Namespace: local-two
fn local_value_terrane_local_two() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(4_i128);
}
// Source: root-one/value.trn
// Namespace: root-one
fn root_value_terrane_root_one() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(1_i128);
}
// Source: root-two/value.trn
// Namespace: root-two
fn root_value_terrane_root_two() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(2_i128);
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
pub fn encode_base64_terrane_core_codecs(
    data: Vec<u8>,
    url_safe: bool,
    padded: bool,
) -> String {
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
