// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: mixed-width-arithmetic
fn main() {
    let left: i8;
    let right: i32;
    let unsigned: u8;
    let total: i32;
    let combined: i16;
    left = 100;
    right = 2;
    unsigned = 120;
    total = __terrane_raised(
        terrane_int_support::fixed_addition(left as i32, right),
        0 /* terrane-site: case.trn:7:17-7:29 */,
    );
    combined = __terrane_raised(
        terrane_int_support::fixed_addition(left as i16, unsigned as i16),
        1 /* terrane-site: case.trn:8:20-8:35 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&total));
    println!("{}", terrane_scalar_support::scalar_text(&combined));
}
