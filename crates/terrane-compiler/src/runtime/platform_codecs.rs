pub fn terrane_platform_hex_encode(data: Vec<u8>) -> String { terrane_platform_support::hex_encode(&data) }
pub fn terrane_platform_hex_decode(text: String) -> TerranePlatformResult { terrane_platform_support::hex_decode(&text) }
pub fn terrane_platform_base64_encode(data: Vec<u8>, url_safe: bool, padded: bool) -> String { terrane_platform_support::base64_encode(&data, url_safe, padded) }
pub fn terrane_platform_base64_decode(text: String, url_safe: bool, padded: bool) -> TerranePlatformResult { terrane_platform_support::base64_decode(&text, url_safe, padded) }
