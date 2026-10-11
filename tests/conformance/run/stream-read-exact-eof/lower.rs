// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: stream-read-exact-eof
fn main() {
    let input: ByteReader;
    let result: ReadResult;
    input = stdin();
    result = input.read_exact(terrane_int_support::Int::from(5_i128));
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::decode(&result
        .data, terrane_string_support::Encoding::Utf8), 0 /* terrane-site: case.trn:6:13-6:37 */))
    );
    println!("{}", terrane_scalar_support::scalar_text(&result.completed));
    println!("{}", terrane_scalar_support::scalar_text(&result.end));
    println!("{}", terrane_scalar_support::scalar_text(&result.failed));
    input.close();
}
