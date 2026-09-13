// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: null-byte-literal
fn main() {
    let raw: Vec<u8> = Vec::from([97, 0, 99]);
    let mut __terrane_iterator_0 = terrane_collection_support::bytes_iterator(&raw);
    loop {
        let byte = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&byte));
    }
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(raw.len() as i128)),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::decode(&raw,
        terrane_string_support::Encoding::Utf8), 0 /* terrane-site: case.trn:6:23-6:39 */))
    );
}
