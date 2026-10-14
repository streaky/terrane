// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: bytes-variadic-concat
fn main() {
    let combined: Vec<u8>;
    let unchanged: Vec<u8>;
    combined = {
        let mut bytes = Vec::<u8>::from([97]);
        let part_0: Vec<u8> = Vec::<u8>::from([]);
        let part_1: Vec<u8> = Vec::<u8>::from([98, 99]);
        let part_2: Vec<u8> = Vec::<u8>::from([100]);
        let additional = match [part_0.len(), part_1.len(), part_2.len()]
            .into_iter()
            .try_fold(0usize, usize::checked_add)
        {
            Some(length) => length,
            None => std::process::abort(),
        };
        if bytes.try_reserve(additional).is_err() {
            std::process::abort();
        }
        bytes.extend(part_0);
        bytes.extend(part_1);
        bytes.extend(part_2);
        bytes
    };
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::decode(&combined,
        terrane_string_support::Encoding::Utf8), 0 /* terrane-site: case.trn:7:11-7:32 */)), terrane_scalar_support::scalar_text(&(combined.len() as
        i128))
    );
    unchanged = {
        let mut bytes = Vec::<u8>::from([122]);
        let additional = match [].into_iter().try_fold(0usize, usize::checked_add) {
            Some(length) => length,
            None => std::process::abort(),
        };
        if bytes.try_reserve(additional).is_err() {
            std::process::abort();
        }
        bytes
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::decode(&unchanged,
        terrane_string_support::Encoding::Utf8), 1 /* terrane-site: case.trn:9:11-9:33 */))
    );
}
