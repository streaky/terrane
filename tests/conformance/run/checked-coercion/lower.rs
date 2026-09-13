// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: checked-coercion
static __TERRANE_F0_SHARED: std::sync::LazyLock<terrane_int_support::Int> = std::sync::LazyLock::new(||
terrane_int_support::Int::from(100_i128));
fn convert(item: terrane_int_support::Int) -> Result<i8, TerraneError> {
    let mut result: i8 = 0;
    if item.clone() > terrane_int_support::Int::from(0_i128) {
        result = __terrane_raised_err(
            terrane_int_support::coerce::<i8>(&item),
            0 /* terrane-site: case.trn:9:14-9:18 */,
        )?;
    }
    return Ok(result);
}
fn helper() {
    println!("{}", terrane_scalar_support::scalar_text(&String::from("helper")));
}
fn main() {
    helper();
    let value: i64 = 300;
    let within: i64 = 100;
    let coerced: i8 = __terrane_raised(
        terrane_int_support::coerce::<i8>(&within),
        1 /* terrane-site: case.trn:19:13-19:19 */,
    );
    let renamed_checked: Option<i8> = terrane_int_support::checked_coerce::<i8>(&within);
    let shared_coerced: i8 = __terrane_raised(
        terrane_int_support::coerce::<i8>(&*__TERRANE_F0_SHARED),
        2 /* terrane-site: case.trn:21:20-21:33 */,
    );
    let parameter_coerced: i8 = __terrane_traced(
        convert(terrane_int_support::Int::from(within as i128)),
        3 /* terrane-site: case.trn:22:23-22:38 */,
    );
    let checked: Option<i8> = terrane_int_support::checked_coerce::<i8>(&value);
    let absent: bool = checked.is_none();
    let present: bool = checked.is_some();
    let shadow_safe: i8 = terrane_int_support::saturating_coerce::<i8>(&value);
    println!("{}", terrane_scalar_support::scalar_text(&coerced));
    println!("{}", terrane_scalar_support::scalar_text(&renamed_checked.is_some()));
    println!("{}", terrane_scalar_support::scalar_text(&shared_coerced));
    println!("{}", terrane_scalar_support::scalar_text(&parameter_coerced));
    println!("{}", terrane_scalar_support::scalar_text(&absent));
    println!("{}", terrane_scalar_support::scalar_text(&present));
    println!("{}", terrane_scalar_support::scalar_text(&shadow_safe));
}
