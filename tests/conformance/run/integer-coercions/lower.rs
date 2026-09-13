// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: integer-coercions
fn main() {
    let value: i64 = 300;
    let same: terrane_int_support::Int = terrane_int_support::Int::from(value as i128);
    let exact: i8 = __terrane_raised(
        terrane_int_support::coerce::<i8>(&120),
        0 /* terrane-site: case.trn:6:11-6:14 */,
    );
    let wrapped: u8 = terrane_int_support::wrapping_coerce::<
        u8,
    >(
        &(terrane_int_support::Int::from(value as i128)
            + terrane_int_support::Int::from(0_i128)),
    );
    let saturated: u8 = terrane_int_support::saturating_coerce::<u8>(&value);
    println!("{}", terrane_scalar_support::scalar_text(&exact));
    println!("{}", terrane_scalar_support::scalar_text(&wrapped));
    println!("{}", terrane_scalar_support::scalar_text(&saturated));
    println!("{}", terrane_scalar_support::scalar_text(&same));
}
