// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: null-byte-literal
fn main() {
    let raw: Vec<u8>;
    let mut byte: u8;
    raw = Vec::<u8>::from([97, 0, 99]);
    let __terrane_iterable_0 = raw.clone();
    let mut __terrane_iterator_0 = terrane_collection_support::bytes_iterator(
        &__terrane_iterable_0,
    );
    loop {
        byte = match __terrane_iterator_0.next() {
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
