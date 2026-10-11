// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: descriptor-constructs
fn main() {
    let value: i64;
    value = 1;
    println!("{}", terrane_scalar_support::scalar_text(&{ let _ = &value; true }));
    println!("{}", terrane_scalar_support::scalar_text(&true));
    println!("{}", terrane_scalar_support::scalar_text(&true));
    println!("{}", terrane_scalar_support::scalar_text(&{ let _ = value; true }));
}
