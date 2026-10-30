// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: codec-class-methods
fn main() {
    let hexadecimal: HexCodec;
    let encoded_hex: String;
    let decoded_hex: DecodeResult;
    let standard_base64: Base64Codec;
    let encoded_base64: String;
    let decoded_base64: DecodeResult;
    hexadecimal = hex();
    encoded_hex = hexadecimal.encode(Vec::<u8>::from([97, 98, 99]));
    decoded_hex = hexadecimal.decode(encoded_hex);
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_argument_0 =
        terrane_string_support::Encoding::Utf8;
        __terrane_raised(terrane_string_support::decode(&decoded_hex.value,
        __terrane_string_argument_0), 0 /* terrane-site: case.trn:8:13-8:43 */) })
    );
    standard_base64 = base64();
    encoded_base64 = standard_base64.encode(Vec::<u8>::from([97, 98, 99]), true);
    decoded_base64 = standard_base64.decode(encoded_base64, true);
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_argument_0 =
        terrane_string_support::Encoding::Utf8;
        __terrane_raised(terrane_string_support::decode(&decoded_base64.value,
        __terrane_string_argument_0), 1 /* terrane-site: case.trn:13:13-13:46 */) })
    );
}
