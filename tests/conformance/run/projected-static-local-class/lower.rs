// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct BytesMut {}
impl BytesMut {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn terrane_static_from_iter() -> terrane_int_support::Int {
        return terrane_int_support::Int::from(7_i128);
    }
}
fn main() {
    println!(
        "{}", terrane_scalar_support::scalar_text(&BytesMut::terrane_static_from_iter())
    );
}
