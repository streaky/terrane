// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: bytes-views-encoding
fn main() {
    let text: String;
    let mut byte_count: terrane_int_support::Int;
    let mut byte: u8;
    let mut scalar_count: terrane_int_support::Int;
    let mut scalar: String;
    let mut grapheme_count: terrane_int_support::Int;
    let mut grapheme: String;
    let encoded: Vec<u8>;
    let decoded: String;
    let utf16le_text: String;
    let utf16be_text: String;
    let utf32le_text: String;
    let utf32be_text: String;
    text = String::from("e\u{301}");
    println!(
        "{}{}{}{}",
        terrane_scalar_support::scalar_text(&(terrane_string_support::length(&text) as
        i128)), terrane_scalar_support::scalar_text(&(text.len() as i128)),
        terrane_scalar_support::scalar_text(&(text.chars().count() as i128)),
        terrane_scalar_support::scalar_text(&(terrane_string_support::length(&text) as
        i128))
    );
    byte_count = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_iterable_0 = &text;
        let mut __terrane_iterator_0 = __terrane_iterable_0.bytes();
        loop {
            byte = match __terrane_iterator_0.next() {
                Some(item) => item,
                None => break,
            };
            if byte == byte {
                byte_count = byte_count.clone() + terrane_int_support::Int::from(1_i128);
            }
        }
    }
    scalar_count = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_iterable_1 = &text;
        let mut __terrane_iterator_1 = __terrane_iterable_1
            .chars()
            .map(|value| value.to_string());
        loop {
            scalar = match __terrane_iterator_1.next() {
                Some(item) => item,
                None => break,
            };
            if scalar.as_str() == scalar.as_str() {
                scalar_count = scalar_count.clone()
                    + terrane_int_support::Int::from(1_i128);
            }
        }
    }
    grapheme_count = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_iterable_2 = &text;
        let mut __terrane_iterator_2 = terrane_string_support::graphemes(
            &__terrane_iterable_2,
        );
        loop {
            grapheme = match __terrane_iterator_2.next() {
                Some(item) => item,
                None => break,
            };
            if grapheme.as_str() == grapheme.as_str() {
                grapheme_count = grapheme_count.clone()
                    + terrane_int_support::Int::from(1_i128);
            }
        }
    }
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&byte_count),
        terrane_scalar_support::scalar_text(&scalar_count),
        terrane_scalar_support::scalar_text(&grapheme_count)
    );
    encoded = {
        let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf8;
        terrane_string_support::encode(&text, __terrane_string_argument_0)
    };
    decoded = {
        let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf8;
        __terrane_raised(
            terrane_string_support::decode(&encoded, __terrane_string_argument_0),
            0 /* terrane-site: case.trn:19:20-19:40 */,
        )
    };
    println!("{}", terrane_scalar_support::scalar_text(&decoded));
    utf16le_text = {
        let __terrane_receiver_value_0 = &{
            let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf16Le;
            terrane_string_support::encode(&text, __terrane_string_argument_0)
        };
        let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf16Le;
        __terrane_raised(
            terrane_string_support::decode(
                &__terrane_receiver_value_0,
                __terrane_string_argument_0,
            ),
            1 /* terrane-site: case.trn:21:25-21:65 */,
        )
    };
    utf16be_text = {
        let __terrane_receiver_value_1 = &{
            let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf16Be;
            terrane_string_support::encode(&text, __terrane_string_argument_0)
        };
        let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf16Be;
        __terrane_raised(
            terrane_string_support::decode(
                &__terrane_receiver_value_1,
                __terrane_string_argument_0,
            ),
            2 /* terrane-site: case.trn:22:25-22:65 */,
        )
    };
    utf32le_text = {
        let __terrane_receiver_value_2 = &{
            let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf32Le;
            terrane_string_support::encode(&text, __terrane_string_argument_0)
        };
        let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf32Le;
        __terrane_raised(
            terrane_string_support::decode(
                &__terrane_receiver_value_2,
                __terrane_string_argument_0,
            ),
            3 /* terrane-site: case.trn:23:25-23:65 */,
        )
    };
    utf32be_text = {
        let __terrane_receiver_value_3 = &{
            let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf32Be;
            terrane_string_support::encode(&text, __terrane_string_argument_0)
        };
        let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf32Be;
        __terrane_raised(
            terrane_string_support::decode(
                &__terrane_receiver_value_3,
                __terrane_string_argument_0,
            ),
            4 /* terrane-site: case.trn:24:25-24:65 */,
        )
    };
    println!("{}", terrane_scalar_support::scalar_text(&utf16le_text));
    println!("{}", terrane_scalar_support::scalar_text(&utf16be_text));
    println!("{}", terrane_scalar_support::scalar_text(&utf32le_text));
    println!("{}", terrane_scalar_support::scalar_text(&utf32be_text));
}
