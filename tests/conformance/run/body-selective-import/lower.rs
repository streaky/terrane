// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: body-selective-import
fn encoded() -> String {
    return encode_base64(Vec::<u8>::from([97, 98, 99]), false, true);
}
fn main() {
    println!("{}", terrane_scalar_support::scalar_text(&encoded()));
}
