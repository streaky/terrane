// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: bytes-views-encoding
fn main() {
    let text: String = String::from("e\u{301}");
    println!(
        "{}{}{}{}",
        terrane_scalar_support::scalar_text(&(terrane_string_support::length(&text) as
        i128)), terrane_scalar_support::scalar_text(&(text.len() as i128)),
        terrane_scalar_support::scalar_text(&(text.chars().count() as i128)),
        terrane_scalar_support::scalar_text(&(terrane_string_support::length(&text) as
        i128))
    );
    let mut byte_count: terrane_int_support::Int = terrane_int_support::Int::from(
        0_i128,
    );
    let mut __terrane_iterator_0 = terrane_collection_support::bytes_iterator(
        &text.as_bytes().to_vec(),
    );
    loop {
        let byte = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        if byte == byte {
            byte_count = byte_count.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    let mut scalar_count: terrane_int_support::Int = terrane_int_support::Int::from(
        0_i128,
    );
    let mut __terrane_iterator_1 = terrane_collection_support::Iterator::new(
        text.chars().map(|value| value.to_string()).collect::<Vec<_>>(),
    );
    loop {
        let scalar = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        if scalar == scalar {
            scalar_count = scalar_count.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    let mut grapheme_count: terrane_int_support::Int = terrane_int_support::Int::from(
        0_i128,
    );
    let mut __terrane_iterator_2 = terrane_collection_support::Iterator::new(
        terrane_string_support::graphemes(&text).collect::<Vec<_>>(),
    );
    loop {
        let grapheme = match __terrane_iterator_2.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        if grapheme == grapheme {
            grapheme_count = grapheme_count.clone()
                + terrane_int_support::Int::from(1_i128);
        }
    }
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&byte_count),
        terrane_scalar_support::scalar_text(&scalar_count),
        terrane_scalar_support::scalar_text(&grapheme_count)
    );
    let encoded: Vec<u8> = terrane_string_support::encode(
        &text,
        terrane_string_support::Encoding::Utf8,
    );
    let decoded: String = __terrane_raised(
        terrane_string_support::decode(&encoded, terrane_string_support::Encoding::Utf8),
        0 /* terrane-site: case.trn:19:20-19:40 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&decoded));
    let utf16le_text: String = __terrane_raised(
        terrane_string_support::decode(
            &terrane_string_support::encode(
                &text,
                terrane_string_support::Encoding::Utf16Le,
            ),
            terrane_string_support::Encoding::Utf16Le,
        ),
        1 /* terrane-site: case.trn:21:25-21:65 */,
    );
    let utf16be_text: String = __terrane_raised(
        terrane_string_support::decode(
            &terrane_string_support::encode(
                &text,
                terrane_string_support::Encoding::Utf16Be,
            ),
            terrane_string_support::Encoding::Utf16Be,
        ),
        2 /* terrane-site: case.trn:22:25-22:65 */,
    );
    let utf32le_text: String = __terrane_raised(
        terrane_string_support::decode(
            &terrane_string_support::encode(
                &text,
                terrane_string_support::Encoding::Utf32Le,
            ),
            terrane_string_support::Encoding::Utf32Le,
        ),
        3 /* terrane-site: case.trn:23:25-23:65 */,
    );
    let utf32be_text: String = __terrane_raised(
        terrane_string_support::decode(
            &terrane_string_support::encode(
                &text,
                terrane_string_support::Encoding::Utf32Be,
            ),
            terrane_string_support::Encoding::Utf32Be,
        ),
        4 /* terrane-site: case.trn:24:25-24:65 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&utf16le_text));
    println!("{}", terrane_scalar_support::scalar_text(&utf16be_text));
    println!("{}", terrane_scalar_support::scalar_text(&utf32le_text));
    println!("{}", terrane_scalar_support::scalar_text(&utf32be_text));
}
