// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: stream-zero-length-read
fn main() {
    let input: ByteReader;
    let zero: ReadResult;
    let rest: ReadResult;
    input = stdin();
    zero = input.read(terrane_int_support::Int::from(0_i128));
    println!("{}", terrane_scalar_support::scalar_text(&zero.completed));
    println!("{}", terrane_scalar_support::scalar_text(&zero.end));
    rest = input.read_exact(terrane_int_support::Int::from(3_i128));
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_argument_0 =
        terrane_string_support::Encoding::Utf8;
        __terrane_raised(terrane_string_support::decode(&rest.data,
        __terrane_string_argument_0), 0 /* terrane-site: case.trn:9:13-9:35 */) })
    );
    input.close();
}
