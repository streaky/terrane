// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: later-coercion-alias-argument
fn main() {
    let value: i64;
    value = 100;
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::coerce::
        < i8 > (&value), 0 /* terrane-site: case.trn:5:11-5:16 */))
    );
}
