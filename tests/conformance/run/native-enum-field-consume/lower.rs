// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
pub struct A {
    pub k: NE,
}
impl A {
    pub fn terrane_construct() -> Self {
        Self {
            k: terrane_query_witness::NE::A,
        }
    }
}
fn main() {
    let a: A;
    a = A::terrane_construct();
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(a.k.code()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::NE::code")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 0 /* terrane-site: src/main.trn:7:11-7:20 */))
    );
}
