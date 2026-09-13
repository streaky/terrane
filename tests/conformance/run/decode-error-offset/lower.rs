// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: decode-error-offset
fn main() {
    let invalid: Vec<u8> = Vec::from([97, 255]);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::decode(&invalid,
        terrane_string_support::Encoding::Utf8), 0 /* terrane-site: case.trn:4:11-4:31 */))
    );
}
