// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: decode-error-offset
fn main() {
    let invalid: Vec<u8>;
    invalid = Vec::<u8>::from([97, 255]);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::decode(&invalid,
        terrane_string_support::Encoding::Utf8), 0 /* terrane-site: case.trn:4:11-4:31 */))
    );
}
