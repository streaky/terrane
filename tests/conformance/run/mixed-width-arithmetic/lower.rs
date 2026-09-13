// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: mixed-width-arithmetic
fn main() {
    let left: i8 = 100;
    let right: i32 = 2;
    let unsigned: u8 = 120;
    let total: i32 = __terrane_raised(
        terrane_int_support::fixed_addition(left as i32, right),
        0 /* terrane-site: case.trn:7:17-7:29 */,
    );
    let combined: i16 = __terrane_raised(
        terrane_int_support::fixed_addition(left as i16, unsigned as i16),
        1 /* terrane-site: case.trn:8:20-8:35 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&total));
    println!("{}", terrane_scalar_support::scalar_text(&combined));
}
