// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: bytes-and-legacy-digests
fn main() {
    let octets: terrane_collection_support::List<u8>;
    let data: Vec<u8>;
    let empty_octets: terrane_collection_support::List<u8>;
    let empty: Vec<u8>;
    let first: DigestResult;
    let second: DigestResult;
    octets = terrane_collection_support::List::<u8>::new(vec![65, 66, 67]);
    data = bytes_from_octets(octets);
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_argument_0 =
        terrane_string_support::Encoding::Utf8;
        __terrane_raised(terrane_string_support::decode(&data,
        __terrane_string_argument_0), 0 /* terrane-site: case.trn:12:11-12:28 */) }),
        terrane_scalar_support::scalar_text(&(data.len() as i128))
    );
    empty_octets = terrane_collection_support::List::<u8>::new(vec![]);
    empty = bytes_from_octets(empty_octets);
    println!("{}", terrane_scalar_support::scalar_text(&(empty.len() as i128)));
    first = digest_bytes_terrane_core_random(sha1(), Vec::<u8>::from([97, 98, 99]));
    println!(
        "{}", terrane_scalar_support::scalar_text(&encode_hex(first.value.value.clone()))
    );
    second = digest_bytes_terrane_core_random(md5(), Vec::<u8>::from([97, 98, 99]));
    println!(
        "{}", terrane_scalar_support::scalar_text(&encode_hex(second.value.value
        .clone()))
    );
}
