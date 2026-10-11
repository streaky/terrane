// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: fixed-overflow
fn main() {
    let left: i8;
    let right: i8;
    left = 120;
    right = 10;
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_addition(left,
        right), 0 /* terrane-site: case.trn:5:11-5:23 */))
    );
}
