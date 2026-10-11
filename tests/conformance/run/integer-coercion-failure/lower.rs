// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: integer-coercion-failure
fn main() {
    let value: i64;
    let narrow: i8;
    value = 128;
    narrow = __terrane_raised(
        terrane_int_support::coerce::<i8>(&value),
        0 /* terrane-site: case.trn:5:17-5:22 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&narrow));
}
