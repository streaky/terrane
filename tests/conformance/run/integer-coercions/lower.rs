// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: integer-coercions
fn main() {
    let value: i64;
    let same: terrane_int_support::Int;
    let exact: i8;
    let wrapped: u8;
    let saturated: u8;
    value = 300;
    same = terrane_int_support::Int::from(value as i128);
    exact = __terrane_raised(
        terrane_int_support::coerce::<i8>(&120),
        0 /* terrane-site: case.trn:6:11-6:14 */,
    );
    wrapped = terrane_int_support::wrapping_coerce::<
        u8,
    >(
        &(terrane_int_support::Int::from(value as i128)
            + terrane_int_support::Int::from(0_i128)),
    );
    saturated = terrane_int_support::saturating_coerce::<u8>(&value);
    println!("{}", terrane_scalar_support::scalar_text(&exact));
    println!("{}", terrane_scalar_support::scalar_text(&wrapped));
    println!("{}", terrane_scalar_support::scalar_text(&saturated));
    println!("{}", terrane_scalar_support::scalar_text(&same));
}
