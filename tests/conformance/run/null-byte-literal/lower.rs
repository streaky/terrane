// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: null-byte-literal
fn main() {
    let raw: Vec<u8>;
    let mut byte: u8;
    raw = Vec::<u8>::from([97, 0, 99]);
    {
        let __terrane_iterable_0 = &raw;
        let mut __terrane_iterator_0 = __terrane_iterable_0.iter().copied();
        loop {
            byte = match __terrane_iterator_0.next() {
                Some(item) => item,
                None => break,
            };
            println!("{}", terrane_scalar_support::scalar_text(&byte));
        }
    }
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(raw.len() as i128)),
        terrane_scalar_support::scalar_text(&{ let __terrane_string_argument_0 =
        terrane_string_support::Encoding::Utf8;
        __terrane_raised(terrane_string_support::decode(&raw,
        __terrane_string_argument_0), 0 /* terrane-site: case.trn:6:23-6:39 */) })
    );
}
